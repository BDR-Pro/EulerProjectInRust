pub mod cube;
pub mod factorial;
pub mod namescore;
pub mod fibonacci;
fn main() {

    cube::main();
    println!("#########################");
    factorial::main();
    println!("#########################");
    namescore::main().unwrap();
    println!("#########################");
    fibonacci::main();

}
