mod the_pokoje {
    use std::io;

    enum Command {
        Move(Position, usize),
        Help,
    }

    enum Position {
        Pokoje(usize, usize),
        Zewnatrz(usize),
    }


    pub struct Pokoje<'a> {
        pokoje: [[&'a str; 2]; 3],
        zewnatrz: Vec<&'a str>
    }

    impl Pokoje<'_> {
        pub fn new() -> Self {
            Pokoje {
                pokoje: [["", ""],["",""],["",""]],
                zewnatrz: vec!["Lechański", "Szary", "Duda", "Lutak", "Mumin", "Forzaob"],
            }
        }

        pub fn get_input(&mut self) {
            let input: Result<Command, String> = {
                let mut input:String = String::new();
                io::stdin().read_line(&mut input).expect("Nie podano tekstu");
                self.string_to_command(input)
            };
            self.execute_command(input);
        }

        fn string_to_command(&self, input: String) -> Result<Command, String> {
            let mut iter = input.split_whitespace();
            match iter.next() {
                Some("move") => {
                    let from: Position = match iter.next() {
                        None => return Err(String::from("Nie napisałeś co chcesz przenieść")),
                        Some(v) => match self.find(v) {
                            Err(e) => return Err(e),
                            Ok(v) => v,
                        },
                    };
                    let to: usize = match iter.next() {
                        None => return Err(String::from("Nie napisałeś gdzie chcesz to przenieść")),
                        Some(v) => match v.parse::<usize>() {
                            Ok(v @ 1..=3) => {
                                let mut is_there_room: bool = false;
                                for miejsce in self.pokoje[v - 1].iter() {
                                    if *miejsce == "" {
                                        is_there_room = true;
                                        break;
                                    } 
                                }
                                if !is_there_room {return Err(String::from("Nie ma miejsca w tym pokoju"))}
                                v
                            },
                            Ok(v @ 0) => v,
                            Ok(_) => return Err(String::from("Nie ma takiego pokoju")),
                            Err(_e) => return Err(String::from("To nie jest liczba")),
                        },
                    };
                    return Ok(Command::Move(from, to));
                },
                Some("help") => {
                    return Ok(Command::Help);
                },
                Some(_) | None => return Err(String::from("Nie ma takiej komendy")),
            }
        }

        fn execute_command(&mut self, input: Result<Command, String>) {
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
                    },
                    Command::Help => {
                        self.help();
                    },
                }
            }

        }

        fn find(&self, what: &str) -> Result<Position, String> {
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
            return Err(String::from("Nie znaleziono człowieka"));
        }
        
        fn help(&self) {
            println!("Komendy:");
            println!("help - wyswietla ta wiadomosc");
            println!("move <osoba> <miejsce> - przenies osobe");
            println!("  <osoba> - osoba ktora chcesz przeniesc");
            println!("  <miejsce> - cyfra oznaczajaca gdzie ja przeniesc\n  0 - zewnatrz, n - pokoj n");
        }

        pub fn wyswietl_pokoje(&self) {
            println!("-------------------------(Zewnatrz)---------------------------");
            println!("{:?}", self.zewnatrz);
            println!("--------------------------------------------------------------");
            println!("-------------------------(Pokoje)-----------------------------");
            for (nrpokoju, pokoj) in self.pokoje.iter().enumerate() {
                println!("Pokoj numero {}", nrpokoju + 1);
                println!("{:?}", pokoj);
            }
            println!("--------------------------------------------------------------");
        }
        pub fn znajdz_pare(&self, dla: &str) -> Option<&str> {
            let mut pokojm: usize = 9; //9 - wartosc nie mozliwa do uzyskania
            for (id, pokoj) in self.pokoje.iter().enumerate() {
                for character in pokoj.iter() {
                    if *character == dla {
                        pokojm = id;
                    }
                }
            }
            if pokojm == 9 {return None} //jesli nie znalazlo pary
            for character in self.pokoje[pokojm].iter() {
                if *character != dla && *character != "" {
                    return Some(character);
                } 
            }
            return None;
        }

    }
}

use std::io;

fn main() {
    let mut gra = the_pokoje::Pokoje::new();
    loop {
        println!("wpisz komende \"help\" aby wyswietlic pomoc");
        gra.wyswietl_pokoje();
        gra.get_input();

        println!("{:?}", gra.znajdz_pare("Szary"));

        println!("Wcisnij enter aby kontynuowac...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("WHAT");
        match input.as_str().trim() {
           "exit" => break,
           _ => print!("{}[2J", 27 as char),
        }

    }
}
