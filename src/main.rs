fn main() {
    if let Err(error) = rvim::run() {
        println!("> Error: {:?}", error);
        std::process::exit(1)
    }
}
