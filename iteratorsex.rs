fn main() {
    let mut rango = 0..20;
    let ramones = vec!["hey", "ho", "lets", "go"];

    loop {
        match rango.next() {
            Some(x) => {
                println!("{}", x);
            }
            None => break,
        }
    }

    for ramon in ramones {
        println!("{}", ramon);
    }

    // consumidores: collect() , find() , fold()

    // Collect
    let uno_cien = (1..42).collect::<Vec<i32>>();
    println!("{:?}", uno_cien);

    // Find
    let findcol = (1..44).find(|n| *n > 42);

    match findcol {
        Some(_) => println!("Tenemos numeros mayores de 42!"),
        None => println!("No tenemos nums mayores..."),
    };
    //println!("{:?}", findcol);

    // Fold
    let suma = (1..5).fold(0, |suma, x| suma + x);
    println!("{}", suma); // 10 -> acumulador

    let iter_test = (0..10).collect::<Vec<_>>();
    for it in iter_test {
        println!("{}", it);
    }

    // adaptadores de iteradores

    // revisar aquest...
    let adapter = (20..31).map(|x| x + 1);
    println!("{:?}", adapter);

    // Take()
    for i in (3..).take(5) {
        println!("{}", i);
    }

    // Filter()
    for i in (1..100).filter(|&x| x % 2 == 0) {
        println!("{}", i);
    }

    println!("EX FINAL:");
    println!(
        "{:?}",
        (1..)
            .filter(|&x| x % 2 == 0)
            .filter(|&x| x % 3 == 0)
            .take(10)
            .collect::<Vec<_>>()
    )
}
