use std::{io, vec};
use rand::Rng;

#[derive(Debug, Clone)]
struct Enemy {
      name: String,
      damage: i32,
      steal: i32
}

#[derive(Clone)]
struct King {
    hp: i32,
    gold: i32
}

#[derive(Clone)]
struct Royalty {
    hp: i32,
    income: i32
}

#[derive(Clone)]
struct Game {
    king: King,
    army: Vec<Royalty>,
    wave_num: i32,
    wave: Vec<Enemy>
}

impl Royalty {
    pub fn new(hp: i32, income: i32) -> Self {
        Self { hp, income }
    }
}

impl King {
    pub fn new(hp: i32, gold: i32) -> Self {
        Self { hp, gold}
    }
}

impl Enemy {
    pub fn new(name: String, damage: i32, steal: i32) -> Self {
        Self { name, damage, steal}
    }
}

impl Game {
    pub fn new(
        king: King,
        army: Vec<Royalty>,
        wave_num: i32,
        wave: Vec<Enemy>
    ) -> Self { Self { king, army, wave_num, wave } }

    pub fn wave_switch(self: &mut Self,) {
        let mut rng = rand::thread_rng();
        for _i in 0..=self.wave_num {
            match rng.gen_range(1..=10) {
                1..=4 => {
                    let enemy = Enemy::new("Рыцарь".to_string(),5, 0);
                    self.wave.push(enemy)
                }
                5..=8 => {
                    let enemy = Enemy::new("Вор".to_string(),10, 50);
                    self.wave.push(enemy)
                }
                9..=10 => {
                    let enemy = Enemy::new("Кавалерия".to_string(),50, 100);
                    self.wave.push(enemy)
                }
                _ => {eprintln!("Err with waves")} 
            };
        }
        self.wave_num +=1;
    }

    pub fn enemy_list(self: &Self) {
        println!("Против вас:");
        for i in self.wave.clone() {
            print!("{} ", i.name);
        }
    }

    pub fn purchases(self: &mut Self) {
        while self.king.gold != 0 {
        println!("\nЖелаете купить войска?
        Ваша казна хранит {} золота
        1. Пехотинец    [10 стоимость, 10 здоровья, 0 дохода]
        2. Министр  [50 стоимость, 5 здоровья, 100 дохода]
        3. Кавалерия    [150 стоимость, 100 здоровья, 0 дохода]
        0. Закончить закупку", self.king.gold);
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        match buffer.trim().parse::<i32>() {
            Ok(v) => {
                match v {
                1 => {
                    if self.king.gold > 10 {
                        self.king.gold -= 10;
                        let royalty = Royalty::new(10, 0);
                        self.army.push(royalty);
                        }
                }
                2 => {
                    if self.king.gold > 50 {
                        self.king.gold -= 50;
                        let royalty = Royalty::new(5, 100);
                        self.army.push(royalty);
                        }
                }
                3 => {
                    if self.king.gold > 150 {
                        self.king.gold -= 150;
                        let royalty = Royalty::new(100, 0);
                        self.army.push(royalty);
                        }
                }
                0 => {break;}
                _ => println!("Ошибка покупки войска")
            }
        }
            Err(_) => todo!(),
        }
        }
        }

        pub fn encounter(self: &mut Self) {
            let mut win = false;
            let mut player_power = 0; 
            for i in &self.army {
                player_power += i.hp;   
            }
            player_power += self.king.hp;
            let mut enemy_power = 0;
            for i in &self.wave {
                enemy_power += i.damage;    
            }

            if player_power > enemy_power {
                for unit in &mut self.army {
                    if unit.hp != 0 && unit.hp < enemy_power{
                        enemy_power -= unit.hp;
                        
                    }
                    else if unit.hp > enemy_power {
                        unit.hp -= enemy_power;
                        enemy_power = 0;
                        break;
                    }
                }
                if self.king.hp != 0 && enemy_power != 0 {
                    self.king.hp -= enemy_power;
                }
                if self.king.hp != 0 {
                println!("Ваше войско победило в сражении потеряв {} здороья", enemy_power);
                self.wave.clear();
                }
                else {
                    println!("Ваше королество пало под натиском вражеских сил");
                }
            }
        }
    }



pub fn play() {
    let mut game = Game::new(King::new(1, 2000), vec![],0, vec![]);
    loop {
        if game.king.hp != 0 {
    Game::wave_switch(&mut game);
    println!("Волна {}:", game.wave_num);
    Game::enemy_list(&game);
    Game::purchases(&mut game);
    Game::encounter(&mut game);
}
    else {
        println!("Вы проиграли...\n\n");
        break;
    }
    }

}