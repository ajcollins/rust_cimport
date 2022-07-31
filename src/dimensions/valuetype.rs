use std::fmt;

#[derive(Clone,PartialEq)]
pub struct ValueType {
  name : String,
  pub description: String
}

impl ValueType {
  pub fn new(vt_name : &String, vt_desc : &String) -> ValueType {
    ValueType{ name : vt_name.clone(), description: vt_desc.clone()}
  }
}

impl fmt::Display for ValueType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"Name: {}, Description: {}",self.name,self.description)
  }
}

#[test]
fn test_new_value_type() {
  let vt = ValueType::new(&"AVT".to_string(),&"XXX".to_string());
  assert_eq!(vt.name,"AVT".to_string());
  assert_eq!(vt.description,"XXX".to_string());
}
