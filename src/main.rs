use std::env::vars;
use std::fs::File;
use std::io::{BufRead, BufReader};

const VERSION: &str = "0.0.1-dev";

fn get_distro() -> (String, String) {
	let mut distro = String::new();
	let distro_art;

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
		"arch" => {
			distro = String::from("Nyarch Linuwux");
			distro_art = String::from(include_str!("distro_art/arch"));
		}
		"artix" => {
			distro = String::from("Nyartix Linuwux");
			distro_art = String::from(include_str!("distro_art/artix"));
		}
		"curtainos" => {
			distro = String::from("Cuwurtain OwOS");
			distro_art = String::from(include_str!("distro_art/curtainos"));
		}
		"debian" => {
			distro = String::from("Debinyan Linuwux");
			distro_art = String::from(include_str!("distro_art/debian"));
		}
		"endeavouros" => {
			distro = String::from("EndeavOwOurOwOS");
			distro_art = String::from(include_str!("distro_art/endeavouros"));
		}
		"gentoo" => {
			distro = String::from("Gentowo Linuwux");
			distro_art = String::from(include_str!("distro_art/gentoo"));
		}
		"linuxlite" => {
			distro = String::from("Linuwux Lite");
			distro_art = String::from(include_str!("distro_art/linuxlite"));
		}
		"ubuntu" => {
			distro = String::from("UbuntUwU");
			distro_art = String::from(include_str!("distro_art/ubuntu"));
		}
		_ => {
			distro = String::from("Unknowown QwQ");
			distro_art = String::from(include_str!("distro_art/unknown"));
		}
	}

	return (distro, distro_art);
}

fn get_env_info() -> (String, String) {
	let mut shell = String::from("Unknowown QwQ");
	let mut session_desktop = String::from("Unknowown QwQ");
	let mut session_type = String::new();
	let desktop;

	for (key, value) in vars() {
		match key.as_str() {
			"SHELL" => shell = String::from(value),
			"XDG_SESSION_DESKTOP" => session_desktop = String::from(value),
			"XDG_SESSION_TYPE" => session_type = String::from(value),
			_ => continue,
		}
	}



	return (shell, desktop);
}

fn main() {
	//	println!("{:#?}", get_distro());
	let (distro_name, distro_art) = get_distro();
	println!("'{}':\n{}", distro_name, distro_art);
}
