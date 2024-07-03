use std::fs;
use std::io::Error as IoError;
use serde::{Deserialize, Serialize};
use::toml;

#[derive(Serialize, Deserialize,  Debug)]
struct ConfigToml{
  database: Option<ConfigTomlDatabase>,
  google: Option<ConfigTomlGoogle>,
  jwt: Option<ConfigTomlJwl>,
  
}
#[derive(Serialize, Deserialize,  Debug)]
struct ConfigTomlDatabase {
  username: Option<String>,
  password: Option<String>,
}
#[derive(Serialize, Deserialize,  Debug)]
struct ConfigTomlGoogle {
  api_key: Option<String>,
  
}
#[derive(Serialize, Deserialize,  Debug)]
struct ConfigTomlJwl {
  secret_token: Option<String>,
  
}



#[derive(Debug)]
pub struct Config{
  pub username:String,
  pub password: String,
  pub api_key: String,
  pub secret_token: String,
}

impl Config{
  pub fn new()-> Self {

    let config_filepaths:[&str; 3] = [
      "./config.toml",
      "./Config.toml",
     "~/config.toml",
    ];

    let mut content: String = "".to_owned();

    for fileppath in config_filepaths{
      let result: Result<String, IoError> =fs::read_to_string(fileppath);

     // println!("El content :{}", fileppath);
      if result.is_ok(){
        
        content= result.unwrap();
        break;
      }

    }
   // println!("El content :{}", content);
    let config_toml = toml::from_str(&content).unwrap_or_else(|_| {

      println!("Failed to create ConfiToml Object out of config file");
        ConfigToml{
          database: None,
          google: None,
          jwt: None,
        } 
    });
    
    let (username, pasword):(String, String)= match config_toml.database{
        Some(database) => {
            let db_username : String = database.username.unwrap_or_else(||{
              println!("Missing field username in table database.");
              "unknown".to_owned()
            });

            let db_pasword : String = database.password.unwrap_or_else(||{
              println!("Missing field username in table database.");
              "unknown".to_owned()
        });
        (db_username,db_pasword)

        } ,
        None =>{
          println!("Missing table database.");  
          ("unknown".to_owned() , "unknown".to_owned() )
        } ,
    };
   // println!("{} {}"  , username , pasword );

    let api_key : String = match config_toml.google {
      Some(google) => google.api_key.unwrap_or_else(||{
        println!("Missing field api_key in table google");
        "unknown".to_owned()
      } ),
      None => {
        println!("Missing table google.");
        "unknown".to_owned()

      }
    };

    let secret_token = match config_toml.jwt{
        Some(jwt) => jwt.secret_token.unwrap_or_else(||{
             println!("Missing field secret_token in table jwt");
            "unknown".to_owned()

        }),
        None =>{
           println!("Missing table jwt.");
          "unknown".to_owned()

        } ,

    };


     // println!("{} {}"  , username , pasword );
     //println!("El content1 :{}", content);
    Config{

      username: username.to_owned(),
      password: pasword.to_owned(),
      api_key: api_key.to_owned(),
      secret_token: secret_token.to_owned(),
    }

  }
}