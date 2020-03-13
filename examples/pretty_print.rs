extern crate compilo;

use compilo::ast::*;

use compilo::parser;

fn foo(s: &str)
{
    let parsed = parser::ExprParser::new().parse(s);
    println!("{} {:?}", s, parsed);
}

fn oof(s: &str)
{
    let parsed = parser::StatParser::new().parse(s);
    println!("{} {:?}", s, parsed);
}

fn main()
{
    oof("--1");
    oof("++1");
    oof("1--");
    oof("1++");
    oof("--1");
    oof("++1");
    oof("-1");
    oof("+1");
    oof("!1");
    oof("1+1");
    oof("1-1");
    oof("1*1");
    oof("1/1");
    oof("1%1");
    oof("1<1");
    oof("1>1");
    oof("1<=1");
    oof("1>=1");
    oof("1==1");
    oof("1!=1");
    oof("1&1");
    oof("1^1");
    oof("1|1");
    oof("1&&1");
    oof("1||1");

    oof("if 1 then 1 else 1 end");

    oof("print(\"lol\")");
    
    println!();

//    let src = r#"(if "zvqeCWbFy026Vbx2V7nKx6GiSEeS7q" then "j1DGS7NFH5Xw7YBFmJEzsQOQIMd7jb" else nil end)"#;
    //let parsed = *parser::ExprParser::new().parse(src).unwrap();

    //println!("\n{:?}", parsed);
    
    if true
    {
	println!("RANDOM_TEST");
	let rand_expr = *Expression::random(4);
	println!("{}\n", rand_expr);
	let pretty_printed = format!("{}", rand_expr);
	println!("{}\n", pretty_printed);
	let parsed = *parser::ExprParser::new().parse(pretty_printed.as_str()).unwrap();
	println!("{}\n", parsed);
	println!("MATCH = {}", rand_expr == parsed)
    }
}
