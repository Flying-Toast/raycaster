use backend::map::Map;
use backend::vector::{Vector};

fn main() {
    let m = Map::from_file("../maps/simple_test.map").expect("Invalid map");
    let a = m.get_tile(Vector::new(3.0, 5.0));
    let b = m.get_tile(Vector::new(3.0, 9.0));
    println!("{:#?}", a);
    println!("{:#?}", b);
}
