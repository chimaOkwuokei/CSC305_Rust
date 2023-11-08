mod data_types;
use data_types::derived::user_defined;
use data_types::primitive::scalar;
use data_types::primitive::compound;

fn main() {
    //SCALARS
    //boolean
    println!("Booleans");
    scalar::boolean::run();
    println!("");
    //float
    println!("Floats");
    scalar::float::run();
    println!("");
    //literals
    println!("Literals");
    scalar::literals::run();
    println!("");
    println!("Integers");
    scalar::int::run();
    println!("");
    println!("Type Casting");
    scalar::casting::run();
    println!("");
    println!("Aliasing");
    scalar::aliasing::run();
    println!("");

    //COMPOUNDS
    println!("Tuples");
    compound::tuples::run();
    println!("");
    println!("Arrays");
    compound::arrays::run();
    

    //USER DEFINED
    println!("");
    println!("Enums");
    user_defined::enums::run();
    println!("");
    println!("Structs");
    user_defined::structs::run();

}
