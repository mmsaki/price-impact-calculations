mod swap_math;
mod swap_no_fees;
fn main() {
    println!("-------- swap w/o fees ---------");
    swap_no_fees::run();

    println!("\n-------- swap with fees ---------");
    swap_math::run();
}
