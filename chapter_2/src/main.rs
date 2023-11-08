use std::io;

fn main() {
    println!("Guess the number!");
    let mystery_num = 32;

    loop{
        println!("Please input your guess: ");

    //how to input an integer
    let mut guess= String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");
    let guess:i32 = guess.trim().parse().expect("Input not an integer");
    println!("Your guess is {} ", guess);

    if mystery_num == guess {
        println!("Correct");
        break;
    }
        
    else{
        println!("Wrong");
    }
    }
    
        
        
}
