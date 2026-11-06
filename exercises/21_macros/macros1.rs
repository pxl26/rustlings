macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($x:expr) => {
        println!("This is a number: {}", $x);
    };
}

fn main() {
    // TODO: Fix the macro call.
    my_macro!(1);
}
