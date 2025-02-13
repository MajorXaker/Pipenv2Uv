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

        result_string.push_str(r#"""#);

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
