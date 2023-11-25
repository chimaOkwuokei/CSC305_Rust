//calls the module shapes
mod shapes;

//basically, from shapes, use the modules circle, and triangle
use shapes::circle;
use shapes::triangle;

fn main() {
    println!("BEGINNING OF CIRCLE;");
    circle::run();//uses the run() fn declared in the module, circle
    println!("END OF CIRCLE!\n");

    println!("BEGINNING OF TRIANGLE;");
    triangle::run();//uses the run() fn declared in the module, triangle
    println!("END OF TRIANGLE!");
}
//THANKS FOR LISTENING
//AWESOME PROJECT!