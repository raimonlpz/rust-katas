fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result: {}", result);

    let mut numb = 4;

    while numb != 0 {
        println!("{}!", numb);
        numb -= 1;
    }
    println!("FUERA!");

    let arr = [0, 12, 20, 343, 54, 443];
    let mut index = 0;

    while index < arr.len() {
        println!("{}", arr[index]);
        index += 1;
    }

    // more safety
    for elem in arr.iter() {
        println!("{}", elem);
    }

    // reverse from 1 to 4
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!");
}
