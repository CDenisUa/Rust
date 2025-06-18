// Modules
mod variables;
mod primitive_data_types;
mod compound_data_types;

fn main() {
    println!("========== 🚀 Main Program ==========\n");

    println!("🔧 Variables:");
    variables::print_variables();
    println!();

    println!("🧱 Primitive Data Types:");
    primitive_data_types::primitive_data_types();
    println!();

    println!("🧬 Compound Data Types:");
    compound_data_types::compound_data_types();
    println!();

    println!("======================================");
}