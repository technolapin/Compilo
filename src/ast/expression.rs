use super::{Unop, Binop, Terminal, VarsRegister, Identifier, Primitive};


#[derive(Debug, PartialEq, Clone)]
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

    fn random(depth: u32) -> Self
    {
	if depth == 0
	{
	    Self(vec![*Expression::random(0)])
	}
	else
	{
	    let r = rand::random::<u32>() % 4+1;
	    (0..r).fold(Self(vec![]), |seq, _| seq.pushed(*Expression::random(depth-1)))
	}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression
{
    Terminal(Terminal),
    Identifier(Identifier),


    Unary(Unop, Box<Expression>),
    Binary(Binop, Box<Expression>, Box<Expression>),

    If(Box<Expression>, Seq, Seq),
    Block(Seq),
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
		match rand::random::<u32>() % 6
		{
		    0 => Self::Unary(Unop::random(), Self::random(depth-1)),
		    1 => Self::Binary(Binop::random(), Self::random(depth-1), Self::random(depth-1)),
		    2 => Self::If(Self::random(depth-1), Seq::random(depth-1), Seq::random(depth-1)),
		    3 => Self::Block(Seq::random(depth-1)),
		    4 => Self::LetIn(VarsRegister::random(depth-1), Seq::random(depth-1)),
		    5 => Self::Primitive(Primitive::random(), Self::random(depth-1)),
		    _ => {println!("EXPR UNREACH"); unreachable!()}
		}
	    }
	)
    }
}

