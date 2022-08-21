use std::fmt;
use crate::db::mysql::initialise_db;

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
  let vt = ValueType::new(&"AVT".to_string(),&"XXX".to_string(),&"vulnerability_type".to_string());
  assert_eq!(vt.name,"AVT".to_string());
  assert_eq!(vt.description,"XXX".to_string());
  assert_eq!(vt.description,"XXX".to_string());
  assert_eq!(vt.vt_type,"vulnerability_type".to_string());
  assert_eq!(vt.score,0);
  assert_eq!(vt.rationale,"".to_string());
  assert_eq!(vt.environment,"".to_string());
}

#[test]
pub fn test_value_types() {
  let mut p = initialise_db();

  let mut no_vts = p.get_value_types(&"vulnerability_type".to_string(),&"".to_string());
  assert_eq!(no_vts.len(),0);

  let ivt = ValueType::new(&"AVT".to_string(),&"XXX".to_string(),&"vulnerability_type".to_string());
  p.add_value_type(&ivt);

  let ovts = p.get_value_types(&"vulnerability_type".to_string(),&"".to_string());
  let ovt = &ovts[0]; 
  assert_eq!(ivt.name,ovt.name);
  assert_eq!(ivt.description,ovt.description);
  assert_eq!(ivt.vt_type,ovt.vt_type);
  assert_eq!(-1,ovt.score);
  assert_eq!(ivt.rationale,ovt.rationale);
  assert_eq!(ivt.environment,ovt.environment);

  p.delete_vulnerability_type(ovt.id);
  no_vts = p.get_value_types(&"vulnerability_type".to_string(),&"".to_string());
  assert_eq!(no_vts.len(),0);

}