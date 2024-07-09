use crate::cli::Cli;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Api {
    pub token: String
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api: Api
}


fn resolve_config_path(arg: &Option<String>) -> String {
    return match arg {
        Some(path) => path.to_string(),
        None => { 
            let mut path = home::home_dir().expect("Unable to resolve home dir");
            path.push(".config");
            path.push("rdoist.toml");
            path.to_str().expect("Unable to read default path").to_string()
        }
    };
}
    

pub fn load_config(cli: &Cli) -> Config {
    let path = resolve_config_path(&cli.config_path);
    let config: Config = toml::from_str(&std::fs::read_to_string(path).expect("Unable to read config")).unwrap();

    return config;
}



#[cfg(test)]
mod tests {

    use super::resolve_config_path;
    #[test]
    fn should_resolve_path_to_given_path() {
        let result = resolve_config_path(&Some(String::from("test")));
        assert_eq!(result, "test");
    }

    #[test]
    fn should_resolve_path_to_default_path() {
        let result = resolve_config_path(&None);
        assert!(result.ends_with(".config/rdoist.toml"));
    }
}
