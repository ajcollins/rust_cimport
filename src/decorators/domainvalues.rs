use std::{collections::HashMap,collections::HashSet};
use xml_oxide::sax::StartElement;
use cairis_core::dimensions::valuetype::ValueType;
use crate::decorators::helpers::attributes_to_dict;
use crate::decorators::parsedecorator::ParseDecorator;
use crate::decorators::savedstate::SavedState;

pub struct DomainValuesHandler {
  in_description : bool,
  dv_types : Vec<ValueType>,
  attr_dict : HashMap<String,String>
}

impl DomainValuesHandler {
  pub fn new() -> DomainValuesHandler {
    DomainValuesHandler {
      in_description : false,
      dv_types : Vec::<ValueType>::new(),
      attr_dict : HashMap::<String,String>::new()
    }
  }
}
impl ParseDecorator for DomainValuesHandler {
  fn parse_start_element(&mut self, el: &StartElement) {
    if el.name == "threat_value" || el.name == "risk_value" || el.name == "countermeasure_value" || el.name == "severity_value" || el.name == "likelihood_value" {
      attributes_to_dict(&mut self.attr_dict, &el,HashSet::from(["name"]));
      let vt_name = match el.name {
        "capability_value" => "capability".to_string(),
        "motivation_value" => "motivation".to_string(),
        "risk_value" => "risk_class".to_string(),
        "severity_value" => "severity".to_string(),
        "likelihood_value" => "likelihood".to_string(),
        &_ => el.name.to_string()
      };
      self.dv_types.push( ValueType::new(self.attr_dict.get("name").unwrap(),&"".to_string(),&vt_name));
    }
    else if el.name == "description" {
      self.in_description = true;
    }
  }
  fn parse_characters(&mut self, data : &str) {
    if self.in_description == true {
      let last_idx = self.dv_types.len() - 1;
      self.dv_types[last_idx].description = data.to_string();   
      self.in_description = false;
    }
  }

  fn save_state(&self, ss: &mut SavedState){
    ss.dv_types = Some(Box::new(self.dv_types.clone()));
  }
}
