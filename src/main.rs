use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    home_dir_reset();

    if args.len() >= 2 {
        match args[1].as_ref() {
            "desktop" => desktop(),
            "list" => {
                if args.len() > 2 {
                    list(args[2].clone());
                } else {
                    list(".".to_string());
                }
            },
            _ => println!("Argument does not result in an action"),
        }
    } else {
        desktop();
    }

}

fn home_dir_reset() {
    let home_dir = env::home_dir();
    let _current_dir = env::set_current_dir(home_dir.unwrap()).unwrap();
}

fn move_file(path: String, dir: &str,) {
        let mut split_path: Vec<_> = path.split("/").collect();
        let file_name = split_path.pop().unwrap();

        let mut new_path = split_path.join("/");

        new_path.push_str(&format!("/{}/", dir)); new_path.push_str(file_name);

        let _rename = fs::rename(path.clone(), new_path.clone());
        println!("Renamed {:?} to {:?}", path, new_path);
}

fn check_if_dir(path: String) -> bool {
    let actual_path = Path::new(&path);
    if actual_path.exists() {
        if actual_path.is_dir() {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn desktop() {
    let _desktop_dir = env::set_current_dir("Desktop").expect("Could not find desktop directory");
    let current_dir = env::current_dir().unwrap();

    for entry in fs::read_dir(current_dir).expect("Could not read desktop directory") {
        let entry = entry.unwrap();
        let path = entry.path().to_string_lossy().to_lowercase();
        println!("{}", entry.path().display());

        if !check_if_dir(path.clone()) {
            if path.contains("screenshot") {
                move_file(path, "Screenshots");
            } else {
                move_file(path, "Misc");
            }
        }
    }

}

fn list(target: String) {
    let out = Command::new("ls")
        .arg(target)
        .output()
        .unwrap();

    println!("{:?}", out);
}

