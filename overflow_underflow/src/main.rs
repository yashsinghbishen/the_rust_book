use std::any::type_name_of_val;
fn main() {
    println!("\n\n Overflow and Underflow example:");
    overflow_underflow_example();
}

fn overflow_underflow_example() {
    let max_u8: u8 = u8::MAX;
    let min_u8: u8 = u8::MIN;
    let max_i8: i8 = i8::MAX;
    let min_i8: i8 = i8::MIN;

    println!(
        "Overflow in {}, {max_u8} + 1 returns {}",
        type_name_of_val(&max_u8),
        max_u8 + 1
    );
    println!(
        "Underflow in {}, {min_u8} - 1 returns {}",
        type_name_of_val(&min_u8),
        min_u8 - 1
    );
    println!(
        "Overflow in {}, {max_i8} + 1 returns {}",
        type_name_of_val(&max_i8),
        max_i8 + 1
    );
    println!(
        "Underflow in {}, {min_i8} - 1 returns {}",
        type_name_of_val(&min_i8),
        min_i8 - 1
    );

}
