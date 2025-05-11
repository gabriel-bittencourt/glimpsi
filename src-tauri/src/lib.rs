mod config;
mod commands;

use log::{info, error};
use std::env;
use tauri::{
    menu::{Menu, MenuBuilder, MenuEvent, MenuItemBuilder},
    tray::{TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Manager, Result, Runtime, WebviewWindow,
};
use tauri_plugin_global_shortcut::{
    Builder as GlobalShortcutBuilder, GlobalShortcutExt,
    Shortcut, ShortcutEvent, ShortcutState,
};
use window_vibrancy::apply_acrylic;

use crate::config::types::{AppFile, Settings};
use shared::types::content::Content;


fn get_main_window<R: Runtime>(app_handle: &impl Manager<R>) -> Result<WebviewWindow<R>> {
    match app_handle.get_webview_window("main") {
        Some(window) => Ok(window),
        None => Err(tauri::Error::WebviewNotFound),
    }
}


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg(desktop)]
fn handle_menu_events<R: Runtime>(app_handle: &AppHandle<R>, event: MenuEvent) -> Result<()> {
    let window = get_main_window(app_handle)?;

    match event.id.as_ref() {
        "show-btn" => commands::window::show(&window),
        "restart-btn" => commands::window::restart(&window),
        "quit-btn" => commands::app::quit(app_handle),
        _ => Ok(()),
    }
}

#[cfg(desktop)]
fn create_menu<R: Runtime>(app_handle: &AppHandle<R>) -> Result<Menu<R>> {
    let show_btn = MenuItemBuilder::new("Show")
        .id("show-btn")
        .build(app_handle)?;
    let quit_btn = MenuItemBuilder::new("Quit")
        .id("quit-btn")
        .build(app_handle)?;
    let restart_btn = MenuItemBuilder::new("Restart")
        .id("restart-btn")
        .build(app_handle)?;

    Ok(MenuBuilder::new(app_handle)
        .items(&[&show_btn, &quit_btn, &restart_btn])
        .build()?)
}

#[cfg(desktop)]
fn handle_tray_icon_events(event: TrayIconEvent) -> Result<()> {
    match event {
        TrayIconEvent::Enter { .. } => {
            // TODO: Seek when hovering for over 1 second
            Ok(())
        }
        _ => Ok(()),
    }
}

#[cfg(desktop)]
fn setup_tray<R: Runtime>(app: &App<R>) -> Result<()> {
    let icon = app.default_window_icon().unwrap().clone();
    let menu = create_menu(app.handle())?;

    TrayIconBuilder::<R>::new()
        .tooltip("Glimpsi")
        .icon(icon)
        .show_menu_on_left_click(true)
        .menu(&menu)
        .on_menu_event(move |app, event| match handle_menu_events(app, event) {
            Err(e) => eprintln!("Error handling menu event: {}", e),
            _ => (),
        })
        .on_tray_icon_event(move |_app, event| match handle_tray_icon_events(event) {
            Err(e) => eprintln!("Error handling tray icon event: {}", e),
            _ => (),
        })
        .build(app)?;

    Ok(())
}

#[cfg(desktop)]
fn handle_seek_shortcut<R: Runtime>(app_handle: &AppHandle<R>, event: ShortcutEvent) -> Result<()> {
    let window = get_main_window(app_handle)?;

    match event.state() {
        ShortcutState::Pressed => commands::window::show(&window),
        ShortcutState::Released => commands::window::hide(&window),
    }
}

#[cfg(desktop)]
fn setup_global_shortcut<R: Runtime>(app: &App<R>, settings: &Settings) -> Result<()> {
    // Shortcut to show/hide the window
    let global_shortcut: Shortcut = Shortcut::from(config::mapper::Shortcut(settings.global_shortcut.clone()));

    info!("Global Shortcut: {:?}", &global_shortcut);

    let shorcut_plugin = GlobalShortcutBuilder::<R>::new()
        .with_handler(move |app, shortcut, event| {
            if shortcut == &global_shortcut {
                match handle_seek_shortcut(app, event) {
                    Err(e) => eprintln!("Error handling seek shortcut: {}", e),
                    _ => (),
                }
            }
        })
        .build();

    app.handle().plugin(shorcut_plugin)?;

    match app.global_shortcut().register(global_shortcut) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to register global shortcut: {}", e);
            app.handle().exit(1);
        }
    }

    Ok(())
}

#[cfg(desktop)]
fn setup_logger<R: Runtime>(app: &App<R>) -> Result<()> {
    match config::log::setup_logger(app.handle()) {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to initialize logger: {}", e);
            app.handle().exit(1);
        }
    }

    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {

            setup_logger(app)?;

            let settings: Settings = config::file::load_data(app.handle(), AppFile::Settings);
            let content: Content = config::file::load_data(app.handle(), AppFile::Content);

            app.manage(content);

            // Setup shortcut to show main window
            setup_global_shortcut(app, &settings)?;

            // Setup tray icon and menu
            setup_tray(app)?;

            // Setup window transparency effect (Windows only)
            #[cfg(target_os = "windows")]
            {
                let window = get_main_window(app.handle())?;
                let color = Some((100, 100, 100, 230)); // TODO: Make configurable
                apply_acrylic(&window, color)?;

                window.set_decorations(false)?;
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, commands::content::get_content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
