extern crate compilo;

use compilo::ast::*;

use compilo::parser;

fn foo(s: &str)
{
    let parsed = parser::ExprParser::new().parse(s);
    println!("{} {:?}", s, parsed);
}

fn main()
{
    foo("--1");
    foo("++1");
    foo("1--");
    foo("1++");
    foo("--1");
    foo("++1");
    foo("-1");
    foo("+1");
    foo("!1");
    foo("1+1");
    foo("1-1");
    foo("1*1");
    foo("1/1");
    foo("1%1");
    foo("1<1");
    foo("1>1");
    foo("1<=1");
    foo("1>=1");
    foo("1==1");
    foo("1!=1");
    foo("1&1");
    foo("1^1");
    foo("1|1");
    foo("1&&1");
    foo("1||1");

    foo("if 1 {1} else {1}");


    if true
    {
	println!("RANDOM_TEST");
	let rand_expr = *Expression::random(3);
	println!("{}\n", rand_expr);
	let pretty_printed = format!("{}", rand_expr);
	println!("{}\n", pretty_printed);
	let parsed = *parser::ExprParser::new().parse(pretty_printed.as_str()).unwrap();
	println!("{}\n", parsed);
	println!("MATCH = {}", rand_expr == parsed)
    }
}
