// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt"); // std::fs::File std::io::Err
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    // arguably cleaner code below
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
    // or even simpler - expect usually prefered to unwrap which causes a panic as well but doesn't have error message
    // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project")
}