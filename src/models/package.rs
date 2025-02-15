pub trait UVPackage {
    fn as_uv(&self) -> String;
    fn state_source(&self) -> String;
}

pub struct Package {
    pub name: String,
    pub version: String,
    pub index: Option<String>,
    pub extras: Option<Vec<String>>,
    pub is_dev: bool,
}

impl UVPackage for Package {
    fn as_uv(&self) -> String {
        let mut result_string: String = r#"""#.to_string();

        result_string.push_str(&self.name);

        if self.extras.is_some() {
            let extras: String = self.extras.clone().unwrap().join(",");
            result_string.push('[');
            result_string.push_str(&extras);
            result_string.push(']');
        }

        if &self.version != "*" {
            result_string.push_str(&self.version);
        }

        result_string.push('"');

        result_string
    }

    fn state_source(&self) -> String {
        let index_name: &str = self.index.as_ref().unwrap();
        let string = format!(
            "{package} = {{index=\"{index}\"}}",
            index = index_name,
            package = self.name,
        );
        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_uv_with_extras() {
        let package = Package {
            name: "requests".to_string(),
            version: "==2.25.1".to_string(),
            index: None,
            extras: Some(vec!["socks".to_string()]),
            is_dev: false,
        };

        let expected = r#""requests[socks]==2.25.1""#;
        let result = package.as_uv();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_state_source() {
        let package = Package {
            name: "requests".to_string(),
            version: "2.25.1".to_string(),
            index: Some("pypi".to_string()),
            extras: None,
            is_dev: false,
        };

        let expected = "requests = {index=\"pypi\"}".to_string();
        assert_eq!(package.state_source(), expected);
    }
}
