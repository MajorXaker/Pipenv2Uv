# What it does

This script takes a `Pipfile` as input and generates a `pyproject.toml` file
that can be used with `uv` to install the dependencies.

The script supports the following features:

- Translates `Pipfile` sources to `pyproject.toml` sources
- Translates `Pipfile` packages to `pyproject.toml` dependencies
- Translates `Pipfile` dev-packages to `pyproject.toml` dev-dependencies
- Handles ` extras` and `index` directives in `Pipfile` packages
- Handles `verify_ssl` directive in `Pipfile` sources