// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod openai;
use crate::openai::query_openai;

use tauri::GlobalShortcutManager;
use tauri::Manager;
use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

#[tauri::command]
async fn handle_command(command: String) -> Result<String, String> {
    let prompt = format!(
        "你是一个帮助用户导航 macOS 设置的助手，因为用户经常不知道具体的指令是什么。当用户输入笼统的命令时，提供他们应该去的具体设置。例子：\n\n用户: \"息屏\"\n助手: \"系统设置 > 锁定屏幕\"\n\n用户输入: \"关闭屏幕\"\n助手: \"系统设置 > 锁定屏幕\"\n到此例子结束。\n现在用户输入: \"{}\"\n请直接给出具体的设置名字",
        command
    );
    match query_openai(&prompt).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Failed to query OpenAI: {}", e)),
    }
}
fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_command])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");

                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }

            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);

                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                        item_handle.set_title("Show").unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let app_handle = app.handle();
            let mut global_shortcut_manager = app_handle.global_shortcut_manager();
            global_shortcut_manager
                .register("Cmd+Option+K", move || {
                    let window = app_handle.get_window("main").unwrap();
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                })
                .expect("failed to register global shortcut");

            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
