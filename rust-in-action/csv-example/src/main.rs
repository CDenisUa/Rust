fn main() {  // 1
                // 2
    let penguin_data = "\
    common name, length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin, 60
    Invalid, data
    ";
    
    let records  = penguin_data.lines();

    for(i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 { // 3
            continue;
        }

        let fields: Vec<_> = record // 4
        .split(',') // 5
        .map(|field| field.trim()) // 6
        .collect(); // 7    
        if cfg!(debug_assertions) { // 8
            eprintln!("debug: {:?} -> {:?}", 
                    record, fields);
                    // 9
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() { // 10
            println!("{}, {}cm", name, length); // 11
        }
    }
}

/*
(1) Исполняемым проектам требуется функция main ()
(2) Отключение завершающего символа новой строки
(3) Пропуск строки заголовка и строк, состоящих из одних пробелов
(4) Начало со строки текста
(5) Разбиение записи на поля
(6) Обрезка пробелов в каждом поле
(7) Сборка набора полей
(8) cfg! проверяет конфигурацию в процессе компиляции.
(9) eprintln! выводит данные на стандартное устройство сообщений об ошибках (stderr)
(10) Попытка выполнения парсинга поля в виде числа с плавающей точкой
(11) println! помещает данные на стандартное устройство вывода (stdout).
*/
