// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> &'static str {
    "blue"
}

fn main() {
    let answer : &str = current_favorite_color();
    println!("My current favorite color is {answer}");
}
