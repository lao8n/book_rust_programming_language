pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // generic type parameter can only be substituted with one concrete type at a time
    // trait objects allow multiple concrete types to fill in for the trait at runtime
    pub components: Vec<Box<dyn Draw>>, 
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw(); // doesn't check the type of component it just calls the method
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

// homogeneous generic collection
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }