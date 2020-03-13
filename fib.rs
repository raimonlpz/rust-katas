fn fibonacci_gen(ind: usize) -> u64 {
    let mut s = vec![0, 1];

    let mut c: usize = 0;
    while c < ind {
        s.push(s[c] + s[c + 1]);
        c += 1;
    }
    s[ind]
}

fn main() {
    println!("f(?): {}", fibonacci_gen(90));
}
