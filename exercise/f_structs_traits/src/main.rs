// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call self method when we
// want to bite something.  Once self trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
trait Bite {
    fn bite(self: &mut Self);
}

// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of self file (you should probably
// use a different field, though).
//
#[derive(Debug)] // include self line right before your struct definition
struct Grapes {
    amount_left: u32,
}

// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of self file.
//
impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

fn main() {
    // Once you finish #1 above, self part should work.
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //
    //bunny_nibbles(&mut carrot);
    //println!("Bunny nibbles for awhile: {:?}", carrot);
}



/* self vs Self
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Associated function (uses Self)
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: String::from(name),
            age,
        }
    }

    // Method (uses self)
    fn greet(&self) {
        println!("Hello, my name is {} and I'm {} years old.", self.name, self.age);
    }

    // Method that takes ownership of self
    fn birthday(self) -> Self {
        Person {
            name: self.name,
            age: self.age + 1,
        }
    }
}

fn main() {
    // Using the associated function (Self)
    let person = Person::new("Alice", 30);
    person.greet(); // Output: Hello, my name is Alice and I'm 30 years old.

    // Using the method (self)
    let older_person = person.birthday();
    older_person.greet(); // Output: Hello, my name is Alice and I'm 31 years old.
}

 */

/* Generics and Traits
fn print-noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

impl Noisy for Dog {
    fn get_noise(&self) -> String {
        String::from("Woof!")
}


/* However, the search results also caution that returning references 
can be dangerous if not done carefully, 
as it's easy to accidentally return a reference to a local variable that 
will be destroyed when the function returns.
 The caller could then end up with an invalid reference. */
impl Noisy for u8 {
    fn get_noise(&self) -> &str { "Beep!" }
}

fn main() {
    let dog = Dog { name: String::from("Rover") };
    print_noise(dog);

    let number = 5_u8;
    print_noise(number);
 */



