fn main() {
    if let Err(e) = wcr::run(wcr::get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
