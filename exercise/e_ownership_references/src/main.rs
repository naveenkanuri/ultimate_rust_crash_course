// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

use std::fmt::Debug;

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //
    fn inspect(str: &String) {
        match str.ends_with("s") {
            true => println!("Plural"),
            false => println!("Singular")
        }
    }
    inspect(&arg);

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    fn change(str: &mut String) {
        if !str.ends_with("s") {
            str.push_str("s");
        }
    }
    change(&mut arg);
    println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    //
    fn eat(str: String) -> bool {
        str.starts_with("b") && str.contains("a")
    }
    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //
    fn bedazzle(str: &mut String) {
        *str = String::from("sparkly");
    }
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);

    let mut vec = vec![1, 2, 5, 6 , 9];
    vec.iter()
        .map(|x| x*2)
        .filter(|x| *x < 5)
        .fold(0, |acc, x| acc + x)
        ;

    for val in vec {
        println!("val = {}", val);
    }
}
