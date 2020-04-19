#[derive(Debug, PartialEq, Clone)]
pub enum Unop
{
    Minus,
    Plus,
    Not,
    BitNot,
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
    LShift,
    RShift,
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
    OrBy,
    LShiftBy,
    RShiftBy,
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
	match rand::random::<u32>() % 4
	{
	    0 => Self::Minus,
	    1 => Self::Plus,
	    2 => Self::Not,
	    _ => Self::BitNot,
	}
    }
}
impl Binop
{
    pub fn random() -> Self
    {
	match rand::random::<u32>() % 17
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
	    15 => Self::LShift,
	    _ => Self::RShift,
	    
	}
    }
}

impl IdopOne
{
    pub fn random() -> Self
    {
	use IdopOne::*;
	match rand::random::<u32>() % 11
	{
	    0 => IncrBy,
	    1 => DecrBy,
	    2 => MulBy,
	    3 => DivBy,
	    4 => ModBy,
	    5 => AndBy,
	    6 => XorBy,
	    7 => OrBy,
	    8 => LShiftBy,
	    9 => RShiftBy,
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
