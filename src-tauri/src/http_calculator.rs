use tauri::command;
use std::sync::Mutex;
use serde::Serialize;
use reqwest;

// Структура для хранения состояния калькулятора
pub struct CalculatorState {
    pub display: Mutex<String>,
    pub previous_value: Mutex<Option<f64>>,
    pub operation: Mutex<Option<String>>,
    pub waiting_for_operand: Mutex<bool>,
}

#[derive(Serialize)]
struct CalculationData {
    expression: String,
    result: String,
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
        
        // Формируем выражение для сохранения
        let expression = format!("{} {} {} = {}", 
            prev_value.unwrap(), 
            operation.as_ref().unwrap(), 
            input_value, 
            result_str
        );
        
        // Сохраняем в базу данных через HTTP
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            println!("\n🔄 Попытка сохранения в БД...");
            match save_to_database_http(expression.clone(), result_str.clone()).await {
                Ok(_) => {
                    println!("✅ Вычисление успешно отправлено в БД!");
                    println!("   Выражение: {}", expression);
                    println!("   Результат: {}", result_str);
                },
                Err(e) => {
                    println!("❌ Ошибка отправки в БД: {}", e);
                    println!("📁 Сохраняем в файл как резервный вариант...");
                    save_to_file_fallback(expression, result_str.clone());
                }
            }
        });
        
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

// Функция для отправки данных в базу через HTTP
async fn save_to_database_http(expression: String, result: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    println!("🌐 Отправка данных в БД...");
    println!("   Выражение: {}", expression);
    println!("   Результат: {}", result);
    
    // Способ 1: Через простой PHP скрипт
    let php_url = "http://192.168.1.92:8080/save_calculation.php";
    let calculation = CalculationData { expression: expression.clone(), result: result.clone() };
    
    println!("   URL: {}", php_url);
    
    let response = client
        .post(php_url)
        .json(&calculation)
        .send()
        .await?;
    
    let status = response.status();
    println!("   Статус ответа: {}", status);
    
    let response_text = response.text().await?;
    println!("   Ответ сервера: {}", response_text);
    
    if status.is_success() {
        println!("✅ Данные успешно отправлены в БД");
        return Ok(());
    }
    
    // Способ 2: Прямой SQL запрос через PHPMyAdmin API (если доступен)
    let sql_url = "http://192.168.1.92:8080/import.php";
    let sql_data = format!(
        "INSERT INTO govno (expression, result) VALUES ('{}', '{}')",
        expression.replace("'", "\\'"),
        result.replace("'", "\\'")
    );
    
    println!("   Попытка через SQL API: {}", sql_url);
    
    let response = client
        .post(sql_url)
        .form(&[("sql", &sql_data)])
        .send()
        .await?;
    
    println!("   SQL API статус: {}", response.status());
    
    if response.status().is_success() {
        println!("✅ Данные отправлены через SQL API");
        return Ok(());
    }
    
    Err(format!("Не удалось отправить данные в базу. Последний статус: {}", response.status()).into())
}

// Функция для сохранения в файл как fallback
fn save_to_file_fallback(expression: String, result: String) {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("calculations.txt") {
        let _ = writeln!(file, "{} = {}", expression, result);
        println!("📁 Вычисление сохранено в файл: {} = {}", expression, result);
    } else {
        println!("❌ Ошибка при сохранении в файл");
    }
}
