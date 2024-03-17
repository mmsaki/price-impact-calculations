mod amount_out;
mod swap_math;
mod swap_no_fees;
fn main() {
    println!("-------- swap w/o fees ---------");
    swap_no_fees::run();

    println!("\n-------- swap: amount in ---------");
    swap_math::run();

    println!("\n-------- swap: amount out ---------");
    amount_out::run();
}
