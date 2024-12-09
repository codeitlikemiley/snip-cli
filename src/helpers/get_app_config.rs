use crate::constants::DEFAULT_CONFIG_PATH;
use crate::helpers::expand_home_dir::expand_home_dir;

pub fn get_app_config() -> String {
    dotenv::dotenv().ok();

    let config_path = match std::env::var("SNIP_CONFIG_PATH") {
        Ok(path) => path,
        Err(_) => {
            let config_path = expand_home_dir(DEFAULT_CONFIG_PATH)
                .to_string_lossy()
                .into_owned();
            config_path
        }
    };
    config_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_env_var_is_not_set_then_use_default_config_path() {
        // Arrange
        std::env::remove_var("SNIP_CONFIG_PATH");

        let expected = expand_home_dir(DEFAULT_CONFIG_PATH)
            .to_string_lossy()
            .into_owned();

        // Act
        let actual = get_app_config();
        // For debugging purpose on CI
        println!("DEBUGGER ENV CONFIG PATH: {}", actual);
        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_env_var_is_set_then_use_env_var_config_path() {
        // Arrange
        let expected = "/tmp/snip.json";
        std::env::set_var("SNIP_CONFIG_PATH", expected);
        // Act
        let actual = get_app_config();
        // Assert
        assert_eq!(actual, expected);
    }
}
