use pdf_splitter::{UserConfig, extract_pages};

#[tokio::main]
async fn main() {
    let config = match UserConfig::build() {
        Ok(c) => {
            println!("Successfully build config");
            c
        },
        Err(e) => {
            println!("Error with building config: {e}");
            std::process::exit(1);
        }
    };

    match extract_pages(&config).await {
        Ok(_) => println!("Success!!"),
        Err(e) => println!("Error: {e}")
    };
}