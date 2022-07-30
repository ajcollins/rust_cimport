use std::{collections::HashMap,collections::HashSet,fs::File};
use xml_oxide::{sax::parser::Parser, sax::Event, sax::StartElement};
mod dimensions;

fn attributes_to_dict(el: &StartElement, attr_names : HashSet::<&str>, d : &mut HashMap<String,String>) {
  d.clear();
  for el_attr in el.attributes() {
    if attr_names.contains(el_attr.name) {
      d.insert(el_attr.name.to_string(),el_attr.value.to_string());
    }
  }
}

fn main() {

  let mut in_text = 0;
  let mut in_rationale = 0;
  let mut in_remarks = 0;
  let mut in_significance = 0;
  let mut in_critical_rationale = 0;
  let mut in_background = 0;
  let mut in_strategic_goals = 0;
  let mut in_scope = 0;
  let mut in_entry = 0;
  let mut in_revision = 0;
  let mut in_role = 0;
  let mut in_value_type = 0;
  let mut in_asset = 0;
  let mut in_security_property = 0;
  let mut p_settings : Option<Box<dimensions::ProjectSettings>> = None;
  let mut tv_types : Option<Box<Vec::<dimensions::ValueType>>> = None;
  let mut roles : Option<Box<Vec::<dimensions::Role>>> = None;
  let mut tags = Vec::<dimensions::Tag>::new();
  let mut assets : Vec::<dimensions::Asset> = Vec::<dimensions::Asset>::new();
  let mut attr_dict : HashMap<String,String> = HashMap::<String,String>::new();

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
              tv_types = Some(Box::new(Vec::<dimensions::ValueType>::new()));
            }
            else if el.name == "domainvalues" && tv_types == None {
              tv_types = Some(Box::new(Vec::<dimensions::ValueType>::new()));
            }
            else if el.name == "vulnerability_type" || el.name == "threat_type" || el.name == "threat_value" || el.name == "risk_value" || el.name == "countermeasure_value" || el.name == "severity_value" || el.name == "likelihood_value" {
              attributes_to_dict(&el,HashSet::from(["name"]),&mut attr_dict);
              if let Some(x) = &mut tv_types {
                x.push( dimensions::ValueType::new(attr_dict.get("name").unwrap(),&"".to_string()));
                in_value_type = 1;
              }
              else {
                panic!("No TVTypes vector defined");
              }
            }
            else if el.name == "description" {
              in_text = 1;
            }
            else if el.name == "definition" {
              in_text = 1;
            }
            else if el.name == "rationale" {
              in_rationale = 1;
            }
            else if el.name == "significance" {
              in_significance = 1;
            }
            else if el.name == "project_settings" {
              attributes_to_dict(&el,HashSet::from(["name"]),&mut attr_dict);
              p_settings = Some(Box::new(dimensions::ProjectSettings::new(attr_dict.get("name").unwrap())));
            }
            else if el.name == "background" || el.name == "definition" {
              in_background = 1;
            }
            else if el.name == "remarks" {
              in_remarks = 1;
            }
            else if el.name == "strategic_goals" {
              in_strategic_goals = 1;
            }
            else if el.name == "scope" {
              in_scope = 1;
            }
            else if el.name == "entry" {
              attributes_to_dict(&el,HashSet::from(["name"]),&mut attr_dict);
              in_entry = 1;
            }
            else if el.name == "contributor" {
              attributes_to_dict(&el,HashSet::from(["first_name","surname","affiliation","role"]),&mut attr_dict);
              let first_name : &String = attr_dict.get("first_name").unwrap();
              let surname : &String = attr_dict.get("surname").unwrap();
              let affil : &String = attr_dict.get("affiliation").unwrap();
              let role : &String = attr_dict.get("role").unwrap();
              match &mut p_settings {
                Some(x) => {
                  x.contributors.push((first_name.clone(),surname.clone(),affil.clone(),role.clone()));
                }
                None => {}
              }
            }
            else if el.name == "revision" {
              attributes_to_dict(&el,HashSet::from(["number","date"]),&mut attr_dict);
              in_revision = 1;
            }
            else if el.name == "role" {
              attributes_to_dict(&el,HashSet::from(["name","type","short_code"]),&mut attr_dict);
              let new_role = dimensions::Role::new(attr_dict.get("name").unwrap(),attr_dict.get("type").unwrap(),attr_dict.get("short_code").unwrap(),&"".to_string());

              match &mut roles {
                Some(x) => { x.push(new_role);}
                None => {roles = Some(Box::new(vec![new_role]));}
              }
              in_role = 1;
            }
            else if el.name == "security_property" {
              attributes_to_dict(&el,HashSet::from(["environment","property","value"]),&mut attr_dict);
              let env_name : &String = attr_dict.get("environment").unwrap();
              let last_idx = assets.len() - 1;
              let env_props = &mut assets[last_idx].environment_properties;
              if !env_props.contains_key(env_name) {
                env_props.insert(env_name.clone(),dimensions::AssetEnvironmentProperties::new(&env_name));
              }
              in_security_property = 1;
            }
            else if el.name == "tag" {
              for el_attr in el.attributes() {
                if el_attr.name == "name" {
                  tags.push(dimensions::Tag::new(&el_attr.value.to_string()));
                }
              }
            }
            else if el.name == "asset" {
              attributes_to_dict(&el,HashSet::from(["name","short_code","type","is_critical"]),&mut attr_dict);
              let mut critical_flag = false;
              if attr_dict.get("is_critical").unwrap() == "1" {
                critical_flag = true;
              }
              assets.push( dimensions::Asset::new(
                attr_dict.get("name").unwrap(),
                attr_dict.get("short_code").unwrap(),
                attr_dict.get("type").unwrap(),
                critical_flag));
              in_asset = 1;
            }
          }
        }
        Event::EndElement(el) => {
          if el.name == "feed" {
            break;
          }
          else if el.name == "vulnerability_type" || el.name == "threat_type" || el.name == "threat_value" || el.name == "risk_value" || el.name == "countermeasure_value" || el.name == "severity_value" || el.name == "likelihood_value" {
            in_value_type = 0;
          }
          else if el.name == "revision" {
            in_revision = 0;
          }
          else if el.name == "entry" {
            in_entry = 0;
          }
          else if el.name == "role" {
            in_role = 0;
          }
          else if el.name == "role" {
            in_asset = 0;
          }
          else if el.name == "security_property" {
            in_security_property = 0;
          }
        }
        Event::Characters(data) => {
          if in_background == 1 {
            match &mut p_settings {
              Some(x) => {
                x.background = data.to_string();
              }
              None => {}
            }
            in_background = 0;
          }
          else if in_strategic_goals == 1 {
            match &mut p_settings {
              Some(x) => {
                x.strategic_goals = data.to_string();
              }
              None => {}
            }
            in_strategic_goals = 0;
          }
          else if in_scope == 1 {
            match &mut p_settings {
              Some(x) => {
                x.scope = data.to_string();
              }
              None => {}
            }
            in_scope = 0;
          }
          else if in_entry == 1 && in_text == 1 {
            match &mut p_settings {
              Some(x) => {
                x.naming_conventions.insert(attr_dict.get("name").unwrap().clone(),data.to_string());
              }
              None => {}
            }
            in_text = 0;
          }
          else if in_revision == 1 && in_remarks == 1 {
            match &mut p_settings {
              Some(x) => {
                let revision_no : &String = attr_dict.get("number").unwrap();
                let revision_date : &String = attr_dict.get("date").unwrap();
                x.revisions.push((revision_no.clone(),revision_date.clone(),data.to_string()));
              }
              None => {}
            }
            in_remarks = 0;
          }
          else if in_role == 1 && in_text == 1 {
            match &mut roles {
              Some(x) => {
                let last_idx = x.len() - 1;
                x[last_idx].description = data.to_string();   
              },
              None => {} 
            } 
            in_text = 0;
          }
          else if in_value_type == 1 && in_text == 1 {
            match &mut tv_types {
              Some(x) => {
                let last_idx = x.len() - 1;
                x[last_idx].description = data.to_string();   
              }
              None => {}
            }
            in_text = 0;
          }
          else if in_asset == 1 && in_significance == 1 {
            let last_idx = assets.len() - 1;
            assets[last_idx].significance = data.to_string();   
            in_significance = 0;
          }
          else if in_asset == 1 && in_text == 1 {
            let last_idx = assets.len() - 1;
            assets[last_idx].description = data.to_string();   
            in_text = 0;
          }
          else if in_asset == 1 && in_critical_rationale == 1 {
            let last_idx = assets.len() - 1;
            assets[last_idx].critical_rationale = data.to_string();   
            in_critical_rationale = 0;
          }
          else if in_security_property == 1 && in_rationale == 1 {
            let last_idx = assets.len() - 1;
            let env_name : &String = attr_dict.get("environment").unwrap();
            let s_prop : &String = attr_dict.get("property").unwrap();
            let s_prop_v : &String = attr_dict.get("value").unwrap();
            assets[last_idx].update_security_property(env_name,s_prop,s_prop_v,&data);
            in_rationale = 0;
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

/*   if let Some(x) = &tv_types {
    for tv in &**x {
      println!("{}",tv);
    }
  } 
  */
  
  if let Some(x) = &p_settings {
    println!("{}",x);
  }
/* 
  if let Some(x) = &roles {
    for role in &**x {
      println!("{}",role);
    }
  }
  for asset in &assets {
    println!("{}",asset);
  }
  */
}
