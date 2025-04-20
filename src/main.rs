use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let old_counter = ui.get_counter();
            println!("Increment old_counter with old value >>{old_counter}<<");
            ui.set_counter(old_counter + 1);
        }
    });

    ui.run()?;

    Ok(())
}
