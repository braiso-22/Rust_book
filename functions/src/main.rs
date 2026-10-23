fn main() {
    println!("Hello, world!");
    my_first_snake_function(1);
    my_first_snake_function(5);
    print_labeled_measurement(5, 'h');
    println!("function call result: {}", plus_one(five()));
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn my_first_snake_function(number: u32) {
    println!("Functions in rust follow snake_case {number}")
}
/* overload doesn't work like this in rust
fn my_first_snake_function(){
    println!("Functions in rust follow snake_case")
}
*/

fn five() -> i32 {
    // return 5;
    // return is not needed as last expression
    5
}

fn plus_one(number: i32) -> i32 {
    // number +1; throws error because adding ; is an statement not an expression
    number + 1
}
