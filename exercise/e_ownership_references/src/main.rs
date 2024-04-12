// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut)]

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
    inspect(&arg);

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    change(&mut arg);
    println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    //
    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "add" that takes *references* to two integer arguments,
    // dereferences them and adds them together, and returns the result.
    //
    // println!("1 + 2 = {}, even via references", add(&1, &2));
}

fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}

fn change(s: &mut String) {
    if !(*s).ends_with("s") {
        (*s).push_str("s");
    }
}

fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}


/*
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // This is an associated function
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }

    // This is a method that takes a self parameter
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // Calling the associated function
    let rect = Rectangle::new(10, 5);
    println!("Area: {}", rect.area());
}


///////////////

struct Celsius(f64);

impl Celsius {
    // This is an associated function that converts Celsius to Fahrenheit
    fn to_fahrenheit(celsius: f64) -> f64 {
        (celsius * 9.0 / 5.0) + 32.0
    }
}

fn main() {
    let celsius = Celsius(20.0);
    let fahrenheit = Celsius::to_fahrenheit(20.0);
    println!("20°C is equal to {:.2}°F", fahrenheit); // Output: 20°C is equal to 68.00°F
}

///////////////
use rand::Rng;

struct Dice {
    sides: u32,
}

impl Dice {
    // This is an associated function that rolls the dice and returns a random value
    fn roll(self: &Dice) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.sides)
    }
}

fn main() {
    let six_sided_dice = Dice { sides: 6 };
    let roll_result = six_sided_dice.roll();
    println!("You rolled a {}", roll_result);
}

///////////////
use rand::Rng;

struct Dice {
    sides: u32,
}

impl Dice {
    // This is an associated function that rolls the dice and returns a random value
    fn roll(self: &Dice) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.sides)
    }
}

fn main() {
    let six_sided_dice = Dice { sides: 6 };
    let roll_result = six_sided_dice.roll();
    println!("You rolled a {}", roll_result);
}

 */
