fn main() {
    if let Err(e) = headr::run(headr::get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
