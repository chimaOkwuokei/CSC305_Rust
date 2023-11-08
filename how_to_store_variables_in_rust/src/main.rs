mod data_types;
use data_types::derived::user_defined;
use data_types::primitive::scalar;
use data_types::primitive::compound;

fn multiplier(array: &[i64]){
    let mut product = 1;

    for i in 0..array.len() { // Oops, one element too far!
        product *= array[i];
    }

    println!("{}", product);
}

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
    println!("");
    println!("Closure");
    user_defined::closure::run();
    println!("");
    println!("Capturing");
    user_defined::capture::run();
    println!("");
    println!("Input parameters");
    user_defined::input_parameters::run();
    println!("");
    println!("type_anonymity");
    user_defined::type_anonymity::run();
    println!("");
    println!("input_functions");
    user_defined::input_functions::run();
    println!("");
    println!("Output Parameters");
    user_defined::output_parameters::run();
    println!("");
    println!("Higher order functions");
    user_defined::higher_order_functions::run();
    println!("");
    println!("Diverging functions");
    user_defined::diverging_functions::run();
    
    let array: [i64; 5] = [1, 2, 3, 4, 5];
    multiplier(&array);
}
