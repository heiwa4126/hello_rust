use hello_rust::calc;

fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const PROCNAME: &'static str = env!("CARGO_PKG_NAME");
    const HOME: &'static str = env!("HOME");

    println!("Hello, Rust!");
    println!("{} v{}", PROCNAME, VERSION);
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("My home is {}", HOME);

    let x = 1;
    let y = 2;

    println!("{}+{}={}", x, y, calc::add(1, 2));
}
