fn main() {
    let mut test = String::from("abcd1234");
    let test2 = test.split_off(4);
    println!("{} {}", test, test2);
}
