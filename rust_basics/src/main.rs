fn greeting(){
    println!("Welcome to my program!");
}

fn add_two(number: i32){
    let result = number + 2;
    println!("Number value is: {}", result);
}

fn main() {
    println!("Hello, world!");
    greeting();

    println!("Conditionals: ");
    let num = 12;
    if num % 2==0 {
        println!("Even number");
    } else {
        println!("Odd number");
    }

    println!("Loops");
    for x in 1..11{
        if x==5 {
            continue;
        }
        println!("X is {}", x);
    }

    println!("Function: ");
    add_two(num);
}
