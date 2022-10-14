mod parsing;

fn main() {
    let args = parsing::parse_args();
    if let Some(args) = args {
        println!("{:#?}", args); 
    }   
}


