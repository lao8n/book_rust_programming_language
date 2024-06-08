enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// note matches have to be exhaustive
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }        
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        // can also have other or _ to catch all other cases
    }
}

fn main(){
    value_in_cents(Coin::Penny);
}