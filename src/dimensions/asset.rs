use std::collections::HashMap;
use std::fmt;
use crate::dimensions::securityproperty::{SecurityProperty,SecurityPropertyValue,QualitativeValue};
use crate::dimensions::tag::Tag;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Asset {
  name : String,
  short_code : String,
  asset_type : String,
  is_critical : bool,
  pub critical_rationale : String,
  pub description : String,
  pub significance : String,
  pub tags : Vec<Tag>,
  pub environment_properties : HashMap<String,AssetEnvironmentProperties>
}

impl Asset {
  pub fn new(a_name : &String, s_code : &String, a_type : &String, i_c: bool) -> Asset {
    Asset{
      name : a_name.clone(), 
      short_code : s_code.clone(), 
      asset_type : a_type.clone(), 
      is_critical : i_c, 
      critical_rationale : "".to_string(), 
      description : "".to_string(), 
      significance : "".to_string(), 
      tags : Vec::<Tag>::new(),
      environment_properties : HashMap::<String,AssetEnvironmentProperties>::new()}  
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
    let tags = &self.tags.iter().map(|t| t.name.clone()).collect::<Vec<String>>().join(",");
    let props = &self.environment_properties.iter().map(|pt| pt.1.to_string()).collect::<Vec<String>>().join(",");
/*     let mut props = "".to_string();
    for pt in &self.environment_properties {
      props.push_str(&pt.1.to_string());
    } */
    write!(f,"Name: {}, Short code: {}, Type: {}, Tags: {}, Description: {}, Significance: {}, Properties: {}",self.name,self.short_code,self.asset_type,tags,self.description,self.significance,props)
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
