use super::SdalcalCore;
use super::SdalcalCoreResults;

impl SdalcalCore {

    // c(0°) = 331,3 m/s
    // c(t) = 331,3 m/s + 0,6 m/(s * C°) * t, where t in C°
    // --- PUBLIC FUNCTIONS ---
    pub fn calculateAllValues(distance: f32, levelL1: f32) -> SdalcalCoreResults {
        // TODO: implement this method
        println!("TODO: Implement this function");
        SdalcalCoreResults {
            delay: 0.1,
            soundLevelL2: 0.1,
            dropLevel: 0.1,
            adjustmentLevel: 0.1,
            setpoint: 0.1
        }
    }   
}