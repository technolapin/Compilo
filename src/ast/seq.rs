use super::*;

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

    pub fn random(depth: u32) -> Self
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

    pub fn infer_type(&self, binder: &mut Binder) -> Result<Type, String>
    {
	let (oks, errs): (Vec<_>, Vec<_>) = self.0.iter()
	    .map(|expr| expr.infer_type(binder))
	    .partition(|res| res.is_ok());
	if let Some(err) = errs.last()
	{
	    return err.clone();
	};
	oks.last().expect("EMPTY SEQ NOT SUPPORTED").clone()
    }

    /// Bind and typecheck
    pub fn check(&self) -> Result<Type, String>
    {
	self.infer_type(&mut Binder::new())
    }
    
    pub fn merge(&self, other: &Self) -> Self
    {
	let mut new = self.0.clone();
	new.extend_from_slice(&other.0);
	Self(new)
    }

    pub fn desugar(&self) -> Self
    {
	Self(
	    self.0.iter().map(|expr| expr
			      .desugar_for()
			      .desugar_idops()
			      .desugar_nil_if()).collect()
	)
    }
    pub fn reduce(&self, context: &mut Context) -> Terminal
    {
	self.0.iter()
	    .fold(Terminal::Nil, |_, expr| expr.reduce(context))
    }
    pub fn propagate<F>(&self, lambda: &F) -> Self
    where
	F: Fn(&Expression) -> Expression
    {
	Self (
	    self.0.iter()
		.map(|expr| lambda(expr))
		.collect::<Vec<Expression>>()
	)
    }

    pub fn run(&self) -> Result<Terminal, String>
    {
	self.infer_type(&mut Binder::new())?;
	let desugar = self.desugar();
	Ok(desugar.reduce(&mut Context::new()))
    }
    
}
