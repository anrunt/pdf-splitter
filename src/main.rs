use pdf_splitter::{Config};

fn main() {
    let config = Config::build();
    match config {
        Ok(_) => println!("Successfully built config"),
        Err(e) => {
            println!("Error with building config: {e}");
            std::process::exit(1);
        }
    }
}