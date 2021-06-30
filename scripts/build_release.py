#!/usr/bin/env python3

def cargo_toml(pkg: dict) -> str :
    return ('''[package]
name = "{name}"
version = "{version}"
authors = [{authors}]
edition = "2018"
description = "{description}"
license = "MIT"
documentation = "https://docs.rs/{name}"
repository = "{repository}"

[lib]
name = "{name}"
path = "src/lib.rs"

[[bin]]
name = "{name}"
path = "src/main.rs"
required-features = ["cli"]

[dependencies]
{dependencies_lib}

# Only for the CLI
{dependencies_cli}

[features]
{dependencies_feat}'''
        .format(name=pkg.name)
        .format(version=pkg.version)
        .format(authors=pkg.authors)
        .format(description=pkg.description)
        .format(repository=pkg.repository)
        .format(dependencies_lib=pkg.dependencies_lib)
        .format(dependencies_cli=pkg.dependencies_cli)
        .format(dependencies_feat=pkg.dependencies_feat))

