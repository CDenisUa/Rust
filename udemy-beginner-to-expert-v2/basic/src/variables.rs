// 📌 Constants
pub const MAX_POINTS: u32 = 100;

pub fn print_variables() {
    println!("============== 📦 Constants, Variables, Scope ==============\n");

    // 🔹 Definition
    let x: i32 = 10;
    let y: f64 = 10.2;
    let z_var: i16 = 10;
    let x_var: i16 = 22;

    println!("📌 Constant (MAX_POINTS):        {}", MAX_POINTS);
    println!("📥 Variable (x: i32):            {}", x);
    println!("📥 Variable (y: f64):            {}", y);
    println!("📥 Variable (z_var: i16):        {}", z_var);
    println!("📥 Variable (x_var: i16):        {}", x_var);

    // 🔄 Mutability
    let mut v: i16 = 5;
    println!("\n✏️  Mutable variable (v):         {}", v);
    v = x_var + z_var;
    println!("🔁 v after modification:          {}", v);

    // 📦 Scope
    println!("\n📦 Scope Block:");
    {
        let _z = 50;
        let _ = 500;
        println!("   (Inside scope: created _z and unnamed value)");
    }
    println!("   Scope ended — _z no longer accessible");

    // 🧥 Shadowing
    let t = 10;
    let t = t + 10;
    println!("\n🧥 Shadowing (t):                {}", t);

    println!("\n============================================================");
}