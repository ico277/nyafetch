use chrono::Duration;
use nyafetch::pci;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::var;
use std::env::{args, vars};
use std::ffi::CString;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

const VERSION: &str = "1.5.0";

struct OsInfo {
    id: String,
    nyame: String,
    kernel_type: String,
    kernel_version: String,
    hostname: String,
    user: String,
    art_lines: usize,
}

impl Default for OsInfo {
    fn default() -> Self {
        OsInfo {
            nyame: String::from("unknown"),
            id: String::from("unknown"),
            kernel_type: String::from(""),
            kernel_version: String::from(""),
            hostname: String::from("unknown"),
            user: String::from("unknown"),
            art_lines: 10,
        }
    }
}

struct HwInfo {
    gpus: Vec<String>,
    cpu: String,
    uptime: String,
    mem_total: u64,
    mem_used: u64,
}

impl Default for HwInfo {
    fn default() -> Self {
        HwInfo {
            gpus: vec![],
            cpu: String::from("unknown"),
            uptime: String::from("unknown"),
            mem_total: 0,
            mem_used: 0,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
enum CaseEnum {
    Lowercase,
    Uppercase,
    Mixed,
}

#[derive(Deserialize, Serialize, Debug)]
struct Configuration {
    separator: Option<String>,
    key_color: Option<u8>,
    value_color: Option<u8>,
    art_color: Option<u8>,
    case: Option<CaseEnum>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            separator: Some(String::from("->")),
            key_color: Some(213),
            value_color: Some(255),
            art_color: Some(213),
            case: Some(CaseEnum::Mixed),
        }
    }
}

fn get_distro_info() -> OsInfo {
    let mut os_info = OsInfo::default();

    // Parse /etc/os-release
    let release_file = match fs::read_to_string("/etc/os-release") {
        Ok(file) => file,
        Err(err) => {
            if let Ok(file) = fs::read_to_string("/etc/lsb-release") {
                file
            } else {
                //panic!("Failed to read /etc/os-release and /etc/lsb-release: {}", err)
                eprintln!(
                    "Failed to read /etc/os-release and /etc/lsb-release: {}",
                    err
                );
                String::new()
            }
        }
    };
    let release_file = release_file.lines();
    for line in release_file {
        if let Some(("ID" | "DISTRIB_ID", id)) = line.split_once("=") {
            os_info.id = String::from(id.replace("\"", "")).to_lowercase();
            os_info.nyame = match os_info.id.as_ref() {
                "arch" => String::from("Nyarch Linuwux"),
                "artix" => String::from("Awartux Linuwux"),
                "debian" => String::from("Debinyan Linuwux"),
                "gentoo" => String::from("Gentowo Linuwux"),
                "endeavouros" => String::from("EndeavOwOurOwOS"),
                _ => String::from("UnknOwOwn :("),
            };
            break;
        }
    }

    // Parse /proc/sys/kernel/ostype
    let kernel_release_file = fs::read_to_string("/proc/sys/kernel/ostype")
        .expect("There was an error whilst reading /proc/sys/kernel/ostype!");
    match kernel_release_file.trim().to_lowercase().as_ref() {
        "linux" => {
            os_info.kernel_type = String::from("Linuwux");
        }
        // I have no idea if this even works
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

    os_info.hostname = String::from(
        fs::read_to_string("/proc/sys/kernel/hostname")
            .expect("There was an error whilst reading /proc/sys/kernel/hostname!")
            .trim(),
    );

    os_info.user = match var("USER") {
        Ok(s) => s,
        Err(_) => os_info.user,
    };
    os_info
}

fn get_hardware_info() -> HwInfo {
    let mut hwinfo = HwInfo::default();

    // Parse /proc/cpuinfo
    let cpuinfo_file = fs::read_to_string("/proc/cpuinfo")
        .expect("There was an error whilst reading /proc/cpuinfo!");
    let mut cpu_model_name = String::from("UnknOwOwn :(");
    let mut cpu_logical_cores = String::new();
    let lines = cpuinfo_file
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    for line in lines {
        if line == "" {
            break;
        }
        let line = line.replace("\t", "");
        match line.split_once(":") {
            Some(("model name", s)) => cpu_model_name = String::from(s.trim()),
            Some(("siblings", s)) => cpu_logical_cores = String::from(s.trim()),
            _ => (),
        }
    }

    if cpu_model_name.chars().any(|v| v == '@') {
        hwinfo.cpu = cpu_model_name.replace("@", format!("({}) @", cpu_logical_cores).as_ref())
    } else {
        fn or_else_zero(_err: std::io::Error) -> std::io::Result<String> {
            Ok(String::from("0.00"))
        }
        let frequency = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq")
            //.expect("There was an error whilst reading sys/devices/system/cpu/cpu0/cpufreq/bios_limit")
            .or_else(or_else_zero)
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap()
            / 1000_f32
            / 1000_f32;
        hwinfo.cpu = format!(
            "{} ({}) @ {:0<4}GHz",
            cpu_model_name, cpu_logical_cores, frequency
        );
    }

    // Get GPU name
    unsafe {
        let gpu_arr = pci::get_gpu();
        let gpu_arr = std::slice::from_raw_parts_mut(gpu_arr, pci::get_gpu_count() as usize);
        let mut gpu_vec = vec![];
        for i in 0..pci::get_gpu_count() as usize {
            gpu_vec.push(CString::from_raw(gpu_arr[i]).into_string().unwrap());
        }
        if gpu_vec.len() > 0 {
            hwinfo.gpus = gpu_vec;
        }
    }

    // Parse uptime
    let uptime = fs::read_to_string("/proc/uptime")
        .expect("There was an error whilst reading /proc/uptime")
        .split_once(" ")
        .unwrap()
        .0
        .parse::<f32>()
        .unwrap()
        .round() as i64;

    let uptime = Duration::seconds(uptime);
    let hours = uptime.num_hours();
    let mut minutes = uptime.num_minutes();
    if hours != 0 {
        minutes = minutes - (hours * 60);
    }
    hwinfo.uptime = format!("{} Hours, {} Minutes", hours, minutes);

    // Parse /proc/meminfo
    let mut mem_total: u64 = 0;
    let mut mem_available: u64 = 0;
    let meminfo = fs::read_to_string("/proc/meminfo")
        .expect("There was an error whilst reading /proc/meminfo");
    for line in meminfo.lines() {
        let line = line.replace("\t", "").replace(" ", "");
        let line = line.split_once(":");
        match line {
            Some(("MemTotal", s)) => {
                let s = s.replace("kB", "");
                mem_total = s
                    .parse::<u64>()
                    .expect(format!("There was an error parsing 'MemTotal:{}'!", s).as_ref());
            }
            Some(("MemAvailable", s)) => {
                let s = s.replace("kB", "");
                mem_available = s
                    .parse::<u64>()
                    .expect(format!("There was an error parsing 'MemAvailable:{}'!", s).as_ref());
            }
            _ => continue,
        }
    }
    hwinfo.mem_total = mem_total / 1024;
    hwinfo.mem_used = (mem_total - mem_available) / 1024;

    hwinfo
}

fn print_ascii_art(info: &mut OsInfo, config: &Configuration, force_distro: Option<String>) {
    let mut distro_id = info.id.clone();
    if let Some(distro) = force_distro {
        distro_id = distro;
    }
    let art = match distro_id.as_ref() {
        "arch" => include_str!("../distro_art/arch").to_string(),
        "artix" => include_str!("../distro_art/artix").to_string(),
        "debian" => include_str!("../distro_art/debian").to_string(),
        "endeavouros" => include_str!("../distro_art/endeavouros").to_string(),
        _ => include_str!("../distro_art/unknown").to_string(),
    };

    info.art_lines = art.lines().count();

    println!();

    for line in art.lines() {
        println!(
            "\x1b[38;5;{}m{}\x1b[0m",
            config.art_color.unwrap_or(255),
            line
        );
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
    print!("\x1b[{}F", os_info.art_lines + 1);
    print!("\x1b[15C");

    println!(
        "{}{}{}@{}{}",
        value_color, os_info.user, key_color, value_color, os_info.hostname
    );
    print!("\x1b[15C");
    for _ in 0..(os_info.user.chars().count() + os_info.hostname.chars().count() + 1) {
        print!("-");
    }
    println!();
    print!("\x1b[15C");
    println!(
        "{}{}     {}{}  {}",
        key_color, owos, value_color, separator, os_info.nyame
    );
    print!("\x1b[15C");
    println!(
        "{}{}   {}{}  {} {}",
        key_color, kewnel, value_color, separator, os_info.kernel_type, os_info.kernel_version
    );
    print!("\x1b[15C");
    println!(
        "{}{} {}{}  {}",
        key_color, uwuptime, value_color, separator, hw_info.uptime
    );
    print!("\x1b[15C");
    println!(
        "{}{}    {}{}  {}",
        key_color, cpuuwu, value_color, separator, hw_info.cpu
    );
    print!("\x1b[15C");
    for gpu in &hw_info.gpus {
        println!(
            "{}{}    {}{}  {}",
            key_color, gpuuwu, value_color, separator, gpu
        );
    }
    print!("\x1b[15C");
    println!(
        "{}{} {}{}  {}MiB/{}MiB",
        key_color, memowory, value_color, separator, hw_info.mem_used, hw_info.mem_total
    );
    println!();
    println!();
    //print!("\x1b[10A");
}

fn create_config_file(file: &std::path::Path) -> Result<Configuration, String> {
    let config = Configuration::default();
    let config_str = toml::to_string_pretty(&config).unwrap();
    match File::create(file) {
        Ok(mut file) => match write!(file, "{}", config_str) {
            Ok(_) => (),
            Err(err) => {
                return Err(format!(
                    "There was an error whilst writing to the config file! {}",
                    err
                ));
            }
        },
        Err(err) => {
            return Err(format!(
                "There was an error whilst writing to the config file! {}",
                err
            ));
        }
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
            let config_file =
                fs::read_to_string(config_file).expect("There was an error whilst parsing config!");
            let mut config = toml::from_str::<Configuration>(config_file.as_ref())
                .expect("There was an error whilst parsing config!");
            config.separator = Some(
                config
                    .separator
                    .unwrap_or(String::from("->"))
                    .replace("\\t", "\t"),
            );
            return config;
        } else {
            return create_config_file(config_file).unwrap();
        }
    } else {
        match fs::create_dir_all(config_folder) {
            Ok(_) => create_config_file(Path::new(&format!(
                "{}/config.toml",
                config_folder.to_str().unwrap()
            )))
            .unwrap(),
            Err(err) => {
                println!(
                    "There was an error whilst writing to the config file! {}",
                    err
                );
                Configuration::default()
            }
        }
    }
}

fn main() {
    let mut distro = None;
    let mut args = args();
    args.next().unwrap();
    for arg in args {
        match arg.as_ref() {
            "--help" => {
                println!("Available arguments");
                println!("--help\t\t\t\tShows this menu");
                println!("--version\t\t\tShows info about the current nyafetch version");
                println!("-d=<distro>|--distro=<distro>\tDisplay <distro>'s ascii art or 'unknown' if not recognised");
                exit(0);
            }
            "--version" => {
                println!("nyafetch v{}", VERSION);
                exit(0);
            }
            s => match s.split_once("=") {
                Some(("--distro", s)) | Some(("-d", s)) => {
                    distro = Some(String::from(s));
                }
                _ => {
                    println!(
                        "Invalid argument '{}'!\nFor a list of arguments use 'nyafetch --help'",
                        s
                    );
                    exit(1);
                }
            },
        }
    }

    let config = parse_config();
    let mut os_info = get_distro_info();
    let hw_info = get_hardware_info();
    print_ascii_art(&mut os_info, &config, distro);
    print_distro_info(&os_info, &hw_info, &config);
}
