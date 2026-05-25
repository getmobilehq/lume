use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager, WindowEvent,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// Reveal the main window and tell the frontend to route to `route`.
fn navigate(app: &AppHandle, route: &str) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let _ = window.emit("navigate", route);
    }
}

// Hard-coded Week 1 hotkey defaults (⌃⌥R / ⌃⌥L); customisation comes later.
fn record_toggle_shortcut() -> Shortcut {
    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyR)
}

fn open_library_shortcut() -> Shortcut {
    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyL)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }
                    if *shortcut == record_toggle_shortcut() {
                        log::info!("hotkey: record-toggle");
                        let _ = app.emit("record-toggle", ());
                    } else if *shortcut == open_library_shortcut() {
                        navigate(app, "/");
                    }
                })
                .build(),
        )
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let library_i = MenuItem::with_id(app, "library", "Library", true, None::<&str>)?;
            let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&library_i, &settings_i, &quit_i])?;

            TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Lume")
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "library" => navigate(app, "/"),
                    "settings" => navigate(app, "/settings"),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            let global_shortcut = app.global_shortcut();
            global_shortcut.register(record_toggle_shortcut())?;
            global_shortcut.register(open_library_shortcut())?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // Closing the main window hides it instead of quitting; the app
            // keeps running in the tray. Quit (app.exit) bypasses this.
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    let _ = window.hide();
                    api.prevent_close();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
