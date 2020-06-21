# ClausiusWebApp
Clausius Application with a web interface.

## Technologies
HTML, CSS, and JS define the structure and design of the site, as well as the drawing of the "location indicator".

The mathematical calculations are done via Rust WebAssembly, where the mathematics are written in Rust. Javascript asks compiled Rust WebAssembly for the values and then displays them in the display view. WebAssembly is a good choice for this app because compiled Rust is much faster than Javascript, and the calculations need to be done quickly.

## Supported Development Environment

All of the development for this project was done on a Windows computer in VS Code. However, the Rust WebAssembly was compiled exclusively in a Linux Ubuntu environment.

# Testing
The primary testing for Clausius will be through unit testing. Test cases are laid out below.

We only lightly test the compressed liquid region because the calculations there are simplistic and the region is small, so we just make sure we don't get NaNs out.

## Test Cases

Top-Left of chart:
699.0 C / 0.0040 m3/kg == 250 kg/m3

Top-right of chart:
700.0 C / 4.1419 m3/kg == 0.24143509017 kg/m3

Bottom-right of chart:
1.0 C / 204.9495 m3/kg == 0.00487925074 kg/m3

Superheated Point 1:
570.01 C / 0.1004 m3/kg == 9.96015936255 kg/m3

Superheated Point 2:
420.38 C / 0.0037 m3/kg == 270.27027027 kg/m3

Superheated Point 3:
393.16 C / 1.2172 m3/kg == 0.82155767334 kg/m3

Superheated Point 4:
334.99 C / 0.0360 m3/kg == 27.7777777778 kg/m3

Superheated Point 5:
245.84 C / 13.1323 m3/kg == 0.07614812332 kg/m3

Superheated Point 6:
160.46 C / 0.3631 m3/kg == 2.75406224181 kg/m3

Superheated Point 7:
260.68 C / 0.0501 m3/kg == 19.9600798403 kg/m3

Superheated Point 8 (near Critical Point):
389.41 C / 0.0016 m3/kg == 625.0 kg/m3

Saturated Point 1 (near Critical Point):
363.14 C / 0.0040 m3/kg == 250.0 kg/m3

Saturated Point 2:
289.01 C / 0.0035 m3/kg == 285.714285714 kg/m3

Saturated Point 3:
191.42 C / 0.0102 m3/kg == 98.0392156863 kg/m3

Saturated Point 4:
101.34 C / 0.0372 m3/kg == 26.8817204301 kg/m3

Saturated Point 5:
4.69 C / 2.4028 m3/kg == 0.41618112202 kg/m3

Saturated Point 6:
8.45 C / 57.1245 m3/kg == 0.01750562368 kg/m3

Saturated Point 7:
6.57 C / 129.1554 m3/kg == 0.00774261083 kg/m3

Saturated Point 8:
99.46 C / 1.6565 m3/kg == 0.60368246302 kg/m3

Saturated Point 9:
198.93 C / 0.1262 m3/kg == 7.92393026941 kg/m3

Saturated Point 10:
301.21 C / 0.0206 m3/kg == 48.5436893204 kg/m3

Saturated Point 11:
180.16 C / 0.0993 m3/kg = 10.0704934542 kg/m3