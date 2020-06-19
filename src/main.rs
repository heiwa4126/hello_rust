use hello_rust::calc;

fn main() {
    println!("Hello, Rust!");

    let x = 1;
    let y = 2;
    println!("{}+{}={}",x,y,calc::add(1,2));
}
