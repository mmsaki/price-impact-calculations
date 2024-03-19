pub fn run() {
    // before transaction
    let a: f32 = 200.0; // amount tokens A in pool
    let b: f32 = 800_000.0; // amount tokens B in pool
    let fee = 1.0 - 0.003; // uniswap fee
    let k = a * b; // constant k

    println!("Pool before swap");
    println!("A={}ETH, B=${}, k={}", a, b, k);

    // after transaction
    let lambda: f32 = 200_000.0; // amount in of token y
    let amount_in = (a * fee * lambda) / (b + fee * lambda);
    let x = a - amount_in; // amount tokens A after
    let y = b + lambda; // amount tokens B after

    let ki = x * y; // constant k after trade

    assert!(ki > k);

    println!("Pool after swap");
    println!("A={}ETH, B=${}, k={}", x, y, ki);

    // Impact of trade
    let p = b / a; // spot price
    let pe = lambda / (a - x); // effective price
    let impact = pe * (a - x) - p * (a - x); // price impact of trade
    let pi = y / x; // new price after trade

    println!("Price impact of swap");
    println!("p=${}, pe=${}, i=${}, pi=${}", p, pe, impact, pi);
}
