use project_newborn::{isf::InternalStateField, disturbance::Disturbance};

#[test]
fn bias_smooths_noisy_experience() {
    let mut brain = InternalStateField::new();

    let noisy = vec![
        Disturbance::new(0.8, 0.2, 0.9),
        Disturbance::new(0.2, 3.0, 0.1),
        Disturbance::new(0.7, 0.3, 0.8),
        Disturbance::new(0.3, 2.5, 0.1),
    ];

    for d in noisy.iter() {
        brain.apply_disturbance(d);
    }

    let before = brain.tension;

    for _ in 0..100 {
        brain.internal_thinking_tick();
    }

    let after = brain.tension;

    assert!(
        (after - before).abs() < 0.2,
        "Bias replay caused unstable drift"
    );
}
