#[cfg(test)]
mod tests {
    // tests the parser (without any consideration for the validity of the types and bindings)
    #[test]
    fn pretty_print()
    {
	use crate::ast::Expression;
	use crate::parser;
	for _ in 0..1000
	{
	    let rand_expr = *Expression::random(8);
	    let pretty_printed = format!("{}", rand_expr);
	    let parsed = *parser::ExprParser::new().parse(pretty_printed.as_str()).unwrap();
	    assert!(rand_expr == parsed);
	}
    }
    
    #[test]
    fn exprs()
    {
	use crate::ast::{Expression, Terminal};
	use crate::parser;
	fn run(s: &str, expected: Terminal)
	{
	    println!("============================");
	    println!("{}", s);
	    println!("- - - - - - - - - - - - - - ");
	    let parser = parser::SeqParser::new();
	    let maybe = parser.parse(s);
	    print!("PARSING: ");
	    let seq = match maybe
	    {
		Err(err) => {println!("ERROR ({})", err); panic!("FAILED TO PARSE")}
		Ok(seq) => {println!("DONE"); seq}
	    };
	    print!("BINDING/TYPE CHECKING: ");
	    match seq.check()
	    {
		Err(err) => {
		    println!("ERROR ({})", err);
		    panic!(format!("{:?}\n{}", err, s))
		},
		Ok(typ) => println!("DONE (infered {} )", typ)
	    };
	    println!("STANDARD OUTPUT:");
	    let res = seq.run();
	    println!("OUTPUT: {:?}", res.clone());
	    assert!(res.clone() == Ok(expected));
	}
	
	fn wrong(s: &str)
	{
	    let parser = parser::SeqParser::new();
	    match parser.parse(s)
	    {
		Err(err) => (),
		Ok(seq) => assert!(seq.check().is_err())
	    }
	}
	use Terminal::*;

	run("-1", Int(-1));
	run("+1", Int(1));
	run("~1", Int(-2));
	run("!true", Bool(false));

	run("1+1", Int(2));
	run("1-1", Int(0));
	run("2*2", Int(4));
	run(r#""ab"*3"#, String(std::string::String::from("ababab")));
	run("4/2", Int(2));
	run("4%3", Int(1));
	run("4<3", Bool(false));
	run("4>3", Bool(true));
	run("4<=4", Bool(true));
	run("4>=3", Bool(true));
	run("4=3", Bool(false));
	run("1=1", Bool(true));
	run(r#""aaa"="ab""#, Bool(false));
	run(r#""aaa"="aaa""#, Bool(true));
	run("true=false", Bool(false));
	run("true=true", Bool(true));
	run("1<>1", Bool(false));
	run("1<>2", Bool(true));
	run(r#""aaaa"<>"aab""#, Bool(true));
	run("true<>true", Bool(false));
	run("5&3", Int(1));
	run("5^3", Int(6));
	run("5|3", Int(7));
	run("true && false", Bool(false));
	run("true || false", Bool(true));
	run("true && true", Bool(true));
	run("false || false", Bool(false));
	run("5<<2", Int(20));
	run("5>>2", Int(1));

	run("let var teto := 0 in teto++ end", Int(0));
	run("let var teto := 0 in ++teto end", Int(1));
	run("let var teto := 0 in teto-- end", Int(0));
	run("let var teto := 0 in --teto end", Int(-1));
	
	run("let var teto := 2 in teto:=4 end", Int(4));
	run("let var teto := 2 in teto+=4 end", Int(6));
	run("let var teto := 2 in teto-=4 end", Int(-2));
	run("let var teto := 2 in teto*=4 end", Int(8));
	run("let var teto := 6 in teto/=3 end", Int(2));
	run("let var teto := 5 in teto%=3 end", Int(2));
	run("let var teto := 5 in teto&=3 end", Int(1));
	run("let var teto := 5 in teto|=3 end", Int(7));
	run("let var teto := 5 in teto^=3 end", Int(6));
	run("let var teto := 5 in teto<<=2 end", Int(20));
	run("let var teto := 5 in teto>>=2 end", Int(1));

	run("if true then 1 else 0", Int(1));
	run("if false then 1 else 0", Int(0));

        run("
let var n:= 6
    var fact := 1
in
  for i := 1 to n+1 do
    fact *= i,
  fact
end
", Int(720));

        run("
let var n:= 6
    var fact := 1
    var i := 2
in
  while i < n+1 do
    fact *= i++,
  fact
end
", Int(720));
	run(r#"(1, 2, 3, true, false, "abzaraz", 5)"#, Int(5));

	wrong(r#"1+true"#);
	wrong(r#"if 1 then 0 else 0"#);
	wrong(r#"if true then 0 else "int""#);
	wrong(r#"1="1""#);

	run(
"let
  var foo := 1
  var bar := 1 + foo
  var baz := bar * bar
in
  foo, bar * baz
end",
	    Int(8)
	);
	run("let in 1 + 1 end", Int(2));

	run("let var i := 1 in print(i) end", Nil);
	run("
let var i := 10
in
  print(i),
  let var i := i * i
  in
    print(i)
  end,
  print(i)
end",
	    Nil
	);
	wrong("let var foo in end");

	run("
let var i := 1
in
  while i < 5 do
    (print(i), i := i + 1)
end
", Nil);
	run("
let var i := 1
in
  for i := 2 to 3 do print(i),
  print(i)
end", Nil);
	run("
for i := 2 to 1 do print(i)
", Nil);
	wrong("(for i := 1 to 2 do i, print(i))");
    }
    
}

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP




pub mod ast;


