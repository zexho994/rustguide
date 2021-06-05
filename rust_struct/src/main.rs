#[derive(Debug)]
struct Rectangle {
    length: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.length * self.height
    }

    fn double_length(&self) -> u32{
         self.length * 2
    }

    fn square(size: u32) -> Rectangle{
        return Rectangle{length:size, height:size};
    }
}


fn main() {
    let l = 10;
    let h = 20;
    let size = get_area(l,h);
    println!("l * h = {}",size);

    let t = (22,53);
    let size2 = get_area_2(t);
    // let size2 = get_area_2((11,12));  //can also be
    println!("l * h = {}",size2);

    let r = Rectangle{length:30,height:33};
    let size3 = get_are_3(&r);
    println!("l * h = {}",size3);
    println!("r = {:?}",r);
    println!("r.l = {}",r.length);
    println!("r.h = {}",r.height);

    let rectangle_area = r.area();
    println!("l * h = {}",rectangle_area);

    let double_length = r.double_length();
    println!("double of length = {}",double_length);

    let squ = Rectangle::square(10);
    println!("squ area = {}",squ.area());

}

fn get_area(l:u32,h:u32) -> u32 {
    l * h
}

fn get_area_2(t:(u32,u32)) -> u32{
    t.0 * t.1
}

fn get_are_3(r:&Rectangle) -> u32{
    r.height * r.length
}