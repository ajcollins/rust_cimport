use mysql::*;
use mysql::prelude::*;
use crate::dimensions::valuetype::ValueType;
//:use crate::dimensions::valuetype::VTOutput;

pub struct MySQLDatabaseProxy {
  pool : Pool,
  conn : PooledConn
}

impl MySQLDatabaseProxy {
  pub fn new(db_host: &String, db_port: &String, db_user: &String, db_passwd : &String, db_name : &String) -> MySQLDatabaseProxy {
    let url = format!("mysql://{}:{}@{}:{}/{}",db_user,db_passwd,db_host,db_port,db_name);
    match Pool::new(url.as_str()) {
      Result::Ok(p) => {
        match p.get_conn() {
          Result::Ok(c) => {
            return MySQLDatabaseProxy {pool: p, conn: c};
          },
          Result::Err(err) => {
            panic!("{:?}",err);
          }
        };
      },
      Result::Err(err) => {
        panic!("{:?}",err);
      }
    };
  }

  pub fn new_id(&mut self) -> i128 {
    let res = self.conn.query_map("call newId()",|nid| nid);
    match res {
      Result::Ok(r) => {
        return r[0]; 
      },
      Result::Err(err) => {
        panic!("{:?}",err);
      }
    };
  }

  pub fn add_value_type(&mut self, vt : &ValueType) {
    let new_id = self.new_id();
    let res : Result::<Option::<u128>> = self.conn.exec_first("call addValueType(:id,:name,:desc,:type,:score,:rat)", params!{
      "id" => new_id,
      "name" => &vt.name,
      "desc" => &vt.description,
      "type" => &vt.vt_type,
      "score" => &vt.score,
      "rat" => &vt.rationale
    });
  }
  
  pub fn get_value_types(&mut self, dim_name : &String, env_name : &String) -> Vec<ValueType> {
    let res = self.conn.exec_map("call getCustomisableValues(:dim,:env)", params!{
      "dim" => dim_name,
      "env" => env_name
    },| (type_id, type_name, type_desc, type_value, type_rat) : (i128,String,String,i128,String) | {
      let mut vt = ValueType::new(&type_name,&type_desc,&"".to_string());
      vt.id = type_id;
      vt.score = type_value;
      vt.rationale = type_rat;
      vt.vt_type = dim_name.clone();
      return vt;
    });
    return res.unwrap();
  }

}