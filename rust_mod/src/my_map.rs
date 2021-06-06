pub struct MyMap {
    pub k : u32,
    pub v : u32,
}

pub fn get_key(m : &MyMap) -> u32{
    m.k
}

pub fn get_val(m: &MyMap) -> u32{
    m.v
}