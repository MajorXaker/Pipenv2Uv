# Pipenv2UV
### What is it for

`Pipenv` and `uv` are two popular Python package managers which different syntax for 
describing dependencies of the project.
This script takes a `Pipfile` as input and generates a `pyproject.toml` file
that can be used with `uv` to install the dependencies.

Pipenv2UV is overwrite safe, if the output file already exists a new one will be created.

### Install and launch

1. Pre-built binaries in [releases](https://github.com/majorxaker/pipenv2uv/releases).
    
    Currently available for Linux and MacOS.


2. Dockerized launch
    ```bash
   docker build -t pipenv2uv .
   docker run --volume ./Pipfile:/app/Pipfile --volume ./output/:/app/output/
   ```
3. Build from source (requires rust toolkit)
    ```bash
    cargo build --release
    ./target/release/pipenv2uv
    ```
   


