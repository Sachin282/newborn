// isf.rs
//
// Internal State Field (ISF)
// --------------------------
// This file represents the INTERNAL PHYSIOLOGY + STRUCTURAL MEMORY
// of Project Newborn.
//
// IMPORTANT PHILOSOPHY:
// - No symbols
// - No language
// - No explicit memory storage (final goal)
// - Memory = structural bias (directional preference)
//
// Step 7: Structural Memory (Phase 1)

use crate::disturbance::Disturbance;
use crate::bias::BiasField;
use crate::memory::ExperienceTrace;

#[derive(Debug, Clone, Copy)]
pub enum ReplayMode {
    TraceBased,
    StructuralBias,
}


#[derive(Debug)]
pub struct InternalStateField {
    // --------------------------------------------------
    // CORE INTERNAL PHYSIOLOGY (SELF STATE)
    // --------------------------------------------------

    /// Nervous arousal / stress tone
    pub tension: f32,

    /// Regulation capacity (slow changing)
    pub stability: f32,

    /// Metabolic / activation energy
    pub energy: f32,

    // --------------------------------------------------
    // PLASTICITY (HOW THE BRAIN REACTS)
    // These are ALREADY structural memory
    // --------------------------------------------------

    /// How strongly shock affects tension
    pub shock_sensitivity: f32,

    /// How fast stability grows during calm
    pub stability_gain_rate: f32,

    /// How energy reacts to stimulation
    pub energy_gain_rate: f32,

    // --------------------------------------------------
    // STRUCTURAL MEMORY (STEP 7 CORE)
    // --------------------------------------------------

    /// Directional bias field
    /// This replaces explicit episodic memory over time
    pub memory: Vec<ExperienceTrace>,
    pub bias: BiasField,
    pub replay_mode: ReplayMode,
}

impl InternalStateField {
    // --------------------------------------------------
    // BIRTH INITIALIZATION (GENETIC PRIOR)
    // --------------------------------------------------
    pub fn new() -> Self {
        Self {
            // Newborn internal state
            tension: 0.5,    // neutral
            stability: 0.1,  // very low regulation
            energy: 0.5,     // baseline vitality

            // Newborn plasticity (highly sensitive)
            shock_sensitivity: 0.2,
            stability_gain_rate: 0.05,
            energy_gain_rate: 0.1,
            

            replay_mode: ReplayMode::StructuralBias, // default testing mode
            // No memory at birth
            memory: Vec::new(),
            bias: BiasField::new(),
        }
    }

    // --------------------------------------------------
    // EXTERNAL EXPERIENCE APPLICATION
    // --------------------------------------------------
    pub fn apply_disturbance(&mut self, d: &Disturbance) {
        // Capture "before" state
        // (used ONLY to compute direction of change)
        let before_tension = self.tension;
        let before_stability = self.stability;
        let before_energy = self.energy;

        // ----------------------------------------------
        // RAW PHYSICAL INTERPRETATION OF INPUT
        // ----------------------------------------------

        // Shock = strong + sudden
        let shock = d.intensity * d.suddenness;

        // Calm = long + predictable
        let calm = d.duration * (1.0 - d.suddenness);

        // ----------------------------------------------
        // PHYSIOLOGICAL RESPONSE (NOT DECISION)
        // ----------------------------------------------

        // Shock increases tension
        self.tension += shock * self.shock_sensitivity;

        // Calm increases regulation capacity
        self.stability += calm * self.stability_gain_rate;

        // Energy dynamics (dual nature)
        self.energy += d.intensity * self.energy_gain_rate;
        self.energy -= shock * (self.energy_gain_rate * 0.5);

        // ----------------------------------------------
        // STRUCTURAL MEMORY REINFORCEMENT (STEP 7 CORE)
        // ----------------------------------------------
        //
        // We DO NOT store the experience.
        // We only reinforce the DIRECTION in which
        // the internal state moved.
        //
        // This is equivalent to synaptic strengthening.

        let dt = self.tension - before_tension;
        let ds = self.stability - before_stability;
        let de = self.energy - before_energy;

        self.bias.reinforce(dt, ds, de);

        // ----------------------------------------------
        // PLASTICITY ADAPTATION (LEARNING HOW TO REACT)
        // ----------------------------------------------

        // Repeated shock with good regulation → desensitization
        if shock > 0.3 && self.stability > 0.6 {
            self.shock_sensitivity *= 0.98;
        }

        // Long calm exposure → faster regulation learning
        if calm > 1.0 {
            self.stability_gain_rate *= 1.02;
        }

        // Chronic overload → energy efficiency adjustment
        if self.tension > 1.0 {
            self.energy_gain_rate *= 0.99;
        }

        // ----------------------------------------------
        // BIOLOGICAL LIMITS (NOT RULES)
        // ----------------------------------------------

        self.tension = self.tension.clamp(0.0, 1.5);
        self.stability = self.stability.clamp(0.0, 1.0);
        self.energy = self.energy.clamp(0.0, 1.0);

        self.shock_sensitivity = self.shock_sensitivity.clamp(0.05, 0.5);
        self.stability_gain_rate = self.stability_gain_rate.clamp(0.01, 0.2);
        self.energy_gain_rate = self.energy_gain_rate.clamp(0.05, 0.3);
    }

    // --------------------------------------------------
    // INTERNAL THINKING LOOP (NO INPUT)
    // --------------------------------------------------

    pub fn internal_thinking_tick(&mut self) {
        match self.replay_mode {
            ReplayMode::TraceBased => self.trace_thinking_tick(),
            ReplayMode::StructuralBias => self.bias_thinking_tick(),
        }

        // common homeostasis
        if self.tension < 0.4 {
            self.stability += 0.01;
        }

        self.tension = self.tension.clamp(0.0, 1.5);
        self.stability = self.stability.clamp(0.0, 1.0);
        self.energy = self.energy.clamp(0.0, 1.0);
    }



    pub fn bias_thinking_tick(&mut self) {
        // --------------------------------------------------
        // STRUCTURAL REPLAY (NO MEMORY ACCESS)
        // --------------------------------------------------
        //
        // The system drifts along previously reinforced
        // internal directions.
        //
        // This is:
        // - subconscious processing
        // - imagination precursor
        // - reasoning substrate

        self.tension += self.bias.dt_pref * self.bias.strength * 0.05;
        self.stability += self.bias.ds_pref * self.bias.strength * 0.05;
        self.energy += self.bias.de_pref * self.bias.strength * 0.05;

        // --------------------------------------------------
        // HOMEOSTASIS (SELF-REGULATION)
        // --------------------------------------------------

        // Low tension allows regulation to consolidate
        if self.tension < 0.4 {
            self.stability += 0.01;
        }

        // Energy balancing
        if self.energy > 0.6 {
            self.energy -= 0.01;
        } else if self.energy < 0.4 {
            self.energy += 0.01;
        }

        // --------------------------------------------------
        // BIOLOGICAL LIMITS
        // --------------------------------------------------

        self.tension = self.tension.clamp(0.0, 1.5);
        self.stability = self.stability.clamp(0.0, 1.0);
        self.energy = self.energy.clamp(0.0, 1.0);
    }

    pub fn trace_thinking_tick(&mut self) {
    
        if self.tension < 0.4 {
            self.stability += 0.01;
        }

        // Energy balancing
        if self.energy > 0.6 {
            self.energy -= 0.01;
        } else if self.energy < 0.4 {
            self.energy += 0.01;
        }
        if self.memory.is_empty() {
            return;
        }

        let mut best = None;
        let mut best_score = f32::MAX;

        for t in &self.memory {
            let score =
                (self.tension - t.tension_before).abs() +
                (self.stability - t.stability_before).abs() +
                (self.energy - t.energy_before).abs();

            if score < best_score {
                best_score = score;
                best = Some(t);
            }
        }

        if let Some(t) = best {
            let dt = t.tension_after - t.tension_before;
            let ds = t.stability_after - t.stability_before;
            let de = t.energy_after - t.energy_before;

            self.tension += dt * 0.2;
            self.stability += ds * 0.2;
            self.energy += de * 0.2;
        }
    }

}
