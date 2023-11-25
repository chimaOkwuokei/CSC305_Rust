#![allow(dead_code)] //This suppresses warnings when a given declared function is  not used.

use core::cmp::Ordering; //Used for comparison of value sizes

// Area = A=(0.5)*b*h
// p = a + b + c

pub fn run() {
    //IMPORTANT GIST

    //trait declaration for triangle 2
    trait Shape {
        //because we're using a trait for triangle2s implementation
        //the triat literally then says, because you're using me
        //you must invoke a 'new' method, as seen below.Parameters
        //used are then passed to the new method
        fn new(a: f64, b: f64, c: f64, h: f64, name: &'static str) -> Self;

        //area and perimeter
        fn area(&self) -> f64; //method that returns the area of the shape with type f64 and takes a mutable reference to self
        fn perimeter(&self) -> f64; //method that returns the perimeter of the shape with type f64 and takes a mutable reference to self

        //a
        fn set_a(&mut self, a: f64); //This method, named set_a, takes a mutable reference to self (an instance implementing the trait) and a parameter a of type f64. It sets the value of some property 'a' within the implementing type.
        fn get_a(&self) -> f64; //This method, named get_a, takes an immutable reference to self and returns a value of type f64. It retrieves the value of the property a from the implementing type.

        //b
        fn set_b(&mut self, b: f64); // same as seen in a
        fn get_b(&self) -> f64; //same as seen in a

        //c
        fn set_c(&mut self, c: f64); //same as seen in a
        fn get_c(&self) -> f64; //same as seen in a

        //h
        fn set_h(&mut self, h: f64); //same as seen in a
        fn get_h(&self) -> f64; //same as seen in a

        fn set_name(&mut self, name: &'static str); //same as seen in a
        fn get_name(&self) -> &str; //same as seen in a but with a string being returned
    }
    //The use of 'static lifetime above ensures that our
    //compiler is clear about the availability of those values, as they are borrowed.
    //static will be available throughout the lifetime of the program.

    ///Use Default to specify the availability of default instance creation without values passed for property
    #[derive(Default, Debug, Clone)]

    //declaration of the struct triangle with its
    //various parameters
    struct Triangle {
        a: f64,
        b: f64,
        c: f64,
        h: f64,

        name: &'static str,
    }

    //for Triangle 1
    //impl(implementations) hold functions that would
    //be used to make implementations with the struct defined

    impl Triangle {
        //default default() function. Will override derived default if any.

        //sample function spoken about above
        //returns the struct ,'Triangle'
        fn default() -> Self {
            Triangle {
                a: 3.4,
                b: 4.5,
                c: 5.2,
                h: 2.3,
                name: "triangle_default_name",
            }
        }
    }

    //for Triangle 2 as well; implementation of the shape trait on the struct, Triangle

    impl Shape for Triangle {
        //Associated function used to create a new Shape
        fn new(a: f64, b: f64, c: f64, h: f64, name: &'static str) -> Self {
            Triangle { a, b, c, h, name }
        }

        //returns the area of the shape
        fn area(&self) -> f64 {
            (0.5) * (self.b * self.h)
        }

        //returns the perimeter of the shape
        fn perimeter(&self) -> f64 {
            self.a + self.b + self.c
        }

        //a
        fn set_a(&mut self, a: f64) {
            self.a = a;
        }

        fn get_a(&self) -> f64 {
            self.a
        }

        //b
        fn set_b(&mut self, b: f64) {
            self.b = b;
        }

        fn get_b(&self) -> f64 {
            self.b
        }

        //c
        fn set_c(&mut self, c: f64) {
            self.c = c;
        }

        fn get_c(&self) -> f64 {
            self.c
        }

        //h
        fn set_h(&mut self, h: f64) {
            self.h = h;
        }

        fn get_h(&self) -> f64 {
            self.h
        }

        //name
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
    impl PartialEq for Triangle {
        fn eq(&self, other: &Self) -> bool {
            println!("{} -- {}", self.perimeter(), other.perimeter());
            self.perimeter() == other.perimeter()
        }
        //method that takes an immutable self, and another
        //immutable Self. The self is the first item being compared
        // the Self is the other item first item would be compared with
        // the method checks if their perimeter is equal
        //returns a bool
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

    impl PartialOrd for Triangle {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            println!("{} -- {}", self.perimeter(), other.perimeter());
            self.perimeter().partial_cmp(&other.perimeter())
            //PartialOrd, in a sense, can check if passed items self and Self
            // are less than, greater than or equal to each other
        }
    }

    //for Triangle 3
    //A conversion implementation into String
    //Expects a string slice with a, b, c, h, name separated by commas
    //the comma is known as it's delimeter
    impl From<&'static str> for Triangle {
        fn from(s: &'static str) -> Triangle {
            let mut parts = s.split(',');

            //a
            let a = match parts.next() {
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

            //b
            let b = match parts.next() {
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

            //c
            let c = match parts.next() {
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

            //h
            let h = match parts.next() {
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
            let name = match parts.next() {
                Some(val) => val,
                None => "",
            };
            Triangle {
                a,
                b,
                c,
                h,
                name: &name,
            }
        }
    }

    let triangle1 = Triangle::default();//traingle1 receives the default triangle struct
    println!("{:#?}", triangle1); //obv prints the struct

    // println!("{}", triangle1.radius);
    // println!("{}", triangle1.name);

    let triangle2 = Triangle::new(5.0, 2.4, 3.0, 2.5, "triangle2");//invokes the implementation of shape
    println!("{:#?}", triangle2); //obv prints the struct

    let triangle3 = Triangle::from("5.0, 2.4, 3.0, 2.5 ,triangle3");//invokes the conversion implementation into String
    println!("{:#?}\n", triangle3); //obv prints the struct

    //part were the implementation of shape actually ocuurs
    //like perimeters and stuff

    //Compare using PartialOrd
    let result1 = triangle1.partial_cmp(&triangle2);//invokes the PartialOrder impl
    println!("result1 = {:?} \n", result1);

    //is triangle1 less than triangle 3?
    let result2 = triangle1.le(&triangle2);//invokes the PartialOrder impl
    println!("result2 = {:?} \n", result2);

    //Compare using PartialEq
    //is triangle2 equal to triangle 3?
    let result3 = triangle2.eq(&triangle3);//invokes the partialEq impl
    println!("result3 = {:?}\n", result3);

    //is triangle2 not equal to triangle 3?
    let result4 = triangle2.ne(&triangle3);//invokes the partialEq impl
    println!("result4 = {:?}", result4);
}
//THANKS FOR LISTENING