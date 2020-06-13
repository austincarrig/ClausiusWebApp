# ClausiusWebApp
Clausius Application with a web interface.

## Technologies
HTML, CSS, and JS define the structure and design of the site, as well as the drawing of the "location indicator".

The mathematical calculations are done via Rust WebAssembly, where the mathematics are written in Rust. Javascript asks compiled Rust WebAssembly for the values and then displays them in the display view. WebAssembly is a good choice for this app because compiled Rust is much faster than Javascript, and the calculations need to be done quickly.

## Supported Development Environment

All of the development for this project was done on a Windows computer in VS Code. However, the Rust WebAssembly was compiled exclusively in a Linux Ubuntu environment.