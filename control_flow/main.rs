fn even_or_odd(number: i32) {
    let result: &str = if number % 2 == 0 {"even"} else {"odd"};
    println!("The number is {result}");
}

fn recusion_example(number: i32) {
    if number > 0 {
        println!("recursion countdown is {number}");
        recusion_example(number-1);
    }
    else if number == 0 {
        println!("BLASTOFF!!!!!");
    }
}

fn match_color_to_number(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn if_color_to_number(color: &str) -> i32 {
    if color == "red" {
        1
    }
    else if  color == "green" {
        2
    }
    else if  color == "blue" {
        3
    }
    else {
        0
    }
}

fn factorial(number: i64) -> i64{
    let mut num: i64 = number;
    let mut sum: i64 = 1;
    while num > 0 {
        sum = sum*num;
        num -= 1;
    } 
    return sum;
}

fn recursion_factorial(number: i64) -> i64{
    if number > 0 {
        number * recursion_factorial(number-1)
    } 
    else{
        1
    }
}

fn main() {
    let season: &str = "fall";

    // if statement must be based on a bool
    if season == "summer" {
        println!("schools out");
    }
    else if  season == "winter" {
        println!("go ski");
    }
    else if  season == "fall" {
        println!("looks at trees");
    }
    else if  season == "spring" {
        println!("go to school");
    }
    else {
        println!("you done messed up AAron");
    }

    even_or_odd(99);
    even_or_odd(100);

    // match allows us to consolidate if statements, doesnt have to be a bool
    let evaluation: bool = true;
    match evaluation {
        true => {
            println!("The value is true");
        }
        false => {
            println!("The value is false");
        }
    }
    let value: i32 = match evaluation {
        true => 20,
        false => 40,
    };
    println!("{value}");
    // pattern or arm is one option to compare match against
    // all match cases must return same data type, here println returns an empty tuple
    // to compile match must cover all possible values, underscore serves as a wildcard
    // strings have infinite possibilities and thus require a '_' wildcard at the end

    match season {
        "fall" =>  println!("looks at trees"),
        "winter" => println!("go ski"),
        "spring" => println!("go to school"),
        "summer" => println!("schools out"),
        _ => println!("wtf dude"),
    }
    let number: i32 = 8;
    match number % 2 {
        0 => println!("{number} is even."),
        _ => println!("{number} is odd."),
    }

    let mut seconds: i16 = 10;
    loop {
        if seconds <= 0 {
            println!("BLASTOFF!!!");
            break;
        }
        else if seconds % 2 == 0 {
            println!("even number");
            seconds -= 1;
            continue;
        }
        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }
    // by default without added code 'loop' is infinite, use if statement and 'break' to stop loop
    // 'CONTINUE' moves to next iteration of loop without finishing that iteration

    seconds = 10;
    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} is even");
        }
        println!("{seconds} to blastoff");
        seconds -= 1;
    } 
    println!("BLASTOFF!!!");
    // while loop requires a boolean condition


    recusion_example(6);
    // recursion is when a function call itselfb")
    // base case is the condition to stop recusion

    println!(" factorial of 5 is {}", factorial(5));
    println!(" factorial of 10 is {}", factorial(10));
    println!(" recur factorial of 10 is {}", recursion_factorial(10));
    println!(" recur factorial of 5 is {}", recursion_factorial(5));
    println!(" match color green is {}", match_color_to_number("green"));
    println!(" match color red is {}", match_color_to_number("red"));
    println!(" match color bet is {}", match_color_to_number("bet"));
    println!(" if color red is {}", if_color_to_number("red"));
    println!(" if color blue is {}", if_color_to_number("blue"));
    println!(" if color hi is {}", if_color_to_number("hi"));

}
// control flow refers to how a program will execute, can alter the flow of execution
// if statement must be based on a bool
