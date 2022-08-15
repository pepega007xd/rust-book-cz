// ANCHOR: all
use std::io;
// ANCHOR: ch07-04
use rand::Rng;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Uhádněte číslo.");

    // ANCHOR: ch07-04
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // ANCHOR_END: ch07-04

    println!("Tajné číslo: {secret_number}");

    println!("Zadejte svůj tip:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Chyba při čtení řádku");

    println!("Váš tip: {guess}");
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
