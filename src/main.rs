fn main() {
    if let Some(filename) = std::env::args().skip(1).next() {
        println!("'{}'", filename);
        match std::fs::read_to_string(filename) {
            Ok(contents) => {
                script_lang::run_program(&contents);
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    } else {
        eprintln!("Usage: <filename>")
    }
}
