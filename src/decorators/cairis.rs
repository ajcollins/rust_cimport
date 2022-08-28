use std::{collections::HashMap,collections::HashSet};
use xml_oxide::{sax::StartElement,sax::EndElement};
use cairis_core::dimensions::projectsettings::ProjectSettings;
use cairis_core::dimensions::environment::Environment;
use crate::decorators::helpers::attributes_to_dict;
use crate::decorators::parsedecorator::ParseDecorator;
use crate::decorators::savedstate::SavedState;

#[derive(Clone)]
pub struct CairisHandler {
  in_background : bool,
  in_strategic_goals : bool,
  in_rich_picture : bool,
  in_scope : bool,
  in_entry : bool,
  in_definition : bool,
  in_remarks : bool,
  in_environment : bool,
  in_none: bool,
  in_low : bool,
  in_medium : bool,
  in_high : bool,
  p_settings : ProjectSettings,
  environments : Vec<Environment>,
  attr_dict : HashMap<String,String>
}

impl CairisHandler {
  pub fn new() -> CairisHandler {
    CairisHandler {
      in_background : false,
      in_strategic_goals : false,
      in_rich_picture : false,
      in_scope : false,
      in_entry : false,
      in_definition : false,
      in_remarks : false,
      in_environment : false,
      in_none : false,
      in_low : false,
      in_medium :  false,
      in_high : false,
      p_settings : ProjectSettings::new(&"".to_string()),
      environments : Vec::<Environment>::new(),
      attr_dict : HashMap::<String,String>::new()
    }
  }
}
impl ParseDecorator for CairisHandler {
  fn parse_start_element(&mut self, el: &StartElement) {
    if el.name == "project_settings" {
      attributes_to_dict(&mut self.attr_dict, &el,HashSet::from(["name"]));
      self.p_settings.name = self.attr_dict.get("name").unwrap().clone();
    }
    else if el.name == "background" {
      self.in_background = true;
    }
    else if el.name == "strategic_goals" {
      self.in_strategic_goals = true;
    }
    else if el.name == "rich_picture" {
      self.in_rich_picture = true;
    }
    else if el.name == "scope" {
      self.in_scope = true;
    }
    else if el.name == "entry" {
      attributes_to_dict(&mut self.attr_dict, &el,HashSet::from(["name"]));
      self.in_entry = true;
    }
    else if el.name == "definition" {
      self.in_definition = true;
    }
    else if el.name == "contributor" {
      attributes_to_dict(&mut self.attr_dict,&el,HashSet::from(["first_name","surname","affiliation","role"]));
      let first_name : &String = self.attr_dict.get("first_name").unwrap();
      let surname : &String = self.attr_dict.get("surname").unwrap();
      let affil : &String = self.attr_dict.get("affiliation").unwrap();
      let role : &String = self.attr_dict.get("role").unwrap();
      self.p_settings.contributors.push((first_name.clone(),surname.clone(),affil.clone(),role.clone()));
    }
    else if el.name == "revision" {
      attributes_to_dict(&mut self.attr_dict, &el,HashSet::from(["number","date"]));
    }
    else if el.name == "remarks" {
      self.in_remarks = true;
    }
    else if el.name == "environment" {
      attributes_to_dict(&mut self.attr_dict, &el,HashSet::from(["name","short_code"]));
      let new_env = Environment::new(self.attr_dict.get("name").unwrap(),self.attr_dict.get("short_code").unwrap());
      self.environments.push(new_env);
      self.in_environment = true;
    }
    else if el.name == "none" {
      self.in_none = true;
    }
    else if el.name == "low" {
      self.in_low = true;
    }
    else if el.name == "medium" {
      self.in_medium = true;
    }
    else if el.name == "high" {
      self.in_high = true;
    }
    else if el.name == "composite_properties" {
      attributes_to_dict(&mut self.attr_dict, &el,HashSet::from(["duplication","overriding_environment"]));
      let last_idx = self.environments.len() - 1;
      let comp_props = &mut self.environments[last_idx].environments;
      comp_props.update_property(self.attr_dict.get("duplication").unwrap());
      comp_props.overriding_environment_name = self.attr_dict.get("overriding_environment").unwrap_or(&"".to_string()).clone();
    }
    else if el.name == "sub_environment" {
      attributes_to_dict(&mut self.attr_dict, &el,HashSet::from(["name"]));
      let last_idx = self.environments.len() - 1;
      let comp_props = &mut self.environments[last_idx].environments;
      comp_props.add(&self.attr_dict.get("name").unwrap());
    }
  }

  fn parse_characters(&mut self, data : &str) {
    if self.in_background == true {
      self.p_settings.background = data.to_string();
      self.in_background = false;
    }
    else if self.in_strategic_goals == true {
      self.p_settings.strategic_goals = data.to_string();
      self.in_strategic_goals = false;
    }
    else if self.in_rich_picture == true {
      self.p_settings.rich_picture = data.to_string();
      self.in_rich_picture = false;
    }
    else if self.in_scope == true {
      self.p_settings.scope = data.to_string();
      self.in_scope = false;
    }
    else if self.in_entry == true && self.in_definition == true {
      self.p_settings.naming_conventions.insert(self.attr_dict.get("name").unwrap().clone(),data.to_string());
      self.in_definition = false;
    }
    else if self.in_remarks == true {
      let revision_no : &String = self.attr_dict.get("number").unwrap();
      let revision_date : &String = self.attr_dict.get("date").unwrap();
      self.p_settings.revisions.push((revision_no.clone(),revision_date.clone(),data.to_string()));
      self.in_remarks = false;
    }
    else if self.in_environment == true && self.in_definition == true {
      let last_idx = self.environments.len() - 1;
      self.environments[last_idx].definition = data.to_string();
      self.in_definition = false;
    }
    else if self.in_environment == true && self.in_none == true {
      let last_idx = self.environments.len() - 1;
      let env = &mut self.environments[last_idx];
      env.asset_values[0].description = data.to_string();
      env.asset_values[0].environment = env.name.clone();
      self.in_none = false;
    }
    else if self.in_environment == true && self.in_low == true {
      let last_idx = self.environments.len() - 1;
      let env = &mut self.environments[last_idx];
      env.asset_values[1].description = data.to_string();
      env.asset_values[1].environment = env.name.clone();
      self.in_low = false;
    }
    else if self.in_environment == true && self.in_medium == true {
      let last_idx = self.environments.len() - 1;
      let env = &mut self.environments[last_idx];
      env.asset_values[2].description = data.to_string();
      env.asset_values[2].environment = env.name.clone();
      self.in_medium = false;
    }
    else if self.in_environment == true && self.in_high == true {
      let last_idx = self.environments.len() - 1;
      let env = &mut self.environments[last_idx];
      env.asset_values[3].description = data.to_string();
      env.asset_values[3].environment = env.name.clone();
      self.in_high = false;
    }
  }

  fn parse_end_element(&mut self, el: &EndElement){
    if el.name == "entry" {
      self.in_entry = false;
    }
    else if el.name == "environment" {
      self.in_environment = false;
    }
  }

  fn save_state(&self, ss: &mut SavedState){
    ss.p_settings = Some(Box::new(self.p_settings.clone()));
    ss.environments = Some(Box::new(self.environments.clone()));
  }
}
