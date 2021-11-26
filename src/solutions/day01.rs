use crate::lib::input::load_input;

pub fn run() {
    let input = load_input("inputs/01");
    println!("{}", input);
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
