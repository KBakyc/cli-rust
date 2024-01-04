fn main() {
    if let Err(e) = findr::run(findr::get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
