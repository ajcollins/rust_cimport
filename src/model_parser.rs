use std::{collections::HashMap,collections::HashSet,fs::File, path::PathBuf};
use xml_oxide::{sax::parser::Parser, sax::Event, sax::StartElement};

use crate::dimensions::{ProjectSettings,ValueType,Role,Asset,AssetEnvironmentProperties,Tag};

pub struct ModelParser {
  in_text : u8,
  in_rationale :u8,
  in_remarks : u8,
  in_significance : u8,
  in_critical_rationale : u8,
  in_background : u8,
  in_strategic_goals : u8,
  in_scope : u8,
  in_entry : u8,
  in_revision : u8,
  in_role : u8,
  in_value_type : u8,
  in_asset : u8,
  in_security_property : u8,
  pub p_settings : Option<Box<ProjectSettings>>,
  pub tv_types : Option<Box<Vec::<ValueType>>>,
  pub roles : Option<Box<Vec::<Role>>>,
  pub assets : Vec<Asset>,
  attr_dict : HashMap<String,String>
}

impl ModelParser {
  pub fn new() -> ModelParser {
    ModelParser {
      in_text : 0,
      in_rationale : 0,
      in_remarks : 0,
      in_significance : 0,
      in_critical_rationale : 0,
      in_background : 0,
      in_strategic_goals : 0,
      in_scope : 0,
      in_entry : 0,
      in_revision : 0,
      in_role : 0,
      in_value_type : 0,
      in_asset : 0,
      in_security_property : 0,
      p_settings : None,
      tv_types : None,
      roles : None,
      assets : Vec::<Asset>::new(),
      attr_dict : HashMap::<String,String>::new()
    }
  }

  fn attributes_to_dict(&mut self, el: &StartElement, attr_names : HashSet::<&str>) {
    self.attr_dict.clear();
    for el_attr in el.attributes() {
      if attr_names.contains(el_attr.name) {
        self.attr_dict.insert(el_attr.name.to_string(),el_attr.value.to_string());
      }
    }
  }

  pub fn parse(&mut self, model_file : &PathBuf) {

//    let f = File::open(&model_file.into_os_string().into_string()).unwrap();
    let f = File::open("./test.xml").unwrap();
    let mut p = Parser::from_reader(f);

    loop {
      let res = p.read_event();

      match res {
        Ok(event) => match event {
          Event::StartDocument => {}
          Event::EndDocument => {
            break;
          }
          Event::StartElement(el) => {
            if !el.is_empty {
              if el.name == "tvtypes" {
                self.tv_types = Some(Box::new(Vec::<ValueType>::new()));
              }
              else if el.name == "domainvalues" && self.tv_types == None {
                self.tv_types = Some(Box::new(Vec::<ValueType>::new()));
              }
              else if el.name == "vulnerability_type" || el.name == "threat_type" || el.name == "threat_value" || el.name == "risk_value" || el.name == "countermeasure_value" || el.name == "severity_value" || el.name == "likelihood_value" {
                self.attributes_to_dict(&el,HashSet::from(["name"]));
                if let Some(x) = &mut self.tv_types {
                  x.push( ValueType::new(self.attr_dict.get("name").unwrap(),&"".to_string()));
                  self.in_value_type = 1;
                }
                else {
                  panic!("No TVTypes vector defined");
                }
              }
              else if el.name == "description" {
                self.in_text = 1;
              }
              else if el.name == "definition" {
                self.in_text = 1;
              }
              else if el.name == "rationale" {
                self.in_rationale = 1;
              }
              else if el.name == "significance" {
                self.in_significance = 1;
              }
              else if el.name == "project_settings" {
                self.attributes_to_dict(&el,HashSet::from(["name"]));
                self.p_settings = Some(Box::new(ProjectSettings::new(self.attr_dict.get("name").unwrap())));
              }
              else if el.name == "background" || el.name == "definition" {
                self.in_background = 1;
              }
              else if el.name == "remarks" {
                self.in_remarks = 1;
              }
              else if el.name == "strategic_goals" {
                self.in_strategic_goals = 1;
              }
              else if el.name == "scope" {
                self.in_scope = 1;
              }
              else if el.name == "entry" {
                self.attributes_to_dict(&el,HashSet::from(["name"]));
                self.in_entry = 1;
              }
              else if el.name == "contributor" {
                self.attributes_to_dict(&el,HashSet::from(["first_name","surname","affiliation","role"]));
                let first_name : &String = self.attr_dict.get("first_name").unwrap();
                let surname : &String = self.attr_dict.get("surname").unwrap();
                let affil : &String = self.attr_dict.get("affiliation").unwrap();
                let role : &String = self.attr_dict.get("role").unwrap();

                match &mut self.p_settings {
                  Some(x) => {
                    x.contributors.push((first_name.clone(),surname.clone(),affil.clone(),role.clone()));
                  }
                  None => {}
                }
              }
              else if el.name == "revision" {
                self.attributes_to_dict(&el,HashSet::from(["number","date"]));
                self.in_revision = 1;
              }
              else if el.name == "role" {
                self.attributes_to_dict(&el,HashSet::from(["name","type","short_code"]));
                let new_role = Role::new(self.attr_dict.get("name").unwrap(),self.attr_dict.get("type").unwrap(),self.attr_dict.get("short_code").unwrap(),&"".to_string());

                match &mut self.roles {
                  Some(x) => { x.push(new_role);}
                  None => {self.roles = Some(Box::new(vec![new_role]));}
                }
                self.in_role = 1;
              }
              else if el.name == "security_property" {
                self.attributes_to_dict(&el,HashSet::from(["environment","property","value"]));
                let env_name : &String = self.attr_dict.get("environment").unwrap();
                let last_idx = self.assets.len() - 1;
                let env_props = &mut self.assets[last_idx].environment_properties;
                if !env_props.contains_key(env_name) {
                  env_props.insert(env_name.clone(),AssetEnvironmentProperties::new(&env_name));
                }
                self.in_security_property = 1;
              }
              else if el.name == "tag" {
                self.attributes_to_dict(&el,HashSet::from(["name"]));
                if self.in_asset == 1 {
                  let last_idx = self.assets.len() - 1;
                  self.assets[last_idx].tags.push(Tag::new(&self.attr_dict.get("name").unwrap().clone()));   
                }
              }
              else if el.name == "asset" {
                self.attributes_to_dict(&el,HashSet::from(["name","short_code","type","is_critical"]));
                let mut critical_flag = false;
                if self.attr_dict.get("is_critical").unwrap() == "1" {
                  critical_flag = true;
                }
                self.assets.push( Asset::new(
                  self.attr_dict.get("name").unwrap(),
                  self.attr_dict.get("short_code").unwrap(),
                  self.attr_dict.get("type").unwrap(),
                  critical_flag));
                self.in_asset = 1;
              }
            }
          }
          Event::EndElement(el) => {
            if el.name == "feed" {
              break;
            }
            else if el.name == "vulnerability_type" || el.name == "threat_type" || el.name == "threat_value" || el.name == "risk_value" || el.name == "countermeasure_value" || el.name == "severity_value" || el.name == "likelihood_value" {
              self.in_value_type = 0;
            }
            else if el.name == "revision" {
              self.in_revision = 0;
            }
            else if el.name == "entry" {
              self.in_entry = 0;
            }
            else if el.name == "role" {
              self.in_role = 0;
            }
            else if el.name == "role" {
              self.in_asset = 0;
            }
            else if el.name == "security_property" {
              self.in_security_property = 0;
            }
          }
          Event::Characters(data) => {
            if self.in_background == 1 {
              match &mut self.p_settings {
                Some(x) => {
                  x.background = data.to_string();
                }
                None => {}
              }
              self.in_background = 0;
            }
            else if self.in_strategic_goals == 1 {
              match &mut self.p_settings {
                Some(x) => {
                  x.strategic_goals = data.to_string();
                }
                None => {}
              }
              self.in_strategic_goals = 0;
            }
            else if self.in_scope == 1 {
              match &mut self.p_settings {
                Some(x) => {
                  x.scope = data.to_string();
                }
                None => {}
              }
              self.in_scope = 0;
            }
            else if self.in_entry == 1 && self.in_text == 1 {
              match &mut self.p_settings {
                Some(x) => {
                  x.naming_conventions.insert(self.attr_dict.get("name").unwrap().clone(),data.to_string());
                }
                None => {}
              }
              self.in_text = 0;
            }
            else if self.in_revision == 1 && self.in_remarks == 1 {
              match &mut self.p_settings {
                Some(x) => {
                  let revision_no : &String = self.attr_dict.get("number").unwrap();
                  let revision_date : &String = self.attr_dict.get("date").unwrap();
                  x.revisions.push((revision_no.clone(),revision_date.clone(),data.to_string()));
                }
                None => {}
              }
              self.in_remarks = 0;
            }
            else if self.in_role == 1 && self.in_text == 1 {
              match &mut self.roles {
                Some(x) => {
                  let last_idx = x.len() - 1;
                  x[last_idx].description = data.to_string();   
                },
                None => {} 
              } 
              self.in_text = 0;
            }
            else if self.in_value_type == 1 && self.in_text == 1 {
              match &mut self.tv_types {
                Some(x) => {
                  let last_idx = x.len() - 1;
                  x[last_idx].description = data.to_string();   
                }
                None => {}
              }
              self.in_text = 0;
            }
            else if self.in_asset == 1 && self.in_significance == 1 {
              let last_idx = self.assets.len() - 1;
              self.assets[last_idx].significance = data.to_string();   
              self.in_significance = 0;
            }
            else if self.in_asset == 1 && self.in_text == 1 {
              let last_idx = self.assets.len() - 1;
              self.assets[last_idx].description = data.to_string();   
              self.in_text = 0;
            }
            else if self.in_asset == 1 && self.in_critical_rationale == 1 {
              let last_idx = self.assets.len() - 1;
              self.assets[last_idx].critical_rationale = data.to_string();   
              self.in_critical_rationale = 0;
            }
            else if self.in_security_property == 1 && self.in_rationale == 1 {
              let last_idx = self.assets.len() - 1;
              let env_name : &String = self.attr_dict.get("environment").unwrap();
              let s_prop : &String = self.attr_dict.get("property").unwrap();
              let s_prop_v : &String = self.attr_dict.get("value").unwrap();
              self.assets[last_idx].update_security_property(env_name,s_prop,s_prop_v,&data);
              self.in_rationale = 0;
            }
          }
          Event::Reference(_) => {}
           _ => {}
        },
        Err(err) => {
          println!("{}", err);
          break;
        }
      } 
    }
  }

}
