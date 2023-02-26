pub enum ZyanKen {
    CHYOUKI,
    GU,
    PA
}

pub fn print_ascii(z: ZyanKen) {
    match z {
        ZyanKen::CHYOUKI => output(include_str!("../ascii/chyouki")),
        ZyanKen::GU => output(include_str!("../ascii/gu")),
        ZyanKen::PA => output(include_str!("../ascii/pa")),
    };
}

fn output(line: &str) {
    println!("{}", line);
}