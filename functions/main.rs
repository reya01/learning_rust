fn main() {
    // functions can capture a reusable set of instructions
    open_store("Kirkland");
    open_store("Bellevue");
    bake_pizza(11, "cheese");
    bake_pizza(2, "pepperoni");
    close_store("Kirkland");
    close_store("Bellevue");
    println!(" Squaring 11 = {}", square(11));
    println!(" Squaring 12 = {}", square(12));

    let multiplier: i32 = 3;
    {
        let value: i32 = 5 + 4;
        println!(" [Add 5 + 4] times 3 is {}", value * multiplier);
    }
    // an additional nested scope within the main function

    apply_to_jobs(113, "Finance");
    println!("{}", is_even(44));
    println!("{}", is_even(33));
    println!("{:?}", alphabets("aardvark")); //-> (true, false)
    println!("{:?}", alphabets("zoology"));  //-> (false, true)
    println!("{:?}", alphabets("zebra"));    //-> (true, true)

}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {} {} jobs.", number, title);
}

fn is_even(number: i32) -> bool {
    return number % 2 == 0;
    // number % == 2    would be an implicit return by omitting 'return' and the ';'
}

fn alphabets(text: &str) -> (bool, bool) {
    return (text.contains('a'), text.contains('z'));
}

fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {}", neighborhood);
}

fn close_store(neighborhood: &str) {
    println!("Closing my pizza store in {}", neighborhood);
}

fn bake_pizza(number: i32, topping: &str) {
    println!(" Baking {} {} pizzas.", number, topping)
}
// parameter is the name for an expected input into a function
// return value is an output of a function

fn square(number: i32) -> i32 {
    number * number
}
// rust has implicit return and will automatically return the last line of a function if you remove the ';'

// a unit is an empty tuple, a tuple without values. Is the default return type if none is specified.
