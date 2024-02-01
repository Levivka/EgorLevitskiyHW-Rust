use std::io;

mod W26_01_2024;

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
            1 => {W26_01_2024::First::wastes()}
            2 => {W26_01_2024::Second::capital()}
            _ => print!("\nНеа, такой работки нет\n")
        }},
        Err(_) => print!("\nВведите число в поле ввода \n"),
    }; 
}
}
