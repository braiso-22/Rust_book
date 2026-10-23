use std::io;

fn main() {
    let tupla: (u32, f32, char) = (22, 3.0, '😅');

    println!("tupla : {tupla:#?}");

    let (x, y, z) = (200, 64, -31);

    let y = tupla.1;
    println!("x position is: {x} and y is: {y}");

    let array: [i32; 5] = [1, 2, 3, 2, 1];

    println!("first item is: {}",array[0]);
    let months = [
        "Enero",
        "Febrero",
        "Marzo",
        "Abril",
        "Mayo",
        "Junio",
        "Julio",
        "Agosto",
        "Septiembre",
        "Octubre",
        "Noviembre",
        "Diciembre",
    ];

    println!("meses: {months:?}");
    
    let empty = [0;10];
    println!("{empty:?}");
    
    
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
