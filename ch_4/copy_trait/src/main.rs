fn main() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); 
    // integers are other simple types implement the Copy trait and are stack only therefore they do not move
    // as clone and move are the same - they are both stack only
    // cannot annotate a type with copy - if we want special drop then need to used derivable traits
}
