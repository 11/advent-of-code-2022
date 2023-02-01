fn main() {
    let test = String::from("A B");
    test.split(' ').collect::<Vec<&str>>());
}
