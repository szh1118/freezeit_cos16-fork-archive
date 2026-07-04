fn main() {
    if let Err(error) = freezeit_daemon::run() {
        eprintln!("freezeit daemon startup failed: {error}");
        std::process::exit(1);
    }
}
