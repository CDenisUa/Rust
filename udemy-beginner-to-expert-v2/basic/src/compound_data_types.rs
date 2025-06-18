pub fn compound_data_types() {
    println!("============== 🧬 Compound Data Types ==============\n");

    // &str and String
    let fixed_str = "Fixed length string";
    let mut flexible_str = String::from("This string will grow");

    flexible_str.push('!');
    flexible_str.push_str(" And grow more");

    let s2: String = fixed_str.to_string(); // &str -> String
    let s3: &str = &s2;                     // String -> &str

    println!("🔤 &str literal:                     {}", fixed_str);
    println!("📦 String (mutable):                {}", flexible_str);
    println!("🔁 Converted &str -> String:         {}", s2);
    println!("🔁 Converted String -> &str:         {}", s3);

    // Arrays
    let mut array_1: [i32; 5] = [4, 5, 8, 9, 3];
    let num = array_1[3];
    array_1[0] = 100;

    let array_2: [i32; 10] = [0; 10];

    println!("\n🧱 Array 1:                         {:?}", array_1);
    println!("📍 Element at index 3:              {}", num);
    println!("🔁 Initialized array with zeros:    {:?}", array_2);

    // Vectors
    let vec_1: Vec<i32> = vec![4, 5, 6, 8, 9];
    println!("\n📚 Vector (Vec<i32>):              {:?}", vec_1);

    // Tuples
    let my_info = ("Salary", 40000, "Age", 40);
    // let salary_value = my_info.1;
    let (salary_label, salary_value, age_label, age_value) = my_info;

    println!("\n👥 Tuple:                          {:?}", my_info);
    println!("💵 {}:                           {}", salary_label, salary_value);
    println!("🎂 {}:                               {}", age_label, age_value);

    println!("\n=====================================================\n");
}