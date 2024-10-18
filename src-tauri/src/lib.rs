// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod evaluation;

use std::sync::Mutex;

use anyhow::{Result, anyhow};

use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};

use evaluation::Operand;


#[derive(Default)]
struct CurrentEquation {
  operand1: String,
  operand2: String,
  operator: String
}

#[tauri::command]
fn process_num_button(button: &str, state: tauri::State<'_, Mutex<CurrentEquation>>) -> String {
  let mut state = state.lock().unwrap();

  match button {
    "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" | "." => {
        let update_operand = |operand: &mut String| {
            if operand == "0" && button != "." {
                *operand = button.to_string();
            } else {
                if button == "." {
                    if operand.is_empty() {
                        *operand = String::from("0");
                    } 
                    match operand.find('.') {
                        Some(_) => {},
                        None => operand.push_str(button)
                    }
                } else {
                    operand.push_str(button)
                }
            }
            operand.clone()
        };

        update_operand(
            if state.operator.is_empty() {
                &mut state.operand1
            } else {
                &mut state.operand2
            }
        )
    }
    "+/-" => {
        let update_operand = |operand: &mut String| {
            if operand.chars().next().unwrap() == '-' {
                *operand = operand[1..].to_string();
            } else {
                if operand != "0" {
                    *operand = format!("{}{}", "-", operand);
                }
            }
            operand.clone()
        };

        update_operand(
            if state.operator.is_empty() {
                &mut state.operand1
            } else if state.operand2.is_empty() {
                return state.operand1.clone();
            } else {
                &mut state.operand2
            }
        )
    }
    _ => {
        // unknown operation
        state.operand1 = String::from("0");
        state.operand2.clear();
        state.operator.clear();
                    
        String::from("error")
    }
  }
}

fn convertion(operand: String) -> Result<Operand> {
    if operand.contains('.') {
        match operand.trim().parse::<f64>() {
            Ok(value) => Ok(Operand::Float(value)),
            Err(_) => Err(anyhow!("convertion error"))
        }
    } else {
        match operand.trim().parse::<i64>() {
            Ok(value) => Ok(Operand::Int(value)),
            Err(_) => Err(anyhow!("convertion error"))
        }
    }
}

#[tauri::command]
fn process_operation_button(button: &str, state: tauri::State<'_, Mutex<CurrentEquation>>) -> String {
    let mut state = state.lock().unwrap();
 
    let result = || {
        let op1 = match convertion(state.operand1.clone()) {
            Ok(value) => value,
            Err(e) => return Err(e)
        };
        let op2 = match convertion(state.operand2.clone()) {
            Ok(value) => value,
            Err(e) => return Err(e)
        };

        match state.operator.as_str() {
            "+" => Ok(op1.add(op2)),
            "-" => Ok(op1.subtract(op2)),
            "*" => Ok(op1.multiply(op2)),
            "/" => {
                match op1.divide(op2) {
                    Ok(res) => Ok(res),
                    Err(e) => Err(e)
                }
            }
            _ => Err(anyhow!("unsupported operator"))
        }
    };

    match button {
        "AC" => {
            state.operand1 = String::from("0");
            state.operand2.clear();
            state.operator.clear();

            state.operand1.clone()
        }
        "+" | "-" | "*" | "/" => {
            if !state.operand2.is_empty() {
                state.operand1 = match result() {
                    Ok(res) => res,
                    Err(_) => {
                        // converion error, division by zero and unsupported operator
                        state.operand1 = String::from("0");
                        state.operand2.clear();
                        state.operator.clear();

                        return String::from("error");
                    }
                };
                state.operand2.clear();
            }

            state.operator = button.to_string();

            state.operand1.clone()
        }
        "=" => {
            if !state.operand2.is_empty() {
                state.operand1 = match result() {
                    Ok(res) => res,
                    Err(_) => {
                        // converion error, division by zero and unsupported operator
                        state.operand1 = String::from("0");
                        state.operand2.clear();
                        state.operator.clear();
                        
                        return String::from("error");
                    }
                };

                state.operand2.clear();
                state.operator.clear();
            }

            state.operand1.clone()
        }
        _ => {
            // unknown operation
            state.operand1 = String::from("0");
            state.operand2.clear();
            state.operator.clear();
                        
            String::from("error")
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Mutex::new(CurrentEquation {
        operand1: String::from("0"), 
        operand2: String::from(""), 
        operator: String::from("")
    });

    tauri::Builder::default()
        .setup(|app| {
            let win_builder =
                WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                    .title("calc")
                    .inner_size(285.0, 468.0)
                    .resizable(false);
  
            #[cfg(target_os = "macos")]
            let win_builder = win_builder
                .title_bar_style(TitleBarStyle::Transparent)
                .hidden_title(true);

            let window = win_builder.build().unwrap();
  
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSWindow, NSWindowStyleMask, NSColor};
                use cocoa::base::{id, nil};

                let ns_window = window.ns_window().unwrap() as id;

                unsafe {
                    let style_mask = ns_window.styleMask()
                        & !NSWindowStyleMask::NSMiniaturizableWindowMask
                        & !NSWindowStyleMask::NSResizableWindowMask;

                    ns_window.setStyleMask_(style_mask);

                    let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                        nil,
                        0.0 / 255.0,
                        0.0 / 255.0,
                        0.0 / 255.0,
                        1.0,
                    );

                    ns_window.setBackgroundColor_(bg_color);
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            process_num_button, 
            process_operation_button
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
