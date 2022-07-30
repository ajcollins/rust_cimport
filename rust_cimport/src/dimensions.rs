use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq)]
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


pub struct ProjectSettings {
  pub name:  String,
  pub background : String,
  pub strategic_goals : String,
  pub scope : String,
  pub naming_conventions : HashMap<String,String>,
  pub contributors :  Vec<(String,String,String,String)>,
  pub revisions : Vec<(String,String,String)>,
  pub rich_picture : String
}

impl ProjectSettings {
  pub fn new(proj_name : &String) -> ProjectSettings {
    ProjectSettings{name : proj_name.clone(), background : "".to_string(), strategic_goals : "".to_string(), scope : "".to_string(), naming_conventions : HashMap::new(), contributors : Vec::new(), revisions : Vec::new(), rich_picture : "".to_string()}
  }
}

impl fmt::Display for ProjectSettings {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut x = format!("Name: {},\n Background: {},\n Strategic goals: {},\n Scope: {},\n Rich picture: {},\n", self.name, self.background, self.strategic_goals, self.scope, self.rich_picture);
    for (name,value) in self.naming_conventions.iter() {
      x.push_str(format!("Name: {}, Value: {}\n",name,value).as_str());
    }
    for c in &self.contributors {
      x.push_str(format!("Firstname: {}, Surname: {}, Affiliation: {}, Role: {}\n",&c.0,&c.1,&c.2,&c.3).as_str());
    }
    for rev in &self.revisions {
      x.push_str(format!("Revision: {}, Date: {}, Remarks: {}\n",&rev.0,&rev.1,&rev.2).as_str());
    }
    write!(f,"{}",x)
  }
}

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

 
#[derive(Clone,PartialEq,Debug)]
enum SecurityProperty {
  Confidentiality = 0,
  Integrity = 1,
  Availability = 2,
  Accountability = 3,
  Anonymity = 4,
  Pseudonymity = 5,
  Unlinkability = 6,
  Unobservability = 7
}

#[derive(Clone,PartialEq,Debug)]
enum QualitativeValue {
  None = 0,
  Low = 1,
  Medium = 2,
  High = 3
}

struct SecurityPropertyValue {
  name : SecurityProperty,
  value : QualitativeValue,
  rationale : String
}

impl SecurityPropertyValue {

  pub fn new(sp : &str, v : &str, r: &str) -> SecurityPropertyValue {
    SecurityPropertyValue{ 
      name : 
        match sp {
          "confidentiality" => SecurityProperty::Confidentiality,
          "integrity" => SecurityProperty::Integrity,
          "availability" => SecurityProperty::Availability,
          "accountability" => SecurityProperty::Accountability,
          "anonymity" => SecurityProperty::Anonymity,
          "pseudonymity" => SecurityProperty::Pseudonymity,
          "unlinkability" => SecurityProperty::Unlinkability,
          "unobservability" => SecurityProperty::Unobservability,
          &_ => panic!("{} is not a property value",sp)
        },
      value : 
        match v {
          "None" => QualitativeValue::None,
          "Low" => QualitativeValue::Low,
          "Medium" => QualitativeValue::Medium,
          "High" => QualitativeValue::High,
          &_ => QualitativeValue::None
        },
      rationale: r.to_string()
    }
  }
  

}

impl fmt::Display for SecurityPropertyValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"Property: {}, Value: {}, Rationale: {} ",
      match self.name {
        SecurityProperty::Confidentiality => "Confidentiality".to_string(),
        SecurityProperty::Integrity => "Integrity".to_string(),
        SecurityProperty::Availability => "Availability".to_string(),
        SecurityProperty::Accountability => "Accountability".to_string(),
        SecurityProperty::Anonymity => "Anonymity".to_string(),
        SecurityProperty::Pseudonymity => "Pseudonymity".to_string(),
        SecurityProperty::Unlinkability => "Unlinkability".to_string(),
        SecurityProperty::Unobservability => "Unobservability".to_string()
      },
      match self.value {
        QualitativeValue::None => "None".to_string(),
        QualitativeValue::Low => "Low".to_string(),
        QualitativeValue::Medium => "Medium".to_string(),
        QualitativeValue::High => "High".to_string()
      },self.rationale)
  }
}
#[test]
fn test_new_security_property() {
  let sp = SecurityPropertyValue::new("confidentiality","None","None");
  assert_eq!(sp.name,SecurityProperty::Confidentiality);
}

#[test]
#[should_panic]
fn test_new_security_property_panics() {
  SecurityPropertyValue::new("foo","None","None");
}
 
pub struct AssetEnvironmentProperties {
  pub name : String,
  properties : [SecurityPropertyValue ; 8]
}

impl AssetEnvironmentProperties {
  pub fn new(env_name: &str) -> AssetEnvironmentProperties {
    AssetEnvironmentProperties{
      name : env_name.to_string(), 
      properties : [
        SecurityPropertyValue::new("confidentiality","None","None"),
        SecurityPropertyValue::new("integrity","None","None"),
        SecurityPropertyValue::new("availability","None","None"),
        SecurityPropertyValue::new("accountability","None","None"),
        SecurityPropertyValue::new("anonymity","None","None"),
        SecurityPropertyValue::new("pseudonymity","None","None"),
        SecurityPropertyValue::new("unlinkability","None","None"),
        SecurityPropertyValue::new("unobservability","None","None")
      ]
    }
  }
  pub fn update(&mut self, p_name : &str, p_value: &str, p_rationale : &str) {
    let p_index =
      match p_name {
        "confidentiality" => SecurityProperty::Confidentiality as usize,
        "integrity" => SecurityProperty::Integrity as usize,
        "availability" => SecurityProperty::Availability as usize,
        "accountability" => SecurityProperty::Accountability as usize,
        "anonymity" => SecurityProperty::Anonymity as usize,
        "pseudonymity" => SecurityProperty::Pseudonymity as usize,
        "unlinkability" => SecurityProperty::Unlinkability as usize,
        "unobservability" => SecurityProperty::Unobservability as usize,
        &_ => panic!("{} is not a property value",p_name)
      };
    let mut prop = &mut self.properties[p_index];
    
    prop.value = 
      match p_value {
        "None" => QualitativeValue::None,
        "Low" => QualitativeValue::Low,
        "Medium" => QualitativeValue::Medium,
        "High" => QualitativeValue::High,
        &_ => panic!("{} is not a qualitative value",p_value)
      };
    prop.rationale = p_rationale.to_string();
  }
}

impl fmt::Display for AssetEnvironmentProperties {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut x : String = format!("Environment: {}",self.name);
    for e in &self.properties {
      x.push_str(e.to_string().as_str());
    }
    write!(f,"{}",x)
  }
}
#[test]
fn test_new_asset_environment_properties() {
  let aep = AssetEnvironmentProperties::new("Default");
  assert_eq!(aep.name,"Default".to_string());
  assert_eq!(aep.properties[0].name,SecurityProperty::Confidentiality);
}

#[test]
fn test_update_asset_environment_properties() {
  let mut aep = AssetEnvironmentProperties::new("Default");
  aep.update("integrity", "Medium", "TBC");
  let prop = &aep.properties[1];
  assert_eq!(prop.name,SecurityProperty::Integrity);
  assert_eq!(prop.value,QualitativeValue::Medium);
  assert_eq!(prop.rationale,"TBC".to_string());
}

pub struct Tag {
  name : String
}

impl Tag {
  pub fn new(t_name: &String) -> Tag {
    Tag{ name : t_name.clone()}
  }
}

pub struct Asset {
  name : String,
  short_code : String,
  asset_type : String,
  is_critical : bool,
  pub critical_rationale : String,
  pub description : String,
  pub significance : String,
  pub environment_properties : HashMap<String,AssetEnvironmentProperties>
}

impl Asset {
  pub fn new(a_name : &String, s_code : &String, a_type : &String, i_c: bool) -> Asset {
    Asset{name : a_name.clone(), short_code : s_code.clone(), asset_type : a_type.clone(), is_critical : i_c, critical_rationale : "".to_string(), description : "".to_string(), significance : "".to_string(), environment_properties : HashMap::<String,AssetEnvironmentProperties>::new()}  
  }

  pub fn update(&mut self, a_attr : &str, a_value : &str) {
    match a_attr {
      "critical_rationale" => {self.critical_rationale = a_value.to_string()},
      "significance" => {self.significance = a_value.to_string()},
      "description" => {self.description = a_value.to_string()},
      &_ => panic!("{} is an unknown asset attribute",a_attr)
    };
  }

  pub fn add_environment(&mut self, env_name: &String) {
    self.environment_properties.insert(env_name.clone(), AssetEnvironmentProperties::new(env_name));
  }

  pub fn update_security_property(&mut self,env_name : &String, p_name: &str, p_value: &str, p_rationale : &str) {
    if let Some(x) = self.environment_properties.get_mut(env_name) {
      x.update(p_name,p_value,p_rationale);
    } 
  }
}

impl fmt::Display for Asset {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut x = "".to_string();
    for pt in &self.environment_properties {
      x.push_str(&pt.1.to_string());
    }
    write!(f,"Name: {}, Short code: {}, Type: {}, Description: {}, Significance: {}, Properties: {}",self.name,self.short_code,self.asset_type,self.description,self.significance,x)
  }
}

#[test]
fn test_create_asset() {
  let a = Asset::new(&"An asset".to_string(),&"SC".to_string(),&"Information".to_string(),false);
  assert_eq!(a.name,"An asset".to_string());
  assert_eq!(a.short_code,"SC".to_string());
  assert_eq!(a.asset_type,"Information".to_string());
  assert_eq!(a.is_critical,false);
  assert_eq!(a.description,"".to_string());
  assert_eq!(a.significance,"".to_string());
  assert_eq!(a.critical_rationale,"".to_string());
}

#[test]
fn test_update_asset() {
  let mut a = Asset::new(&"An asset".to_string(),&"SC".to_string(),&"Information".to_string(),false);
  a.update("critical_rationale","cr TBC");
  a.update("description","description TBC");
  a.update("significance","significance TBC");
  assert_eq!(a.description,"description TBC".to_string());
  assert_eq!(a.significance,"significance TBC".to_string());
  assert_eq!(a.critical_rationale,"cr TBC".to_string());
}

#[test]
fn test_asset_add_environment() {
  let mut a = Asset::new(&"An asset".to_string(),&"SC".to_string(),&"Information".to_string(),false);
  a.add_environment(&"Default".to_string());
  assert_eq!(a.environment_properties.contains_key(&"Default".to_string()),true);
  if let Some(x) = a.environment_properties.get_mut(&"Default".to_string()) {
    assert_eq!(x.name,"Default".to_string());
    assert_eq!(x.properties[SecurityProperty::Confidentiality as usize].name,SecurityProperty::Confidentiality);
    assert_eq!(x.properties[SecurityProperty::Confidentiality as usize].value,QualitativeValue::None);
    assert_eq!(x.properties[SecurityProperty::Confidentiality as usize].rationale,"None".to_string());
    assert_eq!(x.properties[SecurityProperty::Integrity as usize].name,SecurityProperty::Integrity);
    assert_eq!(x.properties[SecurityProperty::Integrity as usize].value,QualitativeValue::None);
    assert_eq!(x.properties[SecurityProperty::Integrity as usize].rationale,"None".to_string());
  }
}

#[test]
fn test_asset_update_security_property() {
  let mut a = Asset::new(&"An asset".to_string(),&"SC".to_string(),&"Information".to_string(),false);
  a.add_environment(&"Default".to_string());
  a.update_security_property(&"Default".to_string(), "confidentiality", "Low", "Low C TBC");
  a.update_security_property(&"Default".to_string(), "integrity", "High", "High I TBC");
  if let Some(x) = a.environment_properties.get_mut(&"Default".to_string()) {
    assert_eq!(x.name,"Default".to_string());
    assert_eq!(x.properties[SecurityProperty::Confidentiality as usize].name,SecurityProperty::Confidentiality);
    assert_eq!(x.properties[SecurityProperty::Confidentiality as usize].value,QualitativeValue::Low);
    assert_eq!(x.properties[SecurityProperty::Confidentiality as usize].rationale,"Low C TBC".to_string());
    assert_eq!(x.properties[SecurityProperty::Integrity as usize].name,SecurityProperty::Integrity);
    assert_eq!(x.properties[SecurityProperty::Integrity as usize].value,QualitativeValue::High);
    assert_eq!(x.properties[SecurityProperty::Integrity as usize].rationale,"High I TBC".to_string());
  }

}