fn main() {
    println!("Hello, world!");
    list_files();
}

fn list_files() {
    use std::fs;

    match fs::read_dir(".") {
        Ok(entries) => {
            println!("{:#?}", entries);
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        println!("{:#?}", entry);
                        println!("{}", entry.file_name().to_string_lossy());
                    },
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        },
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}