use std::io;
use clearscreen;


fn dialog(n: usize) {
    let dialogi = [
        "Jesteś managerem życia. Przed tobą 3 puste pokoje.\nMusisz zalokować 6 osób: Kromke, Szaraka, Mumina, Lechańskiego, Forzaoba i Dude.",
        "Do jakiego pokoju chcesz wrzucić Kromke? (1,2 lub 3)",
        "Do jakiego pokoju chcesz wrzucić Szaraka? (1,2 lub 3)",
        "Do jakiego pokoju chcesz wrzucić Mumina? (1,2 lub 3)",
        "Do jakiego pokoju chcesz wrzucić Lechańskiego? (1,2 lub 3)",
        "Do jakiego pokoju chcesz wrzucić Forzaoba? (1,2 lub 3)",
        "Do jakiego pokoju chcesz wrzucić Dude? (1,2 lub 3)",
    ];
    println!("{}", dialogi[n]);
}
/*
fn wez_pozycje(n_zapytania: usize, pokoje: &[[&str;2];3]) -> usize {
    loop {
        let mut input = String::new();
        dialog(n_zapytania);
        wyswietl_pokoje(&pokoje);

        io::stdin().read_line(&mut input).expect("To nie jest ciag znakow");
        clearscreen::clear().expect("Failed to clear terminal");


        match input.trim().parse::<usize>() {
            Ok(v) => if !(1..=3).contains(&v)  {
                        println!{"Podana liczba nie mieści sie w zasięgu <1,3>"}
                    } else if !(pokoje[v-1][0] == "") && !(pokoje[v-1][1] == "") {
                        println!{"Nie ma wolnego pokoju na tej pozycji"}
                    } else {
                        break v-1
                    },
            Err(e) => println!("Liczba powinna być całkowita: {e}")
        }
    }
}

fn wyswietl_pokoje(pokoje: &[[&str;2];3]) {
    println!("-----------------POKOJE---------------------");
    print!("|");
    for pokoj in pokoje.iter() {
        print!("\"{}\",\"{}\"|", pokoj[0], pokoj[1]);
    }
    println!("\n--------------------------------------------");
}

fn main() {
    let mut pokoje: [[&str;2];3] = [["",""],["",""],["",""]];
    dialog(0);

    let pozycja: usize = wez_pozycje(1, &pokoje);
    if pokoje[pozycja][0] == "" {
        pokoje[pozycja][0] = "Kromka"
    } else if pokoje[pozycja][1] == "" {
        pokoje[pozycja][1] = "Kromka"
    }

    let pozycja = wez_pozycje(2, &pokoje);
    if pokoje[pozycja][0] == "" {
        pokoje[pozycja][0] = "Szarak"
    } else if pokoje[pozycja][1] == "" {
        pokoje[pozycja][1] = "Szarak"
    }

    let pozycja = wez_pozycje(3, &pokoje);
    if pokoje[pozycja][0] == "" {
        pokoje[pozycja][0] = "Mumin"
    } else if pokoje[pozycja][1] == "" {
        pokoje[pozycja][1] = "Mumin"
    }

    let pozycja = wez_pozycje(4, &pokoje);
    if pokoje[pozycja][0] == "" {
        pokoje[pozycja][0] = "Lechański"
    } else if pokoje[pozycja][1] == "" {
        pokoje[pozycja][1] = "Lechański"
    }

    let pozycja = wez_pozycje(5, &pokoje);
    if pokoje[pozycja][0] == "" {
        pokoje[pozycja][0] = "Forzaob"
    } else if pokoje[pozycja][1] == "" {
        pokoje[pozycja][1] = "Forzaob"
    }

    let pozycja = wez_pozycje(6, &pokoje);
    if pokoje[pozycja][0] == "" {
        pokoje[pozycja][0] = "Duda"
    } else if pokoje[pozycja][1] == "" {
        pokoje[pozycja][1] = "Duda"
    }

    wyswietl_pokoje(&pokoje);
    
    for pokoj in pokoje.iter() {
        if (pokoj[0] == "Szarak" && pokoj[1] == "Lechański") || (pokoj[1] == "Szarak" && pokoj[0] == "Lechański") {
            println!("Lechański umarł");
        }
        if (pokoj[0] == "Mumin" && pokoj[1] == "Forzaob") || (pokoj[1] == "Mumin" && pokoj[0] == "Forzaob") {
            println!("Klony się połączyły co wywołało wybuch porównalny do wybuchy bomby atomowej");
        }
    }

    println!("Wciśnij enter aby zakończyć");
    let mut zatrzymajka = String::new();
    io::stdin().read_line(&mut zatrzymajka).unwrap();
}*/

struct PokojeManager<'a> {
    pokoje: [[&'a str;2];3],
}

impl PokojeManager<'_> {
    fn wez_pozycje(&self, n_zapytania: usize) -> usize {
        loop {
            let mut input = String::new();
            dialog(n_zapytania);
            self.wyswietl_pokoje();

            io::stdin().read_line(&mut input).expect("To nie jest ciag znakow");
            clearscreen::clear().expect("Failed to clear terminal");


            match input.trim().parse::<usize>() {
                Ok(v) => if !(1..=3).contains(&v)  {
                            println!{"Podana liczba nie mieści sie w zasięgu <1,3>"}
                        } else if !(&self.pokoje[v-1][0] == &mut "") && !(&self.pokoje[v-1][1] == &mut "") {
                            println!{"Nie ma wolnego pokoju na tej pozycji"}
                        } else {
                            break v-1
                        },
                Err(e) => println!("Liczba powinna być całkowita: {e}")
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
}

fn main() {
    let pokoje: [[&str;2];3] = [["",""],["",""],["",""]];
    let mut game = PokojeManager {
        pokoje: pokoje,
    };

    dialog(0);

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
        game.pokoje[pozycja][0] = "Lechański"
    } else if game.pokoje[pozycja][1] == "" {
        game.pokoje[pozycja][1] = "Lechański"
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
    
    for pokoj in game.pokoje.iter() {
        if (pokoj[0] == "Szarak" && pokoj[1] == "Lechański") || (pokoj[1] == "Szarak" && pokoj[0] == "Lechański") {
            println!("Lechański umarł");
        }
        if (pokoj[0] == "Mumin" && pokoj[1] == "Forzaob") || (pokoj[1] == "Mumin" && pokoj[0] == "Forzaob") {
            println!("Klony się połączyły co wywołało wybuch porównalny do wybuchy bomby atomowej");
        }
    }

    println!("Wciśnij enter aby zakończyć");
    let mut zatrzymajka = String::new();
    io::stdin().read_line(&mut zatrzymajka).unwrap();
}
