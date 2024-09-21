root_dir := justfile_directory()

_default:
	just --list

setup:
	uv venv .venv
	uv sync

build-dev:
	uv run python3 -m ensurepip
	uv tool run maturin develop -m {{root_dir}}/pyo3-async-gen/Cargo.toml

build:
	uv tool run maturin build -m {{root_dir}}/pyo3-async-gen/Cargo.toml

test: build-dev
	uv run {{root_dir}}/scratch.py
