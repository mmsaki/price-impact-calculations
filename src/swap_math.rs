pub fn run() {
    // before transaction
    let x: f32 = 200.0; // amount tokens x in pool
    let y: f32 = 800_000.0; // amount tokens y in pool
    let fee = 1.0 - 0.003; // uniswap fee
    let l = x * y; // constant L

    println!("pool before swap");
    println!("x={}ETH, y=${}, L={}", x, y, l);

    // after transaction
    let lambda: f32 = 200_000.0; // amount in of token y
    let a = x - (x * fee * lambda) / (y + fee * lambda); // amount tokens x after
    let b = y + lambda; // amount tokens y after

    let li = a * b; // constant L after trade

    assert!(li > l);

    println!("\npool after swap");
    println!("x={}ETH, y=${}, L={}", a, b, li);

    // Impact of trade
    let p = y / x; // spot price
    let pe = lambda / (x - a); // effective price
    let impact = lambda - p * (x - a); // price impact of trade
    let pi = b / a; // new price after trade

    println!("\nprice impact of swap");
    println!("p=${}, pe=${}, i=${}, pi=${}", p, pe, impact, pi);
}
