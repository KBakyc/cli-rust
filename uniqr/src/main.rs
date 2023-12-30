fn main() {
    if let Err(e) = uniqr::run(uniqr::get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
