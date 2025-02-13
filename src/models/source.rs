pub trait UVSource {
    fn as_uv(&self) -> String;
}

pub struct Source {
    // This is also an index
    pub name: String,
    pub url: String,
    pub verify_ssl: Option<String>,
}

impl UVSource for Source {
    fn as_uv(&self) -> String {
        let mut result_string: String = String::new();
        let name_string: String = format!(r#"name = "{}""#, self.name);
        result_string.push_str(&name_string);
        result_string.push('\n');

        let url_string: String = format!(r#"url = "{}""#, self.url);
        result_string.push_str(&url_string);
        result_string.push('\n');

        if self.url.starts_with("${") {
            println!("UV Does not support reading .ENV values for pyproject.toml");
            println!(
                "Use path without credentials and supply login and password through ENV variables."
            )
        }

        // verify_ssl not implemented yet
        let verify_ssl: String = self.verify_ssl.clone().unwrap_or("false".to_string());

        if verify_ssl == "true" {
            println!("SSL verification is not implemented yet!")
        }

        result_string.push_str("explicit = true");

        result_string
    }
}
