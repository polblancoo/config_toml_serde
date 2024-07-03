mod config;
use  config::Config;

fn main() {
    let config01: Config= Config::new();
    println!("{:#?}", config01);
    
}
