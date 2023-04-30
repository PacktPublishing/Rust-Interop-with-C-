extern {
    fn add(a: i32, b: i32) -> i32;
    fn fillStruct(s:*mut RustStruct);
}

#[derive(Debug)]
struct RustStruct {
    a: i32,
    b: f32,
    c: [u8; 10],
}


fn main () {
    let result = unsafe {
        add(10, 81)
    };
    print!("10 + 81 = {}\n", result);
    let mut r = RustStruct {
        a: 0,
        b: 0.0,
        c: [0; 10]
    };
    unsafe {
        fillStruct(&mut r as *mut RustStruct);
    }
    print!("{:?}\n", r);
}
