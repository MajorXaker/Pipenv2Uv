use crate::models::package::Package;
use crate::models::pipenv::Pipenv;
use crate::models::source::Source;
use std::collections::HashMap;

fn parse_to_hashmap(source_block: &Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in source_block {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('=').collect();
        let key = parts[0].trim().to_string();
        let value = parts[1].trim().trim_matches('"').to_string();
        map.insert(key, value);
    }
    map
}

pub fn parse_source_block(source_block: &Vec<String>) -> Source {
    let lines_map = parse_to_hashmap(source_block);

    let name: String = lines_map.get("name").unwrap().trim_matches('"').to_string();

    Source {
        name,
        url: lines_map
            .get("url")
            .unwrap()
            .clone()
            .trim_matches('"')
            .to_string(),
        verify_ssl: lines_map.get("verify_ssl").cloned(),
    }
}

pub fn parse_pipenv_block(pipenv_block: &Vec<String>) -> Pipenv {
    let lines_map = parse_to_hashmap(pipenv_block);

    Pipenv {
        python_version: lines_map.get("python_version").unwrap().clone(),
        allow_prereleases: lines_map.get("allow_prereleases").cloned(),
    }
}

fn parse_package(package_line: &str, is_dev: bool) -> Package {
    let split: Option<(&str, &str)> = package_line.split_once('=');

    let sp2 = split.unwrap_or(("", ""));

    if sp2.0.trim().is_empty() {
        return Package {
            name: sp2.1.trim().to_string(),
            version: "".to_string(),
            index: None,
            extras: None,
            is_dev,
        };
    }

    let package_name: &str = split.unwrap().0.trim();
    let package_version: &str = split.unwrap().1.trim();

    let extended_package_data: &str;

    if package_version.starts_with('{') {
        extended_package_data = package_version
            .trim_start_matches('{')
            .trim_end_matches('}')
            .trim();

        let version_regex = regex::Regex::new(r#"version\s?=\s?"([\d<>=,.*]+)""#).unwrap();

        let version: &str;
        if let Some(caps) = version_regex.captures(extended_package_data) {
            version = caps.get(1).unwrap().as_str().trim_matches('"');
        } else {
            version = package_version;
        }

        let index_regex = regex::Regex::new(r#"index\s?=\s?"(\w+)""#).unwrap();
        let index: Option<String>;
        if let Some(caps) = index_regex.captures(extended_package_data) {
            index = Some(caps.get(1).unwrap().as_str().trim_matches('"').to_string());
        } else {
            index = None;
        }

        let extras: Option<Vec<String>>;
        let extras_regex = regex::Regex::new(r#"extras\s?=\s?\[(["\w,]+)]"#).unwrap();
        if let Some(caps) = extras_regex.captures(extended_package_data) {
            extras = Some(
                caps.get(1)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.trim_matches('"').to_string())
                    .collect(),
            );
        } else {
            extras = None;
        }

        Package {
            name: package_name.to_string(),
            version: version.to_string(),
            index,
            extras,
            is_dev,
        }
    } else {
        let package: Package = Package {
            name: package_name.to_string(),
            version: package_version.to_string().trim_matches('"').to_string(),
            index: None,
            extras: None,
            is_dev,
        };
        package
    }
}

pub fn parse_packages_block(packages_block: &Vec<String>, is_dev: bool) -> Vec<Package> {
    let mut packages = Vec::new();

    for line in packages_block {
        if line.starts_with('#') || line.trim().is_empty() {
            // this is a comment or empty line
            continue;
        }
        let package = parse_package(line, is_dev);
        packages.push(package);
    }
    packages
}

pub enum BufferResultEnum<A, B, C> {
    Source(A),
    Pipenv(B),
    Packages(C),
    Unknown,
}

pub fn process_previous_buffer(
    block_name: &str,
    line_buffer: &Vec<String>,
    line: &str,
) -> BufferResultEnum<Source, Pipenv, Vec<Package>> {
    match block_name {
        "source" => {
            // println!("Processing source block");
            let source_block = parse_source_block(line_buffer);
            BufferResultEnum::Source(source_block)
        }
        "pipenv" => {
            // println!("Processing pipenv block");
            let pipenv_block = parse_pipenv_block(line_buffer);
            BufferResultEnum::Pipenv(pipenv_block)
        }
        "packages" => {
            // println!("Processing packages block");
            let packages = parse_packages_block(line_buffer, false);
            BufferResultEnum::Packages(packages)
        }
        "dev-packages" => {
            // println!("Processing dev-packages block");
            let packages = parse_packages_block(line_buffer, true);
            BufferResultEnum::Packages(packages)
        }
        _ => {
            println!("Unknown block: {}", line);
            BufferResultEnum::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_source_block() {
        let source_block = vec![
            String::from("name = \"pypi\""),
            String::from("url = \"https://pypi.org/simple\""),
            String::from("verify_ssl = \"true\""),
        ];

        let source = parse_source_block(&source_block);

        assert_eq!(source.name, "pypi");
        assert_eq!(source.url, "https://pypi.org/simple");
        assert_eq!(source.verify_ssl.unwrap(), "true");
    }

    #[test]
    fn test_parse_pipenv_block() {
        let pipenv_block = vec![
            String::from("python_version = \"3.8\""),
            String::from("allow_prereleases = true"),
        ];

        let pipenv = parse_pipenv_block(&pipenv_block);

        assert_eq!(pipenv.python_version, "3.8");
        assert_eq!(pipenv.allow_prereleases.unwrap(), "true");
    }

    #[test]
    fn test_parse_package() {
        let package_line = String::from("requests = {version=\">=2.25.1\", extras=[socks]}");

        let package = parse_package(&package_line, false);

        assert_eq!(package.name, "requests");
        assert_eq!(package.version, ">=2.25.1");
        assert_eq!(package.extras.unwrap(), vec!["socks".to_string()]);
    }

    #[test]
    fn test_parse_packages_block() {
        let packages_block = vec![
            String::from("requests = \">=2.25.1\""),
            String::from("requests = {version=\">=2.25.1\", extras=[socks]}"),
        ];

        let packages = parse_packages_block(&packages_block, false);

        assert_eq!(packages.len(), 2);
    }
}
