use std::io;

fn main() {
    println!("당신의 무게를 입력하세요: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .unwrap();

    let weight: f32 = input.trim().parse()
        .unwrap();

    let mars_weight = calculator_weight_on_mars(weight);

    println!("Weight on mars: {}", mars_weight);
}

fn calculator_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.7111
}