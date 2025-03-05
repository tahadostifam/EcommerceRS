mod config;
mod core;
mod adapters;

fn main() {
    let cfg = config::read();

    println!("Hello, world!");
}
