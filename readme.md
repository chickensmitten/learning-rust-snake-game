# Learning Snake Game
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
## Return Value with Arrow ->
```
fn add(x: u32, y: u32) -> u32 {
    let sum = x + y;
    sum
}
```
Without `-> u32`, cannot return `sum`