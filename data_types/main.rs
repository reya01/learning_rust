
fn main() {
    let eight_bit: i8 = -112;
    let thirty_two_bit_int: i32 = -1111232;
    println!("{eight_bit} {thirty_two_bit_int} {eight_bit}");
    
    println!("Dear Emily \n \t");

    let pi: f64 = 3.141592653;
    println!("The value of pi is {pi:.2}");
    // format specifier to specify precision of what is printed

    let miles_away: i32 = 50;
    let miles_away_i8: i8 = miles_away as i8;
    let miles_away: f64 = 100.329;
    // "as" can convert one type to another by casting

    let is_handsome: bool = true; 
    let mils_bool: bool = miles_away >= 22.0;
    let miles_away_i8_bool: bool = miles_away_i8 == 50;
    let combined_bool_and: bool = is_handsome && miles_away_i8_bool; // and logic
    let combined_bool_or: bool = is_handsome || miles_away_i8_bool; // or logic
    println!("miles away >= 22 {mils_bool}, is miles away 50 {miles_away_i8_bool}");
    // rust in boolean, 1 byte

    let char_no_1: char = 'B';
    println!("{char_no_1} char no 1");
    println!("{} char no 1 is uppercase", char_no_1.is_uppercase());
    // char type

    let numbers: [i32; 6] = [ 100, 200, 300, 400, 500, 600];
    let words: [&str; 5] = ["hello", "whats up", "hi", "nope not today", "YUP"];
    println!("numbers are {} and the words are {}", numbers[0], words[1]);
    println!("{:?}", words); 
    // showing debug trait with that '?' to show the array
    // array, size cannot change but the elements can change. Vectors can change in size.
    // array elements must all be the same data type

    dbg!(2 + 2);
    //debug macro, purely for developer convenience , '!' denotes a macro in Rust
    // great for showing results of an expression to debug without doing lots of tedious prints

    let employee: (&str, i32, &str) = ("molly", 32, "marketing");
    let (name, age, department) = employee;
    println!("employee info {}, name variable is {}", employee.0, name);
    // tuple type, like array contains multiple elements but in a tuple we can have different types of elements

    let month_days = 1..31; 
    let letters = 'b'..'f'; 
    let colors = ["Red", "Green", "Yellow"];
    let month_num_2: std::ops::Range<i32> = -2..3;
    // Range: Rust will infer the type automatically. If you want to use Range<i32>, you must import it with use std::ops::Range;, but for most cases, type inference is preferred.
    // Range is up to but not include the last one listed

    println!("month days range is {month_days:?}");
    for value_int in month_days {
        println!("{value_int}");
    }
    for value_char in letters {
        println!("{value_char}");
    }
    for color in colors {
        println!("{color:?}"); // using debug to print array values
    }
    for value_1 in month_num_2 {
        println!("{value_1}");
    }
    // range type is a sequence or interval of consecutive values
    // up to but NOT including last value listed, above would be a range with 1 to 30
    // print array values using debug trait with '?' because ranges don't have display implemented


    // generic is a type argument(input to something), e.g. provide say i32 as an argument to something
    // think of generic as a placeholder for an expected type that will come with a concrete value in the future


    // coding challenge
    let int_var_1: i32 = 1_337;
    let int_var_2: i16 = int_var_1 as i16;

    let flt_var_1: f64 = 111.1119;
    println!("{flt_var_1:.3}");

    let with_milk: bool = true;
    let with_sugar: bool = false;
    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    let is_acceptable_coffee: bool = with_milk || with_sugar;
    println!("{is_acceptable_coffee}");

    let new_array_1: [i8; 4] = [1, 2, 3, 11];
    println!("{new_array_1:?}");

    let new_tuple_1: (i32, f64, bool) = (-1_337, -1_337_.001, true);
    println!("{new_tuple_1:?}");

}

// statically typed means compiler needs to know every variable type at compile time
// scalar type holds a single value e.g. int, float, boolean, char
// signed int = i32, etc
// unsigned int = u32, etc
// 8 bits is 1 byte, i32 is 32 bits (4 bytes), f64 is 64 bits (8 bytes)
// value.method() to call a method, value.abs() for absolute value, string.trim() to remove whitespace
// value.pow(2) to raise to exponent of 2
