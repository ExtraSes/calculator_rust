use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Calculation {
    pub expression: String,
    pub result: String,
}

// Альтернативная реализация через HTTP запросы к PHPMyAdmin
// Это можно использовать, если установка MySQL клиента вызывает проблемы

pub async fn save_calculation_http(expression: String, result: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    // URL для PHPMyAdmin API (если доступен)
    let url = "http://192.168.1.92:8080/api/calculations";
    
    let calculation = Calculation { expression, result };
    
    let response = client
        .post(url)
        .json(&calculation)
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("Вычисление сохранено в базу данных");
    } else {
        println!("Ошибка при сохранении: {}", response.status());
    }
    
    Ok(())
}

// Простая реализация через файл (для тестирования)
pub fn save_calculation_file(expression: String, result: String) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("calculations.txt")?;
    
    writeln!(file, "{} = {}", expression, result)?;
    println!("Вычисление сохранено в файл: {} = {}", expression, result);
    
    Ok(())
}
