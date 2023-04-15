mod the_pokoje;
use std::io;

fn main() {
    let mut gra = the_pokoje::Pokoje::new();
    loop {
        println!("wpisz komende \"help\" aby wyswietlic pomoc");
        gra.wyswietl_pokoje();
        gra.get_input();

        println!("Wcisnij enter aby kontynuowac...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("WHAT");
        match input.as_str().trim() {
           "exit" => break,
           _ => print!("{}[2J", 27 as char),
        }

    }
    gra.robienie_par();
}
