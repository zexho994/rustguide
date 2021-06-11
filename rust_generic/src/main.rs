mod generic_foo;
mod generic_struct;

fn main() {
    assert_eq!(generic_foo::foo(1),1);
    assert_eq!(generic_foo::foo("hello"),"hello");

    let p = Point::new(1,1);
    assert_eq!(p,Point{x:1,y:1});

    let i = getVal(1);
    let r = getVal("hello");
    assert_eq!(i,1);
    assert_eq!(r,"hello");
}

fn getVal<T>(i:T) -> T {
    return i
}


#[derive(Debug, PartialEq)]
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T>{
	pub fn new(x:T,y :T) -> Self{
		Point{x:x,y:y}
	}
}