<?php
// Тестовый скрипт для проверки подключения к базе данных
header('Content-Type: text/plain');

// Настройки подключения к базе данных
$host = '192.168.1.92';
$port = '3306';
$dbname = 'testgovna';
$username = 'demid';
$password = 'Gandon345';

echo "🔍 Тестирование подключения к базе данных...\n";
echo "Хост: $host\n";
echo "Порт: $port\n";
echo "База данных: $dbname\n";
echo "Пользователь: $username\n\n";

try {
    // Подключение к базе данных
    $pdo = new PDO("mysql:host=$host;port=$port;dbname=$dbname", $username, $password);
    $pdo->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
    
    echo "✅ Подключение к базе данных успешно!\n\n";
    
    // Проверяем существование таблицы govno
    $stmt = $pdo->query("SHOW TABLES LIKE 'govno'");
    if ($stmt->rowCount() > 0) {
        echo "✅ Таблица 'govno' существует\n";
        
        // Показываем структуру таблицы
        $stmt = $pdo->query("DESCRIBE govno");
        echo "\n📋 Структура таблицы 'govno':\n";
        while ($row = $stmt->fetch(PDO::FETCH_ASSOC)) {
            echo "- {$row['Field']}: {$row['Type']}\n";
        }
        
        // Показываем количество записей
        $stmt = $pdo->query("SELECT COUNT(*) as count FROM govno");
        $count = $stmt->fetch(PDO::FETCH_ASSOC)['count'];
        echo "\n📊 Количество записей в таблице: $count\n";
        
        // Показываем последние 5 записей
        if ($count > 0) {
            $stmt = $pdo->query("SELECT * FROM govno ORDER BY created_at DESC LIMIT 5");
            echo "\n📝 Последние 5 записей:\n";
            while ($row = $stmt->fetch(PDO::FETCH_ASSOC)) {
                echo "- ID: {$row['id']}, Выражение: {$row['expression']}, Результат: {$row['result']}, Время: {$row['created_at']}\n";
            }
        }
        
    } else {
        echo "❌ Таблица 'govno' не существует!\n";
        echo "Создайте таблицу командой:\n";
        echo "CREATE TABLE govno (\n";
        echo "    id INT PRIMARY KEY AUTO_INCREMENT,\n";
        echo "    expression VARCHAR(255) NOT NULL,\n";
        echo "    result VARCHAR(255) NOT NULL,\n";
        echo "    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP\n";
        echo ");\n";
    }
    
} catch (PDOException $e) {
    echo "❌ Ошибка подключения к базе данных: " . $e->getMessage() . "\n";
    echo "\nВозможные причины:\n";
    echo "1. Сервер MariaDB не запущен\n";
    echo "2. Неверные данные подключения\n";
    echo "3. База данных 'testgovna' не существует\n";
    echo "4. Пользователь 'demid' не имеет прав доступа\n";
}
?>
