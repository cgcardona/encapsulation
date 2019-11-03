mod car;
mod tree;
use car::Car;
use tree::Tree;

fn main() {
    let c1: Car = Car::new(Some("blue"));
    println!("{:#?}", c1);

    let c2: Car = Car::new(None);
    println!("{:#?}", c2);

    let t1: Tree = Tree::new(Some("green"));
    println!("{:#?}", t1);

    let t2: Tree = Tree::new(None);
    println!("{:#?}", t2);
}
