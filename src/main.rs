use std::io;

fn main() {
    println!("Rate die Zahl!");

    println!("Bitte gib deine Vermutung ein.");

    let mut vermutung = String::new();

    io::stdin().read_line(&mut vermutung)
        .ok()
        .expect("Fehler beim Lesen der Zeile");

    println!("Deine Vermutung: {}", vermutung);
}
