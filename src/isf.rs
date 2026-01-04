// Internal State of body after birth

#[derive(Debug)]
pub struct InternalStateField { 
    pub tension: f32,
    pub stability: f32,
    pub energy: f32
}
impl InternalStateField {
    pub fn new()-> Self {
        Self {
            tension: 0.5, //neutral
            stability: 0.1, //newborn = unstable
            energy: 0.5, //baseline
        }
    }

    pub fn apply_dicturbance(&mut self, d: &crate::disturbance::Disturbance) {
        //Sudden + intense -> tension spike
        let shock = d.intensity * d.suddenness;
        self.tension += shock * 0.2;

        //Long + predictable -> stable growth
        let calm = d.duration * (1.0 - d.suddenness);
        self.stability += calm * 0.05;

        // Energy  reacts to total load
        self.energy += d.intensity * 0.1;
        self.energy -= shock * 0.05;

        // Soft bounds (no rules, physics limits)
        self.tension = self.tension.clamp(0.0, 1.5);
        self.stability = self.stability.clamp(0.0, 1.0);
        self.energy = self.energy.clamp(0.0, 1.0);
    }

}