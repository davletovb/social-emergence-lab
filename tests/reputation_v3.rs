//! No long-running trial in CI: deterministic V3 design and exact reference checks.
use hiyal::rng::Streams;
use social_emergence_lab::{
    reputation::{Arm, Removal, flow, setup},
    reputation_reference,
    reputation_v3::{
        DECISION_CRITICAL_T, DECISION_MARGIN, FIRST_FEASIBILITY_SEED, MAIN_GRID_PARENTS_PER_CELL,
        Mechanism, PERIODS, SMOKE_PARENTS_PER_CELL, cell_config, cell_design, main_grid_plan,
        main_grid_seed, uniform_edge_survival, uniform_half_life_events,
    },
};

#[test]
fn exactly_nine_distinct_declared_cells_with_independent_feasibility_seeds() {
    let mut cells = Vec::new();
    for period in PERIODS {
        for mode in Mechanism::ALL {
            let c = cell_config(20, 4, period, mode).unwrap();
            assert_eq!((c.beta, c.delta), (4.0, 0.2));
            assert_eq!(c.rewire_every, period);
            assert_eq!(
                (c.removal, c.kappa),
                match mode {
                    Mechanism::RandomTurnover => (Removal::Uniform, 0.0),
                    Mechanism::BeliefDeletion => (Removal::LowestBelief, 0.0),
                    Mechanism::FullFeedback => (Removal::LowestBelief, 6.0),
                }
            );
            cells.push((period, mode.name()));
        }
    }
    assert_eq!(cells.len(), 9);
    for (offset, _) in cells.iter().enumerate() {
        let seed = FIRST_FEASIBILITY_SEED + offset as u64;
        assert!((120_200..=120_208).contains(&seed));
        assert!(!(98_000..=98_035).contains(&seed));
    }
    assert!(cell_config(20, 4, 0, Mechanism::FullFeedback).is_err());
    assert!(cell_config(20, 4, 50, Mechanism::FullFeedback).is_err());
    assert!(Mechanism::parse("unknown").is_err());
}

#[test]
fn null_half_lives_are_predictions_only_for_uniform_removal() {
    let values = PERIODS.map(|m| uniform_half_life_events(m, 80));
    assert!((values[0] - 110.2).abs() < 1.0);
    assert!((values[1] - 551.0).abs() < 5.0);
    assert!((values[2] - 1102.0).abs() < 10.0);
    assert!(values[0] < values[1] && values[1] < values[2]);
    for m in PERIODS {
        assert_eq!(uniform_edge_survival(0, m, 80), 1.0);
        assert!(uniform_edge_survival(400, m, 80) < 1.0);
    }
}

#[test]
fn information_corrected_cell_expectations_match_independent_reference() {
    for (p, period) in PERIODS.into_iter().enumerate() {
        for (k, mechanism) in Mechanism::ALL.into_iter().enumerate() {
            let config = cell_config(10, 2, period, mechanism).unwrap();
            let mut streams =
                Streams::new("v3-grid-reference".into(), 121_000 + (3 * p + k) as u64);
            let mut world = setup(&config, &mut streams).unwrap();
            world.intervene(Arm::Information).unwrap();
            reputation_reference::check(&world, 1e-9).unwrap();
            let f = flow(&world).unwrap();
            assert!(f.selection.abs() < 1e-12);
            assert!(f.added.abs() < 1e-12);
            assert!((f.total - f.inherited).abs() < 1e-12);
        }
    }

    // Exact frozen graph beta=0 null uses expectation, never sampled outcomes.
    let mut config = cell_config(10, 2, 10, Mechanism::RandomTurnover).unwrap();
    config.beta = 0.0;
    config.removal = Removal::None;
    let mut rng = Streams::new("v3-beta-zero-null".into(), 121_099);
    let mut frozen = setup(&config, &mut rng).unwrap();
    frozen.intervene(Arm::Information).unwrap();
    reputation_reference::check(&frozen, 1e-9).unwrap();
    let q = flow(&frozen).unwrap();
    assert!(q.selection.abs() < 1e-12);
    assert!(q.added.abs() < 1e-12);
    assert!((q.total - q.inherited).abs() < 1e-12);
}

#[test]
fn main_grid_seeds_are_fresh_and_distinct() {
    // Every seed range used by earlier Experiment 002 evidence, review checks
    // or unit tests; the frozen main grid must not reuse any of them.
    let used = [
        62_000..=62_000,
        81_000..=83_003,
        90_000..=92_551,
        94_000..=98_035,
        120_200..=120_208,
        121_000..=121_099,
    ];
    let mut seeds = Vec::new();
    for cell in 0..9 {
        for index in 0..MAIN_GRID_PARENTS_PER_CELL {
            let seed = main_grid_seed(cell, index).unwrap();
            assert!(used.iter().all(|range| !range.contains(&seed)), "{seed}");
            seeds.push(seed);
        }
    }
    seeds.sort_unstable();
    seeds.dedup();
    assert_eq!(seeds.len(), 9 * MAIN_GRID_PARENTS_PER_CELL);
    assert!(main_grid_seed(9, 0).is_err());
    assert!(main_grid_seed(0, MAIN_GRID_PARENTS_PER_CELL).is_err());
    assert_eq!((DECISION_MARGIN, DECISION_CRITICAL_T), (0.05, 2.011741));
}

#[test]
fn stage_a_is_the_full_feedback_turnover_cells_and_plans_match_the_seeds() {
    let stage_a: Vec<_> = (0..9)
        .filter(|&cell| cell_design(cell).unwrap().1 == Mechanism::FullFeedback)
        .collect();
    assert_eq!(stage_a, [2, 5, 8]);
    for (cell, period) in stage_a.into_iter().zip(PERIODS) {
        let plan = main_grid_plan(cell, false).unwrap();
        assert_eq!(plan["stage"], "A");
        assert_eq!(plan["rewire_every"], period);
        assert_eq!(plan["mechanism"], "full-feedback");
        assert_eq!(plan["parents"], MAIN_GRID_PARENTS_PER_CELL);
        let seeds: Vec<u64> = serde_json::from_value(plan["seeds"].clone()).unwrap();
        let frozen: Vec<u64> = (0..MAIN_GRID_PARENTS_PER_CELL)
            .map(|i| main_grid_seed(cell, i).unwrap())
            .collect();
        assert_eq!(seeds, frozen);
        assert_eq!(plan["critical_t"], DECISION_CRITICAL_T);
        // The smoke dry run shares the design but never a frozen seed.
        let smoke = main_grid_plan(cell, true).unwrap();
        assert_eq!(smoke["rewire_every"], period);
        assert_eq!(smoke["parents"], SMOKE_PARENTS_PER_CELL);
        let smoke_seeds: Vec<u64> = serde_json::from_value(smoke["seeds"].clone()).unwrap();
        assert!(smoke_seeds.iter().all(|s| !(130_000..=130_847).contains(s)));
        assert!(smoke_seeds.iter().all(|s| !(94_000..=121_099).contains(s)));
    }
    assert!(main_grid_plan(9, false).is_err());
}
