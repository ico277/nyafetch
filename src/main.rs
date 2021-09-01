use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

use std::env::vars;

use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use chrono::Duration;


struct OsInfo {
    id: String,
    nyame: String,
    kernel_type: String,
    kernel_version: String,
}

struct HwInfo {
    gpu: String,
    cpu: String,
    uptime: String,
}

#[derive(Deserialize)]
#[derive(Serialize)]
struct Configuration {
    separator: Option<String>,
    key_color: Option<u8>,
    value_color: Option<u8>,
    art_color: Option<u8>,
}

fn get_distro_info() -> OsInfo {
    let mut os_info = OsInfo {
        nyame: String::from("unknown"),
        id: String::from("unknown"),
        kernel_type: String::from(""),
        kernel_version: String::from(""),
    };
    // Parse /etc/os-release
    let os_release_file = fs::read_to_string("/etc/os-release")
        .expect("There was an error whilst reading /etc/os-release!");
    let os_release_file = os_release_file.lines();
    for line in os_release_file {
        if let Some(("ID", id)) = line.split_once("=") {
            os_info.id = String::from(id.replace("\"", ""));
            match &os_info.id as &str {
                "arch" => os_info.nyame = String::from("Arch LinUwUx"),
                "debian" => os_info.nyame = String::from("Debinyan GNUwU/LinUwU"),
                _ => os_info.nyame = String::from("UnknOwOwn :("),
            }
            break;
        }
    }

    // Parse /proc/sys/kernel/ostype
    let kernel_release_file = fs::read_to_string("/proc/sys/kernel/ostype")
        .expect("There was an error whilst reading /proc/sys/kernel/ostype!");
    match &kernel_release_file.trim().to_lowercase() as &str {
        "linux" => {
            os_info.kernel_type = String::from("LinUwUx");
        }
        "bsd" | "freebsd" | "openbsd" | "netbsd" => {
            os_info.kernel_type = String::from("BSD");
        }
        _ => (),
    }

    // Parse /proc/sys/kernel/osrelease
    os_info.kernel_version = String::from(
        fs::read_to_string("/proc/sys/kernel/osrelease")
            .expect("There was an error whilst reading /proc/sys/kernel/osrelease!")
            .trim(),
    );
    os_info
}

fn get_hardware_info() -> HwInfo {
    // Parse /proc/cpuinfo
    let cpuinfo_file = fs::read_to_string("/proc/cpuinfo")
        .expect("There was an error whilst reading /proc/cpuinfo!");
    let mut model_name = String::from("UnknOwOwn :(");
    let mut logical_cores = String::new();
    let lines = cpuinfo_file.lines().map(String::from).collect::<Vec<String>>();
    for line in lines {
        if line == "" {            
            break;
        }
        let line = line.replace("\t", "");
        match line.split_once(":") {
            Some(("model name", s)) => model_name = String::from(s.trim()),
            Some(("siblings", s)) => logical_cores = String::from(s.trim()),
            //Some(("cpu MHz", s)) => frequency = (s.trim().parse::<f32>().unwrap() / 1000_f32).to_string(),
            _ => (),
        }
    }
    // Parse CPU freq
    let frequency = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/bios_limit")
        .expect("There was an error whilst reading sys/devices/system/cpu/cpu0/cpufreq/bios_limit")
        .trim()
        .parse::<f32>().unwrap() / 1000_f32 / 1000_f32;
    
    // Parse uptime
    let uptime = fs::read_to_string("/proc/uptime")
        .expect("There was an error whilst reading /proc/uptime")
        .split_once(" ")
        .unwrap()
        .0
        .parse::<f32>().unwrap()
        .round() as i64;
    
    let uptime = Duration::seconds(uptime);
    let hours = uptime.num_hours();
    let mut minutes = uptime.num_minutes();
    if hours != 0 {
        minutes = minutes - (hours * 60);
    } else {
        minutes = 0;
    }

    HwInfo {
        gpu: String::from("UnknOwOwn :("),
        cpu: format!("{} ({}) @ {}GHz", model_name, logical_cores, frequency),
        uptime: format!("{} Hours, {} Minutes", hours, minutes),
    }
}

fn print_distro_info(os_info: &OsInfo, hw_info: &HwInfo, config: &Configuration) {
    //println!("\x1b[H");
    //print!("\x1b[0;0H");
    //\x1b[1B
    let key_color = format!("\x1b[38;5;{}m", config.key_color.unwrap_or(255));
    let value_color = format!("\x1b[38;5;{}m", config.value_color.unwrap_or(255));
    let separator = &config.separator.as_ref().unwrap();
    println!("\x1b[19G{}OwOS     {}{}  {}", key_color, value_color, separator, os_info.nyame);
    println!("\x1b[19G{}Kewnel   {}{}  {} {}", key_color, value_color, separator, os_info.kernel_type, os_info.kernel_version);
    println!("\x1b[19G{}UwUptime {}{}  {}", key_color, value_color, separator, hw_info.uptime);
    println!("\x1b[19G{}CPUwU    {}{}  {}", key_color, value_color, separator, hw_info.cpu);
    println!("\x1b[19G{}GPUwU    {}{}  {}", key_color, value_color, separator, hw_info.gpu);
    println!();
    println!();
    println!();
    println!();
    println!();
    print!("\x1b[10A");
}

fn print_ascii_art(info: &OsInfo, config: &Configuration) {
    let nyafetch_folder = Path::new("/usr/local/share/nyafetch/");
    if nyafetch_folder.exists() && nyafetch_folder.is_dir() {
        let art_file = fs::read_to_string(format!("/usr/local/share/nyafetch/{}", info.id));
        if let Ok(art) = art_file {
            print!("\x1b[38;5;{}m", config.art_color.unwrap_or(255));
            for line in art.lines() {
                println!("\x1b[0G{}", line);
            }
            print!("\x1b[0m");
            return;
        }
    }
    /*
    ___
    |__ \
       ) |
      / /
     |_|
     (_)
    */
    println!("\x1b[0G#################");
    println!("\x1b[0G#    ___        #");
    println!("\x1b[0G#    |__ \\      #");
    println!("\x1b[0G#       ) |     #");
    println!("\x1b[0G#       / /     #");
    println!("\x1b[0G#      |_|      #");
    println!("\x1b[0G#      (_)      #");
    println!("\x1b[0G#               #");
    println!("\x1b[0G#################");
}

fn create_config(file: &std::path::Path) -> Result<Configuration, String> {
    let config = Configuration {
        separator: Some(String::from("->")),
        key_color: Some(213),
        value_color: Some(255),
        art_color: Some(213),
    };
    let config_str = toml::to_string_pretty(&config).unwrap();
    match File::create(file) {
        Ok(mut file) => {
            match write!(file, "{}", config_str) {
                Ok(_) => (),
                Err(err) => {
                    return Err(format!("There was an error whilst writing to the config file! {}", err));
                },
            }
        },
        Err(err) => {
            return Err(format!("There was an error whilst writing to the config file! {}", err));
        },
    }
    Ok(config)
}

fn parse_config() -> Configuration {
    let home: HashMap<String, String> = vars().collect();
    let mut home = match home.get("HOME") {
        Some(s) => String::from(s),
        _ => String::from("."),
    }; 
    home.push_str("/.config/nyafetch");
    let config_folder = Path::new(&home);
    if config_folder.exists() && config_folder.is_dir() {
        home.push_str("/config.toml");
        let config_file = Path::new(&home);
        if config_file.exists() && config_file.is_file() {
            let config_file = fs::read_to_string(config_file)
                .expect("There was an error whilst parsing config!");
            let mut config = toml::from_str::<Configuration>(&config_file as &str)
                .expect("There was an error whilst parsing config!");
            config.separator = Some(config.separator.unwrap_or(String::from("->")).replace("\\t", "\t"));
            return config;
        } else {
            return create_config(config_file).unwrap();
        }
    } else {
        match fs::create_dir_all(config_folder) {
            Ok(_) => {
                create_config(Path::new(&format!("{}/config.toml", config_folder.to_str().unwrap()))).unwrap()
            },
            Err(err) => {
                println!("There was an error whilst writing to the config file! {}", err);
                Configuration {
                    separator: Some(String::from("->")),
                    key_color: Some(213),
                    value_color: Some(255),
                    art_color: Some(213),
                }
            },
        }
    }
} 


fn main() {
    let config = parse_config();
    let os_info = get_distro_info();
    let hw_info = get_hardware_info();
    print_distro_info(&os_info, &hw_info, &config);
    print_ascii_art(&os_info, &config);
}
