#[derive(Debug, PartialEq, Clone)]
pub enum Unop
{
    Minus,
    Plus,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum IdopOne
{
    Assign,
    IncrBy,
    DecrBy,
    MulBy,
    DivBy,
    ModBy,
    AndBy,
    XorBy,
    OrBy
}
#[derive(Debug, PartialEq, Clone)]
pub enum IdopNone
{
    IncrPostfix,
    IncrPrefix,
    DecrPostfix,
    DecrPrefix,

}

impl Unop
{
    pub fn random() -> Self
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
    pub fn random() -> Self
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

impl IdopOne
{
    pub fn random() -> Self
    {
	use IdopOne::*;
	match rand::random::<u32>() % 9
	{
	    0 => IncrBy,
	    1 => DecrBy,
	    2 => MulBy,
	    3 => DivBy,
	    4 => ModBy,
	    5 => AndBy,
	    6 => XorBy,
	    7 => OrBy,
	    _ => Assign,
	}
    }
}


impl IdopNone
{
    pub fn random() -> Self
    {
	use IdopNone::*;
	match rand::random::<u32>() % 4
	{
	    0 => IncrPostfix,
	    1 => DecrPostfix,
	    2 => IncrPrefix,
	    _ => DecrPrefix,
	}
    }
}
