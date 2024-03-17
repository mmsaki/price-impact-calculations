mod swap_math;
mod swap_no_fees;
fn main() {
    println!("-------- no fees math ---------");
    swap_no_fees::run();

    println!("\n-------- swap w/ fees ---------");
    swap_math::run();
}
