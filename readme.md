#Learning Snake Game

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