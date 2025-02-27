use tauri::{Manager};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState};
use tauri::menu::{Menu, MenuItem};
use tauri::PhysicalPosition;
use clipboard::{ClipboardContext, ClipboardProvider};
use device_query::{DeviceQuery, DeviceState, Keycode};
use lazy_static::lazy_static;
use std::{sync::Mutex, thread};
use tauri::Listener;

// Создает стейт
lazy_static! {
    static ref ARRAY: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref CTX: Mutex<ClipboardContext> = Mutex::new(ClipboardProvider::new().unwrap());
}

// Чекер на command + C для добвление скопированного элемента в буфер
fn buffer() {
    let device_state = DeviceState::new();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::Command) && keys.contains(&Keycode::C) {
            if let Ok(content) = CTX.lock().unwrap().get_contents() {
                let mut array = ARRAY.lock().unwrap();
                let len_array = array.len();

                if let Some(last_element) = array.last() {
                    if last_element != &content {
                        if len_array == 5 {
                            array.clear();
                        }
                        array.push(content);
                    }
                } else {
                    array.push(content);
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

// Вставка в буфер
#[tauri::command]
fn set_buffer(content: String) -> Result<(), String> {
    let mut ctx = CTX.lock().map_err(|e| e.to_string())?;
    ctx.set_contents(content).map_err(|e| e.to_string())?;
    Ok(())
}

//  Получение данных из буфера
#[tauri::command]
fn get_buffer() -> Vec<String> {
    ARRAY.lock().unwrap().clone()
}

// Чистка буфера
#[tauri::command]
fn clean_buffer() -> Result<(), String> {
    ARRAY.lock().map_err(|e| e.to_string())?.clear();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    thread::spawn(move || {
        buffer();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_buffer,
            set_buffer,
            clean_buffer
        ])
        .setup(|app| {
                    #[cfg(target_os = "macos")]
                    app.set_activation_policy(tauri::ActivationPolicy::Accessory);  // Скрытие из Dock
        
        
                    let _ = TrayIconBuilder::new()
                        .icon(app.default_window_icon().unwrap().clone())  
                        .on_tray_icon_event(|tray, event| match event {
                            TrayIconEvent::Click {
                              button: MouseButton::Left,
                              button_state: MouseButtonState::Up,
                              ..
                            } => {
                              println!("left click pressed and released");
                              // in this example, let's show and focus the main window when the tray is clicked
                              let app = tray.app_handle();
                              if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                              }
                            }
                            _ => {
                              println!("unhandled event {event:?}");
                            }
                          })

                        .build(app)?;
        
                    let window = app.get_webview_window("main").unwrap();
                    window.hide().unwrap();
        
                    window.set_position(PhysicalPosition::new(1700, 0)).unwrap();
                    window.set_resizable(false).unwrap();
                    window.set_decorations(false).unwrap();
                    window.set_always_on_top(true).unwrap();
        
                    let window_clone = window.clone();
                    window.listen("tauri://blur", move |_| {
                        println!("Window lost focus, hiding...");
                        let _ = window_clone.hide();
                    });
        
                    Ok(())
                })

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
