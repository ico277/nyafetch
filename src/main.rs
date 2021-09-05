use std::fs;
use std::fs::File;

use std::path::Path;

use std::io::prelude::*;

use std::env::{vars, args};

use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use chrono::Duration;

use std::process::exit;

const VERSION:&str = "0.1.0-BETA";

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
    mem_total: u64,
    mem_used: u64
}

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
enum CaseEnum {
    Lowercase,
    Uppercase,
    Mixed,
}

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
struct Configuration {
    separator: Option<String>,
    key_color: Option<u8>,
    value_color: Option<u8>,
    art_color: Option<u8>,
    case: Option<CaseEnum>,
    //value_case: Option<CaseEnum>,
}

fn get_distro_info(force_distro: String) -> OsInfo {
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
            os_info.id = match &force_distro as &str {
                "" => String::from(id.replace("\"", "")),
                _ => force_distro,
            };
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
    fn or_else_zero(_err: std::io::Error) -> std::io::Result<String> { Ok(String::from("0")) }
    let frequency = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/bios_limit")
        //.expect("There was an error whilst reading sys/devices/system/cpu/cpu0/cpufreq/bios_limit")
        .or_else(or_else_zero)
        .unwrap()
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
    }

    // Parse /proc/meminfo
    let mut mem_total:u64 = 0;
    let mut mem_available:u64 = 0;
    let meminfo = fs::read_to_string("/proc/meminfo")
        .expect("There was an error whilst reading /proc/meminfo");
    for line in meminfo.lines() {
        let line = line
            .replace("\t", "")
            .replace(" ", "");
        //println!("line: {}", line);
        let line = line.split_once(":");
        //println!("{:#?}", line);
        match line {
            Some(("MemTotal", s)) => {
                let s = s.replace("kB", "");
                mem_total = s.parse::<u64>()
                    .expect(&format!("There was an error parsing 'MemTotal:{}'!", s) as &str);
            },
            Some(("MemAvailable", s)) => {
                let s = s.replace("kB", "");
                mem_available = s.parse::<u64>()
                    .expect(&format!("There was an error parsing 'MemAvailable:{}'!", s) as &str);
            },
            _ => continue,
        }
    }

    HwInfo {
        gpu: String::from("UnknOwOwn :("),
        cpu: format!("{} ({}) @ {}GHz", model_name, logical_cores, frequency),
        uptime: format!("{} Hours, {} Minutes", hours, minutes),
        mem_total: mem_total / 1024,
        mem_used: (mem_total - mem_available) / 1024,
    }
}

fn print_distro_info(os_info: &OsInfo, hw_info: &HwInfo, config: &Configuration) {
    //println!("\x1b[H");
    //print!("\x1b[0;0H");
    //\x1b[1B
    let key_color = format!("\x1b[38;5;{}m", config.key_color.unwrap_or(255));
    let value_color = format!("\x1b[38;5;{}m", config.value_color.unwrap_or(255));
    let separator = &config.separator.as_ref().unwrap();
    let owos = match config.case {
        Some(CaseEnum::Mixed) => "OwOS",
        Some(CaseEnum::Lowercase) => "owos",
        Some(CaseEnum::Uppercase) => "OWOS",
        _ => "OWOS",
    };
    let kewnel = match config.case {
        Some(CaseEnum::Mixed) => "Kewnel",
        Some(CaseEnum::Lowercase) => "kewnel",
        Some(CaseEnum::Uppercase) => "KEWNEL",
        _ => "KEWNEL",
    };
    let uwuptime = match config.case {
        Some(CaseEnum::Mixed) => "UwUptime",
        Some(CaseEnum::Lowercase) => "uwuptime",
        Some(CaseEnum::Uppercase) => "UWUPTIME",
        _ => "UWUPTIME",
    };
    let cpuuwu = match config.case {
        Some(CaseEnum::Mixed) => "CPUwU",
        Some(CaseEnum::Lowercase) => "cpuwu",
        Some(CaseEnum::Uppercase) => "CPUWU",
        _ => "CPUWU",
    };
    let gpuuwu = match config.case {
        Some(CaseEnum::Mixed) => "GPUwU",
        Some(CaseEnum::Lowercase) => "gpuuwu",
        Some(CaseEnum::Uppercase) => "GPUWU",
        _ => "GPUwU",
    };
    let memowory = match config.case {
        Some(CaseEnum::Mixed) => "MemOwOry",
        Some(CaseEnum::Lowercase) => "memowory",
        Some(CaseEnum::Uppercase) => "MEMOWORY",
        _ => "MEMOWORY",
    };
    println!("\x1b[19G{}{}     {}{}  {}", key_color, owos, value_color, separator, os_info.nyame);
    println!("\x1b[19G{}{}   {}{}  {} {}", key_color, kewnel, value_color, separator, os_info.kernel_type, os_info.kernel_version);
    println!("\x1b[19G{}{} {}{}  {}", key_color, uwuptime, value_color, separator, hw_info.uptime);
    println!("\x1b[19G{}{}    {}{}  {}", key_color, cpuuwu, value_color, separator, hw_info.cpu);
    println!("\x1b[19G{}{}    {}{}  {}", key_color, gpuuwu, value_color, separator, hw_info.gpu);
    println!("\x1b[19G{}{} {}{}  {}MiB/{}MiB", key_color, memowory, value_color, separator, hw_info.mem_used, hw_info.mem_total);
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
    print!("\x1b[38;5;{}m", config.art_color.unwrap_or(255));
    println!("\x1b[0G#################");
    println!("\x1b[0G#    ___        #");
    println!("\x1b[0G#    |__ \\      #");
    println!("\x1b[0G#       ) |     #");
    println!("\x1b[0G#       / /     #");
    println!("\x1b[0G#      |_|      #");
    println!("\x1b[0G#      (_)      #");
    println!("\x1b[0G#               #");
    println!("\x1b[0G#################");
    print!("\x1b[0m");
}

fn create_default_config() -> Configuration {
    Configuration {
        separator: Some(String::from("->")),
        key_color: Some(213),
        value_color: Some(255),
        art_color: Some(213),
        case: Some(CaseEnum::Mixed),
        //value_case: Some(CaseEnum::Mixed),
    }
}

fn create_config_file(file: &std::path::Path) -> Result<Configuration, String> {
    let config = create_default_config();
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
            return create_config_file(config_file).unwrap();
        }
    } else {
        match fs::create_dir_all(config_folder) {
            Ok(_) => {
                create_config_file(Path::new(&format!("{}/config.toml", config_folder.to_str().unwrap()))).unwrap()
            },
            Err(err) => {
                println!("There was an error whilst writing to the config file! {}", err);
                create_default_config()
            },
        }
    }
}


fn main() {
    let mut distro = String::new();
    let mut args = args();
    args.next().unwrap();
    for arg in args {
        match &arg as &str {
            "--help" => {
                println!("Available arguments");
                println!("--help\t\t\t\tShows this menu");
                println!("--version\t\t\tShows info about the current nyafetch version");
                println!("-d=<distro>|--distro=<distro>\tForcefully show <distro>'s ascii art or 'unknown' if not recognised");
                exit(0);
            },
            "--version" => {
                println!("nyafetch v{}", VERSION);
                exit(0);
            },
            s => match s.split_once("=") {
                Some(("--distro", s)) | Some(("-d", s)) => {
                    distro = String::from(s);
                },
                _ => {
                    println!("Invalid argument '{}'!\nFor a list of arguments use 'nyafetch --help'", s);
                    exit(1);
                },
            },
        }
    }

    let config = parse_config();
    let os_info = get_distro_info(distro);
    let hw_info = get_hardware_info();
    print_distro_info(&os_info, &hw_info, &config);
    print_ascii_art(&os_info, &config);
    //println!("{:#?}", config);
}
