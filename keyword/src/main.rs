fn main() {
    let mut x = 5;
    println!("The value of x if: {}",x);
    x = 6;
    println!("The value of x if: {}",x);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {}",MAX_POINTS);
    const MAX_POINTS2: u32 = 100000;
    println!("The value of MAX_POINTS2 is {}",MAX_POINTS2);

    let a = 10;
    println!("The value of a is {}",a);
    let b = a + 10;
    println!("The value of b is {}",b);

    let c:i8 = -5;
    println!("The value of c is {}",c);

    let d:u8 = 255;
    println!("The value of d is {}",d);

    let f:f32 = 32.32;
    println!("The value of f is {}",f);

    let tup:(i32,f32,bool) = (400,104.3,true);
    println!("The value of tup is {}",tup.1);

    let arr = [1,2,3,4,5];
    let arr1 = arr[1];
    let arr4 = arr[4];
    println!("The value of arr1 is {},arr4 is {}",arr1,arr4);
}

