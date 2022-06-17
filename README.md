# Mandelbrot Viewer with Rust

Made/worked on whilst at school; a tool that lets you zoom into the Mandelbrot Set within a text terminal.

 ---

Written solely in rust; you will need to have cargo installed to compile and run this.

## Building:
* Download this repo and open a terminal window in its directory.
* Run this command:
  > `cargo run`

The project should build and run; make sure the terminal is of an adequate font size and has large enough dimensions; as these are determined only at the start of execution.

It's reccomended that you increase the size of your terminal window for a higher resolution and frankly better looking output.

## Controls:
* '`q`' will quit the program.
* '`-`' and '`=`' will squeeze and stretch your perspective vertically; to compensate for string characters usually being taller than they are wide.
* the `UP` and `DOWN` arrow keys will increment and decrement respectively the number of iterations; and thus the detail of the fractal.
* Scrolling will zoom in and out.
* Left clicking will shift perspective towards the location that was clicked.

##### **Disclaimer:** The zooming is not infinite; due to the finite accuracy of the variables used for calculations the image will eventually go pixelated.
