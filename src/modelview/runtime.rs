use std::rc::Rc;
use super::SdalcalController;
use slint::{ComponentHandle};
use crate::core::*;
use crate::AppWindow;

impl SdalcalController {
    pub fn new(ui: &AppWindow) -> Rc<Self>
    {
       let ctrl = Rc::new(Self {
           ui: ui.as_weak(),
       });
        ctrl.set_Callback_handling();
        ctrl
    }

    fn set_Callback_handling(self: &Rc<Self>) {
        let ctrl = Rc::clone(self);
        let ui_handle = ctrl.ui.unwrap();

        if let Some(ui) = self.ui.upgrade() {
            ui.on_parameter_changed({
                move |distance, temperature, level| {
                    ui_handle.set_distance_meter(distance);
                    ui_handle.set_temperature(temperature);
                    ui_handle.set_speakerL1(level);
                    ctrl.handleInputChange(distance, temperature, level);
                }
            });
        }
    }

    fn handleInputChange(&self, distance: f32, temperature: f32, level: f32) {
        self.updateUI(self.calculateParameters(distance, temperature, level));
    }
    fn calculateParameters(&self, distance: f32, temperature: f32, level: f32) -> SdalcalCoreResults {
        let res: SdalcalCoreResults = SdalcalCore::calculateAllValues(distance, temperature, level);
        res
    }

    fn updateUI(&self, newResults: SdalcalCoreResults) {
        if let Some(ui) = self.ui.upgrade() {
            ui.set_delay(newResults.delay);
            ui.set_speakerL2(newResults.soundLevelL2);
            ui.set_adjustmentLevel(newResults.adjustmentLevel);
            ui.set_dropLevel(newResults.dropLevel);
            ui.set_setpoint(newResults.setpoint);
        }

    }
}

