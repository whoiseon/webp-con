# WebP Con

A fast, multi-threaded CLI tool for converting images to **WebP** with
recursive directory support.

Built with **Rust**, **Rayon**, **Indicatif**, and **Clap**.

![Rust](https://img.shields.io/badge/Rust-2024-orange)
![Version](https://img.shields.io/badge/version-0.1.0-green)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

-   🚀 Fast multi-threaded conversion using Rayon
-   📁 Recursive directory conversion
-   🖼️ Supports JPG, JPEG, PNG, BMP and WebP
-   🎯 Adjustable WebP quality (1--100)
-   📊 Progress bar for directory conversion
-   📋 Conversion summary
-   🛡️ Skip existing files by default
-   🔄 Optional overwrite mode
-   📦 Installable as a standalone CLI

------------------------------------------------------------------------

## Installation

### From source

``` bash
git clone <repository-url>
cd webp-con

cargo install --path .
```

### Verify installation

``` bash
webp-con --version
```

Expected output:

``` text
webp-con 0.1.0
```

You can also display the built-in help:

``` bash
webp-con --help
```

------------------------------------------------------------------------

## Quick Start

Convert all images in a directory:

``` bash
webp-con convert ./images -o ./output
```

Convert a single image:

``` bash
webp-con convert image.jpg
```

------------------------------------------------------------------------

## Usage

### Convert a directory recursively

``` bash
webp-con convert images
```

### Specify an output directory

``` bash
webp-con convert images -o output
```

### Overwrite existing files

``` bash
webp-con convert images --overwrite
```

### Adjust WebP quality

``` bash
webp-con convert images -q 75
```

Quality range: **1--100** (default: **80**)

------------------------------------------------------------------------

## Supported Formats

### Input

-   JPG
-   JPEG
-   PNG
-   BMP
-   WebP

### Output

-   WebP

------------------------------------------------------------------------

## Example

``` bash
webp-con convert ./images \
    -o ./output \
    -q 80 \
    --overwrite
```

------------------------------------------------------------------------

## Project Structure

``` text
src/
├── main.rs          # CLI entry point
├── lib.rs           # Library entry
├── converter.rs     # Image conversion
├── summary.rs       # Conversion summary
└── utils.rs         # Helper functions
```

------------------------------------------------------------------------

## Performance

Benchmark (100 images)

  Version                   Time
  ------------------ -----------
  Sequential           1m 55.82s
  Parallel (Rayon)        27.80s

CPU utilization

  Version           CPU
  ------------ --------
  Sequential      \~99%
  Parallel       \~626%

The parallel implementation achieved approximately **4.2× faster
execution** by utilizing multiple CPU cores.

------------------------------------------------------------------------

## Technologies

-   Rust
-   Clap
-   Anyhow
-   WalkDir
-   Rayon
-   Indicatif
-   Image
-   WebP

------------------------------------------------------------------------

## Roadmap

-   [ ] AVIF support
-   [ ] Resize command
-   [ ] Batch optimization presets
-   [ ] Configuration file
-   [ ] Benchmark command
-   [ ] Metadata preservation

------------------------------------------------------------------------

## License

MIT
