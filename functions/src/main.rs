fn main() {
    println!("we are about to get greetings");
    // calling a function
    function_without_arg();

    let name = "safeer";
    // calling a function with an argument;
    greetings(name);

    // as the statements in Rust will not return something, 
    // we can't write something like following as we used to write in js or any other language;
    // let x = y = 7;

    // expressions will return value, so we should not include a semicolon(;) at the end of it
    // if we are adding a semicolon at the end of an expression, it will turn into a statement;
    let x = {
        let y = 7;
        // we will not use a semicolon in the following line as it is an expression
        y + 1
    };
    println!("the value of x is: {x}");

    let seven = get_seven();
    println!("we received following from seven: {seven}");
    let seven_plus_one = plus_one(7);
    println!("we received following from plus_one: {seven_plus_one}");
}

fn function_without_arg() {
    println!("hy from function_without_arg");
}

// function with an argument;
fn greetings(name: &str) {
    println!("hy {name}, have a good day.");
}

// function with return value;
fn get_seven() -> i32 {
    // here we are not even writing the 
    7
}

// returning the value of an expression;
fn plus_one(x: i32) -> i32 {
    // we can't add a semicolon at the end of the following line as we want to return the value of expression;
    x + 1
}