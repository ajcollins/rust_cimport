use std::{collections::HashMap,collections::HashSet};
use xml_oxide::sax::StartElement;
use crate::dimensions::valuetype::ValueType;
use crate::decorators::helpers::attributes_to_dict;
use crate::decorators::parsedecorator::ParseDecorator;
use crate::decorators::savedstate::SavedState;

pub struct TVTypesHandler {
  in_description : bool,
  tv_types : Vec<ValueType>,
  attr_dict : HashMap<String,String>
}

impl TVTypesHandler {
  pub fn new() -> TVTypesHandler {
    TVTypesHandler {
      in_description : false,
      tv_types : Vec::<ValueType>::new(),
      attr_dict : HashMap::<String,String>::new()
    }
  }
}
impl ParseDecorator for TVTypesHandler {
  fn parse_start_element(&mut self, el: &StartElement) {
    if el.name == "vulnerability_type" || el.name == "threat_type" {
      attributes_to_dict(&mut self.attr_dict, &el,HashSet::from(["name"]));
      let vt_name = el.name.to_string();
      self.tv_types.push( ValueType::new(self.attr_dict.get("name").unwrap(),&"".to_string(),&vt_name));
    }
    else if el.name == "description" {
      self.in_description = true;
    }
  }
  fn parse_characters(&mut self, data : &str) {
    if self.in_description == true {
      let last_idx = self.tv_types.len() - 1;
      self.tv_types[last_idx].description = data.to_string();   
      self.in_description = false;
    }
  }

  fn save_state(&self, ss: &mut SavedState){
    ss.tv_types = Some(Box::new(self.tv_types.clone()));
  }
}
