fn main() {
    fn2();
    fn3(100,200.20,true);
    fn4();
    let x = fn5();
    println!("x is {}",x)
}

fn fn2(){
    println!("this is fn2");
}

fn fn3(x:i32,y:f32,z:bool){
    println!("x {}, y {}, z {}",x,y,z);
}

fn fn4(){
    let c;
    let x = {
        let _a = 1;
        let _b = 2;
        c = _a + _b;
        _a + _b
    };
    println!("after exec x{},c is {}",x,c);
}

fn fn5() -> i32{
    let a = 100;
    let b = 299;
    a + b
}
