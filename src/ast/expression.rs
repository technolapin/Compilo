use super::{Unop, Binop, IdopOne, IdopNone, Terminal, VarsRegister, Identifier, Primitive, Seq};

use std::sync::Arc;


#[derive(PartialEq, Debug, Clone)]
pub enum Type
{
    Nil,
    Int,
    String,
    Bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression
{
    Terminal(Terminal),
    Identifier(Identifier),

    Unary(Unop, Box<Expression>),
    Binary(Binop, Box<Expression>, Box<Expression>),

    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Block(Seq),
    LetIn(VarsRegister, Seq),
    Primitive(Primitive, Box<Expression>),

    IdopOne(IdopOne, Identifier, Box<Expression>),
    IdopNone(IdopNone, Identifier),
    While(Box<Expression>, Box<Expression>),
    For(Identifier, Box<Expression>, Box<Expression>, Box<Expression>),
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
		match rand::random::<u32>() % 2
		{
		    0 => Self::Terminal(Terminal::random()),
		    _ => Self::Identifier(Identifier::random())
		}
	    }
	    else
	    {
		match rand::random::<u32>() % 9
		{
		    0 => Self::Block(Seq::new(Self::Unary(Unop::random(), Self::random(depth-1)))),
		    1 => Self::Block(Seq::new(Self::Binary(Binop::random(), Self::random(depth-1), Self::random(depth-1)))),
		    2 => Self::Block(Seq::new(Self::If(Self::random(depth-1), Self::random(depth-1), Self::random(depth-1)))),
		    3 => Self::Block(Seq::random(depth-1)),
		    4 => Self::Block(Seq::new(Self::LetIn(VarsRegister::random(depth-1), Seq::random(depth-1)))),
		    5 => Self::Primitive(Primitive::random(), Self::random(depth-1)),
		    6 => Self::Block(Seq::new(Self::IdopOne(IdopOne::random(), Identifier::random(), Self::random(depth-1)))),
		    7 => Self::Block(Seq::new(Self::IdopNone(IdopNone::random(), Identifier::random()))),
		    8 => Self::Block(Seq::new(Self::While(Self::random(depth-1), Self::random(depth-1)))),
		    _ => Self::Block(Seq::new(Self::For(Identifier::random(), Self::random(depth-1), Self::random(depth-1), Self::random(depth-1)))),
		}
	    }
	)
    }
    
    pub fn infer_type(&self, binder: &mut Binder) -> Result<Type, String>
    {
	match self
	{
	    Self::Terminal(ter) => ter.infer_type(),
	    Self::Identifier(id) => binder.check_var(&id),
	    Self::Unary(unop, exp) => match unop
	    {
		Unop::Minus | Unop::Plus | Unop::BitNot =>
		{
		    match (*exp).infer_type(binder)?
		    {
			Type::Int => Ok(Type::Int),
			t => Err(format!("TYPE MISMATCH: EXPECTED int FOUND {}", t))
		    }
		},
		Unop::Not =>
		{
		    match (*exp).infer_type(binder)?
		    {
			Type::Bool => Ok(Type::Bool),
			t => Err(format!("TYPE MISMATCH: EXPECTED bool FOUND {}", t))
		    }
		},
	    },
	    Self::Binary(binop, a, b) =>
	    {
		let t_a = (*a).infer_type(binder)?;
		let t_b = (*b).infer_type(binder)?;
		match binop
		{
		    Binop::Add =>
		    {
			match (t_a, t_b)
			{
			    (Type::Int, Type::Int) => Ok(Type::Int),
			    (Type::String, Type::String) => Ok(Type::String),
			    (a, b) => Err(format!("{} cannot be applied to {} and {}", binop, a, b))
			}
		    },
		    Binop::Mul =>
		    {
			match (t_a, t_b)
			{
			    (Type::Int, Type::Int) => Ok(Type::Int),
			    (Type::String, Type::Int) => Ok(Type::String),
			    (a, b) => Err(format!("{} cannot be applied to {} and {}", binop, a, b))
			}
		    },
		    
		    Binop::Sub | Binop::Div | Binop::Modulo
			| Binop::BitAnd | Binop::Xor | Binop::BitOr | Binop::RShift | Binop::LShift =>
		    {
			match (t_a, t_b)
			{
			    (Type::Int, Type::Int) => Ok(Type::Int),
			    (a, b) => Err(format!("{} cannot be applied to {} and {}", binop, a, b))
			}
		    },
		    Binop::Less | Binop::Greater | Binop::LessEqual | Binop::GreaterEqual =>
		    {
			match (t_a, t_b)
			{
			    (Type::Int, Type::Int) => Ok(Type::Bool),
			    (a, b) => Err(format!("{} cannot be applied to {} and {}", binop, a, b))
			}
		    },
		    Binop::Equal | Binop::NotEqual =>
		    {
			if t_a == t_b
			{
			    Ok(Type::Bool)
			}
			else
			{
			    Err(format!("{} cannot be applied to {} and {}", binop, a, b))
			}
		    },
		    Binop::And | Binop::Or =>
		    {
			match (t_a, t_b)
			{
			    (Type::Bool, Type::Bool) => Ok(Type::Bool),
			    (a, b) => Err(format!("{} cannot be applied to {} and {}", binop, a, b))
			}
		    },
		}
	    },
	    Self::If(cond, sa, sb) =>
	    {
		let t_cond = (*cond).infer_type(binder)?;
		if t_cond != Type::Bool
		{
		    return Err(format!("Expected a boolean expression, found {}", t_cond));
		};
		let t_a = sa.infer_type(binder)?;
		let t_b = sb.infer_type(binder)?;
		if t_a == t_b
		{
		    Ok(t_a)
		}
		else
		{
		    Err(format!("if types differ: {:?} {:?}", t_a, t_b))
		}
		
	    },
	    Self::Block(seq) => seq.infer_type(binder),
	    Self::LetIn(reg, seq) =>
	    {
		binder.push(reg)?;
		let tmp = seq.infer_type(binder);
		binder.pop();
		tmp
	    },
	    Self::Primitive(_, _) => Ok(Type::Nil),

//	    Self::Binding(ptr) => (*ptr).infer_type(),
	    Self::IdopOne(op, id, ptr) =>
	    {
		let typ_exp = (*ptr).infer_type(binder)?;
		let typ_id = binder.check_var(id)?;
		use IdopOne::*;
		match op
		{
		    IdopOne::Assign =>
		    {
			if typ_exp != typ_id
			{
			    return Err(format!("assign error: attempted to assign a {} to a {} variable", typ_exp, typ_id));
			}
		    },
		    IncrBy =>
		    {
			if typ_exp != typ_id || !(typ_exp == Type::Int || typ_exp == Type::String)
			{
			    return Err(format!("{} cannot be applied to types {} and {}", op, typ_id, typ_exp));
			}
		    },
		    DecrBy | MulBy | DivBy | ModBy | AndBy | XorBy | OrBy | LShiftBy | RShiftBy =>
		    {
			if typ_exp != typ_id || typ_exp != Type::Int
			{
			    return Err(format!("{} cannot be applied to types {} and {}", op, typ_id, typ_exp));
			}
		    }
		};
		Ok(typ_exp)
	    },
	    Self::IdopNone(op, id) =>
	    {
		let typ_id = binder.check_var(id)?;
		match op
		{
		    _ =>
		    {
			if typ_id != Type::Int
			{
			    return Err(format!("cannot apply {} to type {}", op, typ_id));
			}
		    },
		};
		Ok(typ_id)
	    },
	    Self::While(expr, seq) =>
	    {
		let t_cond = (*expr).infer_type(binder)?;
		let _ = seq.infer_type(binder)?;
		if t_cond != Type::Bool
		{
		    Err(format!("Expected a boolean expression, found {}", t_cond))
		}
		else
		{
		    Ok(Type::Nil)
		}
	    },
	    Self::For(id, from, to, seq) =>
	    {
		let t_from = from.infer_type(binder)?;
		let t_to = to.infer_type(binder)?;
		if t_from != Type::Int
		{
		    return Err(format!("wrong type for for lower bound: expected int, found {}", t_from))
		};
		if t_to != Type::Int
		{
		    return Err(format!("wrong type for upper lower bound: expected int, found {}", t_to))
		};

		binder.push
		    (
			&VarsRegister::with_first
			    (
				id.clone(),
				Expression::Terminal(Terminal::Int(0))
			    )
		    )?;
		let _ = seq.infer_type(binder)?;
		binder.pop();
		Ok(Type::Nil)
	    },
	    
	}
    }
    
    pub fn reduce(&self, context: &mut Context) -> Terminal
    {
	match self
	{
	    Self::Terminal(t) => t.clone(),
	    Self::Identifier(id) =>
	    {
		(**context.get_var(id)).clone()
	    },
	    Self::Unary(op, ptr) =>
	    {
		let val = (*ptr).reduce(context);
		use Unop::*;
		use Terminal::*;
		match (op, val)
		{
		    (Minus, Int(v)) => Int(-v),
		    (Plus, Int(v)) => Int(v),
		    (Not, Bool(v)) => Bool(!v),
		    (BitNot, Int(v)) => Int(!v),
		    _ => unimplemented!()
		}
	    },
	    Self::Binary(op, ptr_a, ptr_b) =>
	    {
		let a = (*ptr_a).reduce(context);
		let b = (*ptr_b).reduce(context);
		use Terminal::*;
		use Binop::*;
		match (op, a, b)
		{
		    (Add, Int(a), Int(b)) => Int(a+b),
		    (Add, String(a), String(b)) => String(
			{
			    let mut s = a.clone();
			    s.push_str(b.as_str());
			    s
			}),
		    (Sub, Int(a), Int(b)) => Int(a-b),
		    (Mul, Int(a), Int(b)) => Int(a*b),
		    (Mul, String(a), Int(b)) => String(a.repeat(b as usize)),
		    (Div, Int(a), Int(b)) => Int(a/b),
		    (Modulo, Int(a), Int(b)) => Int(a%b),
		    (Less, Int(a), Int(b)) => Bool(a<b),
		    (Greater, Int(a), Int(b)) => Bool(a>b),
		    (LessEqual, Int(a), Int(b)) => Bool(a<=b),
		    (GreaterEqual, Int(a), Int(b)) => Bool(a>=b),
		    (Equal, any_a, any_b) => Bool(any_a==any_b),
		    (NotEqual, any_a, any_b) => Bool(any_a!=any_b),
		    (BitAnd, Int(a), Int(b)) => Int(a&b),
		    (Xor, Int(a), Int(b)) => Int(a^b),
		    (BitOr, Int(a), Int(b)) => Int(a|b),
		    (And, Bool(a), Bool(b)) => Bool(a&&b),
		    (Or, Bool(a), Bool(b)) => Bool(a||b),
		    (LShift, Int(a), Int(b)) => Int(a<<b),
		    (RShift, Int(a), Int(b)) => Int(a>>b),
		    _ => unimplemented!() // guaranteed impossible by typechecking
		}
	    },
	    Self::If(ptr, seq_a, seq_b) =>
	    {
		let cond = (*ptr).reduce(context);
		let a = seq_a.reduce(context);
		let b = seq_b.reduce(context);
		match cond
		{
		    Terminal::Bool(cond) => if cond {a} else {b},
		    _ => unimplemented!()
		    
		}
	    },
	    Self::Block(seq) =>
	    {
		seq.reduce(context)
	    },
	    Self::LetIn(vars, seq) =>
	    {
		context.push(&vars);
		let val = seq.reduce(context);
		context.pop();
		val
	    },
	    Self::Primitive(prim, ptr) =>
	    {
		match prim
		{
		    Primitive::Print =>
		    {
			println!("{}", (*ptr).reduce(context));
			Terminal::Nil
		    }
		}
	    },
	    Self::IdopOne(op, id, expr) =>
	    {
		let val = (*expr).reduce(context);
		match op
		{
		    IdopOne::Assign =>
		    {
			let ptr: &mut Arc<_> = context.get_var_mut(id);
			*Arc::make_mut(ptr) = val.clone();
			val
		    },
		    _ => unimplemented!() // desugared
		}
	    },
	    Self::IdopNone(_, _) => unimplemented!(),
	    Self::While(expr, seq) =>
	    {
		while {
		    if let Terminal::Bool(cond) = (*expr).reduce(context)
		    {
			cond
		    }
		    else
		    {
			panic!("WHILE ERROR: CONDITION NOT BOOL");
		    }
		}
		{
		    seq.reduce(context);
		}
		Terminal::Nil
		
	    },
	    Self::For(_id, _from, _to, _seq) => unimplemented!()
	}
    }

    /// assumed correct expr    
    pub fn desugar_for(&self) -> Self
    {
	match self
	{
	    Self::For(id, from, to, expr) =>
	    {
		let init =
		    VarsRegister::with_first(
			id.clone(),
			from.desugar_for()
		    );

		let cond = Box::new
		    (
			Expression::Binary
			    (
				Binop::Less,
				Box::new(Expression::Identifier(id.clone())),
				Box::new(to.desugar_for())
			    )
		    );
		let incr =
		    Expression::IdopOne
		    (
			IdopOne::Assign,
			id.clone(),
			Box::new(Expression::Binary
			    (
				Binop::Add,
				Box::new(Expression::Identifier(id.clone())),
				Box::new(Expression::Terminal(Terminal::Int(1)))
				    
			    ))
		    );
		let inner = Seq::new(expr.desugar_for()).pushed(incr);
		Self::LetIn
		    (
			init,
			Seq::new
			    (
				Self::While
				    (
					cond,
					Box::new(Expression::Block(inner))
				    )
			    )
		    )
	    },
	    other =>
	    {
		let lambda =
		    |expr: &Expression| expr.desugar_for();
		other.propagate(&lambda)
	    }
	}
    }

    pub fn desugar_idops(&self) -> Self
    {
	match self
	{
	    Self::IdopNone(op, id) =>
	    {
		match op
		{
		    IdopNone::IncrPostfix =>
		    {
			let tmp = Identifier(String::from("tmp"));
			Expression::LetIn(
			    VarsRegister::with_first(
				tmp.clone(),
				Expression::Identifier(id.clone())),
			    Seq::new(
				Expression::IdopOne(
				    IdopOne::Assign,
				    id.clone(),
				    Box::new(
					Expression::Binary(
					    Binop::Add,
					    Box::new(Expression::Identifier(id.clone())),
					    Box::new(Expression::Terminal(Terminal::Int(1)))
					)
				    )
					
				)
			    ).pushed(Expression::Identifier(tmp))
			)
		    },
		    IdopNone::IncrPrefix =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::Add,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new(Expression::Terminal(Terminal::Int(1)))
				)
			    )
			)
		    },
		    IdopNone::DecrPostfix =>
		    {
			let tmp = Identifier(String::from("tmp"));
			Expression::LetIn(
			    VarsRegister::with_first(
				tmp.clone(),
				Expression::Identifier(id.clone())),
			    Seq::new(
				Expression::IdopOne(
				    IdopOne::Assign,
				    id.clone(),
				    Box::new(
					Expression::Binary(
					    Binop::Sub,
					    Box::new(Expression::Identifier(id.clone())),
					    Box::new(Expression::Terminal(Terminal::Int(1)))
					)
				    )
					
				)
			    ).pushed(Expression::Identifier(tmp))
			)
		    },
		    IdopNone::DecrPrefix =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::Sub,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new(Expression::Terminal(Terminal::Int(1)))
				)
			    )
			)
		    },
		}
	    },
	    Self::IdopOne(op, id, ptr) =>
	    {
		match op
		{
		    IdopOne::Assign => Self::IdopOne(op.clone(), id.clone(), Box::new(ptr.desugar_idops())),
		    IdopOne::IncrBy =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::Add,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new((*ptr).desugar_idops())
				)
			    )
			)
		    },
		    IdopOne::DecrBy =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::Sub,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new((*ptr).desugar_idops())
				)
			    )
			)
		    },
		    IdopOne::MulBy =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::Mul,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new((*ptr).desugar_idops())
				)
			    )
			)
		    },
		    IdopOne::DivBy =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::Div,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new((*ptr).desugar_idops())
				)
			    )
			)
		    },
		    IdopOne::ModBy =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::Modulo,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new((*ptr).desugar_idops())
				)
			    )
			)
		    },
		    IdopOne::AndBy =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::BitAnd,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new((*ptr).desugar_idops())
				)
			    )
			)
		    },
		    IdopOne::XorBy =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::Xor,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new((*ptr).desugar_idops())
				)
			    )
			)
		    },
		    IdopOne::OrBy =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::BitOr,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new((*ptr).desugar_idops())
				)
			    )
			)
		    },
		    IdopOne::LShiftBy =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::LShift,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new((*ptr).desugar_idops())
				)
			    )
			)
		    },
		    IdopOne::RShiftBy =>
		    {
			Expression::IdopOne(
			    IdopOne::Assign,
			    id.clone(),
			    Box::new(
				Expression::Binary(
				    Binop::RShift,
				    Box::new(Expression::Identifier(id.clone())),
				    Box::new((*ptr).desugar_idops())
				)
			    )
			)
		    },
		}
	    },
	    other =>
	    {
		let lambda =
		    |expr: &Expression| expr.desugar_idops();
		other.propagate(&lambda)
	    }
	}    
    }

    pub fn propagate<F>(&self, lambda: &F) -> Self
    where
	F: Fn(&Self) -> Self
    {
	match self
	{
	    Self::Unary(op, ptr) =>
		Self::Unary
		(op.clone(),
		 Box::new(lambda(&*ptr))
		),
	    Self::Binary(op, ptra, ptrb) =>
		Self::Binary
		(op.clone(),
		 Box::new(lambda(&*ptra)),
		 Box::new(lambda(&*ptrb)),
		),
	    Self::If(ptr, ptra, ptrb) =>
		Self::If
		(
		    Box::new(lambda(&*ptr)),
		    Box::new(lambda(&*ptra)),
		    Box::new(lambda(&*ptrb)),
		),
	    Self::Block(seq) => Self::Block(seq.propagate(lambda)),
	    Self::LetIn(reg, seq) => Self::LetIn(
		reg.propagate(lambda),
		seq.propagate(lambda)
	    ),
	    Self::Primitive(prim, ptr) =>
		Self::Primitive(
		    prim.clone(),
		    Box::new(lambda(&*ptr))
		),
	    Self::IdopOne(op, id, ptr) =>
		Self::IdopOne(
		    op.clone(),
		    id.clone(),
		    Box::new(lambda(&*ptr))
		),
	    Self::While(cond, expr) =>
		Self::While(
		    Box::new(lambda(&*cond)),
		    Box::new(lambda(&*expr))
		),
	    Self::For(id, ptra, ptrb, expr) =>
		Self::For(
		    id.clone(),
		    Box::new(lambda(&*ptra)),
		    Box::new(lambda(&*ptrb)),
		    Box::new(lambda(&*expr))
		),
	    other => other.clone()
	    
	    
	}
    }
}

use std::collections::HashMap;

pub struct Context
{
    scopes: Vec<HashMap<Identifier, Arc<Terminal>>>,
    current: HashMap<Identifier, Vec<Arc<Terminal>>>
}

impl Context
{
    pub fn new() -> Self
    {
	Self{
	    scopes: vec![],
	    current: HashMap::new()
	}
    }

    pub fn push(&mut self, register: &VarsRegister)
    {
	let scope = register.get_vec().iter()
	    .map(|(id, expr)| {
		let val = Arc::new(expr.clone().reduce(self));
		match self.current.get_mut(id)
		{
		    None =>
		    {
			self.current.insert((*id).clone(), vec![val.clone()]);
		    },
		    Some(vec) =>
		    {
			vec.push(val.clone());
		    }

		};
		((*id).clone(), val)
	    })
	    .collect::<HashMap<Identifier, Arc<Terminal>>>();
	self.scopes.push(scope);
    }
    
    pub fn pop(&mut self)
    {
	match self.scopes.len()
	{
	    0 => panic!("EMPTY CONTEXT POPED"),
	    1 =>
	    {
		self.scopes.clear();
		self.current.clear();
	    }
	    _ =>
	    {
		let life = self.scopes.pop().unwrap(); // guaranteed
		life.keys()
		    .for_each(|id| {
			self.current.get_mut(id).map(|vec| vec.pop());
		    })
	    }
	}
    }
    
    pub fn get_var(&self, identifier: &Identifier) -> &Arc<Terminal>
    {
	match self.current.get(identifier)
	{
	    None => panic!("VAR NOT IN CONTEXT"),
	    Some(vec) => match vec.last()
	    {
		None => panic!("VAR NOT IN CONTEXT"),
		Some(ptr) => ptr
	    }
	}
    }
    pub fn get_var_mut(&mut self, identifier: &Identifier) -> &mut Arc<Terminal>
    {
	match self.current.get_mut(identifier)
	{
	    None => panic!("VAR NOT IN CONTEXT"),
	    Some(vec) => match vec.last_mut()
	    {
		None => panic!("VAR NOT IN CONTEXT"),
		Some(ptr) => ptr
	    }
	}
    }
}



pub struct Binder
{
    scopes: Vec<HashMap<Identifier, Type>>,
    current: HashMap<Identifier, Vec<Type>>
}

impl Binder
{
    pub fn new() -> Self
    {
	Self
	{
	    scopes: vec![],
	    current: HashMap::new(),
	}
    }

    pub fn push(&mut self, register: &VarsRegister) -> Result<(), String>
    {

	let mut scope = HashMap::new();

	for (id, expr) in register.get_vec().iter()
	{
	    let typ = expr.infer_type(self)?;
	    scope.insert((*id).clone(), typ.clone());
	    match self.current.get_mut(&id)
	    {
		None =>
		{
		    self.current.insert((*id).clone(), vec![typ.clone()]);
		},
		Some(vec) =>
		{
		    vec.push(typ.clone());
		}
		
	    }
	}
	self.scopes.push(scope);
	Ok(())
    }
    
    pub fn pop(&mut self)
    {
	match self.scopes.len()
	{
	    0 => panic!("EMPTY CONTEXT POPED"),
	    1 =>
	    {
		self.scopes.clear();
		self.current.clear();
	    }
	    _ =>
	    {
		let life = self.scopes.pop().unwrap(); // guaranteed
		life.keys()
		    .for_each(|id| {
			self.current.get_mut(id).map(|vec| vec.pop());
		    })
	    }
	}
    }

    pub fn check_var(&self, id: &Identifier) -> Result<Type, String>
    {
	match self.current.get(id)
	{
	    Some(vec) =>
		match vec.last()
	    {
		Some(typ) => Ok(typ.clone()),
		None => Err(format!("BINDING ERROR: CANNOT FIND VAR {} (died)", id))
	    },
	    None => Err(format!("BINDING ERROR: CANNOT FIND VAR {} (never defined)", id))
	}
    }

}
