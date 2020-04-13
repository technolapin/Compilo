use super::{Unop, Binop, Ternop, Terminal, VarsRegister, Identifier, Primitive};


#[derive(Debug, PartialEq)]
pub struct Seq(pub Vec<Expression>);

impl Seq
{
    pub fn new(expr: Expression) -> Self
    {
	Self(vec![expr])
    }

    pub fn pushed(self, expr: Expression) -> Self
    {
	let mut v = self.0;
	v.push(expr);
	Self(v)
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression
{
    Terminal(Terminal),
    Identifier(Identifier),

    Unary(Unop, Box<Expression>),
    Binary(Binop, Box<Expression>, Box<Expression>),
//    Ternary(Ternop, Box<Expression>, Box<Expression>, Box<Expression>),

    If(Box<Expression>, Seq, Seq),
    Seq(Seq),
    LetIn(VarsRegister, Seq),
    Primitive(Primitive, Box<Expression>),
}


impl Expression
{
    pub fn pretty_print(&self)
    {
	println!("{}", self);
    }

    pub fn random(depth: u32) -> Box<Self>
    {
	Box::new(
	    if depth == 0
	    {
		Self::Terminal(Terminal::random())
	    }
	    else
	    {
		match rand::random::<u32>() % 3
		{
		    0 => Self::Unary(Unop::random(), Self::random(depth-1)),
		    1 => Self::Binary(Binop::random(), Self::random(depth-1), Self::random(depth-1)),
//		    2 => Self::Ternary(Ternop::random(), Self::random(depth-1), Self::random(depth-1), Self::random(depth-1)),
		    _ => {println!("EXPR UNREACH"); unreachable!()}
		}
	    }
	)
    }
}
