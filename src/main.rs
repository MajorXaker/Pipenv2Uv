mod models;
mod processors;

use models::package::Package;
use models::pipenv::Pipenv;
use models::pipenv_content::{PipenvContent, PipenvUVInterface};
use models::source::Source;
use processors::BufferResultEnum;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn open_file(filename: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
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
                match processors::process_previous_buffer(
                    block_name.as_str(),
                    &line_buffer,
                    line.as_str(),
                ) {
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
