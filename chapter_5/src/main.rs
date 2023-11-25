struct Rect{
    width: u64,
    length: u64,
}

fn main() {
    // let rect = calc_rect(
        // 30,
        // 20
    // );
    let rect = Rect{
        width: 30,
        length: 40
    };

    let calc = rect.width * rect.length;
    println!("{}",calc);
}

// fn calc_rect(width:u64, length:u64) -> Rect{
    // Rect{
        // width,
        // length,
    // }
// }