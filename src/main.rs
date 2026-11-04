use sigmaos_apps::{Config, AppsComponent, Component};
fn main() {
    env_logger::init();
    let mut c = AppsComponent::new(Config::default());
    if let Err(e) = c.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS apps running");
    let _ = c.shutdown();
}
