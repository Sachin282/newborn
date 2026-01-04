use project_newborn::{isf::InternalStateField, disturbance::Disturbance};
use project_newborn::isf::ReplayMode;

#[test]
fn compare_trace_and_structural_replay() {
    let disturbances = vec![
        Disturbance::new(0.9, 0.1, 0.9),
        Disturbance::new(0.3, 2.0, 0.1),
        Disturbance::new(0.4, 1.5, 0.2),
    ];

    // -------- Trace-based --------
    let mut trace_brain = InternalStateField::new();
    trace_brain.replay_mode = ReplayMode::TraceBased;

    for d in disturbances.iter() {
        trace_brain.apply_disturbance(d);
    }

    for _ in 0..50 {
        trace_brain.internal_thinking_tick();
    }

    // -------- Bias-based --------
    let mut bias_brain = InternalStateField::new();
    bias_brain.replay_mode = ReplayMode::StructuralBias;

    for d in disturbances.iter() {
        bias_brain.apply_disturbance(d);
    }

    for _ in 0..50 {
        bias_brain.internal_thinking_tick();
    }

    // -------- Assertions (directional, not exact) --------
    assert!(
        (trace_brain.tension - bias_brain.tension).abs() < 0.05,
        "Tension drift mismatch too large"
    );

    assert!(
        (trace_brain.stability - bias_brain.stability).abs() < 0.05,
        "Stability drift mismatch too large"
    );
}
