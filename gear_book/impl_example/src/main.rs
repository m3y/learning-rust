struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }

    fn say_name(&self) {
        println!("I am {}.", self.name);
    }

    fn say_age(&self) {
        println!("I am {} year(s) old.", self.age);
    }
}

fn main() {
    let p = Person::new("Taro", 20);
    p.say_name();
    p.say_age();
}
