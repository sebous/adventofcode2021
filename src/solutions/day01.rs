use crate::lib::input;

pub fn run() {
    let input = input::load_input("inputs/01");
    println!("{}", input);
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
