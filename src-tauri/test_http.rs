use reqwest;
use serde::Serialize;

#[derive(Serialize)]
struct CalculationData {
    expression: String,
    result: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Тестирование HTTP подключения к базе данных");
    
    // Тест 1: Проверка доступности тестового скрипта
    println!("\n1. Проверка тестового скрипта...");
    match reqwest::get("http://192.168.1.92:8080/test_db_connection.php").await {
        Ok(response) => {
            println!("   ✅ Тестовый скрипт доступен, статус: {}", response.status());
            let text = response.text().await?;
            println!("   Ответ:\n{}", text);
        }
        Err(e) => {
            println!("   ❌ Тестовый скрипт недоступен: {}", e);
        }
    }
    
    // Тест 2: Проверка сохранения данных
    println!("\n2. Тест сохранения данных...");
    let client = reqwest::Client::new();
    let calculation = CalculationData {
        expression: "2 + 3 = 5".to_string(),
        result: "5".to_string(),
    };
    
    match client
        .post("http://192.168.1.92:8080/save_calculation.php")
        .json(&calculation)
        .send()
        .await
    {
        Ok(response) => {
            println!("   ✅ Запрос отправлен, статус: {}", response.status());
            let text = response.text().await?;
            println!("   Ответ сервера: {}", text);
        }
        Err(e) => {
            println!("   ❌ Ошибка отправки: {}", e);
        }
    }
    
    Ok(())
}
