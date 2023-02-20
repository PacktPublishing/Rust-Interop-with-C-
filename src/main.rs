extern {
    fn add(a: i32, b: i32) -> i32;
}

fn main () {
    let result = unsafe {
        add(10, 81)
    };
    print!("10 + 81 = {}", result);
}
