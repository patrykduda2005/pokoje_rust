use std::io;

pub enum Command {
    Move(Position, usize),
    Help,
    Exit,
    Pokaz,
}

enum Zmiana<'a> {
    Flesh(&'a str),
}

pub enum Position {
    Pokoje(usize, usize),
    Zewnatrz(usize),
}

pub struct Pokoje<'a> {
    pokoje: [[&'a str; 2]; 4],
    zewnatrz: Vec<&'a str>,
    zmiany: Vec<Zmiana<'a>>,
    lista_osob: Vec<&'a str>,
}

impl Pokoje<'_> {
    pub fn new() -> Self {
        let osoby = vec!["Lechański", "Szary", "Duda", "Lutak", "Mumin", "Forzaob", "Krzak", "Hospod"];
        Pokoje {
            pokoje: [["", ""],["",""],["",""],["",""]],
            zewnatrz: osoby.to_owned(),
            zmiany: vec![],
            lista_osob: osoby.to_owned(),
        }
    }

    pub fn get_input(&mut self) -> Result<Command, String> {
        let mut input:String = String::new();
        io::stdin().read_line(&mut input).expect("Nie podano tekstu");
        return self.string_to_command(input);
    }

    fn string_to_command(&self, input: String) -> Result<Command, String> {
        let mut iter = input.split_whitespace();
        match iter.next() {
            Some("move" | "mv") => {
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
                        Ok(v @ 1..=4) => {
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
            Some("exit") => {
                return Ok(Command::Exit);
            },
            Some("pokaz") => {
                return Ok(Command::Pokaz)
            },
            Some(_) | None => return Err(String::from("Nie ma takiej komendy")),
        }
    }

    pub fn execute_command(&mut self, input: Command) {
        match input {
            Command::Move(frompos, topos) => {
                self.r#move(frompos, topos);
            },
            Command::Help => {
                self.help();
            },
            Command::Exit => {
                panic!("Funkcja execute_command nie obsluguje komendy Exit");
            },
            Command::Pokaz => {
                panic!("Funkcja execute_command nie obsluguje komendy Pokaz");
            },
        }

    }

    fn find<'a>(&'a self, mut what: &'a str) -> Result<Position, String> {
        let mut osoby = self.lista_osob.to_vec();
        osoby = osoby.iter()
            .filter(|v| v.starts_with(what))
            .map(|v| v.to_owned())
            .collect::<Vec<&'_ str>>();
        match osoby.len() {
            1 => what = osoby.pop().unwrap(),
            0 => return Err(String::from("Nie podales czlowieka")),
            _ => return Err(String::from("Twoje zapytanie obejmuje wiecej niz jednego czlowieka")),
        }
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
        return Err(String::from("Nie znaleziono czlowieka"));
    }

    fn r#move(&mut self, frompos: Position, topos: usize) {
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
    
    fn help(&self) {
        println!("Komendy:");
        println!("help - wyswietla ta wiadomosc");
        println!("move <osoba> <miejsce> - przenies osobe");
        println!("  <osoba> - osoba ktora chcesz przeniesc (moze byc podana tylko czesc slowa)");
        println!("  <miejsce> - cyfra oznaczajaca gdzie ja przeniesc\n  0 - zewnatrz, n - pokoj n");
        println!("exit - wyjdz i dowiedz sie co sie stalo z ludzmi");
        println!("pokaz - pokaz co sie stalo z ludzmi po 24 godzinach");
    }

    pub fn wyswietl_pokoje(&self) {
        println!("wpisz komende \"help\" aby wyswietlic pomoc");
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

    fn znajdz_pare(&self, dla: &str) -> Option<&str> {
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

    pub fn robienie_par(&mut self) {
        for pokoj in self.pokoje {
            for osoba in pokoj {
                match self.znajdz_pare(osoba) {
                    Some(para) => {
                        match (osoba, para) {
                            ("Forzaob", "Mumin") => {
                                println!("Klony nieznanego człowieka (Forzaob i Mumin) połączyły się w wielką kulę energii i zniszczyły wszechświat.");
                            },
                            ("Szary", "Lechański") => {
                                println!("Szary zabił Lechańskiego");
                                println!("Zyskales nowy przedmiot: Flesh !");
                                self.zmiany.push(Zmiana::Flesh("Lechański"))
                            },
                            ("Szary", "Duda") => {
                                println!("Szary zabił Dude");
                                println!("Zyskales nowy przedmiot: Flesh !");
                                self.zmiany.push(Zmiana::Flesh("Duda"))
                            },
                            ("Szary", "Hospod") => {
                                println!("Szary zabił Hospoda");
                                println!("Zyskales nowy przedmiot: Flesh !");
                                self.zmiany.push(Zmiana::Flesh("Hospod"))
                            },
                            ("Szary", "Krzak") => {
                                println!("Szary zabił Krzaka");
                                println!("Zyskales nowy przedmiot: Flesh !");
                                self.zmiany.push(Zmiana::Flesh("Krzak"))
                            },
                            ("Duda", "Lutak") => {
                                println!("Gaming");
                            },
                            ("Krzak", "Hospod") => {
                                println!("Powstaje totalny banger");
                            },
                            _ => (),
                        }
                    }
                    None => (),
                }
            }
        }
    }

    pub fn soft_reset(&self) -> Self {
        let mut nowy = self::Pokoje::new();
        for zmiana in self.zmiany.iter() {
            match *zmiana {
                Zmiana::Flesh(ofiara) => {
                    let id: usize = nowy.zewnatrz.iter()
                        .enumerate()
                        .filter(|(_, osoba)| *osoba == &ofiara)
                        .map(|x| x.0)
                        .collect::<Vec<usize>>()
                        .pop().expect("Zmiana Flesh nie znalazla ofiary");
                    nowy.zewnatrz.remove(id);
                    nowy.zewnatrz.push("Flesh");
                    nowy.lista_osob.remove(id);
                    nowy.lista_osob.push("Flesh");
                    nowy.zmiany.push(Zmiana::Flesh(ofiara));
                }
            }
        }
        return nowy;
    }
}
