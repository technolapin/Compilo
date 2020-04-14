use super::{Unop, Binop, Terminal, VarsRegister, Identifier, Primitive};

use std::sync::Arc;

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

    pub fn infer_type(&self) -> Result<Type, String>
    {
	self.0.last().expect("EMPTY SEQ NOT SUPPORTED").infer_type()
    }
    pub fn binder(&self) -> Self
    {
	self.bind(&VarsRegister::new())
    }

    fn bind(&self, scope: &VarsRegister) -> Self
    {
	Seq(
	    self.0.iter()
		.map(|expr|
		     {
			 expr.bind(scope)
		     }
		).collect::<Vec<_>>()
	)
    }

    fn merge(&self, other: &Self) -> Self
    {
	let mut new = self.0.clone();
	new.extend_from_slice(&other.0);
	Self(new)
    }

    pub fn reduce(&self) -> Terminal
    {
	self.0.iter()
	    .fold(Terminal::Nil, |_, expr| expr.reduce())
    }
}


#[derive(PartialEq, Debug, Clone)]
pub enum Type
{
    Nil,
    Int,
    String,
    Bool,
    Unknown // used durring type inference
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

    Binding(Arc<Expression>),
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
    
    pub fn infer_type(&self) -> Result<Type, String>
    {
	match self
	{
	    Self::Terminal(ter) => ter.infer_type(),
	    Self::Identifier(_id) => Ok(Type::Unknown),
	    Self::Unary(_unop, exp) => (*exp).infer_type(),
	    Self::Binary(binop, a, b) =>
	    {
		let t_a = (*a).infer_type()?;
		let t_b = (*b).infer_type()?;
		if t_a == t_b
		{
		    Ok(t_a)
		}
		else if t_a == Type::Unknown
		{
		    Ok(t_b)
		}
		else if t_b == Type::Unknown
		{
		    Ok(t_a)
		}
		else
		{
		    Err(format!("{:?} cannot be applied to {:?} and {:?}", binop, t_a, t_b))
		}
	    },
	    Self::If(_cond, sa, sb) =>
	    {
		let t_a = sa.infer_type()?;
		let t_b = sb.infer_type()?;
		if t_a == t_b
		{
		    Ok(t_a)
		}
		else if t_a == Type::Unknown
		{
		    Ok(t_b)
		}
		else if t_b == Type::Unknown
		{
		    Ok(t_a)
		}
		else
		{
		    Err(format!("if types differ: {:?} {:?}", t_a, t_b))
		}
		
	    },
	    Self::Block(seq) => seq.infer_type(),
	    Self::LetIn(_reg, seq) => seq.infer_type(),
	    Self::Primitive(_, _) => Ok(Type::Nil),

	    Self::Binding(ptr) => (*ptr).infer_type()
	    
	}
    }
    
    pub fn bind(&self, scope: &VarsRegister) -> Self
    {
	match self
	{
	    Self::Terminal(_) => self.clone(),
	    Self::Identifier(id) =>
		scope.get_binding(&id, scope).unwrap(),

	    Self::Unary(op, ptr) =>
		Self::Unary(
		    op.clone(),
		    Box::new((*ptr).bind(scope))
		),

	    Self::Binary(op, ptr_a, ptr_b) =>
		Self::Binary(
		    op.clone(),
		    Box::new((*ptr_a).bind(scope)),
		    Box::new((*ptr_b).bind(scope))
		),

	    Self::If(ptr, seq_a, seq_b) =>
		Self::If(
		    Box::new((*ptr).bind(scope)),
		    seq_a.bind(scope),
		    seq_b.bind(scope),
		),

	    Self::Block(seq) => Self::Block(seq.bind(scope)),

	    Self::LetIn(local_scope, seq) =>
	    {
		//	let binded_scope = local_scope.as_seq().bind(scope);
			let binded_scope = local_scope.bind(scope);
		Self::Block(
		    binded_scope.as_seq()
			.merge(
			    &seq.bind(&scope.merged(&binded_scope))
			)
		)
	    },
	    Self::Primitive(prim, ptr) =>
		Self::Primitive(prim.clone(), Box::new((*ptr).bind(scope))),
	    Self::Binding(_)  => self.clone()
	}
    }

    pub fn reduce(&self) -> Terminal
    {
	match self
	{
	    Self::Terminal(t) => t.clone(),
	    Self::Identifier(id) => panic!("UNBINDED IDENTIFIER"),
	    Self::Unary(op, ptr) =>
	    {
		match op
		{
		    Unop::Minus =>
		    {
			match (*ptr).reduce()
			{
			    Terminal::Int(v) => Terminal::Int(-v),
			    _ => panic!("RUNTIME ERROR: ATTEMPT TO USE UNOP - ON A NON-INT VALUE")
			}
		    },
		    Unop::Plus =>
		    {
			match (*ptr).reduce()
			{
			    Terminal::Int(v) => Terminal::Int(v),
			    _ => panic!("RUNTIME ERROR: ATTEMPT TO USE UNOP + ON A NON-INT VALUE")
			}
		    },
		    _ => unimplemented!()
		}
	    },
	    Self::Binary(op, ptr_a, ptr_b) =>
	    {
		let a = (*ptr_a).reduce();
		let b = (*ptr_b).reduce();
		match op
		{
		    Binop::Add =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Int(a+b),
			    _ => unimplemented!()
			}
		    },
		    Binop::Sub =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Int(a-b),
			    _ => unimplemented!()
			}
		    },
		    Binop::Mul =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Int(a*b),
			    _ => unimplemented!()
			}
		    },
		    Binop::Div =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Int(a/b),
			    _ => unimplemented!()
			}
		    },
		    Binop::Modulo =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Int(a%b),
			    _ => unimplemented!()
			}
		    },
		    Binop::Less =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Bool(a<b),
			    _ => unimplemented!()
			}
		    },
		    Binop::Greater =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Bool(a>b),
			    _ => unimplemented!()
			}
		    },
		    Binop::LessEqual =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Bool(a<=b),
			    _ => unimplemented!()
			}
		    },
		    Binop::GreaterEqual =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Bool(a>=b),
			    _ => unimplemented!()
			}
		    },
		    Binop::Equal =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Bool(a==b),
			    _ => unimplemented!()
			}
		    },
		    Binop::NotEqual =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Bool(a!=b),
			    _ => unimplemented!()
			}
		    },
		    Binop::BitAnd =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Int(a&b),
			    _ => unimplemented!()
			}
		    },
		    Binop::Xor =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Int(a^b),
			    _ => unimplemented!()
			}
		    },
		    Binop::BitOr =>
		    {
			match (a, b)
			{
			    (Terminal::Int(a), Terminal::Int(b)) => Terminal::Int(a|b),
			    _ => unimplemented!()
			}
		    },
		    Binop::And =>
		    {
			match (a, b)
			{
			    (Terminal::Bool(a), Terminal::Bool(b)) => Terminal::Bool(a&&b),
			    _ => unimplemented!()
			}
		    },
		    Binop::Or =>
		    {
			match (a, b)
			{
			    (Terminal::Bool(a), Terminal::Bool(b)) => Terminal::Bool(a||b),
			    _ => unimplemented!()
			}
		    },
		}
	    },
	    Self::If(ptr, seq_a, seq_b) =>
	    {
		let cond = (*ptr).reduce();
		let a = seq_a.reduce();
		let b = seq_b.reduce();
		match cond
		{
		    Terminal::Bool(cond) => if cond {a} else {b},
		    _ => unimplemented!()
		    
		}
	    },
	    Self::Block(seq) =>
	    {
		seq.reduce()
	    },
	    Self::LetIn(_, _) => unreachable!(),
	    Self::Primitive(prim, ptr) =>
	    {
		match prim
		{
		    Primitive::Print =>
		    {
			println!("{}", (*ptr).reduce());
			Terminal::Nil
		    }
		}
	    },
	    Self::Binding(ptr) =>
	    {
		(*ptr).reduce()
	    }
	}
    }
}

