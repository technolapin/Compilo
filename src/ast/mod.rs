mod expression;
mod operators;
mod terminal;
mod display_impls; // for the pretty print (no structs)
mod seq;

pub use expression::*;
pub use operators::*;
pub use terminal::*;
pub use seq::Seq;




#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Identifier(String);

impl Identifier
{
    pub fn new(name: &str) -> Self
    {
	Self(String::from(name))
    }

    pub fn random() -> Self
    {
	use rand::{thread_rng, Rng};
	use rand::distributions::Alphanumeric;
	let mut s = String::from("a");
	let s2: String = thread_rng()
	    .sample_iter(&Alphanumeric)
	    .take(8)
	    .collect();
	s.push_str(s2.as_str());
	Self(s)
    }
}

use std::collections::HashMap;
#[derive(Debug, PartialEq, Clone)]
pub struct VarsRegister(HashMap<Identifier, Expression>, Vec<Identifier>);
impl VarsRegister
{
    pub fn new() -> Self
    {
	Self(HashMap::new(), Vec::new())
    }
    pub fn with_first(key: Identifier, val: Expression) -> Self
    {
	Self::new().with_added(key, val)
    }
    pub fn with_added(self, key: Identifier, val: Expression) -> Self
    {
	let mut hashmap = self.0;
	let mut vec = self.1;
	hashmap.insert(key.clone(), val);
	vec.push(key);
	Self(hashmap, vec)
    }
    /*
    pub fn merged(&self, other: &Self) -> Self
    {
	let mut hashmap = self.0.clone();
	other.iter()
	    .for_each(|(k, v)| {
		if hashmap.insert(
		    k.clone(),
		    v.clone()
		) == None
		{
		    self.push()
		};
	    });
	Self(hashmap)
    }
*/
    /*
    pub fn get_binding(&self, id: &Identifier, scope: &Self) -> Result<Expression, String>
    {
	match self.0.get(id)
	{
	    None => Err(format!("BINDING ERROR: identifier {} out of scope", id)),
	    Some(exp) => Ok(Expression::Binding(
		exp.bind(scope)))
	}
    }
*/

    pub fn random(depth: u32) -> Self
    {
	(0..(rand::random::<u32>()%6+1))
	    .fold(Self::new(),
		  |reg, _|
		  reg.with_added(Identifier::random(), *Expression::random(depth.max(1)-1)
		  )
	    )
    }

    pub fn get_vec(&self) -> Vec<(Identifier, Expression)>
    {
	self.1.iter().map(|key| (key.clone(), self.0.get(key).unwrap().clone())).collect()
    }

    pub fn propagate<F>(&self, lambda: &F) -> Self
    where
	F: Fn(&Expression) -> Expression
    {
	Self(
	    self.0.iter()
		.map(|(id, expr)| (id.clone(), expr.propagate(lambda))).collect(),
	    self.1.clone()
		
	)
    }    
}


#[derive(Debug, PartialEq, Clone)]
pub enum Primitive
{
    Print
}

impl Primitive
{
    fn random() -> Self
    {
	Self::Print
    }
}
