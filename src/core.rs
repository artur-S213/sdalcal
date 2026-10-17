mod runtime;

pub struct SdalcalCoreResults{
    pub delay: f32,
    pub soundLevelL2: f32,
    pub dropLevel: f32,
    pub adjustmentLevel: f32,
    pub setpoint: f32
}

pub struct SdalcalCore {}