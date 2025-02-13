mod models;
mod processors;

use models::package::{Package, UVPackage};
use models::pipenv::Pipenv;
use models::source::{Source, UVSource};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
fn open_file(filename: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}

pub trait PipenvUVInterface {
    fn export(&self) -> String;
}

struct PipenvContent {
    sources: Vec<Source>,
    packages: Vec<Package>,
    pipenv: Pipenv,
}

impl PipenvUVInterface for PipenvContent {
    fn export(&self) -> String {
        let mut resulting_lines: String = String::new();

        let static_lines: String = r#"[project]
name = "description-processor"
version = "0.1.0"
description = "Add your description here"
readme = "README.md""#
            .to_string();

        // start with general project data
        resulting_lines.push_str(&static_lines);
        resulting_lines.push_str("\n");

        let python_v_line: String = format!("requires-python = \"{}\"", self.pipenv.python_version);
        resulting_lines.push_str(&python_v_line);
        resulting_lines.push_str("\n");

        let mut dev_dependencies: Vec<&Package> = Vec::new();
        let mut indexed_packages: Vec<&Package> = Vec::new();

        resulting_lines.push_str("dependencies = [\n");
        for package in &self.packages {
            if package.is_dev {
                //     dev packages are declared later in a separate group
                dev_dependencies.push(package);
                continue;
            }
            resulting_lines.push_str("\t");
            resulting_lines.push_str(&package.as_uv());
            resulting_lines.push_str(", \n");

            if package.index.is_some() {
                //     packages are declared later in a separate group
                indexed_packages.push(package);
            }
        }
        resulting_lines.push_str("]\n\n");

        // adding some dev dependecies if there are any
        if dev_dependencies.len() > 0 {
            resulting_lines.push_str("[dependency-groups]\n");
            resulting_lines.push_str("dev = [\n");
            for package in dev_dependencies {
                resulting_lines.push_str("\t");
                resulting_lines.push_str(&package.as_uv());
                resulting_lines.push_str(", \n");
            }
            resulting_lines.push_str("]\n\n");
        }

        // setting info on indexes and sources
        if self.sources.len() > 0 {
            for source in &self.sources {
                resulting_lines.push_str("[[tool.uv.index]]\n");
                resulting_lines.push_str(&source.as_uv());
                resulting_lines.push_str("\n\n");
            }

            resulting_lines.push_str("[tool.uv.sources]\n");
            for dependant_package in indexed_packages {
                let ln: String = dependant_package.state_source();
                resulting_lines.push_str(&ln);
                resulting_lines.push_str("\n");
            }
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

fn read_lines(reader: BufReader<File>) -> Result<PipenvContent, std::io::Error> {
    let mut line_buffer = Vec::<String>::new();
    let mut block_name = "empty".to_string();

    let mut sources: Vec<Source> = Vec::new();
    let mut packages: Vec<Package> = Vec::new();
    let mut pipenv: Pipenv = Pipenv {
        python_version: "".to_string(),
        allow_prereleases: None,
    };

    for line in reader.lines() {
        let line = line?;

        if line.starts_with('[') && line.ends_with(']') {
            let new_block_name = line
                .trim_start_matches('[')
                .trim_end_matches(']')
                .to_string();

            if !line_buffer.is_empty() {
                match process_previous_buffer(block_name.as_str(), &line_buffer, line.as_str()) {
                    BufferResultEnum::SourceResult(processed_source) => {
                        sources.push(processed_source);
                    }
                    BufferResultEnum::PipenvResult(processed_pipenv) => {
                        pipenv = processed_pipenv;
                    }
                    BufferResultEnum::PackagesResult(processed_packages) => {
                        packages.extend(processed_packages);
                    }
                    _ => {}
                }
            }
            block_name = new_block_name;
            line_buffer.clear();
        } else {
            line_buffer.push(line);
        }
    }
    Ok(PipenvContent {
        sources,
        packages,
        pipenv,
    })
}

enum BufferResultEnum<A, B, C> {
    SourceResult(A),
    PipenvResult(B),
    PackagesResult(C),
    UnknownResult,
}

fn process_previous_buffer(
    block_name: &str,
    line_buffer: &Vec<String>,
    line: &str,
) -> BufferResultEnum<Source, Pipenv, Vec<Package>> {
    match block_name {
        "source" => {
            // println!("Processing source block");
            let source_block = processors::parse_source_block(line_buffer);
            BufferResultEnum::SourceResult(source_block)
        }
        "pipenv" => {
            // println!("Processing pipenv block");
            let pipenv_block = processors::parse_pipenv_block(line_buffer);
            BufferResultEnum::PipenvResult(pipenv_block)
        }
        "packages" => {
            // println!("Processing packages block");
            let packages = processors::parse_packages_block(line_buffer, false);
            BufferResultEnum::PackagesResult(packages)
        }
        "dev-packages" => {
            // println!("Processing dev-packages block");
            let packages = processors::parse_packages_block(line_buffer, true);
            BufferResultEnum::PackagesResult(packages)
        }
        _ => {
            println!("Unknown block: {}", line);
            BufferResultEnum::UnknownResult
        }
    }
}

fn create_file_for_writing(filename: &str) -> File {
    let mut new_filename = String::from(filename);
    let mut counter = 1;
    while std::path::Path::new(&new_filename).exists() {
        println!("File {} already exists, creating new", new_filename);
        new_filename = format!("new-{}-{}", counter, filename);
        counter += 1;
    }
    let export_file = File::create(&new_filename).expect("Unable to create file");
    export_file
}

fn main() {
    println!("Reading Pipfile's content");
    let filename = "Pipfile";
    let reader = open_file(filename).unwrap();
    let file_content: PipenvContent = read_lines(reader).unwrap();

    let exported_lines: String = file_content.export();

    let filename = "pyproject.toml".to_string();

    let mut export_file: File = create_file_for_writing(&filename);

    write!(export_file, "{}", exported_lines).expect("Writing failed");
}
