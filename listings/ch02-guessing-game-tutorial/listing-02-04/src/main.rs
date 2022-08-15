// ANCHOR: here
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--
    // ANCHOR_END: here
    println!("Uhádněte číslo.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Tajné číslo: {secret_number}");

    println!("adejte svůj tip:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Chyba při čtení řádku");
    // ANCHOR: here

    println!("Váš tip: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Příliš malé číslo."),
        Ordering::Greater => println!("Příliš velké číslo."),
        Ordering::Equal => println!("Vyhráli jste!"),
    }
}
// ANCHOR_END: here
