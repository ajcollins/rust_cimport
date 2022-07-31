
mod model_parser;
mod dimensions;
mod decorators;

use model_parser::ModelParser;


fn main() {

  let mut mp = ModelParser::new();
  mp.parse(&"./test.xml".to_string());  

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
}
