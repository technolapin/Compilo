extern crate compilo;
use compilo::clean;

fn main()
{
    let text = "/*comented*/ uncommented /* very commented /*more commented*/ */ still noncommented";
    let cleaned = clean::CleanCodeParser::new().parse(text);
    println!("{:?}", cleaned);
}

