mod capture;
mod commands;
mod db;
mod error;

use std::sync::Mutex;

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

            // Stronghold encrypts the API-key vault with an argon2-derived key;
            // the salt lives next to the vault in the app data dir.
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let salt_path = data_dir.join("salt.txt");
            app.handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

            // SQLite (+ sqlite-vec) lives beside the vault; hold the connection
            // in managed state for future query commands.
            let conn = db::init(&data_dir.join("db.sqlite"))?;
            app.manage(Mutex::new(conn));

            app.manage(capture::CaptureState::default());

            let library_i = MenuItem::with_id(app, "library", "Library", true, None::<&str>)?;
            let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&library_i, &settings_i, &quit_i])?;

            // Monochrome template icon: macOS renders it black on a light menu
            // bar and white on a dark one, so it stays crisp and visible.
            let tray_icon = tauri::image::Image::from_bytes(include_bytes!("../icons/tray.png"))?;

            TrayIconBuilder::with_id("main")
                .icon(tray_icon)
                .icon_as_template(true)
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
        .invoke_handler(tauri::generate_handler![
            commands::vault_password,
            commands::screen_recording_permission,
            commands::request_screen_recording_permission,
            commands::restart_app,
            capture::start_capture,
            capture::stop_capture,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
