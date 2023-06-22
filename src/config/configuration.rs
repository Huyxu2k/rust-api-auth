use serde::{Deserialize};

#[derive(Deserialize,Debug)]
pub struct Settings{
   pub app_port:u16,
   pub database:DatabaseSettings,
}


#[derive(Deserialize,Debug)]
pub struct  DatabaseSettings{
pub username:String,
pub password:String,
pub port:u16,
pub host:String,
pub database_name:String,
}
pub fn get_configuration()->Result<Settings,config::ConfigError>
{
  let mut setting= config::Config::default();
  setting.merge(config::File::with_name("configuration"))?;

  setting.try_deserialize()
}
//not use
impl DatabaseSettings {
    pub fn connection_string(&self)->String
    {
        format!(
            "postgres://{username}:{password}@{host}:{port}/{database_name}",
            username = self.username,
            password = self.password,
            host = self.host,
            port = self.port,
            database_name = self.database_name
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{username}:{password}@{host}:{port}",
            username = self.username,
            password = self.password,
            host = self.host,
            port = self.port,
        )
    }
}