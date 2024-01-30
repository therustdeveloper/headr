fn main() {
    if let Err(e) = headr::exec() {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}
