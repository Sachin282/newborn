// memory.rs

use crate::disturbance::Disturbance;

#[derive(Clone, Debug)]
pub struct ExperienceTrace {
    pub tension_before: f32,
    pub stability_before: f32,
    pub energy_before: f32,

    pub disturbance: Disturbance,

    pub tension_after: f32,
    pub stability_after: f32,
    pub energy_after: f32,
}
