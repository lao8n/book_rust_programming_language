fn main() {
    let config_max = Some(3u8);
    // rather than 
    match config_max {
        Some(max) => {
            println!("The maximum is configured to be {}", max);
        },
        _ => (), // annoying boiler plate
    }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
        // lose exhaustive checking
    }
}

