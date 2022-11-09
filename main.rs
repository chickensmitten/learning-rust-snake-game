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