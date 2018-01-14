#[no_mangle]
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[no_mangle]
pub extern fn add_integers(x: i32, y: i32) -> i32 {
    x + y
}

extern { fn emscripten_exit_with_live_runtime(); }

fn main() {
    println!("Hello RustC!");
    println!("{}", add_one(add_integers(1,2)));
    unsafe { emscripten_exit_with_live_runtime(); }
}