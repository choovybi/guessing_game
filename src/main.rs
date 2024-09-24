use std::io; // prelude
use rand::Rng; // trait(接口)
use std::cmp::Ordering;

fn main() {
    println!("猜数!");
    let secret_number = rand::thread_rng().gen_range(1, 101);  // 左闭右开
    println!("神秘数字是{}", secret_number);
    println!("猜测一个数!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
    // io::Result Ok, Err

    // shadow
    let guess:u32 = guess.trim().parse().expect("Please type a number!");

    println!("你猜测的数是：{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),  // arm
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big"),
    }
}
