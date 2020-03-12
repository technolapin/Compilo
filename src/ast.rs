


#[derive(Debug, PartialEq)]
pub enum Unop
{
    Increment,
    Decrement,
    Minus,
    Plus,
    Not,
}
#[derive(Debug, PartialEq)]
pub enum Binop
{
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    BitAnd,
    Xor,
    BitOr,
    And,
    Or
    
}

#[derive(Debug, PartialEq)]
pub enum Ternop
{
    If
}


#[derive(Debug, PartialEq)]
pub enum Terminal
{
    Int(u32),
    Float(f32),
    Bool(bool),
    Char(char),
    String(String)
}

#[derive(Debug, PartialEq)]
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
		    Unop::Decrement => write!(f, "(-- {})", *exp),
		    Unop::Increment => write!(f, "(++ {})", *exp),
		    Unop::Minus => write!(f, "(- {})", *exp),
		    Unop::Plus => write!(f, "(+ {})", *exp),
		    Unop::Not => write!(f, "(! {})", *exp),
		}
	    }
	    Expression::Binary(op, exp_a, exp_b) =>
	    {
		match op
		{
		    Binop::Add => write!(f, "({} + {})", *exp_a, exp_b),
		    Binop::Sub => write!(f, "({} - {})", *exp_a, exp_b),
		    Binop::Mul => write!(f, "({} * {})", *exp_a, exp_b),
		    Binop::Div => write!(f, "({} / {})", *exp_a, exp_b),
		    Binop::Modulo => write!(f, "({} % {})", *exp_a, exp_b),
		    Binop::Less => write!(f, "({} < {})", *exp_a, exp_b),
		    Binop::Greater => write!(f, "({} > {})", *exp_a, exp_b),
		    Binop::LessEqual => write!(f, "({} <= {})", *exp_a, exp_b),
		    Binop::GreaterEqual => write!(f, "({} >= {})", *exp_a, exp_b),
		    Binop::Equal => write!(f, "({} == {})", *exp_a, *exp_b),
		    Binop::NotEqual => write!(f, "({} != {})", *exp_a, *exp_b),
		    Binop::BitAnd => write!(f, "({} & {})", *exp_a, *exp_b),
		    Binop::Xor => write!(f, "({} ^ {})", *exp_a, *exp_b),
		    Binop::BitOr => write!(f, "({} | {})", *exp_a, *exp_b),
		    Binop::And => write!(f, "({} && {})", *exp_a, *exp_b),
		    Binop::Or => write!(f, "({} || {})", *exp_a, *exp_b),
		}
	    }
	    
	    Expression::Ternary(op, exp_a, exp_b, exp_c) =>
	    {
		match op
		{
		    Ternop::If => write!(f, "(if {} {{{}}} else {{{}}})",
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


impl Unop
{
    fn random() -> Self
    {
	match rand::random::<u32>() % 2
	{
	    0 => Self::Minus,
	    1 => Self::Not,
	    _ => {println!("UN UNREACH"); unreachable!()}
	}
    }
}
impl Binop
{
    fn random() -> Self
    {
	match rand::random::<u32>() % 15
	{
	    0 => Self::Add,
	    1 => Self::Sub,
	    2 => Self::Mul,
	    3 => Self::Div,
	    4 => Self::Less,
	    5 => Self::Greater,
	    6 => Self::LessEqual,
	    7 => Self::GreaterEqual,
	    8 => Self::Equal,
	    9 => Self::NotEqual,
	    10 => Self::BitAnd,
	    11 => Self::Xor,
	    12 => Self::BitOr,
	    13 => Self::And,
	    14 => Self::Or,
	    i => {println!("BIN UNREACH {}", i); unreachable!()}
	    
	}
    }
}
impl Ternop
{
    fn random() -> Self
    {
	Self::If
    }
}
impl Terminal
{
    fn random() -> Self
    {
	Terminal::Int(rand::random::<u32>())
    }
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
		    2 => Self::Ternary(Ternop::random(), Self::random(depth-1), Self::random(depth-1), Self::random(depth-1)),
		    _ => {println!("EXPR UNREACH"); unreachable!()}
		}
	    }
	)
    }
}
