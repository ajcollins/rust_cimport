use std::fmt;

#[derive(Clone,PartialEq)]
pub struct ValueType {
  pub id : i128,
  pub name : String,
  pub description: String,
  pub vt_type : String,
  pub score : i128,
  pub rationale : String,
  pub environment : String
}

impl ValueType {
  pub fn new(vt_name : &String, vt_desc : &String,v_t : &String) -> ValueType {
    ValueType{ id: -1, name : vt_name.clone(), description: vt_desc.clone(), vt_type : v_t.clone(), score : 0, rationale : "".to_string(), environment : "".to_string()}
  }
}

impl fmt::Display for ValueType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"Name: {}, Description: {}, Type : {}, Score: {}, Rationale : {}, Environment : {}",self.name,self.description,self.vt_type,self.score,self.rationale,self.environment)
  }
}

#[test]
fn test_new_value_type() {
  let vt = ValueType::new(&"AVT".to_string(),&"XXX".to_string(),&"threat_type".to_string());
  assert_eq!(vt.name,"AVT".to_string());
  assert_eq!(vt.description,"XXX".to_string());
  assert_eq!(vt.description,"XXX".to_string());
  assert_eq!(vt.vt_type,"threat_type".to_string());
  assert_eq!(vt.score,0);
  assert_eq!(vt.rationale,"".to_string());
  assert_eq!(vt.environment,"".to_string());
}