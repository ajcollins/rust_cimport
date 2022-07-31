use std::fmt;

enum RoleType {
  Stakeholder,
  Attacker,
  DataController,
  DataProcessor,
  DataSubject,
  Machine
}

impl fmt::Display for RoleType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      RoleType::Stakeholder => write!(f,"Stakeholder"),
      RoleType::Attacker => write!(f,"Attacker"),
      RoleType::DataController => write!(f,"Data Controller"),
      RoleType::DataProcessor => write!(f,"Data Processor"),
      RoleType::DataSubject => write!(f,"Data Subject"),
      RoleType::Machine => write!(f,"Machine")
    }
  }
}

pub struct Role {
  name : String,
  role_type : RoleType,
  short_code : String,
  pub description : String    
}

impl Role {
  pub fn new(role_name: &String, r_type: &String, s_code: &String, r_desc: &String) -> Role {
    Role{
      name : role_name.clone(), 
      role_type : 
        match r_type.as_str() {
          "Stakeholder" => RoleType::Stakeholder,
          "Attacker" => RoleType::Attacker,
          "Data Controller" => RoleType::DataController,
          "Data Processor" => RoleType::DataProcessor,
          "Data Subject" => RoleType::DataSubject,
          "Machine" => RoleType::Machine,
          _ => panic!("{} is an invalid role type",r_type)
        }, 
      short_code : s_code.clone(), 
      description : r_desc.clone()}
  }
}

impl fmt::Display for Role {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"Name: {}, Type: {}, Short code: {}, Description: {}",self.name,self.role_type.to_string(),self.short_code,self.description)
  }
}

#[test]
pub fn test_new_role() {
  let r = Role::new(&"A role".to_string(),&"Stakeholder".to_string(),&"AR".to_string(),&"A role description".to_string());
  assert_eq!(r.name,"A role".to_string()); 
}
