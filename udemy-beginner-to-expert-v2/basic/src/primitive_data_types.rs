pub fn primitive_data_types() {
    println!("============== 🧱 Primitive Data Types in Rust ==============\n");

    // Unsigned integers
    let unsigned_num: u8 = 5;
    println!("🔢 Unsigned Integer (u8):             {}", unsigned_num);

    // Signed integers
    let signed_num: i8 = 5;
    println!("➖ Signed Integer (i8):               {}", signed_num);

    // Floating point numbers
    let float_num: f32 = 5.0;
    println!("🌊 Floating Point (f32):             {}", float_num);

    // Platform specific integers
    let arch_1: usize = 5;
    let arch_2: isize = 5;
    println!("🧱 Platform-dependent (usize):        {}", arch_1);
    println!("🧱 Platform-dependent (isize):        {}", arch_2);

    // Characters
    let char: char = 'a';
    println!("🔤 Character (char):                 '{}'", char);

    // Boolean
    let b: bool = true;
    println!("✅ Boolean (bool):                   {}", b);

    // Type aliasing
    type Age = u8;
    let denys_age: Age = 36;
    println!("👤 Type Alias (Age = u8):            {} (Denys's age)", denys_age);

    // Type Conversion
    let a: i32 = 10;
    let b: f64 = a as f64;
    println!("🔁 Type Conversion (i32 to f64):     i32 = {}, f64 = {}", a, b);

    println!("\n==============================================================");
}