pub mod supertraits{
    pub fn run(){
        trait Person {
            fn name(&self) -> String;
        }
        
        // Person is a supertrait of Student.
        // Implementing Student requires you to also impl Person.
        trait Student: Person {
            fn university(&self) -> String;
        }
        
        trait Programmer {
            fn fav_language(&self) -> String;
        }
        
        // CompSciStudent (computer science student) is a subtrait of both Programmer 
        // and Student. Implementing CompSciStudent requires you to impl both supertraits.
        trait CompSciStudent: Programmer + Student {
            fn git_username(&self) -> String;
        }
        
        fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
            format!(
                "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
                student.name(),
                student.university(),
                student.fav_language(),
                student.git_username()
            )
        }
        
    }
}

pub mod iterators{
    pub fn run(){
        struct Fibonacci {
            curr: u32,
            next: u32,
        }
        
        // Implement `Iterator` for `Fibonacci`.
        // The `Iterator` trait only requires a method to be defined for the `next` element.
        impl Iterator for Fibonacci {
            // We can refer to this type using Self::Item
            type Item = u32;
        
            // Here, we define the sequence using `.curr` and `.next`.
            // The return type is `Option<T>`:
            //     * When the `Iterator` is finished, `None` is returned.
            //     * Otherwise, the next value is wrapped in `Some` and returned.
            // We use Self::Item in the return type, so we can change
            // the type without having to update the function signatures.
            fn next(&mut self) -> Option<Self::Item> {
                let current = self.curr;
        
                self.curr = self.next;
                self.next = current + self.next;
        
                // Since there's no endpoint to a Fibonacci sequence, the `Iterator` 
                // will never return `None`, and `Some` is always returned.
                Some(current)
            }
        }
        
        // Returns a Fibonacci sequence generator
        fn fibonacci() -> Fibonacci {
            Fibonacci { curr: 0, next: 1 }
        }
        
       
            // `0..3` is an `Iterator` that generates: 0, 1, and 2.
            let mut sequence = 0..3;
        
            println!("Four consecutive `next` calls on 0..3");
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());
        
            // `for` works through an `Iterator` until it returns `None`.
            // Each `Some` value is unwrapped and bound to a variable (here, `i`).
            println!("Iterate through 0..3 using `for`");
            for i in 0..3 {
                println!("> {}", i);
            }
        
            // The `take(n)` method reduces an `Iterator` to its first `n` terms.
            println!("The first four terms of the Fibonacci sequence are: ");
            for i in fibonacci().take(4) {
                println!("> {}", i);
            }
        
            // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
            println!("The next four terms of the Fibonacci sequence are: ");
            for i in fibonacci().skip(4).take(4) {
                println!("> {}", i);
            }
        
            let array = [1u32, 3, 3, 7];
        
            // The `iter` method produces an `Iterator` over an array/slice.
            println!("Iterate the following array {:?}", &array);
            for i in array.iter() {
                println!("> {}", i);
            }
        }
    }

pub mod drop{
    pub fn run(){
        struct Droppable {
            name: &'static str,
        }
        
        // This trivial implementation of `drop` adds a print to console.
        impl Drop for Droppable {
            fn drop(&mut self) {
                println!("> Dropping {}", self.name);
            }
        }
       
            let _a = Droppable { name: "a" };
        
            // block A
            {
                let _b = Droppable { name: "b" };
        
                // block B
                {
                    let _c = Droppable { name: "c" };
                    let _d = Droppable { name: "d" };
        
                    println!("Exiting block B");
                }
                println!("Just exited block B");
        
                println!("Exiting block A");
            }
            println!("Just exited block A");
        
            // Variable can be manually dropped using the `drop` function
            drop(_a);
            // TODO ^ Try commenting this line
        
            println!("end of the main function");
        
            // `_a` *won't* be `drop`ed again here, because it already has been
            // (manually) `drop`ed
        }
    }


pub mod r#dyn{
    pub fn run(){
        struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());


    }
}

pub mod derived{

    pub fn run(){
// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, a tuple struct with no additional attributes
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    
    //println!("One second looks like: {:?}", _one_second);
    // TODO ^ Try uncommenting this line

    // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
    //let _this_is_true = (_one_second == _one_second);
    // TODO ^ Try uncommenting this line

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
}
}

pub mod traits{
    pub fn run(){
        struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}


    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();

    }
}

pub mod diverging_functions{
    pub fn run(){
            fn sum_odd_numbers(up_to: u32) -> u32 {
                let mut acc = 0;
                for i in 0..up_to {
                    // Notice that the return type of this match expression must be u32
                    // because of the type of the "addition" variable.
                    let addition: u32 = match i%2 == 1 {
                        // The "i" variable is of type u32, which is perfectly fine.
                        true => i,
                        // On the other hand, the "continue" expression does not return
                        // u32, but it is still fine, because it never returns and therefore
                        // does not violate the type requirements of the match expression.
                        false => continue,
                    };
                    acc += addition;
                }
                acc
            }
            println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
        
    }
}

pub mod higher_order_functions{
    pub fn run(){
        fn is_odd(n: u32) -> bool {
            n % 2 == 1
        }
        
      
            println!("Find the sum of all the squared odd numbers under 1000");
            let upper = 1000;
        
            // Imperative approach
            // Declare accumulator variable
            let mut acc = 0;
            // Iterate: 0, 1, 2, ... to infinity
            for n in 0.. {
                // Square the number
                let n_squared = n * n;
        
                if n_squared >= upper {
                    // Break loop if exceeded the upper limit
                    break;
                } else if is_odd(n_squared) {
                    // Accumulate value, if it's odd
                    acc += n_squared;
                }
            }
            println!("imperative style: {}", acc);
        
            // Functional approach
            let sum_of_squared_odd_numbers: u32 =
                (0..).map(|n| n * n)                             // All natural numbers squared
                     .take_while(|&n_squared| n_squared < upper) // Below upper limit
                     .filter(|&n_squared| is_odd(n_squared))     // That are odd
                     .sum();                                     // Sum them
            println!("functional style: {}", sum_of_squared_odd_numbers);
        }
    
}

pub mod output_parameters{
    pub fn run(){
        fn create_fn() -> impl Fn() {
            let text = "Fn".to_owned();
        
            move || println!("This is a: {}", text)
        }
        
        fn create_fnmut() -> impl FnMut() {
            let text = "FnMut".to_owned();
        
            move || println!("This is a: {}", text)
        }
        
        fn create_fnonce() -> impl FnOnce() {
            let text = "FnOnce".to_owned();
        
            move || println!("This is a: {}", text)
        }
        
        
            let fn_plain = create_fn();
            let mut fn_mut = create_fnmut();
            let fn_once = create_fnonce();
        
            fn_plain();
            fn_mut();
            fn_once();
      
    }
}

pub mod input_functions{
    pub fn run(){
        // Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

    }
}

pub mod type_anonymity{
    pub fn run(){
        // `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(f: F) where
F: Fn() {
f();
}

let x = 7;

// Capture `x` into an anonymous type and implement
// `Fn` for it. Store it in `print`.
let print = || println!("{}", x);

apply(print);

    }
}

pub mod input_parameters{
    pub fn run(){
        // A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
        fn apply<F>(f: F) where
// The closure takes no input and returns nothing.
        F: FnOnce() {
// ^ TODO: Try changing this to `Fn` or `FnMut`.

        f();
    }

// A function which takes a closure and returns an `i32`.
        fn apply_to_3<F>(f: F) -> i32 where
// The closure takes an `i32` and returns an `i32`.
        F: Fn(i32) -> i32 {

        f(3)
    }


use std::mem;

let greeting = "hello";
// A non-copy type.
// `to_owned` creates owned data from borrowed one
let mut farewell = "goodbye".to_owned();

// Capture 2 variables: `greeting` by reference and
// `farewell` by value.
let diary = || {
    // `greeting` is by reference: requires `Fn`.
    println!("I said {}.", greeting);

    // Mutation forces `farewell` to be captured by
    // mutable reference. Now requires `FnMut`.
    farewell.push_str("!!!");
    println!("Then I screamed {}.", farewell);
    println!("Now I can sleep. zzzzz");

    // Manually calling drop forces `farewell` to
    // be captured by value. Now requires `FnOnce`.
    mem::drop(farewell);
};

// Call the function which applies the closure.
apply(diary);

// `double` satisfies `apply_to_3`'s trait bound
let double = |x| 2 * x;

println!("3 doubled: {}", apply_to_3(double));
}
    
}

pub mod capture{
    pub fn run(){
            use std::mem;
            
            let color = String::from("green");
        
            // A closure to print `color` which immediately borrows (`&`) `color` and
            // stores the borrow and closure in the `print` variable. It will remain
            // borrowed until `print` is used the last time. 
            //
            // `println!` only requires arguments by immutable reference so it doesn't
            // impose anything more restrictive.
            let print = || println!("`color`: {}", color);
        
            // Call the closure using the borrow.
            print();
        
            // `color` can be borrowed immutably again, because the closure only holds
            // an immutable reference to `color`. 
            let _reborrow = &color;
            print();
        
            // A move or reborrow is allowed after the final use of `print`
            let _color_moved = color;
        
        
            let mut count = 0;
            // A closure to increment `count` could take either `&mut count` or `count`
            // but `&mut count` is less restrictive so it takes that. Immediately
            // borrows `count`.
            //
            // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
            // calling the closure mutates the closure which requires a `mut`.
            let mut inc = || {
                count += 1;
                println!("`count`: {}", count);
            };
        
            // Call the closure using a mutable borrow.
            inc();
        
            // The closure still mutably borrows `count` because it is called later.
            // An attempt to reborrow will lead to an error.
            // let _reborrow = &count; 
            // ^ TODO: try uncommenting this line.
            inc();
        
            // The closure no longer needs to borrow `&mut count`. Therefore, it is
            // possible to reborrow without an error
            let _count_reborrowed = &mut count; 
        
            
            // A non-copy type.
            let movable = Box::new(3);
        
            // `mem::drop` requires `T` so this must take by value. A copy type
            // would copy into the closure leaving the original untouched.
            // A non-copy must move and so `movable` immediately moves into
            // the closure.
            let consume = || {
                println!("`movable`: {:?}", movable);
                mem::drop(movable);
            };
        
            // `consume` consumes the variable so this can only be called once.
            consume();
            // consume();
            // ^ TODO: Try uncommenting this line.
    }
}

pub mod closure{
    pub fn run() {
        let outer_var = 42;
        
        // A regular function can't refer to variables in the enclosing environment
        //fn function(i: i32) -> i32 { i + outer_var }
        // TODO: uncomment the line above and see the compiler error. The compiler
        // suggests that we define a closure instead.
    
        // Closures are anonymous, here we are binding them to references
        // Annotation is identical to function annotation but is optional
        // as are the `{}` wrapping the body. These nameless functions
        // are assigned to appropriately named variables.
        let closure_annotated = |i: i32| -> i32 { i + outer_var };
        let closure_inferred  = |i     |          i + outer_var  ;
    
        // Call the closures.
        println!("closure_annotated: {}", closure_annotated(1));
        println!("closure_inferred: {}", closure_inferred(1));
        // Once closure's type has been inferred, it cannot be inferred again with another type.
        //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
        // TODO: uncomment the line above and see the compiler error.
    
        // A closure taking no arguments which returns an `i32`.
        // The return type is inferred.
        let one = || 1;
        println!("closure returning one: {}", one());
    
    }
}

pub mod enums{
    pub fn run(){
        // Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
        enum WebEvent {
            // An `enum` variant may either be `unit-like`,
            PageLoad,
            PageUnload,
            // like tuple structs,
            KeyPress(char),
            Paste(String),
            // or c-like structures.
            Click { x: i64, y: i64 },
        }

        // A function which takes a `WebEvent` enum as an argument and
        // returns nothing.
        fn inspect(event: WebEvent) {
            match event {
                WebEvent::PageLoad => println!("page loaded"),
                WebEvent::PageUnload => println!("page unloaded"),
                // Destructure `c` from inside the `enum` variant.
                WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
                WebEvent::Paste(s) => println!("pasted \"{}\".", s),
                // Destructure `Click` into `x` and `y`.
                WebEvent::Click { x, y } => {
                    println!("clicked at x={}, y={}.", x, y);
                },
            }
        }


            let pressed = WebEvent::KeyPress('x');
            // `to_owned()` creates an owned `String` from a string slice.
            let pasted  = WebEvent::Paste("my text".to_owned());
            let click   = WebEvent::Click { x: 20, y: 80 };
            let load    = WebEvent::PageLoad;
            let unload  = WebEvent::PageUnload;

            inspect(pressed);
            inspect(pasted);
            inspect(click);
            inspect(load);
            inspect(unload);


    }
}

pub mod structs{
    pub fn run(){
        // An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
    
}