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
        
        // Сохраняем в базу данных через прямой SQL запрос
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            println!("\n🔄 Попытка сохранения в БД...");
            match save_to_database_direct(expression.clone(), result_str.clone()).await {
                Ok(_) => {
                    println!("✅ Вычисление успешно сохранено в БД!");
                    println!("   Выражение: {}", expression);
                    println!("   Результат: {}", result_str);
                },
                Err(e) => {
                    println!("❌ Ошибка сохранения в БД: {}", e);
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

// Функция для отправки данных напрямую в MariaDB через SQL запрос
async fn save_to_database_direct(expression: String, result: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    println!("🌐 Отправка данных напрямую в MariaDB...");
    println!("   Выражение: {}", expression);
    println!("   Результат: {}", result);
    
    // Способ 1: Через PHPMyAdmin SQL интерфейс
    let sql_url = "http://192.168.1.92:8080/sql.php";
    
    // Экранируем данные для SQL
    let escaped_expression = expression.replace("'", "\\'").replace("\"", "\\\"");
    let escaped_result = result.replace("'", "\\'").replace("\"", "\\\"");
    
    let sql_query = format!(
        "INSERT INTO govno (expression, result) VALUES ('{}', '{}')",
        escaped_expression, escaped_result
    );
    
    println!("   SQL запрос: {}", sql_query);
    
    // Отправляем SQL запрос через форму PHPMyAdmin
    let form_data = [
        ("sql_query", sql_query.as_str()),
        ("db", "testgovna"),
        ("goto", "sql.php"),
        ("back", "sql.php"),
    ];
    
    let response = client
        .post(sql_url)
        .form(&form_data)
        .send()
        .await?;
    
    let status = response.status();
    println!("   Статус ответа: {}", status);
    
    let response_text = response.text().await?;
    
    // Проверяем, есть ли в ответе признаки успешного выполнения
    if status.is_success() && (response_text.contains("Query executed successfully") || 
                               response_text.contains("Affected rows") ||
                               response_text.contains("INSERT")) {
        println!("✅ SQL запрос выполнен успешно");
        return Ok(());
    }
    
    // Способ 2: Через простой PHP скрипт (если есть)
    let php_url = "http://192.168.1.92:8080/save_calculation.php";
    let calculation = CalculationData { expression, result };
    
    println!("   Попытка через PHP скрипт: {}", php_url);
    
    let response = client
        .post(php_url)
        .json(&calculation)
        .send()
        .await?;
    
    let status = response.status();
    println!("   PHP скрипт статус: {}", status);
    
    if status.is_success() {
        let response_text = response.text().await?;
        println!("   PHP ответ: {}", response_text);
        if response_text.contains("success") {
            println!("✅ Данные сохранены через PHP скрипт");
            return Ok(());
        }
    }
    
    Err(format!("Не удалось сохранить данные в БД. Последний статус: {}", status).into())
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
