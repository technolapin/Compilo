#[derive(Debug, PartialEq)]
pub enum Terminal
{
    Int(u32),
    String(String),
    Nil,
}






impl Terminal
{
    pub fn random() -> Self
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

