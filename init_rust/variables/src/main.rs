fn main() {
    let x = 20;
    let y = 30;
    let sum = x + y;
    println!("The sum of {} and {} is {}", x, y, sum);

    // Calling the function to demonstrate variable types
   // var_types();
    // Calling the function to demonstrate if statements
   //if_statements();

    // Calling the function to demonstrate loops
    //loops();
    // Calling the function to demonstrate while loop
   // while_loop();
// Calling the function to demonstrate match statements
     matching();
    
    // // Calling the function to demonstrate functions
    // functions();
    
    // // Calling the function to demonstrate structs
    // structs();
    
    // // Calling the function to demonstrate enums
    // enums();
    
    // // Calling the function to demonstrate error handling
    // error_handling();
    
    // // Calling the function to demonstrate generics
    // generics();
    
    // // Calling the function to demonstrate traits
    // traits();
    
    // // Calling the function to demonstrate lifetimes
    // lifetimes();

}

fn var_types(){

    let unsigned_integer: u32 = 10; // Unsigned 32-bit integer
    let signed_integer: i32 = -10; // Signed 32-bit integer
    let floating_point: f64 = 3.14; // 64-bit floating point number
    let character: char = 'A'; // Single Unicode character
    let boolean: bool = true; // Boolean value (true or false)
    let string: &str = "Hello, Rust!"; // String slice
    let tuple: (i32, f64, char) = (42, 3.14, 'R'); // Tuple containing different types
    let array: [i32; 3] = [1, 2, 3]; // Fixed-size array of integers
    let vector: Vec<i32> = vec![1, 2, 3]; // Dynamic array (vector) of integers
    println!("Unsigned Integer: {}", unsigned_integer);
    println!("Signed Integer: {}", signed_integer);
    println!("Floating Point: {}", floating_point);
    println!("Character: {}", character);
    println!("Boolean: {}", boolean);
    println!("String: {}", string);
    println!("Tuple: {:?}", tuple);
    println!("Array: {:?}", array);
    println!("Vector: {:?}", vector);
    // Demonstrating variable shadowing
    let x = 5; // Immutable variable
    let x = x + 1; // Shadowing the previous value of x
    println!("Shadowed x: {}", x); // Prints 6
    // Demonstrating mutability
    let mut y = 10; // Mutable variable
    y += 5; // Modifying the value of y 
    println!("Mutable y: {}", y); // Prints 15
    // Demonstrating constant
    const PI: f64 = 3.14159; // Constant value
    println!("Constant PI: {}", PI); // Prints 3.14159
    // Demonstrating type inference
    let inferred_integer = 42; // Type inferred as i32
    let inferred_float = 3.14; // Type inferred as f64
    println!("Inferred Integer: {}", inferred_integer);
    println!("Inferred Float: {}", inferred_float);
    // Demonstrating type casting
    let int_value: i32 = 10;
    let float_value: f64 = int_value as f64; // Casting i32 to f64
    println!("Casted Float Value: {}", float_value); // Prints 10.0
    // Demonstrating variable scope
    {
        let scoped_variable = "I am scoped!";
        println!("Scoped Variable: {}", scoped_variable); // Prints "I am scoped!"
    }
    // Uncommenting the next line would cause a compile-time error because scoped_variable is out of scope
    // println!("Scoped Variable: {}", scoped_variable); // This line would cause an error
    // Demonstrating lifetime of variables
    let lifetime_variable = String::from("I have a lifetime!");
    println!("Lifetime Variable: {}", lifetime_variable); // Prints "I have a lifetime!"
    // Demonstrating ownership and borrowing
    let owner_variable = String::from("I own this string!");
    let borrowed_variable = &owner_variable; // Borrowing the variable
    println!("Owner Variable: {}", owner_variable); // Prints "I own this string!"
    println!("Borrowed Variable: {}", borrowed_variable); // Prints "I own this string!"
}

fn if_statements() {
    let number = 10;
    if number < 5 {
        println!("Number is less than 5");
    } else if number < 10 {
        println!("Number is less than 10 but greater than or equal to 5");
    } else {
        println!("Number is greater than or equal to 10");
    }
}

fn loops(){
    for i in 0..5 {
        println!("Loop iteration: {}", i);
    }
}
fn while_loop() {
    let mut count = 0;
    while count < 5 {
        println!("While loop iteration: {}", count);
        count += 1;

        if count == 3{
            println!("continue statement at count = {}", count);
        }
    }
}

fn matching() {
    let number = 5;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
}