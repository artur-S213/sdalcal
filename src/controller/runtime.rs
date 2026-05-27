use super::SdalcalController

use sdalcal::core::SdalcalCoreResults;

impl SdalcalController {
    pub fn new (ui: AppWindow, core: SdalcalCore) -> Self {
        Self {ui, core}
    }

    fn set_Callback_handling(&self){
        self.ui.on_parameter_changed({
            let ui_handle = self.ui.as_weak();
            move |distance, level| {

            }
        })
    }

    fn calculateParameters(&self, distance: f64, level: f64) -> SdalcalCoreResults {
        
    }

    fn updateUI(&self) {
        println!("TODO: Implement this function");
    }
}