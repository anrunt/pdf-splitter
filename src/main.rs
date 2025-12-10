use pdfsplit::{UserConfig, extract_pages};

#[tokio::main]
async fn main() {
    let config = match UserConfig::build() {
        Ok(c) => {
            c
        },
        Err(e) => {
            println!("Error with building config: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = extract_pages(&config).await {
        println!("Error: {e}");
    }
}