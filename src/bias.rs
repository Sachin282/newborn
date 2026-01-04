#[derive(Debug, Clone)]
pub struct BiasField {
    // Directional preferences
    pub dt_pref: f32, // tension direction preference
    pub ds_pref: f32, // stability direction preference
    pub de_pref: f32, // energy direction preference

    // Strength of bias (memory depth)
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

    pub fn reinforce(&mut self, dt: f32, ds: f32, de: f32) {
        // Move preference toward experienced direction
        self.dt_pref += dt * 0.1;
        self.ds_pref += ds * 0.1;
        self.de_pref += de * 0.1;

        // Memory deepens
        self.strength += 0.05;
        self.strength = self.strength.clamp(0.0, 1.0);
    }
}
