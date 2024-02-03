use rand::prelude::*;

pub fn wastes() {
    let mut rng = rand::thread_rng();
    let wastes: Vec<i32> = vec![7; rng.gen_range(1..1000)];
    
    print!("\nТраты за неделю по дням [Пн-Вс]: {:?}", wastes);
    let mut sum = 0;
    
    for i in wastes {
        sum += i;
    }
    
    print!("\n\tСумма трат за неделю {}", sum);
}
