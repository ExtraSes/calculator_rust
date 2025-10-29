use tauri::command;
use std::sync::Mutex;
use std::fs::OpenOptions;
use std::io::Write;

// Структура для хранения состояния калькулятора
pub struct CalculatorState {
    pub display: Mutex<String>,
    pub previous_value: Mutex<Option<f64>>,
    pub operation: Mutex<Option<String>>,
    pub waiting_for_operand: Mutex<bool>,
}

#[command]
pub fn input_digit(digit: String, state: tauri::State<CalculatorState>) -> String {
    let mut display = state.display.lock().unwrap();
    let mut waiting = state.waiting_for_operand.lock().unwrap();
    
    if *waiting {
        *display = digit.clone();
        *waiting = false;
    } else {
        if *display == "0" {
            *display = digit.clone();
        } else {
            display.push_str(&digit);
        }
    }
    
    display.clone()
}

#[command]
pub fn input_operation(op: String, state: tauri::State<CalculatorState>) -> String {
    let mut display = state.display.lock().unwrap();
    let mut prev_value = state.previous_value.lock().unwrap();
    let mut operation = state.operation.lock().unwrap();
    let mut waiting = state.waiting_for_operand.lock().unwrap();
    
    let input_value = display.parse::<f64>().unwrap_or(0.0);
    
    if prev_value.is_none() {
        *prev_value = Some(input_value);
    } else if operation.is_some() {
        let current_value = prev_value.unwrap_or(0.0);
        let new_value = calculate(current_value, input_value, operation.as_ref().unwrap());
        *display = format!("{}", new_value);
        *prev_value = Some(new_value);
    }
    
    *waiting = true;
    *operation = Some(op);
    
    display.clone()
}

#[command]
pub fn calculate_result(state: tauri::State<CalculatorState>) -> String {
    let mut display = state.display.lock().unwrap();
    let mut prev_value = state.previous_value.lock().unwrap();
    let mut operation = state.operation.lock().unwrap();
    let mut waiting = state.waiting_for_operand.lock().unwrap();
    
    let input_value = display.parse::<f64>().unwrap_or(0.0);
    
    if prev_value.is_some() && operation.is_some() {
        let new_value = calculate(prev_value.unwrap(), input_value, operation.as_ref().unwrap());
        let result_str = format!("{}", new_value);
        
        // Сохраняем вычисление в файл
        let expression = format!("{} {} {} = {}", 
            prev_value.unwrap(), 
            operation.as_ref().unwrap(), 
            input_value, 
            result_str
        );
        
        save_calculation_to_file(expression, result_str.clone());
        
        *display = result_str;
        *prev_value = None;
        *operation = None;
        *waiting = true;
    }
    
    display.clone()
}

#[command]
pub fn clear_calculator(state: tauri::State<CalculatorState>) -> String {
    let mut display = state.display.lock().unwrap();
    let mut prev_value = state.previous_value.lock().unwrap();
    let mut operation = state.operation.lock().unwrap();
    let mut waiting = state.waiting_for_operand.lock().unwrap();
    
    *display = "0".to_string();
    *prev_value = None;
    *operation = None;
    *waiting = false;
    
    display.clone()
}

fn calculate(first: f64, second: f64, op: &str) -> f64 {
    match op {
        "+" => first + second,
        "-" => first - second,
        "*" => first * second,
        "/" => {
            if second != 0.0 {
                first / second
            } else {
                0.0
            }
        },
        _ => second,
    }
}

fn save_calculation_to_file(expression: String, result: String) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("calculations.txt") {
        let _ = writeln!(file, "{} = {}", expression, result);
        println!("Вычисление сохранено в файл: {} = {}", expression, result);
    } else {
        println!("Ошибка при сохранении в файл");
    }
}
