use std::fmt;

#[derive(Clone,PartialEq,Debug)]
pub enum SecurityProperty {
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
pub enum QualitativeValue {
  None = 0,
  Low = 1,
  Medium = 2,
  High = 3
}

#[derive(Clone)]
pub struct SecurityPropertyValue {
  pub name : SecurityProperty,
  pub value : QualitativeValue,
  pub rationale : String
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
 
