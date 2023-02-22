use std::io;
use std::{thread, time};

enum Command {
    Move(Position, usize),
}

enum Position {
    Pokoje(usize, usize),
    Zewnatrz(usize),
}


struct Pokoje<'a> {
    pokoje: [[&'a str; 2]; 3],
    zewnatrz: Vec<&'a str>
}

impl Pokoje<'_> {
    fn new() -> Self {
        Pokoje {
            pokoje: [["", ""],["",""],["",""]],
            zewnatrz: vec!["Lechański", "Szary", "Duda", "Lutak", "Mumin", "Forzaob"],
        }
    }

    fn get_input<'a>(&'a mut self) {
        let input: Result<Command, &str> = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Nie podano tekstu");
            let mut iter = input.split_whitespace();
            match iter.next() {
                Some("move") => {
                    let arg1: Position = match iter.next() {
                        None => break Err("Nie napisałeś co chcesz przenieść"),
                        Some(v) => match self.find(v) {
                            Err(e) => break Err(e),
                            Ok(v) => v,
                        },
                    };
                    let arg2: usize = match iter.next() {
                        None => break Err("Nie napisałeś gdzie chcesz to przenieść"),
                        Some(v) => match v.parse::<usize>() {
                            Ok(v) => v,
                            Err(_e) => break Err("To nie jest liczba")
                        },
                    };
                    break Ok(Command::Move(arg1, arg2));
                },
                None => break Err("Nie ma takiej komendy"),
                _ => break Err("Nie ma takiej komendy"),
            }
        };

        match input {
            Err(e) => println!("{}", e),
            Ok(c) => match c {
                Command::Move(frompos, topos) => {
                    let what: &str = match frompos {
                        Position::Pokoje(pokoj, character) => {
                            let what = self.pokoje[pokoj][character];
                            self.pokoje[pokoj][character] = "";
                            what
                        }
                        Position::Zewnatrz(character) => self.zewnatrz.remove(character)
                    };
                    match topos {
                        0 => self.zewnatrz.push(&what),
                        _ => {
                            let iter = self.pokoje[topos - 1].iter_mut();
                            for place in iter {
                                if *place == "" {
                                    *place = &what;
                                    break;
                                }
                            }
                        },
                    }
                }
            }
        }
    }

    fn find(&self, what: &str) -> Result<Position, &str> {
        for p in self.pokoje.iter().enumerate() {
            for v in p.1.iter().enumerate() {
                if *v.1 == what {
                    return Ok(Position::Pokoje(p.0, v.0));
                }
            }
        }
        for v in self.zewnatrz.iter().enumerate() {
            if *v.1 == what {
                return Ok(Position::Zewnatrz(v.0));
            }
        }
        return Err("Nie znaleziono człowieka");
    }

    fn wyswietl_pokoje(&self) {
        println!("-----------------------------------(Zewnatrz)-------------------------------------");
        println!("{:?}", self.zewnatrz);
        println!("----------------------------------------------------------------------------------");
        println!("-----------------------------------(Pokoje)---------------------------------------");
        for (nrpokoju, pokoj) in self.pokoje.iter().enumerate() {
            println!("Pokoj numero {}", nrpokoju + 1);
            println!("{:?}", pokoj);
        }
        println!("----------------------------------------------------------------------------------");
    }

}

fn main() {
    let mut gra = Pokoje::new();
    loop {
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
}
