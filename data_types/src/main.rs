fn main() {
    let edad = 24;
    println!("let {edad}");

    let edad: i32 = "25".parse().expect("edad tiene que ser un numero");
    println!("shadow {edad}");

    let mut edad: i8 = 24;
    println!("mut i8 {edad}");

    // panics in debug
    // edad = edad + 127;

    edad = edad.wrapping_add(127);
    println!("mutated wrapping {edad}");

    let mut edad: u8 = 24;
    println!("mut u8 {edad}");

    edad = edad.wrapping_add(255);
    println!("mutated wrapping {edad}");

    let mut tarta: f64 = 1.0;
    println!("tarta sin modificar {tarta}");

    tarta = tarta / 3.0;
    println!("tarta cortada {tarta}");

    let tarta: f32 = tarta as f32;
    println!("tarta con menos precision {tarta}");
    // panics a shadowed not mut cannot be modified
    // tarta = 2

    let personas = 7;

    let turnos = 3;

    let ejecutor = personas % turnos;

    println!("hay {personas} personas y {turnos} turnos, le toca a {ejecutor}");
    let hecho = false;
    println!("hecho {hecho}");
    let (letra1, letra2) = ('A', 'A');
    let resultado = letra1 == letra2;
    println!("A = A {resultado}");
}
