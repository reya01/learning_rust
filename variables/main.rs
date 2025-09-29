type Meters = i32;
//creating a type that can be used instead of i32 throughout for readability and clarity 

fn main() {
    let my_age: i32 = 50;
    let oranges: i32 = 14 + 15;
    let concats: i32 = my_age - oranges;
    let mile_length: Meters = 1600; 
    
    println!("my age is {0}, and I have {1} oranges. Concats means {2} {1} {0}.", my_age, oranges, concats);
    
    // all variables in rust are immutable by default and require tagging as mut to be mutable
    // an underscore before a variable name will tell rust to expect that variable not to be used

    println!("A one mile race in meters is {0}", mile_length);

    //a compiler directive to allow a certain behavior such as not using a variable, put at top with #! to affect whole file
    #[allow(unused_variables)]
    let var_not_used: i32 = 1192;

}

//coding challenge
/*
Declare a `season` variable set to a string with
your favorite season. Provide an explicit type annotation.
The type of a string is a `&str`. We'll discuss what
the & symbol means later in the course.

Declare a `points_scored` variable set to 28.
Provide an explicit type annotation. The type of
an integer is `i32`.

It's time to update the team's score. Declare the
`points_scored` variable to be mutable. Set its
new value to 35.

Declare a `TOUCHDOWN_POINTS` constant at the file
level set to the value 6.

Declare a `event_time` variable set to a string of
"06:00".

Use variable shadowing to redeclare `event_time` set
to a integer of 6.

Use interpolation to print out all of the
declared variables and constants in a println! call.
Practice with direct interpolation, sequential
arguments, and numeric arguments.

Declare a `favorite_beverage` variable set to a string
of your favorite drink. Use an underscore to silence
the compiler warning about the variable being unused.

Remove the underscore. Provide a compiler directive
to silence the compiler warning about the variable
being unused.
*/
