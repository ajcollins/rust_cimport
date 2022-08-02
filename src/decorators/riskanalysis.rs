use std::{collections::HashMap,collections::HashSet};
use xml_oxide::{sax::StartElement,sax::EndElement};
use crate::dimensions::role::Role;
use crate::dimensions::asset::Asset;
use crate::dimensions::vulnerability::{Vulnerability, VulnerabilityEnvironment};
use crate::dimensions::tag::Tag;
use crate::dimensions::asset::AssetEnvironmentProperties;
use crate::decorators::helpers::attributes_to_dict;
use crate::decorators::parsedecorator::ParseDecorator;
use crate::decorators::savedstate::SavedState;

pub struct RiskAnalysisHandler {
  in_role : bool,
  in_asset : bool,
  in_security_property : bool,
  in_description : bool,
  in_significance : bool,
  in_critical_rationale : bool,
  in_rationale : bool,
  in_vulnerability : bool,
  roles : Vec<Role>,
  assets : Vec<Asset>,
  vulnerabilities : Vec<Vulnerability>,
  attr_dict : HashMap<String,String>
}

impl RiskAnalysisHandler {
  pub fn new() -> RiskAnalysisHandler {
    RiskAnalysisHandler {
      in_role : false,
      in_asset : false,
      in_security_property : false,
      in_description : false,
      in_significance : false,
      in_critical_rationale : false,
      in_rationale : false,
      in_vulnerability : false,
      roles : Vec::<Role>::new(),
      assets : Vec::<Asset>::new(),
      vulnerabilities : Vec::<Vulnerability>::new(),
      attr_dict : HashMap::<String,String>::new()
    }
  }
}
impl ParseDecorator for RiskAnalysisHandler {
  fn parse_start_element(&mut self, el: &StartElement) {
    if el.name == "role" {
      attributes_to_dict(&mut self.attr_dict,&el,HashSet::from(["name","type","short_code"]));
      let new_role = Role::new(self.attr_dict.get("name").unwrap(),self.attr_dict.get("type").unwrap(),self.attr_dict.get("short_code").unwrap(),&"".to_string());
      self.roles.push(new_role);
      self.in_role = true;
    }
    else if el.name == "description" {
      self.in_description = true;
    }
    else if el.name == "rationale" {
      self.in_rationale = true;
    }
    else if el.name == "significance" {
      self.in_significance = true;
    }
    else if el.name == "critical_rationale" {
      self.in_critical_rationale = true;
    }
    else if el.name == "security_property" {
      attributes_to_dict(&mut self.attr_dict,&el,HashSet::from(["environment","property","value"]));
      let env_name : &String = self.attr_dict.get("environment").unwrap();
      let last_idx = self.assets.len() - 1;
      let env_props = &mut self.assets[last_idx].environment_properties;
      if !env_props.contains_key(env_name) {
        env_props.insert(env_name.clone(),AssetEnvironmentProperties::new(&env_name));
      }
      self.in_security_property = true;
    }
    else if el.name == "tag" {
      attributes_to_dict(&mut self.attr_dict,&el,HashSet::from(["name"]));
      if self.in_asset == true {
        let last_idx = self.assets.len() - 1;
        self.assets[last_idx].tags.push(Tag::new(&self.attr_dict.get("name").unwrap().clone()));   
      }
    }
    else if el.name == "asset" {
      attributes_to_dict(&mut self.attr_dict,&el,HashSet::from(["name","short_code","type","is_critical"]));
      let mut critical_flag = false;
      if self.attr_dict.get("is_critical").unwrap() == "1" {
        critical_flag = true;
      }
      self.assets.push( Asset::new(
        self.attr_dict.get("name").unwrap(),
        self.attr_dict.get("short_code").unwrap(),
        self.attr_dict.get("type").unwrap(),
        critical_flag));
      self.in_asset = true;
    }
    else if el.name == "vulnerability" {
      attributes_to_dict(&mut self.attr_dict,&el,HashSet::from(["name","type"]));
      self.vulnerabilities.push (Vulnerability::new(
        self.attr_dict.get("name").unwrap(),
        self.attr_dict.get("type").unwrap())
      );
      self.in_vulnerability = true;
    }
    else if el.name == "vulnerability_environment" {
      attributes_to_dict(&mut self.attr_dict,&el,HashSet::from(["name","severity"]));
      let last_idx = self.vulnerabilities.len() - 1;
      self.vulnerabilities[last_idx].environments.push(VulnerabilityEnvironment::new(self.attr_dict.get("name").unwrap(),self.attr_dict.get("severity").unwrap()));   
    }
    else if el.name == "vulnerable_asset" {
      attributes_to_dict(&mut self.attr_dict,&el,HashSet::from(["name"]));
      let last_vul_idx = self.vulnerabilities.len() - 1;
      let envs = &mut self.vulnerabilities[last_vul_idx].environments;
      let last_env_idx = envs.len();
      envs[last_env_idx - 1].assets.push(self.attr_dict.get("name").unwrap().clone());
    }
  }
  fn parse_characters(&mut self, data : &str) {
    if self.in_role == true && self.in_description == true {
      let last_idx = self.roles.len() - 1;
      self.roles[last_idx].description = data.to_string();   
      self.in_description = false;
    }
    else if self.in_asset == true && self.in_description == true {
      let last_idx = self.assets.len() - 1;
      self.assets[last_idx].description = data.to_string();   
      self.in_description = false;
    }
    else if self.in_asset == true && self.in_significance == true {
      let last_idx = self.assets.len() - 1;
      self.assets[last_idx].significance = data.to_string();   
      self.in_significance = false;
    }
    else if self.in_asset == true && self.in_critical_rationale == true {
      let last_idx = self.assets.len() - 1;
      self.assets[last_idx].critical_rationale = data.to_string();   
      self.in_critical_rationale = false;
    }
    else if self.in_security_property == true && self.in_rationale == true {
      let last_idx = self.assets.len() - 1;
      let env_name : &String = self.attr_dict.get("environment").unwrap();
      let s_prop : &String = self.attr_dict.get("property").unwrap();
      let s_prop_v : &String = self.attr_dict.get("value").unwrap();
      self.assets[last_idx].update_security_property(env_name,s_prop,s_prop_v,&data);
      self.in_rationale = false;
    }
    else if self.in_vulnerability == true && self.in_description == true {
      let last_idx = self.vulnerabilities.len() - 1;
      self.vulnerabilities[last_idx].description = data.to_string();   
      self.in_description = false;
    }
  }
  
  fn parse_end_element(&mut self, el: &EndElement){
    if el.name == "role" {
      self.in_role = false;
    }
    else if el.name == "asset" {
      self.in_asset = false;
    }
    else if el.name == "security_property" {
      self.in_security_property = false;
    }
    else if el.name == "vulnerability" {
      self.in_vulnerability = false;
    }
  }

  fn save_state(&self, ss: &mut SavedState){
    ss.roles = Some(Box::new(self.roles.clone()));
    ss.assets = Some(Box::new(self.assets.clone()));
    ss.vulnerabilities = Some(Box::new(self.vulnerabilities.clone()));
  }
}
