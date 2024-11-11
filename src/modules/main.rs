use crate::garden::vegetables::Asparagus;
use crate::garden::vegetables::Testvege;


pub mod garden;

fn main() {
    let plant = Asparagus {};
    let test: Testvege = Testvege {};
    println!("I'm growing {plant:?}!");
    println!("I'm growing {test:?}!")

}