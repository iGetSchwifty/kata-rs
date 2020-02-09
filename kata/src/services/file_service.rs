use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use super::super::models::data_type::*;
use super::super::models::kata_error::*;

pub fn read_lines(filename: Box<&Path>) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(*filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn validate_file(args: &Vec<String>) -> Result<Box<&Path>, KataError> {
    if args.len() < 2 {
        Err(KataError::new("Not enough arguments! Missing filename."))
    } else {
        let file_name = Path::new(&args[1]);
        if file_name.exists() {
            Ok(Box::new(file_name))
        } else {
            Err(KataError::new("Path does not exist!"))
        }
    }
}

pub fn validate_line_type(line_tokens: &Vec<&str>) -> DataType {
    if line_tokens.len() == 2 && line_tokens[0] == "Driver" {
        DataType::Driver(line_tokens[1].to_string())
    } else if line_tokens.len() == 5 && line_tokens[0] == "Trip" {
        DataType::Trip(line_tokens[1].to_string())
    } else {
        DataType::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::models::data_type::DataType::*;

    #[test]
    fn test_validate_file_errors_less_than_two_paramets() {
        let new_vec: Vec<String> = vec!["value".to_string()];
        let test_instance = validate_file(&new_vec);
        assert!(test_instance.is_err());
        assert!(test_instance.is_ok() == false);
        if let Err(result) = test_instance {
            assert_eq!(result.message, "Not enough arguments! Missing filename.");
        }
    }

    #[test]
    fn test_validate_file_returns_path_error() {
        let new_vec: Vec<String> = vec!["value".to_string(), "filename".to_string()];
        let test_instance = validate_file(&new_vec);
        assert!(test_instance.is_err());
        assert!(test_instance.is_ok() == false);
        if let Err(result) = test_instance {
            assert_eq!(result.message, "Path does not exist!");
        }
    }

    //
    //  Could be coding to an interface to avoid this and to actually test with mocks..
    //  Since this is a kata not going to go that far
    //
    #[cfg(unix)]
    #[test]
    fn test_validate_file_valid() {
        let new_vec: Vec<String> = vec!["value".to_string(), "/".to_string()];
        let test_instance = validate_file(&new_vec);
        assert!(test_instance.is_err() == false);
        assert!(test_instance.is_ok());
        if let Ok(result) = test_instance {
            assert_eq!(*result, Path::new("/"));
        }
    }

    #[test]
    fn test_validate_line_type_driver() {
        if let Driver(result) = validate_line_type(&["Driver", "TestName"].to_vec()) {
            assert_eq!(result, "TestName");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_validate_line_type_trip() {
        if let Trip(result) = validate_line_type(&["Trip", "TestName", "07:15", "07:45", "17.3"].to_vec()) {
            assert_eq!(result, "TestName");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_validate_line_type_unknown() {
        match validate_line_type(&["Test", "TestName"].to_vec()) {
            Trip(_) => assert!(false),
            Driver(_) => assert!(false),
            Unknown => assert!(true)
        }
    }
}