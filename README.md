# Buddhabrot Renderer

> A fast Buddhabrot renderer written in Rust.

## Description

The Buddhabrot is an extension of the Mandelbrot set. 
Instead of simply testing if a complex number `z` escapes the set after `n` iterations, we track where `z` goes and leave a trail for each iteration. 
This allows to the inner, 'black' part of the Mandelbrot set to gain more complexity and detail.


For more information, visit:
- [The Buddhabrot Technique by Melinda Green](https://superliminal.com/fractals/bbrot/)
- [Buddhabrot (Wikipedia)](https://en.wikipedia.org/wiki/Buddhabrot)


## Features

- Multithreading
- Support for both EXR and PNG image formats
- The three-color Buddhabrot (Nebulabrot)


## Screenshots

![Buddhabrot 4k-res 50x-samples 500k-iters colorless](./screenshots/processed-4k-50x-500k-colorless.png)
