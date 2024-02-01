use std::io;

mod w26_01_2024;

fn main() {
    loop {
    print!("\nКакую работу запускаем?\n\t
    1. [26.01.2024] Траты за неделю\n\t
    2. [26.01.2024] Накопления
    \n\nДля закрытия программы используйте ctrl + c\n");
    let mut buffer: String = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    match buffer.trim().parse::<i32>() {
        Ok(n) => {    match n {
            1 => {w26_01_2024::first::wastes()}
            2 => {w26_01_2024::second::capital()}
            3 => {w26_01_2024::third::currency()}
            _ => print!("\nНеа, такой работки нет\n")
        }},
        Err(_) => print!("\nВведите число в поле ввода \n"),
    }; 
}
}
