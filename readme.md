# Learning Rust
## General Thoughts
The language is strict on the following:
- ordering: creating and consuming variables should be in a certain order, else there will be errors
- types: declaration of types are needed for memory management

## Basic Setup
- Use brew to install. Instructions here [https://stackoverflow.com/questions/67656028/rustup-gives-command-not-found-error-with-zsh-even-after-installing-with-brew](https://stackoverflow.com/questions/67656028/rustup-gives-command-not-found-error-with-zsh-even-after-installing-with-brew)
- Execute code below to check if everything working `rustc —version`
- Install version management `rustup default nightly`
- use below to see install toolchains `rustup show`
- Create rust by creating a folder. then create a `.rs` file with the code below
```
fn main() {
  let message = "Hellow World";
  println!("Some text: {}", message); // messages transported into curly brackets
}
```
- then in project folder run code below to compile `rustc main.rs`
- in the same directory run to run the file. `./main`
- mut variable `let mut variable = "hellow world";` allows you to create a variable that is mutable

## Run Cargo
- `cargo init` to create a binary application package in the main directory
- add the following code
- `cargo run` to compile the application
- rust normally uses 4 spaces

## Rust Analyzer
- use rust analyzer in VS code to better read Rust code.
- there is a difference between `str` and `String` in Rust code.

## Primatives
### String
- `str` is ready only
- `String::from(<text>)` can read and write
```
fn main() {
  let message = "Hello World";
  let message_2 = print_welcome(message);
  println!("{}", message_2);
}

fn print_welcome(text: &str) -> &str {
  println!("{}", text);
  let new_message = "Hi there";
  new_message
}
```
- slicing string
```

fn main() {
    let mut message = String::from("Hello");
    let slice = &message[2..4]; // 2 -> 3

    // message.clear();

    println!("{}", slice);
}

fn move_me(val: String) {}
```
- cloning strings
```
fn main() {
    let mut message = String::from("Hello");
    let message_3 = message.clone();

    message.clear();

    println!("{}", message);
    println!("{}", message_3);

}
```

### Boolean and Numbers
```
fn main() {
  let is_it_fun = false;
  let num = 10; // i32 is signed integer of 32 bits. it can hold positive and negative values

  // u8 -> unsigned integer of 8 bits
  // 2^8 - 1 = 255
  let small_num: u8 = 255;

  // 2^7 -> 2^7 -1
  // -128 -> 127
  let small_num_2: i8 = 127;

  // operating sytem related type
  let sys_num: isize = -10;
  let sys_num_2: usize = 10;

}

```
- different number types
```
fn main() {
    let custom_num = 98_000;
    let hex_num = 0xfa;
    let bin_num = 0b0010_1011;
    let byte_num = b'A';

    println!("{}", custom_num); // 98
    println!("{}", hex_num); // 250
    println!("{}", bin_num); // 43
    println!("{}", byte_num); // 65

    let float_num: f32 = 3.14;
    let float_num_2: f64 = 3.2334327489;

    let tup: (i32, &str, u8) = (20, "Hello", 1);

    println!("{}", tup.1);

    let (a, b, c) = tup;
    println!("{}", a);

    let x = [1, 5, 6, 7];

    println!("{}", x[2]);

    let y = [2; 6]; // [2, 2, 2, 2, 2, 2]
    println!("{}", y[5]);
}
```
## More Explanations
### Return Value with Arrow ->
Without `-> u32`, cannot return `sum`
```
fn add(x: u32, y: u32) -> u32 {
    let sum = x + y;
    sum
}
```

### References
`&` sign is used to create references
```
fn main() {
     let message = String::from("Hello");
     let message_2: &String = &message; // type is &String and not str
     // message_2 is not owner of data
     // message_2 is "borrowing" a reference to message

     println!("{}", message);
     println!("{}", message_2);
 }
 // message and message_2 going out of the scope
 // message_2 is not dropped because it doesn't have ownership of what it refers to
 // message is dropped
```

### Mutable references
```
fn main() {
     let mut message = String::from("Hello");
     let message_2: &mut String = &mut message;
     // message_2 is not owner of data
     // message_2 is "borrowing" a reference to message

     message_2.push_str(" World");

     println!("{}", message_2);
     println!("{}", message);
 }
```

### Dereference
using asteriks `*` to dereference. there more references with ampersand, the more `*` is required.

```
fn main() {
     let a = 10;
     let b = &a;
     let c = &b;

     println!("{}", a == **c);
 }
```

### Box, Structs, Constructors
- creating a box `Box::new(<value>)`
- creating structs and methods. use `impl` as an associated function for the structs
```
struct Person {
    name: String, // fields
    last_name: String,
    age: u32,
}

impl Person {
    // associated function
    fn some_function() {
        println!("some_function");
    }

    // method
    // first parameter is always self, which represents the instance of the struct the
    // method is being called on
    // Within an impl block, the type Self is an alias for the current type
    fn display_age(&self) {
       println!("Current Age: {}", self.age);
    }
}

fn main() {
    Person::some_function();

    let person =  Person {
       name: "Filip".to_string(),
       last_name: "Jerga".to_string(),
       age: 30,
    };

    let person_2 =  Person {
        name: "John".to_string(),
        last_name: "Snow".to_string(),
        age: 35,
     };

    person.display_age();
    person_2.display_age();

    println!("{} {} {}", person.name, person.last_name, person.age);
}
```
- constructors
```
struct Person {
    name: String, // fields
    last_name: String,
    age: u32,
}

impl Person {
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0
        }
    }

    fn from(name: String, last_name: String, age: u32) -> Person {
        Person {
            name,
            last_name,
            age
        }
    }

    fn change_age(&mut self, new_age: u32) {
       self.age = new_age;
    }
}

fn main() {
    let mut person = Person::new();
    let person_2 = Person::from(
        String::from("John"),
        String::from("Snow"),
        35
    );

    person.change_age(50);

    println!("{} {} {}", person.name, person.last_name, person.age);
    println!("{} {} {}", person_2.name, person_2.last_name, person_2.age);
}
```

### ENUM
- values and match. Match is used for pattern matching.
```
#[derive(Debug)]
enum PersonId {
  Passport(u32),
  IndentityCard(u32, u32, u32),
}

struct Person {
    name: String, // fields
    last_name: String,
    age: u32,
    id: PersonId,
}

struct Animal(String);

impl Person {
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
            id: PersonId::IndentityCard(540, 320, 100)
        }
    }

    fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
        Person {
            name,
            last_name,
            age,
            id
        }
    }

    fn change_age(&mut self, new_age: u32) {
       self.age = new_age;
    }

    fn display_info(&self) {
        println!("{} {} {} {:?}", self.name, self.last_name, self.age, self.id)
    }
}

fn main() {
    let mut person = Person::new();
    let person_2 = Person::from(
        String::from("John"),
        String::from("Snow"),
        35,
        PersonId::Passport(123172371)
    );

    person.change_age(38);
    person.display_info();

    check_person_id(person.id);
    check_person_id(person_2.id);
}

fn check_person_id(id: PersonId) {

    if let PersonId::Passport(num) = id {
        println!("It matching Passport {}", num);
    } else {
        println!("It doesn't match!");
    }

    let result = match id {
        PersonId::IndentityCard(x, y, z) => {
            y
        },
        PersonId::Passport(val) => {
            val
        }
    };

    let animal = Animal(String::from("dog"));
    let Animal(animal_type) = animal;

    println!("{}", animal_type);
    println!("Result: {}", result);
}
```

### Trait
- creating trait and trait narrowing
```
trait Log {
    fn display_info(&self);
    fn alert_something(&self) {
        println!("Default implementation!!!!!!!")
    }
}

#[derive(Debug)]
enum PersonId {
  Passport(u32),
  IndentityCard(u32, u32, u32),
}

struct Person {
    name: String, // fields
    last_name: String,
    age: u32,
    id: PersonId,
}

struct Animal(String);

impl Log for Animal {
    fn display_info(&self) {
        println!("{}", self.0)
    }

    fn alert_something(&self) {
        println!("ANIMAL implementation!!!!!!!")
    }
}

impl Log for Person {
    fn display_info(&self) {
        println!("{} {} {} {:?}", self.name, self.last_name, self.age, self.id)
    }
}

impl Person {
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
            id: PersonId::IndentityCard(540, 320, 100)
        }
    }

    fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
        Person {
            name,
            last_name,
            age,
            id
        }
    }

    fn change_age(&mut self, new_age: u32) {
       self.age = new_age;
    }
}

fn main() {
    let mut person = Person::new();
    let animal = Animal(String::from("dog"));

    person.change_age(38);

    log_info(person);
    log_info_2(&animal);
}

// impl makes the compiler determine type at the compile time
// it will create multiple versions of the function, depending on
// how many types Log trait implements (Person, Animal)
fn log_info(val: impl Log) {
    val.alert_something();
}


// dyn is short for dynamic, and says that function should perform dynamic dispatch
// decission of exactly which function to call at the runtime
fn log_info_2(val: &dyn Log) {
    val.alert_something();
}
```
## Modularity
### pub
- use `pb struct Person { ... }` to allow calling `use <project name>::Person;`
