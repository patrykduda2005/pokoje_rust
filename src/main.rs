use std::io;
use clearscreen;

#[derive(Clone, Copy)]
struct PokojeManager<'a> {
    pokoje: [[&'a str;2];3],
}


impl PokojeManager<'_> {

    fn r#move<'a>(&'a mut self, what: &'a str, r#where: &'a usize) {
        for pokoj in self.pokoje.iter_mut() {
            for mut miejsce in pokoj.iter_mut() {
                if miejsce as &str == what {
                    miejsce = &mut "";
                    self.pokoje[*r#where][1] = what;
                }
            }
        }
    }

    fn input_handler(&mut self) {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("To nie jest ciag znakow");

            let komenda = match self.interpretator(input.as_str()) {
                Err(e) => (e as &str, "sample", 0 as usize),
                Ok(v) => v,
            };

            match komenda.0 {
                "move" => &mut self.r#move(komenda.1, &komenda.2),
                _ => &mut println!("{}", komenda.0),
            };
        }
    }

    fn interpretator<'a>(self, input: &'a str) -> Result<(&str, &str, usize), &str> {
        let mut iter = input.split_whitespace();
        
         match iter.next() {
            None => Err("Podana komenda pusta"),
            Some("move") => { //<(&str,usize),&str>
                let arg1 = match iter.next() {
                    None => Err("Brak argumentu nr 1"),
                    Some(v) => Ok(v),
                };
                let arg2 = match iter.next() {
                    None => Err("Brak argumentu nr 2"),
                    Some(v) => Ok(v.parse::<usize>().unwrap()),
                };
                match arg1 {
                    Err(e) => Err(e),
                    Ok(v) => match arg2 {
                        Err(e) => Err(e),
                        Ok(dv) => Ok(("move",v,dv))
                    }
                }
            },
            _ => Err("Komenda nie istnieje"),
        }
        
    }

    fn wez_pozycje(&self, n_zapytania: usize) -> usize {
        loop {
            let mut input = String::new();
            self.dialog(n_zapytania);
            self.wyswietl_pokoje();

            io::stdin().read_line(&mut input).expect("To nie jest ciag znakow");
            clearscreen::clear().expect("Failed to clear terminal");


            match input.trim().parse::<usize>() {
                Ok(v) => if !(1..=3).contains(&v)  {
                            println!{"Podana liczba nie mie??ci sie w zasi??gu <1,3>"}
                        } else if !(&self.pokoje[v-1][0] == &mut "") && !(&self.pokoje[v-1][1] == &mut "") {
                            println!{"Nie ma wolnego pokoju na tej pozycji"}
                        } else {
                            break v-1
                        },
                Err(e) => println!("Liczba powinna by?? ca??kowita: {e}")
            }
        }
    }
    
    fn wyswietl_pokoje(&self) {
        println!("-----------------POKOJE---------------------");
        print!("|");
        for pokoj in self.pokoje.iter() {
            print!("\"{}\",\"{}\"|", pokoj[0], pokoj[1]);
        }
        println!("\n--------------------------------------------");
    }
    
    fn dialog(&self, n: usize) {
        let dialogi = [
            "Jeste?? managerem ??ycia. Przed tob?? 3 puste pokoje.\nMusisz zalokowa?? 6 os??b: Kromke, Szaraka, Mumina, Lecha??skiego, Forzaoba i Dude.",
            "Do jakiego pokoju chcesz wrzuci?? Kromke? (1,2 lub 3)",
            "Do jakiego pokoju chcesz wrzuci?? Szaraka? (1,2 lub 3)",
            "Do jakiego pokoju chcesz wrzuci?? Mumina? (1,2 lub 3)",
            "Do jakiego pokoju chcesz wrzuci?? Lecha??skiego? (1,2 lub 3)",
            "Do jakiego pokoju chcesz wrzuci?? Forzaoba? (1,2 lub 3)",
            "Do jakiego pokoju chcesz wrzuci?? Dude? (1,2 lub 3)",
        ];
        println!("{}", dialogi[n]);
    }
}

fn main() {
    let pokoje: [[&str;2];3] = [["",""],["",""],["",""]];
    let mut game = PokojeManager {
        pokoje: pokoje,
    };


    game.dialog(0);

    let pozycja: usize = game.wez_pozycje(1);
    if game.pokoje[pozycja][0] == "" {
        game.pokoje[pozycja][0] = "Kromka"
    } else if game.pokoje[pozycja][1] == "" {
        game.pokoje[pozycja][1] = "Kromka"
    }

    let pozycja = game.wez_pozycje(2);
    if game.pokoje[pozycja][0] == "" {
        game.pokoje[pozycja][0] = "Szarak"
    } else if game.pokoje[pozycja][1] == "" {
        game.pokoje[pozycja][1] = "Szarak"
    }

    let pozycja = game.wez_pozycje(3);
    if game.pokoje[pozycja][0] == "" {
        game.pokoje[pozycja][0] = "Mumin"
    } else if game.pokoje[pozycja][1] == "" {
        game.pokoje[pozycja][1] = "Mumin"
    }

    let pozycja = game.wez_pozycje(4);
    if game.pokoje[pozycja][0] == "" {
        game.pokoje[pozycja][0] = "Lecha??ski"
    } else if game.pokoje[pozycja][1] == "" {
        game.pokoje[pozycja][1] = "Lecha??ski"
    }

    let pozycja = game.wez_pozycje(5);
    if game.pokoje[pozycja][0] == "" {
        game.pokoje[pozycja][0] = "Forzaob"
    } else if game.pokoje[pozycja][1] == "" {
        game.pokoje[pozycja][1] = "Forzaob"
    }

    let pozycja = game.wez_pozycje(6);
    if game.pokoje[pozycja][0] == "" {
        game.pokoje[pozycja][0] = "Duda"
    } else if game.pokoje[pozycja][1] == "" {
        game.pokoje[pozycja][1] = "Duda"
    }

    game.wyswietl_pokoje();

    game.input_handler();
    
    for pokoj in game.pokoje.iter() {
        if (pokoj[0] == "Szarak" && pokoj[1] == "Lecha??ski") || (pokoj[1] == "Szarak" && pokoj[0] == "Lecha??ski") {
            println!("Lecha??ski umar??");
        }
        if (pokoj[0] == "Mumin" && pokoj[1] == "Forzaob") || (pokoj[1] == "Mumin" && pokoj[0] == "Forzaob") {
            println!("Klony si?? po????czy??y co wywo??a??o wybuch por??wnalny do wybuchy bomby atomowej");
        }
    }

    println!("Wci??nij enter aby zako??czy??");
    let mut zatrzymajka = String::new();
    io::stdin().read_line(&mut zatrzymajka).unwrap();
}
