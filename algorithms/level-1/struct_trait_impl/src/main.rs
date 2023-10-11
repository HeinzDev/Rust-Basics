struct Person {
    name: String,
    age: i32,
}

trait Voice {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}

impl Voice for Person {
    fn speak(&self) {
        println!("Hello, I am speaking.");
    }
    fn can_speak(&self) -> bool {
        self.age > 1
    }
}

fn main() {
    let person = Person {
        name: String::from("Enzo"),
        age: 20,
    };

    if person.can_speak() {
        println!("Hello, my name is {} and I am {} years old.", person.name, person.age);
        person.speak();
    }
}
