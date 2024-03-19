pub fn run() {
    // before transaction
    let a = 200; // amount tokens A in pool
    let b = 800_000; // amount tokens B in pool
    let k = a * b; // constant k

    println!("Pool before");
    println!("A={}ETH, B=${}, k={}", a, b, k);

    // after transaction
    let lambda = 200_000; // amount in of token y
    let y = b + lambda; // amount tokens b after
    let x = k / y; // amount tokens a after

    let ki = x * y; // constant k after trade

    assert_eq!(k, ki); // check invariant

    println!("Pool after");
    println!("A={}ETH, B=${}, k={}", x, y, ki);

    // Impact of trade
    let p = b / a; // spot price
    let pe = lambda / (a - x); // effective price
    let impact = lambda - p * (a - x); // price impact of trade
    let pi = y / x; // price after trade
    println!("p=${}, pe=${}, i=${}, pi=${}", p, pe, impact, pi);
}
