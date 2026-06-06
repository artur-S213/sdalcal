use super::SdalcalCore;
use super::SdalcalCoreResults;

impl SdalcalCore {

    // --- PUBLIC FUNCTIONS ---
    pub fn calculateAllValues(distance: f32, levelL1: f32) -> SdalcalCoreResults {
        // here comes the implementation of the core calculations
        SdalcalCoreResults {
            delay: 1.0,
            soundLevelL2: 0.0,
            dropLevel: 0.0,
            adjustmentLevel: 0.0,
            setpoint: 0.0
        }
    }   
}