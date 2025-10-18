fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}");
}

fn bake_cake() -> String {
    let cake = String::from("Chocolate mousse");
    return cake;
}

fn add_flour (mut meal: String) -> String {
    meal.push_str(" Add 2 cups flour.");
    println!("{meal}");
    return meal;
}

fn eat_meal (mut meal: String) -> String {
    meal.clear();
    return meal;
}

fn main() {
    let age: i32 = 33;
    // 33 pushed onto stack and owner is the variable 'age'
    
    let time: i32 = 2025;
    let year: i32 = time;
    let age_reference: &i32 = &age;
    // using copy trait rust creates a duplicate independent copy
    // so there are two 2025 values on the stack owned by different variables
    // & in front is creating a reference

    let food: &str = "pasta"; //known at compile time rust string type, not dynamic, hardcoded into binary executable
    let text: String = String::new(); //an empty mutable dynamic string dynamic that is stored on the heap
    let candy: String = String::from("KitKat bar");
    let mut name: String = String::from("Boris"); //saved on the heap, but also an entry on the stack to reference its spot on the heap as well as it length and size
    name.push_str("Pask"); //add to the end of a String(not &str or String slice), aka concat
    println!("{name}");
    
    let person: String = String::from("Boris");
    let genius: String = person.clone(); //String does not implement copy so it will be a movement of ownership, 2 references on stack to same heap data
    //heap allocated string does not implement the copy trait, no copy is made, ownership moves
    println!("{person}");
    println!("{genius}");

    println!("{}", *age_reference);
    // * is the derefence operator - access the data stored at an address, must supply an address not the value itself

    let ice_cream: &str = "cookies and cream";
    println!("{ice_cream}");

    // stack items like ints, bools then rust creates a copy
    // heap items like Strings do not implement copy trait and instead use a reference
    fn print_my_value(value: i32) {
        println!("Your value is {}.", value);
    }
    print_my_value(time);
    print_my_value(year);
    print_my_value(*age_reference);// * follows a reference to the value at the memory address it holds

    let burger = String::from("Burger");
    add_fries(burger);

    let cake: String = bake_cake();
    println!("I now have a {cake} cake.");

    let mut meal = String::new();
    meal = add_flour(meal);

    let is_concert: bool = true;
    let is_event = is_concert;
    println!("{is_concert}");
    println!("{is_event}"); //boolean implements copy trait and full copy is made

    let sushi: &str = "Salmon";
    let dinner = sushi;
    println!("{sushi}");
    println!("{dinner}");  //string literal implements copy trait and full copy is made

    let sushi = String::from("Salmon");
    let dinner = sushi;
    // println!("{sushi}"); ownership moved for heap string so cannot print, heap string doesn't have copy trait
    println!("{dinner}");
    println!("string is now {}", eat_meal(dinner));

}
    // ownership is rust's solution to garbage collection/memory management
    // owner is who/what is responsible for cleaning up a piece of data when not in use
    /*every value in rust has one owner
    the owner can change but there can only be one
    owner is usually a name, variable can be an owner, parameter can be an owner
     
     Stack and Heap are 2 different parts of the computer's memory
     
     stack is faster but only supports data of fixed size, LIFO, push onto stack, pop off of stack
     heap is slower but can support changing data size

     */

    // stack - all stack data must have fixed consistent size known at compile time 
    // such as int, bool, char, array
    
    //heap is a large area of storage space that can give dynamic size such as user input 
    
    // reference is an address the memory allocator returns, and the reference is stored on the stack
    // pushing to stack or reading from memory is faster than allocating on the heap
    
    // owner knows to clean up data when variable goes out of scope, e.g. block of code ends,
    // owner then either pops from stack or deallocates from heap

    // move is a transfer of ownership
    // drop function removes from heap when it goes our of scope
    
    // rust wants to be fast and minimal memory, so you must force it to make a copy
    // clone is a method that makes a copy

