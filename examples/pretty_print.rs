extern crate compilo;

use compilo::ast::*;

use compilo::parser;

fn seq(s: &str)
{
    let parsed = parser::SeqParser::new().parse(s);
    println!("{} {}", s, parsed.map(|a| format!("{}", a)).unwrap_or(String::from("#ERR")));
}

fn type_inference(s: &str)
{
    let parsed = parser::SeqParser::new().parse(s);
    if let Ok(seq) = parsed
    {
	println!("{:?}", seq);
	println!("INFERED TYPE: {:?}", seq.infer_type(&mut Binder::new()));
    }
    else
    {
	println!("failed parsing:\n{:?}", parsed);
    }
}


fn run(s: &str)
{
    println!("running: {}", s);
    let seq = parser::SeqParser::new().parse(s).expect("FAILED PARSING");
    println!("Infered type: {:?}", seq.infer_type(&mut Binder::new()));
    let desug = seq.desugar();
    print!("DESUGARIZED CODE:\n{}", desug);
    println!("OUTPUT:");
    println!("RESULT: {}", desug.reduce(&mut Context::new()));
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
    seq("if {1} then 2 else 3");

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



    println!();
    
    
    if false
    {
	println!("RANDOM_TEST");
	let rand_expr = *Expression::random(2);
	println!("{}\n", rand_expr);
	let pretty_printed = format!("{}", rand_expr);
	println!("{}\n", pretty_printed);
	let parsed = *parser::ExprParser::new().parse(pretty_printed.as_str()).unwrap();
	println!("{}\n", parsed);
	let same = rand_expr == parsed;
	println!("MATCH = {}", same);
	if !same
	{
	    println!("ORIGINAL:\n {:?}", rand_expr);
	    println!("PARSED  :\nq {:?}", parsed);
	    
	}
	println!();
	type_inference(pretty_printed.as_str());
    }

    let lol = r#"
let var iA_b4:= 3
in
  print(iA_b4+1)
end
"#;
    run(lol);

    let lol = r#"
let var v:= 0
in
  print(v);
  v := 1;
  print(v);
end
"#;

    let lol = r#"
let var v:= 0
in
  print(v);
  let var v:= 1
  in
    print(v);
  end;
  let var v:= 2
  in
    print(v);
  end
end
"#;
    run(lol);

    run(r#"
let var i := 0
in
  while i < 8 do
    {print(i);
   i := i+1};

end
"#);

    let comp = Expression::Binary(Binop::Less, Box::new(Expression::Terminal(Terminal::Int(1))), Box::new(Expression::Terminal(Terminal::Int(2)))); 
    println!("ZAEAZEZAEAZ {}", comp.infer_type(&mut Binder::new()).unwrap());

    println!("\n\nAZIEMZAEMZAMOEAZ");
    type_inference(r#"
let var i:="lol"
in
  print(i);
  i := "b";
end
"#);

    let lol = r#"
let var i:= 1
in
  for i := 1 to 10
  do
    print(i);
print(i)

end
"#;

    let parsed = parser::ExprParser::new().parse(lol).expect("CASSÉ LE CODE");

    println!("\n\n\n\n");
    println!("CODE à DÉSUCRER:\n{}\n", parsed);
    println!("FOR:\n{}\n", parsed.desugar_for());
    println!("FOR:\n{}\n", parsed.desugar_for()
	     .reduce(&mut Context::new()));
    
    let lol = "if if false then true else false then 1+2 else 2+3";
    run(lol);

    run("let var i:= 1 in print(i++); print(i) end");
    run("let var i:= 1 in print(++i); print(i) end");
    run("let var i:= 1 in print(i--); print(i) end");
    run("let var i:= 1 in print(--i); print(i) end");
}


 
