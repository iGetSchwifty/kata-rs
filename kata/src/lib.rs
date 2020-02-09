mod models;
mod services;

use self::models::kata_error::*;
use self::services::*;

pub fn run(args: Vec<String>) -> Result<Vec<String>, KataError> {
    match file_service::validate_file(&args) {
        Ok(path) => {
            if let Ok(lines) = file_service::read_lines(path) {
                Ok(data_service::process_lines(lines))
            } else {
                Err(KataError::new("Error reading line data."))
            }
        },
        Err(error) => Err(error)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run_less_than_two() {
        let new_vec: Vec<String> = vec!["value".to_string()];
        let result = super::run(new_vec);
        assert!(result.is_err());
        if let Err(result) = result {
            assert_eq!(result.message, "Not enough arguments! Missing filename.");
        }
    }

    #[test]
    fn test_run_returns_path_error() {
        let new_vec: Vec<String> = vec!["value".to_string(), "filename".to_string()];
        let result = super::run(new_vec);
        assert!(result.is_err());
        if let Err(result) = result {
            assert_eq!(result.message, "Path does not exist!");
        }
    }
}
