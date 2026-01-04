#[derive(Debug, Clone)]
pub struct Disturbance { //setting disturbance in gradient
    pub intensity: f32, // 0.0 -> 1.0
    pub duration: f32, //time unit
    pub suddenness: f32 // 0.0 smooth, 1.0 shock
}
 impl Disturbance {
    pub fn new(intensity: f32, duration: f32, suddenness: f32) -> Self {
        Self {
            intensity,  
            duration,   
            suddenness, 
        }
    }
 }