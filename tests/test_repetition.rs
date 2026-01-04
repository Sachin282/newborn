use project_newborn::{isf::InternalStateField, disturbance::Disturbance};

#[test]
fn repeated_experience_builds_stronger_bias() {
    let mut brain = InternalStateField::new();
    let d = Disturbance::new(0.6, 1.0, 0.2);

    for _ in 0..10 {
        brain.apply_disturbance(&d);
    }

    assert!(brain.bias.strength > 0.4, "Bias did not strengthen with repetition");
}
