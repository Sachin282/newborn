use project_newborn::{isf::InternalStateField, disturbance::Disturbance};

#[test]
fn bias_memory_does_not_grow_unbounded() {
    let mut brain = InternalStateField::new();
    let d = Disturbance::new(0.5, 1.0, 0.3);

    for _ in 0..1000 {
        brain.apply_disturbance(&d);
    }

    let dominant = brain.biases
        .iter()
        .max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap())
        .unwrap();

    assert!(
        dominant.strength <= 1.0,
        "Bias strength exceeded biological limit"
    );
}
