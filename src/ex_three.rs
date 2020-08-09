// generate the nth Fibonacci number

pub fn calc_fib_num(n: i32) -> i32 {
    let mut fib_l: Vec<i32> = vec![1, 1];
    for x in 1..(n - 2) as usize {
        fib_l.push(fib_l[(x - 1)] + fib_l[x]);
    }
    fib_l[(n - 2) as usize]
    //println!("{:?}", fib_list);
}
