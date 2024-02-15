use std::io;
use std::ptr::null;

use crate::w13_02_2024::budget;

#[derive(Debug)]
struct Climber {
    name: String,
    age: u32,
    adress: String,
}

#[derive(Debug)]
struct Mountain {
    name: String,
    country: String,
    height: u32,
}

#[derive(Debug)]
struct Group {
    ammount_climbers: u32,
    climbers: Vec<Climber>,
    mountain: Mountain,
    recruitment_status: bool,
}

impl Group {
    pub fn new(
        ammount_climbers: u32,
        climbers: Vec<Climber>,
        mountain: Mountain,
        recruitment_status: bool,
    ) -> Self {
        Self {
            ammount_climbers,
            climbers,
            mountain,
            recruitment_status,
        }
    }

    pub fn add_member(self: &mut Self) {
        if self.recruitment_status {
            println!("\nВведите своё имя:\n");
            let mut buffer: String = String::new();
            let _ = io::stdin().read_line(&mut buffer);
            let name = &mut buffer.clone();
            buffer.clear();

            println!("Введите свой возраст:\n");
            let _ = io::stdin().read_line(&mut buffer);
            let mut age: u32 = 0;
            match buffer.trim().parse::<u32>() {
                Ok(v) => age = v,
                Err(_) => println!("В следующий раз введите целое положительное число\n\n"),
            }
            buffer.clear();

            println!("Введите адрес проживания одной строкой:\n");
            let _ = io::stdin().read_line(&mut buffer);
            let adress = &mut buffer.clone();
            buffer.clear();
            self.climbers
                .push(Climber::new(name.to_string(), age, adress.to_string()));

            if self.climbers.iter().count() >= self.ammount_climbers as usize {
                self.recruitment_status = false;
            }
        } else {
            println!("Набор в группу закрыт\n\n");
        }
    }

    pub fn display(self: &Self) {
        println!(
            "\nКоличество участников в группе: {}
            
                    Участники
------------------------------------------------",
            self.ammount_climbers
        );
        self.climbers.iter().for_each(|x| {
            println!(
                "\nИмя участника {}
Возраст участника {}
Адрес проживания {}",
                x.name, x.age, x.adress
            )
        });
        println!("\n------------------------------------------------");
        println!(
            "
                    Гора
------------------------------------------------"
        );
        println!(
            "\nГора на которую отправляется экспедиция: {}
Страна в которой гора находится: {}
Высота горы: {}",
            self.mountain.name, self.mountain.country, self.mountain.height
        );
        println!("\n------------------------------------------------");
        match self.recruitment_status {
            true => println!("\nСтатус группы: набор открыт"),
            false => println!("\nСтатус группы: набор закрыт"),
        }
    }
}

impl Climber {
    pub fn new(name: String, age: u32, adress: String) -> Self {
        Self { name, age, adress }
    }
}

impl Mountain {
    pub fn new(name: String, country: String, height: u32) -> Self {
        Self {
            name,
            country,
            height,
        }
    }
}

pub fn alpinist_fn() {
    let mut first_group = Group::new(
        3,
        vec![
            Climber::new(
                "Айсберг-Витязь".to_string(),
                42,
                "Ущелье Бурного Штурма д.10".to_string(),
            ),
            Climber::new(
                "Вершинник Взбиралов".to_string(),
                68,
                "Ущелье Бурного Штурма д.148".to_string(),
            ),
            Climber::new(
                "Трудогорец Вершинович".to_string(),
                154,
                "Деревня Вершиноград, ул.Высотная, д.70".to_string(),
            ),
        ],
        Mountain::new(
            "Штурмовая скала".to_string(),
            "Штурмария".to_string(),
            999999,
        ),
        false,
    );

    let mut second_group = Group::new(
        3,
        vec![],
        Mountain::new("Высокая гора".to_string(), "Штурмария".to_string(), 1000),
        true,
    );

    let mut third_group = Group::new(
        3,
        vec![],
        Mountain::new(
            "Не очень высокая гора".to_string(),
            "Штурмария".to_string(),
            100,
        ),
        true,
    );
    loop {
        println!(
            "
Доска объявлений:
1 группа - {}
2 группа - {}
3 группа - {}",
            {
                match first_group.recruitment_status {
                    true => "Набор открыт",
                    false => "Набор закрыт",
                }
            },
            {
                match second_group.recruitment_status {
                    true => "Набор открыт",
                    false => "Набор закрыт",
                }
            },
            {
                match third_group.recruitment_status {
                    true => "Набор открыт",
                    false => "Набор закрыт",
                }
            }
        );

        println!("\nЧтобы вывести информацию о группе введите её номер, для выхода введите 0:\n");
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        match buffer.trim().parse::<u32>() {
            Ok(v) => match v {
                1 => first_group.display(),
                2 => second_group.display(),
                3 => third_group.display(),
                0 => {
                    break;
                }
                _ => println!("\nВведите корректный номер группы!\n"),
            },
            Err(_) => println!("В следующий раз введите целое положительное число\n\n"),
        }
        buffer.clear();
    }
}
