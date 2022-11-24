extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("숫자게임");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("랜덤으로 생성되는 정수: {}", secret_number);

    println!("사용자 입력");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("읽지 못했습니다.");

    println!("사용자가 입력한 정수: {}", guess);
}