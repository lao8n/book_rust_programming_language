// declarative macros or macros by example, similar to a rust match expression
#[macro_export]
macro_rules! vec { // example let v: Vec<u32> = vec![1, 2, 3];
    // $ declares a macro variable, then pattern match on optional , and *
    ( $( $x:expr ),* ) => { // macros don't need to know the number of number of parameters unlike functions
        {
            let mut temp_vec = Vec::new();
            $( // code for each part fthat match $()*
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}