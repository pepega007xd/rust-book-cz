// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("Uhádněte číslo.");

    println!("Zadejte svůj tip:");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut guess = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
        .read_line(&mut guess)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("Chyba při čtení řádku");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("Váš tip: {guess}");
    // ANCHOR_END: print_guess
}
// ANCHOR: all
