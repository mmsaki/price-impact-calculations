pub fn run() {
    // before transaction
    let a: f32 = 10_000.0; // amount tokens A in pool
    let b: f32 = 40_000_000.0; // amount tokens B in pool
    let fee = 1.0 - 0.003; // uniswap fee
    let k = a * b; // constant k

    println!("Pool before swap");
    println!("A={}ETH, B=${}, k={}", a, b, k);

    // after transaction
    let delta: f32 = 250.0; // amount out of token x
    let amount_out = (delta * b) / (fee * (a - delta));
    let x = a - delta; // amount tokens A after
    let y = b + amount_out; // amount tokens B after

    let ki = x * y; // constant k after trade

    assert!(ki > k);

    println!("\t1. Pool after 1st swap");
    println!("\tA={}ETH, B=${}, k={}, USDC_in=${}", x, y, ki, amount_out);

    // if we buy after swap
    let amount_out2 = (delta * (b + amount_out)) / (fee * (a - 2.0 * delta));
    let x2 = x - delta;
    let y2 = y + amount_out2;
    let k2 = x2 * y2; // constant k after trade
    println!("\t2. Price on 2nd swap");
    println!(
        "\tA=${}, B=${}, k=${}, USDC_in=${}\n",
        x2, y2, k2, amount_out2
    );

    // Impact of trade
    let p = b / a; // spot price
    let pe = amount_out / (a - x); // effective price
    let impact = pe * (a - x) - p * (a - x); // price impact of trade
    let pi = y / x; // new price after trade

    println!("Price impact of swap");
    println!("p=${}, pe=${}, i=${}, pi=${}", p, pe, impact, pi);
}
