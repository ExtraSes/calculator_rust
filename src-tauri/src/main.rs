mod calculator;

use calculator::{input_digit, input_operation, calculate_result, clear_calculator, CalculatorState};
use std::sync::Mutex;

fn main() {
    println!("🚀 Запуск простого калькулятора");
    
    tauri::Builder::default()
        .manage(CalculatorState {
            display: Mutex::new("0".to_string()),
            previous_value: Mutex::new(None),
            operation: Mutex::new(None),
            waiting_for_operand: Mutex::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            input_digit,
            input_operation,
            calculate_result,
            clear_calculator
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}