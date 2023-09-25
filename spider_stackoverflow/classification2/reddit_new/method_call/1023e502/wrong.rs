
use serde_json::Value;
use serde_json::from_str;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub enum ConfigurationSource {
    StringContent(String),
    FileContent(String)    
}

pub struct ConfigurationBuilder {
    config: Value,
    bundles: HashMap<String, Vec<ConfigurationSource>>    
}

impl<'a> ConfigurationBuilder{

    pub fn new(base_source: ConfigurationSource) -> ConfigurationBuilder{
        let base_config: Value = from_str("{}").unwrap();
        
        let mut config_builder = ConfigurationBuilder{
            config: base_config,
            bundles: HashMap::new()
        };

        config_builder.merge_source(&base_source);

        return config_builder;
    }

    pub fn merge_sources(&mut self, config_sources: &Vec<ConfigurationSource>){        
        for source in config_sources{
            self.merge_source(source);
        }
    }

    pub fn merge_source(&mut self, config_source: &ConfigurationSource){            
        match config_source {
            &ConfigurationSource::StringContent(ref content) => {                
                let config_override: Value = from_str(&content[..]).unwrap();
                merge(&mut self.config, config_override);
                //merge(&mut config, &json_override);
            },
            &ConfigurationSource::FileContent(ref path) => {
                let mut config_file = File::open(path).unwrap();
                let mut config_file_content = String::new();
                config_file.read_to_string(&mut config_file_content).unwrap();
                
                let config_override: Value = from_str(&config_file_content[..]).unwrap();
                merge(&mut self.config, config_override);
            }
        }      
    }

    pub fn merge_bundle(&mut self, bundle_key: &str){
        let sources = self.bundles.get(bundle_key).unwrap();
        self.merge_sources(sources);
    }
}

fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}