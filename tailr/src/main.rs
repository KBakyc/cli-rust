fn main() {
    if let Err(e) = tailr::run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
