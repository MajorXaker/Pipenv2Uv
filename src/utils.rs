use std::path::{Path, PathBuf};

pub fn get_output_file_name(is_docker: bool) -> String {
    let mut output_path = PathBuf::from("pyproject.toml");

    let mut counter = 1;

    if is_docker {
        // when we work via docker export files are created in special directory
        // also the file is overwritten
        let parent_path = Path::new("output/");
        output_path = parent_path.join(output_path);

        while output_path.exists() {
            println!(
                "File {} already exists, creating new",
                output_path.to_str().unwrap()
            );
            let filename = format!("{}-new-{}.toml", "pyproject", counter);
            counter += 1;
            output_path = parent_path.join(filename);
        }
    } else {
        while output_path.exists() {
            println!(
                "File {} already exists, creating new",
                output_path.clone().to_str().unwrap()
            );
            let filename = format!("{}-new-{}.toml", "pyproject", counter);
            counter += 1;
            output_path = PathBuf::from(&filename);
        }
    }
    output_path.to_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_output_file_name() {
        let file_name = get_output_file_name(false);
        assert_eq!(file_name, "pyproject.toml".to_string());
    }

    #[test]
    fn test_get_output_file_name_docker() {
        let file_name = get_output_file_name(true);

        assert_eq!(file_name, "output/pyproject.toml");
    }

    #[test]
    fn test_get_output_file_name_existing_file() {
        std::fs::File::create("pyproject.toml").unwrap();

        let new_file_name = get_output_file_name(false);

        // cleaning up
        std::fs::remove_file("pyproject.toml").unwrap();

        assert_ne!(new_file_name, "pyproject.toml".to_string());
        assert_eq!(new_file_name, "pyproject-new-1.toml");
    }

    #[test]

    fn test_get_output_file_name_existing_file_docker() {
        std::fs::create_dir_all("output").unwrap();
        std::fs::File::create("output/pyproject.toml").unwrap();

        let first_file_name = get_output_file_name(true);
        std::fs::File::create("output/pyproject-new-1.toml").unwrap();
        let second_file_name = get_output_file_name(true);

        // teardown since new_file_name is just a string
        std::fs::remove_file("output/pyproject.toml").unwrap();
        std::fs::remove_file("output/pyproject-new-1.toml").unwrap();
        std::fs::remove_dir_all("output").unwrap();

        assert_ne!(first_file_name, "output/pyproject.toml".to_string());
        assert_eq!(first_file_name, "output/pyproject-new-1.toml");

        assert_ne!(second_file_name, "output/pyproject.toml".to_string());
        assert_eq!(second_file_name, "output/pyproject-new-2.toml");
    }
}
