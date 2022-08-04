use std::env;
mod model_parser;
mod dimensions;
mod decorators;

use model_parser::ModelParser;


fn main() {

  let args : Vec<String> = env::args().collect();
  let model_file = &args[1];
  let mut mp = ModelParser::new();
  mp.parse(model_file);
  
  println!("THREAT/VULNERABILITY TYPES");
  if let Some(tvs) = &mp.state.tv_types {
    for tv in tvs.iter().enumerate() {
      println!("{}",tv.1);
    }
  }
  println!("DOMAIN VALUES");
  if let Some(dvs) = &mp.state.dv_types {
    for dv in dvs.iter().enumerate() {
      println!("{}",dv.1);
    }
  }
  println!("PROJECT SETTINGS");
  if let Some(ps) = &mp.state.p_settings {
    println!("{}",ps);
  }
  
  println!("ENVIRONMENTS");
  if let Some(envs) = &mp.state.environments {
     for env in envs.iter().enumerate() {
      println!("{}",env.1);
    }
  }

  println!("ROLES");
  if let Some(roles) = &mp.state.roles {
    for role in roles.iter().enumerate() {
      println!("{}",role.1);
    }
  }

  println!("ASSETS");
  if let Some(assets) = &mp.state.assets {
    for asset in assets.iter().enumerate() {
      println!("{}",asset.1);
    } 
  }
  println!("VULNERABILITIES");
  if let Some(vuls) = &mp.state.vulnerabilities {
    for vul in vuls.iter().enumerate() {
      println!("{}",vul.1);
    } 
  }

  println!("ATTACKERS");
  if let Some(attackers) = &mp.state.attackers {
    for attacker in attackers.iter().enumerate() {
      println!("{}",attacker.1);
    } 
  }
}
