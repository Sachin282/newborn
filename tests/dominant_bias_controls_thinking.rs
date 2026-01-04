use project_newborn::{isf::InternalStateField, disturbance::Disturbance};

#[test]
fn dominant_bias_controls_thinking() {
    let mut brain = InternalStateField::new();

    let calm = Disturbance::new(0.3, 3.0, 0.1);
    let shock = Disturbance::new(0.9, 0.2, 0.9);

    for _ in 0..10 {
        brain.apply_disturbance(&calm);
    }

    for _ in 0..3 {
        brain.apply_disturbance(&shock);
    }

    let dominant = brain.biases
        .iter()
        .max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap())
        .unwrap();

    assert!(dominant.ds_pref.abs() > dominant.dt_pref.abs());
}
