use super::Type;

#[derive(Debug, PartialEq, Clone)]
pub enum Terminal
{
    Int(i32),
    String(String),
    Nil,
    Bool(bool),
}





impl Terminal
{
    pub fn random() -> Self
    {
	use rand::{thread_rng, Rng};
	use rand::distributions::Alphanumeric;
	match rand::random::<u32>() % 4
	{
	    0 => Terminal::Int(rand::random::<i32>().abs()),
	    1 => {
		Terminal::String(thread_rng()
				 .sample_iter(&Alphanumeric)
				 .take(30)
				 .collect())
	    },
	    2 => Terminal::Nil,
	    3 => Terminal::Bool(rand::random::<bool>()),
	    _ => unreachable!()
	}
    }
    pub fn infer_type(&self) -> Result<Type, String>
    {
	Ok(match self
	{
	    Self::Nil => Type::Nil,
	    Self::Int(_) => Type::Int,
	    Self::String(_) => Type::String,
	    Self::Bool(_) => Type::Bool,
	})
    }
}

