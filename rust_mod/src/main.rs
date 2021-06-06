mod my_map;
mod my_list;

use my_list::list;

fn main() {

    let m = my_map::MyMap{k:10,v:20};
    let k = my_map::get_key(&m);
    println!("m.k is {}", k);
    let v = my_map::get_val(&m);
    println!("m.v is {}", v);

    let l = list::List{size:20};
    let size = l.get_size();
    println!("list.size = {}", size);
}


