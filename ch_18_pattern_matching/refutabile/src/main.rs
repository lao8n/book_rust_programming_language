// irrefutable = patterns that match any possible value e.g. let x = 5;
// refutabile = patterns can fail to match for some possible values e.g. let Some(x) = a_value
fn main() {
    // let Some(x) = some_option_value; /expects irrefutible pattern

    // if let x = 5 { // expects refutable pattern
    //     println!("{x}");
    // };
}
