fn greet_hello() {
    println! ("Hello, world!");
    let southern_germany = "Hallo Welt!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_hello();
}
