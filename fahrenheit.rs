fn main() {
    println!("{}", fahrenheit_to_celcius(54.0));
    println!("{}", celsius_to_fahrenheit(32.0));
}

fn fahrenheit_to_celcius(degs: f32) -> f32 {
    (degs - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(degs: f32) -> f32 {
    (degs * 9.0 / 5.0) + 32.0
}
