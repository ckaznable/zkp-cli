mod ascii;

use ascii::{ print_ascii, ZyanKen };
use dialoguer::{Select, theme::ColorfulTheme, console::Term};

fn main() -> std::io::Result<()> {
    let items = vec!["paper", "scissors", "rock"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            print_ascii(match index {
                0 => ZyanKen::PA,
                1 => ZyanKen::CHYOUKI,
                2 => ZyanKen::GU,
                _ => panic!()
            })
        },
        None => println!("User did not select anything")
    }

    Ok(())
}

