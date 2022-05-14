use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // randomowa liczba
    let random_num = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Enter the positive integer: ");

        // buffer na input
        let mut guess = String::new();
        
        // czytanie z konsoli
        io::stdin()
            .read_line(&mut guess)
            .expect("Coś poszło nie tak");
    
        // konwersja na typ liczbowy (unsigned 32-bit integer)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        // porównanie wygenerowanej liczby z inputem
        match guess.cmp(&random_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            },
        }        
    }
}