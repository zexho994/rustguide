fn main() {
    // fn_if();
    // println!("10 > 6 ? {}",if_condition(10,6));
    // fn_loop(5);
    // fn_while(10);
    // fn_for_item();
    fn_rev();
}

fn fn_if(){
    let a = 10;
    if a > 20 {
        println!("a is more than 20");
    }else if a < 5 {
        println!("a is less than 5");
    }else{
        println!("a is more than 5 and less than 20");
    }
}

fn if_condition(p1:u32,p2:u32) -> bool{
    let r = if p1 > p2 {
        true
    }else{
        false
    };
    r
}

fn fn_loop(count:u8){
    let mut a = 0;
    loop {
        println!("a -> {}",a);
        a += 1;

        if a > count{
            break;
        }
    }
}

fn fn_while(count:u8){
    let mut a = 0;
    while a < count{
        println!("a is {}",a);
        a += 1
    }
}

fn fn_for_item(){
    let a = [1,2,3,4,5];
    for e in a.iter(){
        println!("e is {}", e);
    }
}

fn fn_rev(){
    for n in (1..5){
        println!("n is {}",n);
    }
    println!("fn_rev() over~");
}
