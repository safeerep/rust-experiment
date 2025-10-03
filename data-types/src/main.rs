fn main() {
    // type: bool (boolean)
    let t = true;
    // with explicit type annotation
    let f: bool = false;

    // type: i32 (default integer type in Rust for literals like 5, 10)
    let sum = 5 + 10;

    // type: f64 (default floating-point type in Rust for literals like 95.5, 4.3)
    let difference = 95.5 - 4.3;

    // type: i32 (integer multiplication defaults to i32)
    let product = 4 * 30;

    // type: f64 (floating-point division, since 56.7 and 32.2 are f64)
    let quotient = 77.7 / 32.7;

    // type: i32 (integer division, result is truncated toward zero)
    let truncated = -5 / 3;

    // type: i32 (remainder operator)
    let remainder = 43 % 5;

    let character_a = 'a';
    // type - char
    let character_b: char = 'b';

    println!("t = {t}, f = {f}, sum = {sum}, difference = {difference}, product = {product}, quotient = {quotient}, truncated = {truncated}, remainder = {remainder}, character_a = {character_a}, character_b = {character_b}");

    // type: tuple (&str, f64, i32)
    let tup = ("safeer", 23.25, 7);
    // destructuring tuple
    let (name, age, number) = tup;
    println!("a person named {name} with the age of {age} prefer the number {number}");
    let the_name = tup.0;
    println!("the name is equal to {the_name}");

    // type: array - array can contain the fixed number of elements only;
    let months = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];
    // explicitly mentioning the type of elements in an array and the size as well;
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_2: [&str; 5] = ["one", "two", "three", "four", "five"];
    let seven_three_times = [7; 3];
    let first_element = seven_three_times[0];
    println!("first element is {first_element}");
    

}
