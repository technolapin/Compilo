


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
    Or,

	
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
    String(String),
    Nil
}

#[derive(Debug, PartialEq)]
pub enum Expression
{
    Terminal(Terminal),
    Unary(Unop, Box<Expression>),
    Binary(Binop, Box<Expression>, Box<Expression>),
    Ternary(Ternop, Box<Expression>, Box<Expression>, Box<Expression>),
    
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
		    Ternop::If => write!(f, "(if {} then {} else {} end)",
					 *exp_a, *exp_b, *exp_c)
		}
	    },
	    Expression::Terminal(term) =>
	    {
		match term
		{
		    Terminal::Int(x) => write!(f, "{}", x),
		    Terminal::String(x) => write!(f, r#""{}""#, x),
		    Terminal::Nil => write!(f, "nil"),		    
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
	use rand::{thread_rng, Rng};
	use rand::distributions::Alphanumeric;
	match rand::random::<u32>() % 3
	{
	    0 => Terminal::Int(rand::random::<u32>()),
	    1 => {
		Terminal::String(thread_rng()
				 .sample_iter(&Alphanumeric)
				 .take(30)
				 .collect())
	    },
	    2 => Terminal::Nil,
	    _ => unreachable!()
	}
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

/*
enum Program
{
    Exp(Expression),
    Decl(Declaration)
}
*/


pub enum Declaration
{
    Type,
    Class,
    Var,
    Fun,
    Prim,
    Meth,
    Import    
}




pub struct Identifier;

#[derive(Debug, PartialEq)]
pub enum Statement
{
    Expr(Expression),
    Prim(Primitive, Expression),
    StatBlock(Vec<Statement>)
}

impl fmt::Display for Statement
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	match self
	{
	    Self::Expr(e) => write!(f, "{}", e),
	    Self::Prim(prim, e) => write!(f, "{}({})", prim, e),
	    Self::StatBlock(exp_vec) =>
	    {
		let listed =
		    exp_vec.iter().fold(String::new(), |s, stat|
			     {
				 let mut smut = s;
				 smut.push_str(format!("{}", stat).as_str());
				 smut
			     }
		);
		write!(f, "{{\n{}\n}}", listed)
	    },
	}
    }
}




#[derive(Debug, PartialEq)]
pub enum Primitive
{
    Print
}

impl fmt::Display for Primitive
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	match self
	{
	    Self::Print => write!(f, "print")
	}
    }
}
