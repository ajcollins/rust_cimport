use std::fmt;

#[derive(Clone,PartialEq)]
pub struct Tag {
  pub name : String
}

impl Tag {
  pub fn new(t_name: &String) -> Tag {
    Tag{ name : t_name.clone()}
  }
}

impl fmt::Display for Tag {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"Tag: {}",self.name)
  }
}
