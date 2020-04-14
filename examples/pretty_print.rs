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
	println!("INFERED TYPE: {:?}", seq.infer_type());
    }
    else
    {
	println!("failed parsing:\n{:?}", parsed);
    }
}

fn bind(s: &str)
{
    let parsed = parser::SeqParser::new().parse(s);
    if let Ok(seq) = parsed
    {
	println!("ORIGINAL:\n{}", seq);
	println!();
	println!("BINDED  :\n{}", seq.binder());
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
    let binded = seq.binder();
    println!("Binded: {}", binded);
    println!("Infered type: {:?}", binded.infer_type());
    println!("OUTPUT:");
    println!("RESULT: {}", binded.reduce());
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



    bind("let var i:=1 in 2+i end");
    println!();
    
    bind(&format!("{}", *Expression::random(4)));
    
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
    
    println!("\n\n");
    let lol = r#"(let  var atdnfs0NL := { nil;
"UG4lkvHFBlWMipSPrtvAtx84lkEOm5" }
 var aCkbYId0N := (let  var aLocZmrw4 := "Srgf0lJmqiFTZ1Q0vR6FyC6gvv0Ugn"
 var avp6h41Fj := true
 var anE4JRh1k := "K4JdZMyR4ZMYSuFf95uM82N5uOMXLD"
 var azFJzIz9C := true
 var aFOFkGqYC := true
 var a2s3kLlXQ := false
 in false;
"d8QSY8b5jK3yBdoqmLRtFFaUGkQOoH" end)
 var a64ZGcP40 := { true;
770611747;
true;
980049709 }
 in (if (let  var aFuUaReJN := true
 var aHkqx2XCR := nil
 var axaZ0ek5B := true
 var aMJZjDxq6 := nil
 var arPCQBCZW := (- 458676909)
 in "7iPrLyCL0LyEPQcShk4j0LHebpgNA4" end) then "5xNoGYkisYsabu7Z6wovICyDYzahX2";
"E3tJzbk5Lhrsnhrc43u3MlJko6outC";
"IjrcrPbGGxsShbNPW6HDtRn58YFhYm" else true;
"4rRBL1ueK1YpWswq5zr4W5kUsQPn4C") end)"#;
    bind(lol);

    println!("\n\n");
    let lol = "{let var i:= (let var j := 2 in j end) in i+1 end}";
    bind(lol);


    let lol = r#"
let var i:= 3
in
  print(i+1)
end
"#;

    run(lol);

    
}


 
