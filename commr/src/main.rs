fn main() {
    if let Err(e) = commr::run(commr::get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
