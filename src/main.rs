mod models;
mod processors;
mod utils;

use crate::utils::get_output_file_name;
use models::package::Package;
use models::pipenv::Pipenv;
use models::pipenv_content::{PipenvContent, PipenvUVInterface};
use models::source::Source;
use processors::BufferResultEnum;
use std::env;
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
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let full_path = current_dir.join(filename).to_str().unwrap().to_string();
    File::create(&filename).expect(&format!(
        "Unable to create new file in current directory: {}",
        full_path,
    ))
}

fn process_data() -> Result<(), std::io::Error> {
    println!("Reading Pipfile's content");
    let original_file = "Pipfile";
    let reader = open_file(original_file).expect("Cannot open Pipfile");
    let file_content: PipenvContent =
        read_lines(reader).expect("Error while reading Pipfile content");

    let exported_lines: String = file_content.export();

    let is_docker = env::var("DOCKER").unwrap_or("0".to_string()) == "1";
    let result_filename = get_output_file_name(is_docker);

    println!("Saving processed data to {}", result_filename);

    let mut export_file: File = create_file_for_writing(&result_filename);

    write!(export_file, "{}", exported_lines).expect("Writing to file failed");

    Ok(())
}

fn main() {
    match process_data() {
        Ok(_) => println!("Processing completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
