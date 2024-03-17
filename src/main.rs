fn main() {
    // before transaction
    let x = 200; // amount tokens x in pool
    let y = 800_000; // amount tokens y in pool

    let l = x * y; // constant L

    println!("x={}, y={}, L={}", x, y, l);

    // after transaction
    let lambda = 200_000; // amount in of token y
    let b = y + lambda; // amount tokens y after
    let a = l / b; // amount tokens x after

    let li = a * b; // constant L after trade

    assert_eq!(l, li); // check invariant

    println!("x={}, y={}, L={}", a, b, li);

    // Impoact
    let p = y / x; // spot price
    let pe = lambda / (x - a); // effectie price
    let pi = b / a; // price after trade
    println!("p={}, pe={}, pi={}", p, pe, pi);
}
