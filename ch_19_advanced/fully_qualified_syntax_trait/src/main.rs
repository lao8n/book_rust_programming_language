trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name()); // calls original implementation
    // println!("A baby dog is called a {}", Animal::baby_name()); // doesn't compile could be other implementations
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}