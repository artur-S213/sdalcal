// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_parameter_changed(distance, level)({
        let ui_handle = ui.as_weak();
        move |distance, level| {
            let ui = ui_handle.unwrap();
            ui.set_distance_meter(distance);
            ui.set_speakerL1(level);
        }
    });

    ui.run()?;

    Ok(())
}
