extern crate compilo;

use compilo::ast::*;

use compilo::parser;

fn foo(s: &str)
{
    let parsed = parser::ExprParser::new().parse(s);
    println!("{} {:?}", s, &parsed);
}


fn seq(s: &str)
{
    let parsed = parser::SeqParser::new().parse(s);
    println!("{} {}", s, parsed.map(|a| format!("{}", a)).unwrap_or(String::from("#ERR")));
}

fn test_identif(s: &str)
{
    let parsed = parser::IdentifParser::new().parse(s);
    println!("{} {:?}", s, parsed);
}

fn main()
{
    seq("--1");
    seq("++1");
    seq("1--");
    seq("1++");
    seq("--1");
    seq("++1");
    seq("-1");
    seq("+1");
    seq("!1");
    seq("1+1");
    seq("1-1");
    seq("1*1");
    seq("1/1");
    seq("1%1");
    seq("1<1");
    seq("1>1");
    seq("1<=1");
    seq("1>=1");
    seq("1==1");
    seq("1!=1");
    seq("1&1");
    seq("1^1");
    seq("1|1");
    seq("1&&1");
    seq("1||1");

    seq("if 1 then 1 else 1");

    seq(r#"print("lol")"#);
    seq(r#"{
print("lol");
print("not lol");
1;
2
};
"#);

    seq("1 + iiiii");
    test_identif("iiiii");
    use regex::Regex;
    let re = Regex::new(r#"^[[:alpha:]][[[:word:]]&&[^a]]$"#).unwrap();
    assert!(re.is_match("ii"));
    
    seq(r#"
let var i := 10
in
1 + i + 1
end
"#);
    
    println!();

//    let src = r#"(if "zvqeCWbFy026Vbx2V7nKx6GiSEeS7q" then "j1DGS7NFH5Xw7YBFmJEzsQOQIMd7jb" else nil end)"#;
    //let parsed = *parser::ExprParser::new().parse(src).unwrap();

    //println!("\n{:?}", parsed);
    
    if false
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

 
