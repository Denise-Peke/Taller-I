use std::env;
use gre_rustico_2::reader::Reader;


fn main(){
    
    let args: Vec<String> = env::args().collect();

    
    if args.len() != 3 {
        std::process::exit(1);
    }

    let reader = Reader::new(args[2].to_string(), args[1].to_string());
    let _ = reader.read_and_search();
}
