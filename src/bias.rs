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
}
