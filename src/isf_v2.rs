// Internal State of body after birth

use crate::memory::ExperienceTrace;

#[derive(Debug, Clone, Copy)]
pub enum ReplayMode {
    TraceBased,
    StructuralBias,
}

#[derive(Debug)]
pub struct InternalStateField { 
    // Core physiological states
    pub tension: f32,
    pub stability: f32,
    pub energy: f32,

    // Plasticity parameters (learning happens here over time)
    pub shock_sensitivity: f32, //how strongly shocks affect tension
    pub stability_gain_rate: f32, //how quickly stability recovers
    pub energy_gain_rate: f32, //how quickly energy recovers

    // Memory of past experiences (not implemented yet)
    pub memory: Vec<ExperienceTrace>,

     // 🧠 Structural memory (NO episode storage)
    pub bias: BiasField,
    pub replay_mode: ReplayMode,
}
impl InternalStateField {
    pub fn new()-> Self {
        Self {
            // Initial DNA  defined states
            tension: 0.5, //neutral
            stability: 0.1, //newborn = unstable
            energy: 0.5, //baseline

            // Newborn brain defaults
            //every child or body have different sensitivity, and recovery rate but keep it 0.2, 0.05, 0.1 for now (low at birth)
            // eg: when our brain receives shocks for first time, it is more sensitive to it and it hurts more, but when we face same type of pain again and agian, our brain learn to handle it better and our sensories becomes more tollerent to that signal and our shock_sensitivity decreases and stability_recovery_rate and energy_recovery_rate increases timely
            // eg: when we get some type of shock (like failure) first time, we are more sensitive to it and it hurts more, but when we face same type of pain again and agian, we learn to handle it better and our shock_sensitivity decreases and stability_recovery_rate and energy_recovery_rate increases timely
            shock_sensitivity: 0.2, 
            stability_gain_rate: 0.05, // stability_recovery_rate
            energy_gain_rate: 0.1, //energy_recovery_rate,
            memory: Vec::new(),
            bias: BiasField::new(),
            replay_mode: ReplayMode::TraceBased, // default testing mode
        }
    }

    pub fn apply_dicturbance(&mut self, d: &crate::disturbance::Disturbance) {
        // Record state before applying disturbance for memory trace
        let before = (self.tension, self.stability, self.energy);

        //Sudden + intense -> tension spike
        let shock = d.intensity * d.suddenness;
        let calm = d.duration * (1.0 - d.suddenness);

        
        self.tension += shock * self.shock_sensitivity;
        self.stability += calm * self.stability_gain_rate;
        //Long + predictable -> stable growth

        // Energy reacts to total load
        self.energy += d.intensity * self.energy_gain_rate; //overall intensity increases energy (like exercise) 
        self.energy -= shock * (self.energy_gain_rate * 0.05); //shock drains energy slightly

        println!("shock_sensitivity before: {:?}", self.shock_sensitivity);
        println!("stability_gain_rate before: {:?}", self.stability_gain_rate);
        println!("energy_gain_rate before: {:?}", self.energy_gain_rate);
        
        
        self.update_plasticity(shock, calm);
        // println!("shock, calm applied: {}, {}", shock, calm);
        // println!("if shock > 0.3 && self.stability > 0.6{{\n            self.shock_sensitivity *= 0.98; //less sensitive to shocks\n        }}");

        // println!("shock_sensitivity after: {:?}", self.shock_sensitivity);
        // println!("if calm > 1.0{{\n            self.stability_gain_rate *= 1.02; //learn to stabilize faster\n        }}");
        // println!("stability_gain_rate after: {:?}", self.stability_gain_rate);
        // println!("if self.tension > 1.0{{\n            self.energy_gain_rate *= 0.99; //become more efficient with energy use\n        }}");
        // println!("energy_gain_rate after: {:?}", self.energy_gain_rate);


        // Soft bounds (no rules, physics limits) - max and min range to survive the body or neuron
        self.tension = self.tension.clamp(0.0, 1.5);
        self.stability = self.stability.clamp(0.0, 1.0);
        self.energy = self.energy.clamp(0.0, 1.0);

        // let trace = ExperienceTrace {
        //     tension_before: before.0,
        //     stability_before: before.1,
        //     energy_before: before.2,

        //     disturbance: d.clone(),

        //     tension_after: self.tension,
        //     stability_after: self.stability,
        //     energy_after: self.energy,
        // };

        // self.memory.push(trace);

        let dt = self.tension - before.0;
        let ds = self.stability - before.1;
        let de = self.energy - before.2;

        self.bias.reinforce(dt, ds, de);
    }


    fn update_plasticity(&mut self, shock: f32, calm: f32) {
        // Plasticity updates based on experiences
        // Rule 1 : Repeated shock + high stability -> desensitization
        if shock > 0.3 && self.stability > 0.6{
            self.shock_sensitivity *= 0.98; //less sensitive to shocks
        }

        // Rule 2 : Prolonged calm exposure -> faster regulation learning
        if calm > 1.0 {
            self.stability_gain_rate *= 1.02; //learn to stabilize faster
        }

        // Rule 3 : chronic overload (high tension) -> energy efficiency adaptation
        if self.tension > 1.0 {
            self.energy_gain_rate *= 0.99; //become more efficient with energy use
        }

        // soft bounds on plasticity parameters
        self.shock_sensitivity = self.shock_sensitivity.clamp(0.05, 0.5);
        self.stability_gain_rate = self.stability_gain_rate.clamp(0.01, 0.2);
        self.energy_gain_rate = self.energy_gain_rate.clamp(0.05, 0.3);
    }


    pub fn trace_thinking_tick(&mut self) {
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

      /// Internal thinking / resting dynamics
    /// Runs even when there is NO disturbance
    pub fn internal_tick(&mut self) {

        
        // 1. Tension naturally decays if stability exists
        // let tension_release = self.stability * 0.02;
        // self.tension -= tension_release; //if brain is stable, tension reduces over time

        // 2. Stability slowly increases when tension is low
        // if self.tension < 0.4 {
        //     self.stability += 0.01; // id brain is in less stress for long time, it try to restore or increase stability
        // }

        // 3. Energy redistributes (not always increases)
        /* 
        Real human:
            zyada excited → thoda settle 
            thaka hua → thoda recover
        */
        // if self.energy > 0.6 {
        //     self.energy -= 0.01;  //if brain is too energetic, it uses up energy faster or tries to distribute energy to other source to maintain balance
        // } else if self.energy < 0.4 {
        //     self.energy += 0.01; //if brain is low on energy, it tries to recover energy
        // }

        // 4. Internal drift (prevents freezing)

        // if self.memory.is_empty() {
        //     return;
        // }
        // // random thoughts / noise (random internal chemical reactions or neural data processing in brain)
        // /*
        //     ye:
        //         boredom
        //         curiosity
        //         mind wandering
        //         spontaneous thought
        //     Agar ye na ho:
        //         system freeze ho jaata
        //         thinking impossible
        // */

        // // 1️⃣ Find memory closest to current state
        // let mut best_match = None;
        // let mut best_score = f32::MAX;

        // for trace in &self.memory {
        //     let score =
        //         (self.tension - trace.tension_before).abs() +
        //         (self.stability - trace.stability_before).abs() +
        //         (self.energy - trace.energy_before).abs();

        //     if score < best_score {
        //         best_score = score;
        //         best_match = Some(trace);
        //     }
        // }

        // let trace = best_match.unwrap();

        // // 2️⃣ Replay internal transition (compressed)
        // let dtension = trace.tension_after - trace.tension_before;
        // let dstability = trace.stability_after - trace.stability_before;
        // let denergy = trace.energy_after - trace.energy_before;

        // // 3️⃣ Apply partially (imagination, not real event)
        // self.tension += dt * 0.2;
        // self.stability += ds * 0.2;
        // self.energy += de * 0.2;


        self.tension += self.bias.dt_pref * self.bias.strength * 0.05;
        self.stability += self.bias.ds_pref * self.bias.strength * 0.05;
        self.energy += self.bias.de_pref * self.bias.strength * 0.05;

         // Homeostasis still applies
        if self.tension < 0.4 {
            self.stability += 0.01;
        }
        
        // Physical bounds
        self.tension = self.tension.clamp(0.0, 1.5);
        self.stability = self.stability.clamp(0.0, 1.0);
        self.energy = self.energy.clamp(0.0, 1.0);
    }
}

/// Very small noise (proto-chaos)
fn rand_noise() -> f32 {
    // simple pseudo-noise without external crate
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos % 1000) as f32 / 1000.0
}
