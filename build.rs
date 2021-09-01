use std::fs;

fn main() {
    let paths = fs::read_dir("./distro_art/").unwrap();

    for path in paths {
        let path = &path.unwrap().path();
        let file_name = path.to_str().unwrap().split("/").collect::<Vec<&str>>();
        let file_name = file_name[file_name.len() - 1];
        fs::create_dir_all("/usr/local/share/nyafetch/").unwrap();
        let file_name = format!("/usr/local/share/nyafetch/{}", file_name);
        match fs::copy(&path, &file_name) {
            Ok(_) => {
                println!("Copied '{}' to '{}'", &path.to_str().unwrap(), &file_name);
            },
            Err(err) => {
                println!("cargo:warning=There was an error whilst copying the distro art files: {}", err);
            },
        }
    }
}
