pub fn run() {
    // before transaction
    let x: f32 = 10_000.0; // amount tokens x in pool
    let y: f32 = 40_000_000.0; // amount tokens y in pool
    let fee = 1.0 - 0.003; // uniswap fee
    let l = x * y; // constant L

    println!("1. pool before swap");
    println!("x={}ETH, y=${}, L={}", x, y, l);

    // after transaction
    let delta: f32 = 250.0; // amount out of token x
    let amount_out = (delta * y) / (fee * (x - delta));
    let x1 = x - delta; // amount tokens x after
    let y1 = y + amount_out; // amount tokens y after

    let li = x1 * y1; // constant L after trade

    assert!(li > l);

    println!("\n2. pool after swap");
    println!("x={}ETH, y=${}, L={}, USDC_in=${}", x1, y1, li, amount_out);

    // Impact of trade
    let p = y / x; // spot price
    let pe = amount_out / (x - x1); // effective price
    let impact = pe * (x - x1) - p * (x - x1); // price impact of trade
    let pi = y1 / x1; // new price after trade

    println!("\n\tprice impact of swap");
    println!("\tp=${}, pe=${}, i=${}, pi=${}", p, pe, impact, pi);

    // if we buy after swap
    let amount_out2 = (delta * (y + amount_out)) / (fee * (x - 2.0 * delta));
    let x2 = x1 - delta;
    let y2 = y1 + amount_out2;
    let l2 = x2 * y2;
    println!("\n3. price on 2nd swap");
    println!("x=${}, y=${}, L=${}, USDC_in=${}", x2, y2, l2, amount_out2);
}
