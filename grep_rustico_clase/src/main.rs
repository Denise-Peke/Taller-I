use std::result;

use grep_rustico::regex::Regex;

fn main() {
    
    let regex = Regex::new("ab.*");

    let value = "abacdf";
    println!("Your value is {:?}", value);

    match regex.unwrap().test(value) {
        Ok(Result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err), 
    }

}
