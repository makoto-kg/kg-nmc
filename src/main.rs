use std::env;
use std::path::Path;
use std::fs;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn visit_dir(dir: &Path) -> std::io::Result<()> {
    println!("visiting {:?}", dir.to_str().unwrap());

    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let path = entry.unwrap().path();

        if path.is_dir() {
            match path.file_name().unwrap().to_str().unwrap() {
                "node_modules" => fs::remove_dir_all(path)?,
                _ => visit_dir(&path)?,
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Invalid argument!");
        println!("usage: kg-nmc <directory>");
        std::process::exit(1);
    }


    let arg_path = Path::new(&args[1]);
    
    let abs_path = if arg_path.is_relative() {
        let pwd = env::current_dir().unwrap();
        pwd.join(arg_path)
    } else {
        arg_path.to_path_buf()
    };

    if !arg_path.is_dir() {
        println!("Invalid directory!");
        println!("usage: kg-nmc <directory>");
        std::process::exit(1);
    }

    println!("Target dir: {:?}", abs_path);
    visit_dir(&abs_path)?;

    Ok(())
}
