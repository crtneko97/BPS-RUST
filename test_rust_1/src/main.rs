use std::io;
use std::cmp::Ordering;
use rand::Rng;

/*
* So this is more or less my first piece of rust code.
* This is also the same code provided by the doc.rust-lang.org -- So nothing special here.
* BPS. 
*/
fn main() 
{

    println!("Gues the number!\n");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop 
    {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() 
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {guess}");

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => 
            {
                println!("You win!");
                break;
            }

        }

        
    }


}
