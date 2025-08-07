mod macros;

fn main() {
    introduce!();
    introduce!("John");
    introduce!("John", 20);
    introduce!("John", 20, vec!["apple", "banana", "cherry"]);
}
