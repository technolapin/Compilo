use std::fmt;

use super::*;

impl fmt::Display for Seq
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	let n = self.0.len();

	for expr in (0..n-1).map(|i| &self.0[i])
	{
	    write!(f, "{},\n", *expr)?
	}
	
	if let Some(last) = self.0.last()
	{
	    write!(f, "{}", last)
	}
	else
	{
	    write!(f, "")
	}
    }
}

impl fmt::Display for Terminal
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {

	match self
	{
	    Terminal::Int(x) => write!(f, "{}", x),
	    Terminal::String(x) => write!(f, r#""{}""#, x),
	    Terminal::Nil => write!(f, "nil"),
	    Terminal::Bool(b) => write!(f, "{}", b)
	}
    }
}



impl fmt::Display for Expression
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	match self
	{
	    Expression::Unary(op, exp) =>
	    {
		write!(f, "{} {}", op, *exp)
	    }
	    Expression::Binary(op, exp_a, exp_b) =>
	    {
		write!(f, "{} {} {}", *exp_a, op, *exp_b)
	    }
	    
	    Expression::If(cond, seq_a, seq_b) =>
	    {
		write!(f, "if {} then {} else {}", *cond, seq_a, seq_b)
	    },
	    Expression::Terminal(term) =>
	    {
		write!(f, "{}", term)
	    },
	    Expression::Block(seq) =>
	    {
		write!(f, "( {} )", seq)
	    },
	    Expression::LetIn(var_register, boxed_expr) =>
	    {
		write!(f, "let {} in {} end", var_register, boxed_expr)
	    }
	    Expression::Identifier(identifier) =>
	    {
		write!(f, "{}", identifier)
	    },
	    Expression::Primitive(prim, expr) =>
	    {
		write!(f, "{}({})", prim, *expr)
	    },
	    Expression::IdopOne(op, id, ptr) =>
	    {
		write!(f, "{} {} {}", id, op, ptr)
	    },
	    Expression::IdopNone(op, id) => match op
	    {
		IdopNone::IncrPostfix | IdopNone::DecrPostfix =>
		{
		    write!(f, "{} {}", id, op)
		},
		IdopNone::IncrPrefix | IdopNone::DecrPrefix =>
		{
		    write!(f, "{} {}", op, id)
		},
	    },
	    Expression::For(id, from, to, seq) =>
	    {
		write!(f, "for {} := {} to {} do\n {}",
		       id,
		       *from,
		       *to,
		       seq
		)
	    },
	    Expression::While(expr, seq) =>
	    {
		write!(f, "while {} do\n{}",
		       *expr,
		       seq
		)
	    },
	}
    }
}


impl fmt::Display for Type
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	match self
	{
	    Type::Nil => write!(f, "nil"),
	    Type::Int => write!(f, "int"),
	    Type::Bool => write!(f, "bool"),
	    Type::String => write!(f, "str"),	
	}
    }
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

impl fmt::Display for VarsRegister
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	let s = self.0.iter()
	    .fold(String::new(), |s, (identifier, expr)|
		  {
		      format!("{} var {} := {}\n", s, identifier, expr)
		  });
	write!(f, "{}", s)
    }
}


impl fmt::Display for Identifier
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "{}", self.0)
    }
}


impl fmt::Display for Binop
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	use Binop::*;
	match self
	{
	    Add => write!(f, "+"),
	    Sub => write!(f, "-"),
	    Mul => write!(f, "*"),
	    Div => write!(f, "/"),
	    Modulo => write!(f, "%"),
	    Less => write!(f, "<"),
	    Greater => write!(f, ">"),
	    LessEqual => write!(f, "<="),
	    GreaterEqual => write!(f, ">="),
	    NotEqual => write!(f, "<>"),
	    Equal => write!(f, "="),
	    BitAnd => write!(f, "&"),
	    Xor => write!(f, "^"),
	    BitOr => write!(f, "|"),
	    And => write!(f, "&&"),
	    Or => write!(f, "||"),
	    LShift => write!(f, "<<"),
	    RShift => write!(f, ">>"),
	}
    }
}

impl fmt::Display for Unop
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	use Unop::*;
	match self
	{
	    Plus => write!(f, "+"),
	    Minus => write!(f, "-"),
	    Not => write!(f, "!"),
	    BitNot => write!(f, "~"),

	}
    }
}


impl fmt::Display for IdopNone
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	use IdopNone::*;
	match self
	{
	    IncrPostfix => write!(f, "++"),
	    IncrPrefix => write!(f, "++"),
	    DecrPostfix => write!(f, "--"),
	    DecrPrefix => write!(f, "--"),

	}
    }
}

impl fmt::Display for IdopOne
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	use IdopOne::*;
	match self
	{
	    Assign => write!(f, ":="),
	    IncrBy => write!(f, "+="),
	    DecrBy => write!(f, "-="),
	    MulBy => write!(f, "*="),
	    DivBy => write!(f, "/="),
	    ModBy => write!(f, "%="),
	    AndBy => write!(f, "&="),
	    XorBy => write!(f, "^="),
	    OrBy => write!(f, "|="),
	    LShiftBy => write!(f, "<<="),
	    RShiftBy => write!(f, ">>="),
	}
    }
}
