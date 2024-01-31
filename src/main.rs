mod guessing;

use rand::Rng;

const PRICE_PER_TIME: i32 = 5;
const PRIZE: i32 = 50;
fn main() {

    let mut _guessing_times: i32 = 0;
    let mut profits: i32 = 0;

    for _ in 0..1000 {
        let input_random_string = generate_random_string();
        let random_string = generate_random_string();
        let is_correct_number = input_random_string == random_string;

        if is_correct_number{
            profits -= PRIZE;
            _guessing_times = 0;
        }else{
            _guessing_times += 1;
            profits += PRICE_PER_TIME;
        }
    }

    println!("{}", profits);
}

fn generate_random_string() -> String {
    let mut rng = rand::thread_rng();

    let random_string: String = rng.gen_range(1..=10).to_string();
    
    random_string
}

