# e-ink-image-formatter

A Rust project for a simple CLI/desktop tool that converts common image formats (such as JPG and PNG) into an e-Ink readable format for ESP32-C3 based devices.

## Vision

The vision of this project is to make preparing images for e-Ink displays straightforward and accessible:

- Load standard image formats (starting with JPG and PNG).
- Convert and optimize image data for e-Ink display constraints.
- Export output suitable for use on ESP32-C3 microcontroller projects.
- Offer a simple Rust-first workflow that can evolve from CLI to desktop experience.

## Project structure

This repository uses a shared core library plus multiple app binaries:

- `src/lib.rs` → shared core implementation
- `src/bin/cli.rs` → CLI application entrypoint
- `src/bin/desktop.rs` → desktop application entrypoint
