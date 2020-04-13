use std::fmt;

use super::*;

impl fmt::Display for Seq
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	let n = self.0.len();
	self.0.get(0..(n.max(1)-1)).map(
	    |elements|
	    elements.iter().for_each(
	    |expr| {write!(f, "{};\n", *expr);}
	));
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
	    
	    Expression::If(cond, seq_a, seq_b) =>
	    {
		write!(f, "(if {} then {} else {})", *cond, seq_a, seq_b)
	    },
	    Expression::Terminal(term) =>
	    {
		match term
		{
		    Terminal::Int(x) => write!(f, "{}", x),
		    Terminal::String(x) => write!(f, r#""{}""#, x),
		    Terminal::Nil => write!(f, "()"),		    
		}
	    },
	    Expression::Block(seq) =>
	    {
		write!(f, "{{ {} }}", seq)
	    },
	    Expression::LetIn(var_register, boxed_expr) =>
	    {
		write!(f, "(let {} in {} end)", var_register, boxed_expr)
	    }
	    Expression::Identifier(identifier) =>
	    {
		write!(f, "{}", identifier)
	    },
	    Expression::Primitive(prim, expr) =>
	    {
		write!(f, "{}({})", prim, *expr)
	    },
	    
	    
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
