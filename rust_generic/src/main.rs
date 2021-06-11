mod generic_foo;
mod generic_struct;

fn main() {
    assert_eq!(generic_foo::foo(1),1);
    assert_eq!(generic_foo::foo("hello"),"hello");

    let p = Point::new(1,1);
    assert_eq!(p,Point{x:1,y:1});
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