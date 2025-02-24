use device_query::{DeviceQuery, DeviceState, Keycode};
use clipboard::{ClipboardProvider, ClipboardContext};
use std::{sync::Mutex, thread};
use lazy_static::lazy_static;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};

lazy_static! {
    static ref ARRAY: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref CTX: Mutex<ClipboardContext> = Mutex::new(ClipboardProvider::new().unwrap());

}



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

#[tauri::command]
fn set_buffer(content: String) -> Result<(), String> {
    let mut ctx = CTX.lock().map_err(|e| e.to_string())?; 
    ctx.set_contents(content).map_err(|e| e.to_string())?; 
    Ok(()) 
}


#[tauri::command]
fn get_buffer() -> Vec<String> {
    ARRAY.lock().unwrap().clone()
}

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
        .invoke_handler(tauri::generate_handler![get_buffer, set_buffer, clean_buffer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
 
}

