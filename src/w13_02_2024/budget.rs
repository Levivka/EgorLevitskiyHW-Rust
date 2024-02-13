use std::io;

use crate::w26_01_2024::wastes;

pub fn budget_input() {
    loop {
        print!("\nВведите свою зарплату\n");
        let mut salary = 0;
        let mut buffer: String = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        match buffer.trim().parse::<i32>() {
            Ok(n) => salary = n,
            Err(_) => {
                print!("\n В следующий раз введите целое число в консоль.\n\n");
                break;
            }
        };
        buffer.clear();

        print!("\nСколько у вас сбережений?\n");
        let mut savings = 0;
        let mut buffer: String = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        match buffer.trim().parse::<i32>() {
            Ok(n) => savings = n,
            Err(_) => {
                print!("\n В следующий раз введите целое число в консоль.\n\n");
                break;
            }
        };
        buffer.clear();

        print!("\nСколько планируете потратить на транспорт этом месяце?\n");
        let mut wastes_move = 0;
        let mut buffer: String = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        match buffer.trim().parse::<i32>() {
            Ok(n) => wastes_move = n,
            Err(_) => {
                print!("\n В следующий раз введите целое число в консоль.\n\n");
                break;
            }
        };
        buffer.clear();

        print!("\nСколько планируете потратить на транспорт этом месяце?\n");
        let mut wastes_food = 0;
        let mut buffer: String = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        match buffer.trim().parse::<i32>() {
            Ok(n) => wastes_food = n,
            Err(_) => {
                print!("\n В следующий раз введите целое число в консоль.\n\n");
                break;
            }
        };
        buffer.clear();
        budget_calc(salary, savings, wastes_move, wastes_food);
        break;
    }
}

pub fn budget_calc(salary: i32, savings: i32, wastes_move: i32, wastes_food: i32, ) {
    let wastes = wastes_food + wastes_move;
    if wastes > salary {
        println!("\nВаши планируемы траты за месяц превышают вашу зарплату на {} рублей \n", wastes - salary);
        if wastes_food > wastes_move {
            println!("Ваши траты на еду больше затрат на транспорт, попробуйте сэкономить на них");
        }
        else {
            println!("Ваши траты на транспорт больше затрат на еду, иногда лучше прогулятся пешком!");
        }
        if savings != 0 && savings > wastes - salary{
            println!("Так-же вы можете закрыть недостаток из сбережений, тогда у вас останется {} рублей \n\n", {savings - (wastes - salary)});
        }
        else {
            println!("Даже если вы потратите все свои сбережния вы не сможете столько потратить.\n\n")
        }
    }
    else {
        println!("\nВаши траты укладываются в ваш заработок, после затрат у вас останется {} рублей\n\n", salary - wastes);
    }
}
