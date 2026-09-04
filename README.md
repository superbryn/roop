# roop

A lightweight terminal image viewer for terminal emulators that support the [Kitty Graphics Protocol (KGP)](https://sw.kovidgoyal.net/kitty/graphics-protocol/).

## Status

**Early WIP**

`roop` is currently under active development and is not yet ready for general use.

### Currently supports

* PNG
* JPEG
* AVIF
* BMP
* TIFF
* WEBP
* PNM
* TGA
* ICO
* HDR
* OpenEXR
* Farbfeld
* QOI
* PCX
* DDS

### In development

* GIF support
* Image scaling and positioning

## Usage

`roop` is still under development and is not yet available as an installed CLI.

For now, run it through Cargo:

```bash
cargo run -- path/to/image
```

## Requirements

A terminal emulator with **Kitty Graphics Protocol (KGP)** support, such as [Kitty](https://sw.kovidgoyal.net/kitty/) or [Ghostty](https://ghostty.org/).

## License

This project is licensed under the MIT License.
