use std::io;
use std::io::Write;

use rand::prelude::*;


pub fn guessing() {
    let mut buffer: String = String::new();
    let mut rng = rand::thread_rng();
    let  secret_number = rng.gen_range(1..9);
    let mut attempts: u32 = 0;
    println!("Выберите уровень сложности
    1. Лёгкий - 10 попыток
    2. Средний - 5 попыток
    3. Тяжёлый - 3 попытки");
    
    let _ = io::stdin().read_line(&mut buffer);
    match buffer.trim().parse::<i32>() {
        Ok(n) => {
            match n {
            1 => attempts = 10,
            2 => attempts = 5,
            3 => attempts = 3,
            _ => println!("Введите число от 1 до 3\n")
        }
        }
        Err(_) => println!("В следующий раз введите целое число в консоль.\n"),
    };
    buffer.clear();
    println!("Введите число от 1 до 9, чтобы узнать было оно больше или меньше загаданного:");
    
    loop {
        if attempts > 0 {
            let _ = io::stdin().read_line(&mut buffer);
            match buffer.trim().parse::<i32>() {
                Ok(n) => {
                    if secret_number > n {
                        println!("Загаданное число больше вашего");
                        attempts -=1;
                    }
                    else if secret_number < n {
                        println!("Загаданное число меньше вашего");
                        attempts -=1;
                    }
                    else {
                        println!("В яблочко загаданным числом было {}", secret_number);
                        break;
                    }
                }
                Err(_) => println!("В следующий раз введите целое число от 1 до 9 в консоль.\n"),
            }
            buffer.clear()
        }
        else {
            println!("У вас кончились попытки, загаданное число было {}", secret_number);
            break;
        }
    }
}
