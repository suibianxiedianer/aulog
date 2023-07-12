fn main() {
    if ! aulog::is_root() {
        println!("Error: You should run as root.");
        std::process::exit(1);
    }

    let app = aulog::app::app();
    aulog::aulog::run(app);
}
