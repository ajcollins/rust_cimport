use std::path::PathBuf;
mod model_parser;
mod dimensions;
use model_parser::ModelParser;


fn main() {

  let mut mp = ModelParser::new();
  let model_file = PathBuf::from("./test.xml");
  mp.parse(&model_file);  

  if let Some(x) = &mp.tv_types {
    for tv in &**x {
      println!("{}",tv);
    }
  } 
  
  if let Some(x) = &mp.p_settings {
    println!("{}",x);
  }
 
  if let Some(x) = &mp.roles {
    for role in &**x {
      println!("{}",role);
    }
  } 
  for asset in &mp.assets {
    println!("{}",asset);
  } 
  
}
