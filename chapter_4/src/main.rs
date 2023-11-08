//ownership
// fn main() {
    // let a: [i32; 5]= [1,2,3,4,5];
    // let slice: &[i32] = &a[0..2];

    // println!("{}",slice);
// }


// structs
/*struct parameters{
    length: u64,
    height: u64
}
fn main(){
   /* let mut user1 = User{
        email: String::from("chima@gmail.com");
        username:String::from("Chima");
        sign_in_count: 1;
        active: true;
    }*/

   /*  let name  = user1.username;
    user1.username = String::from("Daniel");*/
 
    let param = parameters{length:30, height:50};
   
    println!("The area of the rectangle is {}", calculate_area(&param));
}

fn calculate_area(param: &parameters) -> u64{
    
    param.length * param.height  
}*/

//Strings
use unicode_segmentation::UnicodeSegmentation

fn main(){
    for g in "chima".graphemes(true){
        println!("{}", g[0]);
    }
}