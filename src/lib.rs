#[cfg(test)]
mod tests {
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

}

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP




pub mod ast;


