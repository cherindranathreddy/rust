fn main() {
    let guess : u32 = "42 ".trim().parse().expect("Not a number");
    println!("The number is {guess}");

    let (test_1, test_2) = (1_100, 0xfff);
    println!("The Integers are: {test_1} and {test_2}");

    let float_test:f32 = 2.0;
    println!("The flaoting values are: {float_test}");

    let f:bool = false;
    println!("The boolean values are: {f}");


    println!("Scalar types: single values - integer, floating point, char, boolean");
}
