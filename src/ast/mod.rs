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
#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Identifier(String);

impl Identifier
{
    pub fn new(name: &str) -> Self
    {
	Self(String::from(name))
    }
}

use std::collections::HashMap;
#[derive(Debug, PartialEq)]
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
}


#[derive(Debug, PartialEq)]
pub enum Primitive
{
    Print
}

