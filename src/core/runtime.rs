use super::SdalcalCore;
use super::SdalcalCoreResults;

impl SdalcalCore {

    // c(0°) = 331,3 m/s
    // c(t) = 331,3 m/s + 0,6 m/(s * C°) * t, where t in C°
    // --- PUBLIC FUNCTIONS ---
    pub fn calculateAllValues(distance: f32, temperature: f32, levelL1: f32) -> SdalcalCoreResults {

        let soundSpeed = 331.3 + 0.6 * temperature;

        let delay: f32 = (distance / soundSpeed) * 1000.0_f32;

        // soundLevelL2 =  soundlevelL1 - 20 * LOG_10(distance)
        let soundLevelL2: f32;
        if distance <= 0.0{
            soundLevelL2 = 0.0;
        }
        else {
            let calL2: f32 = levelL1 - (20.0_f32 - distance.log10());
            soundLevelL2 = calL2.max(0.0);
        }

        // =soundLevelL1 - soundLevelL2
        let dropLevel: f32 = levelL1 - soundLevelL2;

        // Wenn der Abfallpegel kleiner 11 -> 0 sonst Abfallpegel - 11
        let adjustmentLevel: f32 = (dropLevel - 11.0_f32).max(0.0);

        // =soundLevelL1 - Einstellspegel
        let setpoint: f32 = levelL1 - adjustmentLevel;

        //println!("#########################");
        //println!("NEW CALCULATION");
        //println!("Temperature: {temperature}");
        //println!("Soundspeed: {soundSpeed}");
        //println!("Delay: {delay}");
        //println!("SoundLevelL2: {soundLevelL2}");
        //println!("Droplevel: {dropLevel}");
        //println!("Adjustmentlevel: {adjustmentLevel}");
        //println!("Setpoint: {setpoint}");
        //println!(" ");

        SdalcalCoreResults {
            delay,
            soundLevelL2,
            dropLevel,
            adjustmentLevel,
            setpoint
        }
    }   
}