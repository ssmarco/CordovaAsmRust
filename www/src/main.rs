#[no_mangle]
pub fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello RustC!");
}