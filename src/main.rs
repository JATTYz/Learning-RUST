use std::io::{self, Write};
use rand::Rng;

fn main() {

    const PRICE_PER_TIME: i32 = 5;

    let mut guessing_times: i32 = 0;

    loop {
        println!("Guess Number Between 1-10");
        print!("Type a number: ");
        io::stdout().flush().unwrap();
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        println!("You guessd: {}", guess);

        let random_string = generate_random_string();
        let is_correct_number = guess.contains(&random_string);

        if is_correct_number{
            println!("CORRECT!");
            println!("Number of guessing {} \n", guessing_times);
            println!("TOTAL PRICE FOR THIS GAME ${}", guessing_times*PRICE_PER_TIME);
            return;
        }else{
            println!("TRY AGAIN!");
            guessing_times += 1;
        }
        println!("THE CORRECT NUMBER IS {}", random_string);
        println!("Number of guessing {} \n", guessing_times);
    }

}


fn generate_random_string() -> String {
    let mut rng = rand::thread_rng();

    let random_string: String = rng.gen_range(1..=10).to_string();
    
    random_string
}