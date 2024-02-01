use std::io;

pub fn capital() {
    print!("\nВведите начальную сумму:\n");
    let mut buffer: String = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    match buffer.trim().parse::<i32>() {
        Ok(n) => {
            let mut savings = n as f64;
            let mut years = 0;
            while savings < 1000000.0 {
                savings += (savings / 100.0) * 10.0;
                years += 1;
            }
            print!("\nЧерез {} лет вы накопите {} денег и достигните цели в 1млн.", years, savings as i32);
        }
        Err(_) => print!("\n В следующий раз введите целое число в консоль.\n\n"),
    };
}