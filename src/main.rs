use std::fs::File;
use std::io::{BufRead, BufReader};

const VERSION: &str = "1.0.0-dev";

fn get_distro() -> (String, String) {
    let mut distro = String::new();
    let mut distro_art;

    // open os-release file read-only
    let file = File::open("/etc/os-release").unwrap();
    let reader = BufReader::new(file);

    // read os-release line by line
    for (_, line) in reader.lines().enumerate() {
        if let Some(("ID", id)) = line.unwrap().split_once("=") {
            distro = String::from(id.to_lowercase().trim());
            break;
        };
    }
    
    // match distro and return name and art
    match distro.as_str() {
        "gentoo" => {
            distro = String::from("Gentowo Linuwux");
            distro_art = String::from(include_str!("distro_art/gentoo"));
        },
        "arch" => {
            distro = String::from("Nyarch Linuwux");
            distro_art = String::from(include_str!("distro_art/arch"));
        }
        _ => {
            distro = String::from("Unknowown qwq");
            distro_art = String::from(include_str!("distro_art/unknown"));
        }
    }

    return (distro, distro_art);
}

fn main() {
    println!("{:#?}", get_distro());
    let (distro_name, distro_art) = get_distro();
    println!("'{}';'{}'", distro_name, distro_art);
}
