use meta::btree::*;

mod meta;

fn main() {
    let a: IntItem = 1;
    let b: IntItem = 2;
    println!("{}", a.less(&b));
}
