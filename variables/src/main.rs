fn main() {
    let name = "safeer";
    // making the variable as mutable instead the default behaviour of immutability;
    let mut fullname = "safeer ep";
    // shadowing for the variable name;
    let name = "safeer ep";
    // changing the value as the variable is declared as mutable;
    fullname = "safeer mon ep";
    let age = 23;
    println!("{} is {} years old and their fullname is {}", name, age, fullname);
    println!("{}'s fullname contains {} letters", name, fullname.len());

    // mutation doesn't allow to mutate the type, that means if we have initially assigned a string to a variable which can be mutated, and if we are trying to assign a number in later stage to that same variable (without shadowing) then we will get an error;
}
