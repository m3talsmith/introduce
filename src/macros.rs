macro_rules! introduce {
    () => {
        println!("Hello, I'm World!");
    };
    ($name:expr) => {
        let name: &str = $name;
        println!("Hello, I'm {}!", name);
    };
    ($name:expr, $age:expr) => {
        let name: &str = $name;
        let age: i32 = $age;
        println!("Hello, I'm {} and I'm {} years old!", name, age);
    };
    ($name:expr, $age:expr, $($items:tt)*) => {
        let name: &str = $name;
        let age: i32 = $age;
        let items: Vec<&str> = $($items)*;
        let mut message = format!("Hello, I'm {} and I'm {} years old!\n\nI have {} items in my backpack:\n", name, age, items.len());
        for item in items {
            message.push_str(&format!("\t- {}\n", item));
        }
        println!("{}", message);
    };
}
pub(crate) use introduce;
