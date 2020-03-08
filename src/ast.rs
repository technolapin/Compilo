


#[derive(Debug)]
pub enum Unop
{
    Minus
}
#[derive(Debug)]
pub enum Binop
{
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Ternop
{
    If
}


#[derive(Debug)]
pub enum Terminal
{
    Int(i32),
    Float(f32),
    Bool(bool),
    Char(char),
    String(String)
}

#[derive(Debug)]
pub enum Expression
{
    Terminal(Terminal),
    Unary(Unop, Box<Expression>),
    Binary(Binop, Box<Expression>, Box<Expression>),
    Ternary(Ternop, Box<Expression>, Box<Expression>, Box<Expression>)
}



use std::fmt;

impl fmt::Display for Expression
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	match self
	{
	    Expression::Unary(op, exp) =>
	    {
		match op
		{
		    Unop::Minus => write!(f, "-{}", *exp)
		}
	    }
	    Expression::Binary(op, exp_a, exp_b) =>
	    {
		match op
		{
		    Binop::Add => write!(f, "({} + {})", *exp_a, exp_b),
		    Binop::Sub => write!(f, "({} - {})", *exp_a, exp_b),
		    Binop::Mul => write!(f, "{} * {}", *exp_a, exp_b),
		    Binop::Div => write!(f, "{} / {}", *exp_a, exp_b),
		}
	    }
	    
	    Expression::Ternary(op, exp_a, exp_b, exp_c) =>
	    {
		match op
		{
		    Ternop::If => write!(f, "if {} then {} else {} end",
					 *exp_a, *exp_b, *exp_c)
		}
	    },
	    Expression::Terminal(term) =>
	    {
		match term
		{
		    Terminal::Int(x) => write!(f, "{}", x),
		    Terminal::Float(x) => write!(f, "{}", x),
		    Terminal::Bool(x) => write!(f, "{}", x),
		    Terminal::Char(x) => write!(f, "{}", x),
		    Terminal::String(x) => write!(f, "{}", x),
		}
	    }
	    
	}
    }}




impl Expression
{
    pub fn pretty_print(&self)
    {
	println!("{}", self);
    }
}
