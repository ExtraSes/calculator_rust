// Импортируем необходимые модули из Tauri
use tauri::command;
use std::sync::Mutex;

// Структура для хранения состояния калькулятора
// Mutex нужен для безопасного доступа из разных потоков
pub struct CalculatorState {
    pub display: Mutex<String>,              // Текущее отображаемое значение
    pub previous_value: Mutex<Option<f64>>,  // Предыдущее значение для вычислений
    pub operation: Mutex<Option<String>>,    // Текущая операция (+, -, *, /)
    pub waiting_for_operand: Mutex<bool>,    // Флаг ожидания нового операнда
}

// Команда для ввода цифры
// #[command] - макрос Tauri, делает функцию доступной из фронтенда
#[command]
pub fn input_digit(digit: String, state: tauri::State<CalculatorState>) -> String {
    // Получаем блокировку для изменения display
    let mut display = state.display.lock().unwrap();
    // Получаем блокировку для проверки флага ожидания
    let mut waiting = state.waiting_for_operand.lock().unwrap();
    
    // Если ждем новый операнд (после операции)
    if *waiting {
        *display = digit.clone();  // Заменяем текущее значение
        *waiting = false;          // Сбрасываем флаг ожидания
    } else {
        // Если это продолжение ввода числа
        if *display == "0" {
            *display = digit.clone();  // Заменяем "0" на новую цифру
        } else {
            display.push_str(&digit);  // Добавляем цифру к существующему числу
        }
    }
    
    // Возвращаем обновленное значение для отображения
    display.clone()
}

// Команда для ввода операции (+, -, *, /)
#[command]
pub fn input_operation(op: String, state: tauri::State<CalculatorState>) -> String {
    // Получаем блокировки для всех полей состояния
    let mut display = state.display.lock().unwrap();
    let mut prev_value = state.previous_value.lock().unwrap();
    let mut operation = state.operation.lock().unwrap();
    let mut waiting = state.waiting_for_operand.lock().unwrap();
    
    // Парсим текущее отображаемое значение в число
    let input_value = display.parse::<f64>().unwrap_or(0.0);
    
    // Если это первая операция (нет предыдущего значения)
    if prev_value.is_none() {
        *prev_value = Some(input_value);  // Сохраняем текущее значение
    } else if operation.is_some() {
        // Если уже есть операция, выполняем предыдущее вычисление
        let current_value = prev_value.unwrap_or(0.0);
        let new_value = calculate(current_value, input_value, operation.as_ref().unwrap());
        
        // Обновляем отображение результатом
        *display = format!("{}", new_value);
        *prev_value = Some(new_value);  // Сохраняем результат как предыдущее значение
    }
    
    // Устанавливаем флаги для новой операции
    *waiting = true;           // Следующий ввод будет новым операндом
    *operation = Some(op);     // Сохраняем тип операции
    
    // Возвращаем текущее отображаемое значение
    display.clone()
}

// Команда для выполнения вычисления (кнопка "=")
#[command]
pub fn calculate_result(state: tauri::State<CalculatorState>) -> String {
    // Получаем блокировки для всех полей
    let mut display = state.display.lock().unwrap();
    let mut prev_value = state.previous_value.lock().unwrap();
    let mut operation = state.operation.lock().unwrap();
    let mut waiting = state.waiting_for_operand.lock().unwrap();
    
    // Парсим текущее значение
    let input_value = display.parse::<f64>().unwrap_or(0.0);
    
    // Если есть предыдущее значение и операция
    if prev_value.is_some() && operation.is_some() {
        // Выполняем вычисление
        let new_value = calculate(prev_value.unwrap(), input_value, operation.as_ref().unwrap());
        *display = format!("{}", new_value);  // Обновляем отображение
        *prev_value = None;                   // Очищаем предыдущее значение
        *operation = None;                    // Очищаем операцию
        *waiting = true;                      // Готовы к новому вычислению
    }
    
    // Возвращаем результат
    display.clone()
}

// Команда для очистки калькулятора (кнопка "C")
#[command]
pub fn clear_calculator(state: tauri::State<CalculatorState>) -> String {
    // Получаем блокировки для всех полей
    let mut display = state.display.lock().unwrap();
    let mut prev_value = state.previous_value.lock().unwrap();
    let mut operation = state.operation.lock().unwrap();
    let mut waiting = state.waiting_for_operand.lock().unwrap();
    
    // Сбрасываем все поля к начальному состоянию
    *display = "0".to_string();  // Отображение показывает "0"
    *prev_value = None;          // Нет предыдущего значения
    *operation = None;           // Нет операции
    *waiting = false;            // Не ждем новый операнд
    
    // Возвращаем "0"
    display.clone()
}

// Вспомогательная функция для выполнения математических операций
fn calculate(first: f64, second: f64, op: &str) -> f64 {
    match op {
        "+" => first + second,    // Сложение
        "-" => first - second,    // Вычитание
        "*" => first * second,    // Умножение
        "/" => {
            if second != 0.0 {
                first / second    // Деление (если не на ноль)
            } else {
                0.0               // Возвращаем 0 при делении на ноль
            }
        },
        _ => second,              // По умолчанию возвращаем второе число
    }
}
