use std::io;

mod w26_01_2024;
mod w29_01_2024;

fn main() {
    loop {
    print!("\nКакую работу запускаем?

    1. [26.01.2024] Траты за неделю
    2. [26.01.2024] Накопления
    3. [26.01.2024] Конвертор валют
    4. [29.01.2024] Робот Хронос
    5. [29.01.2024] Угадывание числа

    Для закрытия программы используйте ctrl + c\n");
    let mut buffer: String = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    match buffer.trim().parse::<i32>() {
        Ok(n) => {    
            match n {
            1 => {w26_01_2024::wastes::wastes()}
            2 => {w26_01_2024::capital::capital()}
            3 => {w26_01_2024::currency::currency()}
            4 => {w29_01_2024::time_bot::time_bot()}
            5 => {w29_01_2024::guessing::guessing()}
            _ => print!("\nНеа, такой работки нет\n")
        }},
        Err(_) => print!("\nВведите число в поле ввода \n"),
    }; 
}
}
