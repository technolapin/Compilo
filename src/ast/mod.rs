mod expression;
mod operators;
mod terminal;
mod display_impls; // for the pretty print (no structs)

pub use expression::*;
pub use operators::*;
pub use terminal::*;





/*
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


*/
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
pub struct VarsRegister(HashMap<Identifier, Expression>);
impl VarsRegister
{
    pub fn new() -> Self
    {
	Self(HashMap::new())
    }
    pub fn with_first(key: Identifier, val: Expression) -> Self
    {
	Self::new().with_added(key, val)
    }
    pub fn with_added(self, key: Identifier, val: Expression) -> Self
    {
	let mut hashmap = self.0;
	hashmap.insert(key, val);
	Self(hashmap)
    }
    pub fn random(depth: u32) -> Self
    {
	(0..(rand::random::<u32>()%6+1))
	    .fold(Self::new(),
		  |reg, _|
		  reg.with_added(Identifier::random(), *Expression::random(depth.max(1)-1)
		  )
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
