pub struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T>{
	pub fn new(x:T,y :T) -> Self{
		Point{x:x,y:y}
	}
}