#![allow(dead_code)] //This suppresses warnings when a given declared function is  not used.

use core::cmp::Ordering; //Used for comparison of value sizes

// Area = πr2
// C = 2πr
pub fn run() {
    //IMPORTANT GIST

    //trait declaration for circle 2
    trait Shape {
        //because we're using a trait for circle2s implementation
        //the triat literally then says, because you're using me
        //you must invoke a new method, as seen below.Parameters
        //used are then passed to the new method
        fn new(radius: f64, name: &'static str) -> Self;

        //area and perimeter
        fn area(&self) -> f64; //method that returns the area of the shape with type f64 and takes a mutable reference to self
        fn perimeter(&self) -> f64; //method that returns the perimeter of the shape with type f64 and takes a mutable reference to self

        //radius
        fn set_radius(&mut self, radius: f64); //This method, named set_radius, takes a mutable reference to self (an instance implementing the trait) and a parameter radius of type f64. It sets the value of some property 'radius' within the implementing type.
        fn get_radius(&self) -> f64; //This method, named get_radius, takes an immutable reference to self and returns radius value of type f64. It retrieves the value of the property radius from the implementing type.

        //name
        fn set_name(&mut self, name: &'static str); //same as seen in radius
        fn get_name(&self) -> &str; //same as seen in radius
    }
    //The use of 'static lifetime above ensures that our
    //compiler is clear about the availability of those values, as they are borrowed.
    //static will be available throughout the lifetime of the program.

    ///Use Default to specify the availability of default instance creation without values passed for property
    #[derive(Default, Debug, Clone)]
    //declaration of the struct circle with its
    //various parameters
    struct Circle {
        radius: f64,
        name: &'static str,
    }

    //for circle 1
    //impl(implementations) hold functions that would
    //be used to make implementations with the struct defined

    impl Circle {
        //default default() function. Will override derived default if any.
        //sample function spoken about above
        //returns the struct ,'Circle'

        fn default() -> Self {
            Circle {
                radius: 3.9,
                name: "circle_default_name",
            }
        }
    }
    //impl returns a struct, Circle

    //for Circle 2 as well; implementation of the shape trait on the struct, Circle

    impl Shape for Circle {
        //Associated function used to create a new Shape
        fn new(radius: f64, name: &'static str) -> Self {
            Circle { radius, name }
        }

        //returns the area of the shape
        fn area(&self) -> f64 {
            (3.142) * (self.radius * self.radius)
        }

        //returns the perimeter of the shape
        fn perimeter(&self) -> f64 {
            (2.0) * (3.142) * (self.radius)
        }

        fn set_radius(&mut self, radius: f64) {
            self.radius = radius;
        }

        fn get_radius(&self) -> f64 {
            self.radius
        }

        fn set_name(&mut self, name: &'static str) {
            self.name = name;
        }

        fn get_name(&self) -> &str {
            self.name
        }
    }

    //its only at this place that the perimeter function is used
    //lines 88 - 107
    //implement Partial Eq
    impl PartialEq for Circle {
        fn eq(&self, other: &Self) -> bool {
            println!("{} -- {}", self.perimeter(), other.perimeter());
            self.perimeter() == other.perimeter()
            //method that takes an immutable self, and another
            //immutable Self. The self is the first item being compared
            // the Self is the other item first would be compared with
            // the method checks if their perimeter is equal
        }

        fn ne(&self, other: &Self) -> bool {
            //println!("{} -- {}", self.perimeter(), other.perimeter());
            !self.eq(other)
            //method that takes an immutable self, and another
            //immutable Self. The self is the first item being compared
            // the Self is the other item first would be compared with
            // the method checks if their perimeter is not equal
            //returns a bool
        }
    }

    impl PartialOrd for Circle {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            println!("{} -- {}", self.perimeter(), other.perimeter());
            self.perimeter().partial_cmp(&other.perimeter())
            //PartialOrd, in a sense, can check if passed items self and Self
            // are less than, greater than or equal to each other

        }
    }

    //for Circle 3
    //A conversion implementation into String
    //Expects a string slice with radius,name, separated by commas
    //the comma is known as its delimeter
    impl From<&'static str> for Circle {
        fn from(s: &'static str) -> Circle {
            let mut parts = s.split(',');
            let radius = match parts.next() {
                Some(val) => match val.trim().parse::<f64>() {
                    Ok(parsed_val) => parsed_val,
                    Err(e) => {
                        // Handle the error, e.g., print it or return a default value
                        println!("Error parsing float: {}", e);
                        // Return a default value or handle the error in another way
                        // For example, you might want to return a Result<f64, YourErrorType>
                        0.0
                    }
                },
                None => 0.0, // Default value when the string is empty
            };

            // let width = match parts.next() {
            //     Some(val) => val.parse::<f64>().unwrap(),
            //     None => 0,
            // };
            let name = match parts.next() {
                Some(val) => val,
                None => "",
            };
            Circle {
                radius,
                name: &name,
            }
        }
    }

    let circle1 = Circle::default();//circle1 recieves the default circle struct
    println!("{:#?}", circle1); //obv prints the struct

    // println!("{}", circle1.radius);
    // println!("{}", circle1.name);

    let circle2 = Circle::new(5.0, "circle2");//invokes the implementation of shape
    println!("{:#?}", circle2); //obv prints the struct

    let circle3 = Circle::from("5.0 ,circle3");//invokes the conversion implementation into String
    println!("{:#?}\n", circle3); //obv prints the struct

    //part were the implementation of shape actually ocuurs
    //like perimeters and stuff

    //Compare using PartialOrd
    let result1 = circle1.partial_cmp(&circle2);//invokes the PartialOrder impl
    println!("result1 = {:?} \n", result1);

    //is circle1 less than circle 3?
    let result2 = circle1.le(&circle2);//invokes the PartialOrder impl
    println!("result2 = {:?} \n", result2);

    //Compare using PartialEq
    //is circle2 equal to circle 3?
    let result3 = circle2.eq(&circle3);//invokes the partialEq impl
    println!("result3 = {:?}\n", result3);

    //is circle2 not equal to circle 3?
    let result4 = circle2.ne(&circle3);//invokes the partialEq impl
    println!("result4 = {:?}", result4);
}
//THANKS FOR LISTENING
