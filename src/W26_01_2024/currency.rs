use std::io;
pub fn currency() {
    println!("\nКакая валюта у вас имеется на руках?

    1. Доллары
    
    2. Рубли
    
    3. Евро
    
    4. Йены
    ");
    let mut buffer: String = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    match buffer.trim().parse::<i32>() {
        Ok(n) => {
            println!("\nСколько у вас этой валюты?\n");
            let mut buffer: String = String::new();
            let _ = io::stdin().read_line(&mut buffer);
            match buffer.trim().parse::<i32>() {
                Ok(k) => {
            match n {
                1 =>  flight(vec![10.42, 7.20], k as f64),
                2 => flight(vec![0.11, 0.08], k as f64),
                3 => flight(vec![11.28, 7.79], k as f64),
                4 => flight(vec![0.07, 0.04], k as f64),
                _ => print!("\nВы выбрали номер несуществующей валюты.")
            }
                }
                Err(_) => print!("\nВ следующий раз введите целое число в консоль.\n\n"),
            }

        }
        Err(_) => print!("\nВ следующий раз введите целое число в консоль.\n\n"),
    };
}
fn flight(currency_rate: Vec<f64>, currency_ammount: f64) {
    println!("\nКуда собираетесь лететь?\n\t
    1. Швеция\n\t
    2. Китай\n
    ");
    let mut buffer: String = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    match buffer.trim().parse::<i32>() {
        Ok(n) => {
            match n {
                1 => {
                    let total_currency = currency_ammount * currency_rate[0];
                    print!("\nПолетев в Швецию у вас будет {} Шведских крон\n\n", total_currency);
                }, 
                2 => {
                    let total_currency = currency_ammount * currency_rate[1];
                    print!("\nПолетев в Китай у вас будет {} Китайских юаней\n\n", total_currency);
                },
                _ => print!("\nБольще cтран пока что нет.\n\n")
            }
        }
        Err(_) => print!("\nВ следующий раз введите целое число в консоль.\n\n"),
    };
}