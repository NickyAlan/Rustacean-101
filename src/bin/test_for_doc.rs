/// cargo doc --open
/// rustup doc : std lib document


/// receive 2 number -> then sum it
fn add_to_number(a: i32, b: i32) -> i32 {
    a + b
}

/// find Catalan_number : <https://en.wikipedia.org/wiki/Catalan_number>
fn catalan_number(n: i32) -> i32 {
    // Cn = k=2->n[PI(n+k / k)]
    let mut res: f64 = 1.0;
    for k in 2..=n {
        let n = n as f64;
        let k = k as f64;
        res *= (n+k)/k;
    }
    res.round().ceil() as i32
}

fn main() {
    let total = add_to_number(10, 20);
    println!("total: {:?}", total);

    let n = 8;
    let catalan_n = catalan_number(n);
    println!("catalan number of {:?}: {:?}", n, catalan_n);
}