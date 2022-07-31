use std::{collections::HashMap,collections::HashSet};
use xml_oxide::{sax::StartElement,sax::EndElement};
use crate::dimensions::projectsettings::ProjectSettings;
use crate::decorators::helpers::attributes_to_dict;
use crate::decorators::parsedecorator::ParseDecorator;
use crate::decorators::savedstate::SavedState;

#[derive(Clone)]
pub struct ProjectSettingsHandler {
  in_background : bool,
  in_strategic_goals : bool,
  in_rich_picture : bool,
  in_scope : bool,
  in_entry : bool,
  in_definition : bool,
  in_remarks : bool,
  p_settings : ProjectSettings,
  attr_dict : HashMap<String,String>
}

impl ProjectSettingsHandler {
  pub fn new() -> ProjectSettingsHandler {
    ProjectSettingsHandler {
      in_background : false,
      in_strategic_goals : false,
      in_rich_picture : false,
      in_scope : false,
      in_entry : false,
      in_definition : false,
      in_remarks : false,
      p_settings : ProjectSettings::new(&"".to_string()),
      attr_dict : HashMap::<String,String>::new()
    }
  }
}
impl ParseDecorator for ProjectSettingsHandler {
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
  }

  fn parse_end_element(&mut self, _el: &EndElement){
    if self.in_entry == true {
      self.in_entry = false;
    }
  }

  fn save_state(&self, ss: &mut SavedState){
    ss.p_settings = Some(Box::new(self.p_settings.clone()));
  }
}
