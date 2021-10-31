use std::fmt;

pub struct DispFunc<D>
where
  D: fmt::Display,
{
  pub func: Box<dyn Fn(D) -> String>,
}

impl<D> DispFunc<D>
where
  D: fmt::Display,
{
  pub fn new(func: Box<dyn Fn(D) -> String>) -> DispFunc<D> {
    DispFunc { func }
  }
  pub(crate) fn eval(&self, param: D) -> String {
    (*self.func)(param)
  }
}
