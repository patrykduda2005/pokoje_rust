mod the_pokoje;
use clearscreen;

fn main() {
    let mut gra = the_pokoje::Pokoje::new();
    loop{
        let mut komenda = Err(String::from(""));
        loop {
            match komenda {
                Err(e) if e != String::from("") => println!("{e}"),
                Err(_) => (),
                Ok(the_pokoje::Command::Exit|the_pokoje::Command::Pokaz) => break,
                Ok(c) => gra.execute_command(c),
            }
            println!("--------------------------------------------------------------");

            gra.wyswietl_pokoje();

            komenda = gra.get_command();

            clearscreen::clear().expect("Nie udało sie wyczyscic ekranu");
        }
        if let Ok(the_pokoje::Command::Exit) = komenda {
            break;
        } else {
            gra.robienie_par();
            gra = gra.soft_reset();
        }
    }
}
