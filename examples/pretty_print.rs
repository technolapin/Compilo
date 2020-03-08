extern crate compilo;

use compilo::ast::*;

use compilo::expressions;

fn main()
{
    let e = Expression::Unary(Unop::Minus, Box::new(Expression::Binary(Binop::Mul, Box::new(Expression::Terminal(Terminal::Int(8))), Box::new(Expression::Terminal(Terminal::Int(3))))));
    e.pretty_print();
    println!("{:?}", expressions::ExprParser::new().parse("if (64+57)-12*8 {71/33} else {666}"));
    println!("{:?}", expressions::ExprParser::new().parse("(1+1)"));
    println!("{:?}", expressions::ExprParser::new().parse("1-1"));
    println!("{:?}", expressions::ExprParser::new().parse("1*1"));
    println!("{:?}", expressions::ExprParser::new().parse("1/1"));

}
