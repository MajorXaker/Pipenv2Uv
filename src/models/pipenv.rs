pub struct Pipenv {
    pub python_version: String,
    pub allow_prereleases: Option<String>,
}

pub trait PipenvRequirements {
    fn set_py_version(&mut self, value: &str) -> Result<(), std::io::Error>;
    fn set_prereleases_status(&mut self, value: &str) -> Result<(), std::io::Error>;

    fn new() -> Self;
}

impl PipenvRequirements for Pipenv {
    fn set_py_version(&mut self, value: &str) -> Result<(), std::io::Error> {
        let split: Vec<&str> = value.split('=').collect();
        let value = split[1]
            .trim()
            .trim_start_matches('"')
            .trim_end_matches('"');
        self.python_version = value.to_string();
        Ok(())
    }
    fn set_prereleases_status(&mut self, value: &str) -> Result<(), std::io::Error> {
        let split: Vec<&str> = value.split('=').collect();

        let value = split[1].trim().to_string();
        self.allow_prereleases = Some(value);

        Ok(())
    }

    fn new() -> Self {
        Pipenv {
            python_version: "".to_string(),
            allow_prereleases: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_py_version() -> Result<(), std::io::Error> {
        let mut pipenv = Pipenv {
            python_version: "".to_string(),
            allow_prereleases: None,
        };

        pipenv.set_py_version("python_version = \"3.8\"")?;
        assert_eq!(pipenv.python_version, "3.8");
        Ok(())
    }

    #[test]
    fn test_set_prereleases_status() -> Result<(), std::io::Error> {
        let mut pipenv = Pipenv {
            python_version: "".to_string(),
            allow_prereleases: None,
        };

        pipenv.set_prereleases_status("allow_prereleases = true")?;
        assert_eq!(pipenv.allow_prereleases.unwrap(), "true");
        Ok(())
    }
}
