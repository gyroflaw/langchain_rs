# langchain_rs

[![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://github.com/gyroflaw/langchain_rs/actions/workflows/ci.yml/badge.svg
[actions]: https://github.com/gyroflaw/langchain_rs/actions/workflows/ci.yml
[Latest Version]: https://img.shields.io/crates/v/langchain_rs.svg
[crates.io]: https://crates.io/crates/langchain_rs

`langchain_rs` is a Rust implementation of LangChain, a library for building applications with Large Language Models (LLMs). This version of LangChain is specifically designed for use with Rust and provides seamless integration with LLMs.

## Features

- Integration with LLMs for natural language processing tasks
- Flexible prompt management and optimization
- Generic interface for working with different LLMs
- Support for building chains of LLM calls
- Data augmentation functionality for improved generation
- Memory management to persist state across calls

## Installation

To use `langchain_rs` in your Rust project, add the following to your `Cargo.toml` file:

> [!NOTE]  
> `openai`, `mongodb`, and `qdrant` features are enabled by default.

```toml
[dependencies]
langchain_rs = "0.0.2"
```
