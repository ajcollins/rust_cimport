use std::collections::HashMap;
use std::fmt;

pub struct ProjectSettings {
  pub name:  String,
  pub background : String,
  pub strategic_goals : String,
  pub scope : String,
  pub naming_conventions : HashMap<String,String>,
  pub contributors :  Vec<(String,String,String,String)>,
  pub revisions : Vec<(String,String,String)>,
  pub rich_picture : String
}

impl ProjectSettings {
  pub fn new(proj_name : &String) -> ProjectSettings {
    ProjectSettings{name : proj_name.clone(), background : "".to_string(), strategic_goals : "".to_string(), scope : "".to_string(), naming_conventions : HashMap::new(), contributors : Vec::new(), revisions : Vec::new(), rich_picture : "".to_string()}
  }
}

impl fmt::Display for ProjectSettings {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut x = format!("Name: {},\n Background: {},\n Strategic goals: {},\n Scope: {},\n Rich picture: {},\n", self.name, self.background, self.strategic_goals, self.scope, self.rich_picture);
    for (name,value) in self.naming_conventions.iter() {
      x.push_str(format!("Name: {}, Value: {}\n",name,value).as_str());
    }
    for c in &self.contributors {
      x.push_str(format!("Firstname: {}, Surname: {}, Affiliation: {}, Role: {}\n",&c.0,&c.1,&c.2,&c.3).as_str());
    }
    for rev in &self.revisions {
      x.push_str(format!("Revision: {}, Date: {}, Remarks: {}\n",&rev.0,&rev.1,&rev.2).as_str());
    }
    write!(f,"{}",x)
  }
}
