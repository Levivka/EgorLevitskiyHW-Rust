use std::time::SystemTime;

use chrono::prelude::*;

pub fn time_bot() {
    
    match chrono::Local::now().time().hour() {
        4..=11 => {print!("Доброе утро")}
        12..=17 => {print!("Добрый день")}
        18..=22 => {print!("Добрый вечер")}
        23 => {print!("Доброй ночи")}
        0..=3 => {print!("Доброй ночи")}
        _ => {print!("Вы находитесь вне времени и пространства")}
    }
}