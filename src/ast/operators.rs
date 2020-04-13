#[derive(Debug, PartialEq, Clone)]
pub enum Unop
{
    Increment,
    Decrement,
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
