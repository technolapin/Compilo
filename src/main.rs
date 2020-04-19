use std::env::args;
use std::io::{BufReader, Read};
use std::fs::File;
use lalrpop_util::ParseError;
#[derive(Debug)]
struct Error(String);

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error(format!("{}", error))
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error(error)
    }
}
use std::fmt::Debug;
impl<A: Debug, B: Debug, C: Debug> From<ParseError<A, B, C>> for Error    
{
    fn from(error: ParseError<A, B, C>) -> Self {
        Error(format!("{:?}", error))
    }
}


use compilo::parser;




fn main() -> Result<(), Error>
{
    let filename = match args().nth(1)
    {
	None => return Err(Error(String::from("please specify a file name"))),
	Some(name) => name
    };
    
    let file = File::open(filename)?;

    let mut reader = BufReader::new(file);

    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let text = content.as_str();


    let parsed = parser::SeqParser::new().parse(text)?;

    println!("===========================================");
    println!("PRETTY PRINT:");
    println!("- - - - - - - - - - - - - - - - - - - - - -");
    println!("{}", parsed);
    println!("- - - - - - - - - - - - - - - - - - - - - -");
    parsed.run()?;

    Ok(())
    
}
