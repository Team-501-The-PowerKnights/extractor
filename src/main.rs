use conf::Configuration;

mod conf;
mod sftp;

fn main() {
    let config = Configuration::read().expect("Failed to read from configuration file");
    sftp::setup(config).expect("Failed to setup for sftp usage");
}
