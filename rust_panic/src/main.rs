use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // ver_panic();
    define_error_msg();
}

#[warn(dead_code)]
fn file_panic() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening file: {:?}", error),
        },
    };
}

fn ver_panic() {
    let mut vec = vec![1, 2, 3];
    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);
    vec.insert(4, 5);
    assert_eq!(vec, [1, 4, 2, 3, 5]);
}

fn define_error_msg() {
    let x = false;
    assert!(x, "x is false");

    let a = 3;
    let b = 4;
    assert!(a > b, "a is less b");
}
