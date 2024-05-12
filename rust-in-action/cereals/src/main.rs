#[derive(Debug)] // 1
enum Cereal { // 2
    Barley, Millet, Rice, 
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![]; // 3
    grains.push(Cereal::Rye); // 4
    drop(grains); // 5
    println!("{:?}", grains); // 6
}

/*
(1) Разрешение макросу println! вывести перечисление Cereal
(2) enurn (перечисление) - тип с фиксированным количеством допустимых вариантов37
(3) Инициализация пустого вектора Cereal
(4) Добавление элемента к вектору grains
(5) Удаление grains и его содержимого
(6) Попытка обращения к значению, которое уже удалено
*/