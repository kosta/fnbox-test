extern crate fnbox_lib;

fn main() {
    let plus2 = fnbox_lib::named("plus 2", | x: i32| Ok::<i32, i32>(x+2));
    println!("3 + 2 = {:?}", plus2(3))
}
