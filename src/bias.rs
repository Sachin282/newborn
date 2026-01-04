// bias.rs

#[derive(Debug, Clone)]
pub struct BiasField {
    // Preferred direction (structural memory)
    pub dt_pref: f32,
    pub ds_pref: f32,
    pub de_pref: f32,

    // How deep this attractor is
    pub strength: f32,
}

impl BiasField {
    pub fn new() -> Self {
        Self {
            dt_pref: 0.0,
            ds_pref: 0.0,
            de_pref: 0.0,
            strength: 0.0,
        }
    }

    /// How well this bias matches the current state change
    pub fn similarity(&self, dt: f32, ds: f32, de: f32) -> f32 {
        (self.dt_pref - dt).abs()
            + (self.ds_pref - ds).abs()
            + (self.de_pref - de).abs()
    }

    pub fn reinforce(&mut self, dt: f32, ds: f32, de: f32) {
        self.dt_pref += dt * 0.1;
        self.ds_pref += ds * 0.1;
        self.de_pref += de * 0.1;

        self.strength += 0.05;
        self.strength = self.strength.clamp(0.0, 1.0);
    }

    pub fn decay(&mut self) {
        self.strength *= 0.995; // slow forgetting
    }


     /// Distance between two biases (structural similarity)
    pub fn distance(&self, other: &BiasField) -> f32 {
        (self.dt_pref - other.dt_pref).abs()
            + (self.ds_pref - other.ds_pref).abs()
            + (self.de_pref - other.de_pref).abs()
    }

    /// Merge another bias into this one
    pub fn merge(&mut self, other: &BiasField) {
        // Weighted average based on strength
        let total = self.strength + other.strength;
        if total > 0.0 {
            self.dt_pref =
                (self.dt_pref * self.strength + other.dt_pref * other.strength) / total;
            self.ds_pref =
                (self.ds_pref * self.strength + other.ds_pref * other.strength) / total;
            self.de_pref =
                (self.de_pref * self.strength + other.de_pref * other.strength) / total;
        }

        // Basin deepens
        self.strength = (self.strength + other.strength).clamp(0.0, 1.0);
    }
}
