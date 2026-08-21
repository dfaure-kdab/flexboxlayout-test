// Copyright © Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author David Faure <david.faure@kdab.com>
// SPDX-License-Identifier: MIT

// Include the slint-generated code
slint::include_modules!();

use slint::{Model, ModelRc, VecModel};
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = MainWindow::new()?;

    // The model stays in its canonical order; the pinned state alone drives
    // the visual order via `layout-order: task.pinned ? 0 : 1`.
    let tasks = Rc::new(VecModel::from(vec![
        Task { name: "Write report".into(), pinned: false },
        Task { name: "Fix flexbox bug".into(), pinned: true },
        Task { name: "Review MR 123".into(), pinned: false },
        Task { name: "Update docs".into(), pinned: false },
    ]));
    ui.set_tasks(ModelRc::from(tasks.clone()));

    // Exercise the bindings without user interaction, then quit.
    if std::env::var_os("DEMO_SELF_TEST").is_some() {
        let ui_weak = ui.as_weak();
        slint::Timer::single_shot(std::time::Duration::from_millis(500), move || {
            let ui = ui_weak.unwrap();
            // Pin "Update docs": it floats up next to "Fix flexbox bug".
            let mut task = tasks.row_data(3).unwrap();
            task.pinned = true;
            tasks.set_row_data(3, task);
            ui.set_windows_button_order(true);
            // Shrink the window below the breakpoint: the sidebar moves below the content.
            ui.window().set_size(slint::LogicalSize::new(400.0, 620.0));
            let ui_weak = ui.as_weak();
            slint::Timer::single_shot(std::time::Duration::from_millis(500), move || {
                let ui = ui_weak.unwrap();
                println!("narrow after resize to 400px wide: {}", ui.get_narrow());
                slint::quit_event_loop().unwrap();
            });
        });
    }

    ui.run()?;
    Ok(())
}
