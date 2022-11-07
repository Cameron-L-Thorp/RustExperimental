fn main() {
    println!("Hello, world!");

    //Statements
    //It is valid to have a statement without a variable name, but it is inaccessible
    "cyan";
    //using let allows you to instantiate it as a variable
    let favcolor = "cyan";
    println!("{favcolor}");
    //functions or macros that don't return data can also be statements
    print_func();
    5;
    true;

    //Expressions 
    //When a segment is not ended with a semi-colon, it will implicitly return its value
    println!("{}", expression_function());

    const THIS_IS_CONSTANT: u32 = 5;
    println!("Jordan's favorite number is {THIS_IS_CONSTANT}")
}

fn print_func() {
    let statement = "You know what it stands for.";
    println!("{statement}");
}

fn expression_function() -> String {
    let expression = "This is returned as an expression".to_string();
    expression
}