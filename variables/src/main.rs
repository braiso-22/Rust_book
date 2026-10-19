fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The new value of x is {x}");

    let y = 2;
    println!("The value of y is: {y}");

    let y = y + 5;
    println!("Now it is {y}");

    let string :i32 = "132".parse().expect("Was not a number");
    let string = string*2;
    println!("{string}");
}
