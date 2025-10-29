<?php
// Простой PHP скрипт для сохранения вычислений в базу данных
header('Content-Type: application/json');
header('Access-Control-Allow-Origin: *');
header('Access-Control-Allow-Methods: POST, GET, OPTIONS');
header('Access-Control-Allow-Headers: Content-Type');

if ($_SERVER['REQUEST_METHOD'] == 'OPTIONS') {
    exit(0);
}

// Настройки подключения к базе данных
$host = '192.168.1.92';
$port = '3306';
$dbname = 'testgovna';
$username = 'demid';
$password = 'Gandon345';

try {
    // Подключение к базе данных
    $pdo = new PDO("mysql:host=$host;port=$port;dbname=$dbname", $username, $password);
    $pdo->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
    
    if ($_SERVER['REQUEST_METHOD'] == 'POST') {
        // Получаем JSON данные
        $input = file_get_contents('php://input');
        $data = json_decode($input, true);
        
        if ($data && isset($data['expression']) && isset($data['result'])) {
            $expression = $data['expression'];
            $result = $data['result'];
            
            // Вставляем данные в таблицу govno
            $stmt = $pdo->prepare("INSERT INTO govno (expression, result) VALUES (?, ?)");
            $stmt->execute([$expression, $result]);
            
            echo json_encode([
                'success' => true,
                'message' => 'Вычисление сохранено в базу данных',
                'id' => $pdo->lastInsertId()
            ]);
        } else {
            echo json_encode([
                'success' => false,
                'message' => 'Неверные данные'
            ]);
        }
    } else {
        echo json_encode([
            'success' => false,
            'message' => 'Только POST запросы'
        ]);
    }
    
} catch (PDOException $e) {
    echo json_encode([
        'success' => false,
        'message' => 'Ошибка базы данных: ' . $e->getMessage()
    ]);
}
?>
