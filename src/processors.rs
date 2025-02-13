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
        let value = parts[1].trim().to_string();
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

fn parse_package(package_line: &String, is_dev: bool) -> Package {
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
        if let Some(caps) = version_regex.captures(&extended_package_data) {
            version = caps.get(1).unwrap().as_str().trim_matches('"');
        } else {
            version = package_version;
        }

        let index_regex = regex::Regex::new(r#"index\s?=\s?"(\w+)""#).unwrap();
        let index: Option<String>;
        if let Some(caps) = index_regex.captures(&extended_package_data) {
            index = Some(caps.get(1).unwrap().as_str().trim_matches('"').to_string());
        } else {
            index = None;
        }

        let extras: Option<Vec<String>>;
        let extras_regex = regex::Regex::new(r#"extras\s?=\s?\[(["\w,]+)]"#).unwrap();
        if let Some(caps) = extras_regex.captures(&extended_package_data) {
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

        let package = Package {
            name: package_name.to_string(),
            version: version.to_string(),
            index: index,
            extras,
            is_dev,
        };
        package
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
