use crate::models::package::{Package, UVPackage};
use crate::models::pipenv::Pipenv;
use crate::models::source::{Source, UVSource};

pub trait PipenvUVInterface {
    fn export(&self) -> String;
    fn _export_project_part(&self) -> (String, Vec<&Package>, Vec<&Package>);
    fn _prepare_dev_dependencies(&self, dev_dependencies: Vec<&Package>) -> String;
    fn _prepare_sources(&self, indexed_packages: Vec<&Package>) -> String;
}

pub struct PipenvContent {
    pub sources: Vec<Source>,
    pub packages: Vec<Package>,
    pub pipenv: Pipenv,
}

impl PipenvUVInterface for PipenvContent {
    fn _export_project_part(&self) -> (String, Vec<&Package>, Vec<&Package>) {
        let mut resulting_lines: String = String::new();

        let static_lines: String = r#"[project]
name = "type-your-project-name-here"
version = "0.1.0"
description = "Add your description here"
readme = "README.md""#
            .to_string();

        // start with general project data
        resulting_lines.push_str(&static_lines);
        resulting_lines.push('\n');

        let python_v_line: String = format!("requires-python = \"{}\"", self.pipenv.python_version);
        resulting_lines.push_str(&python_v_line);
        resulting_lines.push('\n');

        let mut dev_dependencies: Vec<&Package> = Vec::new();
        let mut indexed_packages: Vec<&Package> = Vec::new();

        resulting_lines.push_str("dependencies = [\n");
        for package in &self.packages {
            if package.is_dev {
                //     dev packages are declared later in a separate group
                dev_dependencies.push(package);
                continue;
            }
            resulting_lines.push('\t');
            resulting_lines.push_str(&package.as_uv());
            resulting_lines.push_str(",\n");

            if package.index.is_some() {
                //     packages are declared later in a separate group
                indexed_packages.push(package);
            }
        }
        resulting_lines.push_str("]\n");

        (resulting_lines, dev_dependencies, indexed_packages)
    }

    fn _prepare_dev_dependencies(&self, dev_dependencies: Vec<&Package>) -> String {
        let mut resulting_lines: String = String::new();
        resulting_lines.push_str("[dependency-groups]\n");
        resulting_lines.push_str("dev = [\n");
        for package in dev_dependencies {
            resulting_lines.push('\t');
            resulting_lines.push_str(&package.as_uv());
            resulting_lines.push_str(",\n");
        }
        resulting_lines.push_str("]\n");
        resulting_lines
    }

    fn _prepare_sources(&self, indexed_packages: Vec<&Package>) -> String {
        let mut resulting_lines: String = String::new();

        for source in &self.sources {
            resulting_lines.push_str("[[tool.uv.index]]\n");
            resulting_lines.push_str(&source.as_uv());
            resulting_lines.push_str("\n\n");
        }

        if !indexed_packages.is_empty() {
            resulting_lines.push_str("[tool.uv.sources]\n");
            for dependant_package in indexed_packages {
                let ln: String = dependant_package.state_source();
                resulting_lines.push_str(&ln);
                resulting_lines.push('\n');
            }
        }

        resulting_lines
    }

    fn export(&self) -> String {
        let mut resulting_lines: String = String::new();

        let (project_lines, dev_dependencies, indexed_packages) = self._export_project_part();

        resulting_lines.push_str(&project_lines);
        resulting_lines.push('\n');

        // adding some dev dependecies if there are any
        if !dev_dependencies.is_empty() {
            let dev_dependencies_lines = self._prepare_dev_dependencies(dev_dependencies);
            resulting_lines.push_str(&dev_dependencies_lines);
            resulting_lines.push('\n');
        }

        // setting info on indexes and sources
        if !self.sources.is_empty() {
            let sources_lines = self._prepare_sources(indexed_packages);
            resulting_lines.push_str(&sources_lines);
            resulting_lines.push('\n');
        }

        println!(
            "Allow pre-releases handling not yet implemented. Current: {}",
            self.pipenv
                .allow_prereleases
                .clone()
                .unwrap_or("false".to_string())
        );

        resulting_lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_project_part() {
        let packages = vec![
            Package {
                name: "requests".to_string(),
                version: "2.25.1".to_string(),
                index: None,
                extras: None,
                is_dev: false,
            },
            Package {
                name: "uvicorn".to_string(),
                version: "0.14.0".to_string(),
                index: None,
                extras: None,
                is_dev: true,
            },
        ];

        let pipenv = Pipenv {
            python_version: "3.8".to_string(),
            allow_prereleases: Some("true".to_string()),
        };

        let sources = vec![Source {
            name: "pypi".to_string(),
            url: "https://pypi.org/simple".to_string(),
            verify_ssl: Some("true".to_string()),
        }];

        let pipenv_content = PipenvContent {
            packages,
            pipenv,
            sources,
        };

        let (_, dev_packages, indexed_packages) = pipenv_content._export_project_part();

        assert_eq!(dev_packages.len(), 1);
        assert_eq!(indexed_packages.len(), 0);
    }

    #[test]
    fn test_prepare_dev_dependencies() {
        let packages = vec![
            Package {
                name: "requests".to_string(),
                version: "==2.25.1".to_string(),
                index: None,
                extras: None,
                is_dev: true,
            },
            Package {
                name: "uvicorn".to_string(),
                version: "==0.14.0".to_string(),
                index: None,
                extras: None,
                is_dev: true,
            },
        ];

        let pipenv_content = PipenvContent {
            packages,
            pipenv: Pipenv {
                python_version: "3.8".to_string(),
                allow_prereleases: Some("true".to_string()),
            },
            sources: vec![],
        };

        let packages_dup: Vec<&Package> = pipenv_content.packages.iter().collect();

        let lines = pipenv_content._prepare_dev_dependencies(packages_dup);

        assert_eq!(
            lines,
            "[dependency-groups]\ndev = [\n\t\"requests==2.25.1\",\n\t\"uvicorn==0.14.0\",\n]\n"
        );
    }

    #[test]
    fn test_prepare_sources() {
        let sources = vec![Source {
            name: "pypi".to_string(),
            url: "https://pypi.org/simple".to_string(),
            verify_ssl: Some("true".to_string()),
        }];

        let pipenv_content = PipenvContent {
            packages: vec![],
            pipenv: Pipenv {
                python_version: "3.8".to_string(),
                allow_prereleases: Some("true".to_string()),
            },
            sources,
        };
        let packages_dup: Vec<&Package> = pipenv_content.packages.iter().collect();

        let lines = pipenv_content._prepare_sources(packages_dup);

        assert_eq!(lines.lines().count(), 5);
    }
}
