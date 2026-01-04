use project_newborn::{isf::InternalStateField, disturbance::Disturbance};

#[test]
fn bias_memory_does_not_grow_unbounded() {
    let mut brain = InternalStateField::new();
    let d = Disturbance::new(0.5, 1.0, 0.3);

    for _ in 0..1000 {
        brain.apply_disturbance(&d);
    }

    assert!(
        brain.bias.strength <= 1.0,
        "Bias strength exceeded biological limit"
    );
}
