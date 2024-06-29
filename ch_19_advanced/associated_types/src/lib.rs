// associated type = connect a type placeholder with a trait such that the trait method defintions can use the 
// placeholder types in their signatures
// the implementor of the type sill specify the concrete type to be used instead of the placeholder type

pub trait Iterator { // trait
    type Item; // type placedholder & associated type
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {} // concrete implementation
}

// very similar to generics but there you always have to annotate the type in every implementation, with associated types you do not
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }