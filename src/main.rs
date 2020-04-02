mod configuration_loader;

fn main() {
    println!("Hello, world!");
    let configuration = configuration_loader::load("config.json").unwrap();
    println!("{:#?}", configuration);
}
