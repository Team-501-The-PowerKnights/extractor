use conf::Configuration;

mod conf;

fn main() {
    let config = Configuration::read().expect("Failed to read from configuration file");
    println!("{:?}", config);
}
