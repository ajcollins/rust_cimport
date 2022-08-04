use std::fmt;
use crate::dimensions::tag::Tag;

#[derive(Clone,PartialEq)]
pub struct AttackerEnvironment {
  name : String,
  pub roles : Vec<String>,
  pub motivations : Vec<String>, 
  pub capabilities : Vec<String>  
}

impl AttackerEnvironment {
  pub fn new(env_name : &String) -> AttackerEnvironment {
    AttackerEnvironment { name: env_name.clone(), roles: Vec::<String>::new(), motivations: Vec::<String>::new(), capabilities: Vec::<String>::new() }
  }
}

#[test]
fn test_new_attacker_environment() {
  let ae = AttackerEnvironment::new(&"Default".to_string());
  assert_eq!(ae.name,"Default".to_string());
  assert_eq!(ae.roles.len(),0);
  assert_eq!(ae.motivations.len(),0);
  assert_eq!(ae.capabilities.len(),0);
}

impl fmt::Display for AttackerEnvironment {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"Environment: {}, Roles: {}, Motivations: {}, Capabilities {}",self.name,self.roles.join(","),self.motivations.join(","),self.capabilities.join(","))
  }
}

#[derive(Clone,PartialEq)]
pub struct Attacker {
  name : String,
  image : String,
  pub description : String,
  pub tags : Vec<Tag>,
  pub environments : Vec<AttackerEnvironment>  
}

impl Attacker {
  pub fn new(attacker_name : &String, attacker_image : &String) -> Attacker {
    Attacker { 
      name: attacker_name.clone(), 
      image: attacker_image.clone(),
      description: "".to_string(), 
      tags: Vec::<Tag>::new(), 
      environments: Vec::<AttackerEnvironment>::new() 
    }
  }
}

#[test]
fn test_new_attacker() {
  let a = Attacker::new(&"Peppa".to_string(),&"".to_string());
  assert_eq!(a.name,"Peppa".to_string());
  assert_eq!(a.image,"".to_string());
  assert_eq!(a.description,"".to_string());
  assert_eq!(a.tags.len(),0);
  assert_eq!(a.environments.len(),0);
}

impl fmt::Display for Attacker {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let envs = &self.environments.iter().map(|env| env.to_string()).collect::<Vec<String>>().join(",");
    let tags = &self.tags.iter().map(|t| t.name.clone()).collect::<Vec<String>>().join(",");
    write!(f,"Name: {}, Image: {}, Description: {}, Tags: {}, Environments: {}",self.name,self.image,self.description,tags,envs)
  }
}
