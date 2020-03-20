use backend::map::Map;

fn main() {
    let m = Map::from_file("../maps/simple_test.map").expect("Invalid map");
    println!("{:#?}", m);
}
