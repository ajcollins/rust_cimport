mod model_parser;
mod dimensions;
mod decorators;
mod db;
use model_parser::ModelParser;
use db::mysql::MySQLDatabaseProxy;
use clap::Parser;

#[derive(Parser)]
#[clap(author,version,about,long_about = None)]
struct Cli {
  host : String,
  port : String,
  user : String,
  passwd : String,
  db_name : String,
  model_file : String
}

fn main() {

  let cli = Cli::parse();

  let mut mp = ModelParser::new();
  mp.parse(&cli.model_file);
  
  let mut proxy = MySQLDatabaseProxy::new(&cli.host,&cli.port, &cli.user, &cli.passwd, &cli.db_name);
  println!("THREAT/VULNERABILITY TYPES");
  if let Some(tvs) = &mp.state.tv_types {
    for vt in tvs.iter().enumerate() {
      proxy.add_value_type(vt.1);
    }
  }
  
  for vt in proxy.get_value_types(&"vulnerability_type".to_string(), &"".to_string()) {
    println!("{}",vt);
  }

  for vt in proxy.get_value_types(&"threat_type".to_string(), &"".to_string()) {
    println!("{}",vt);
  }

}
