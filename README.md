# Mandelbrot Viewer with Rust

Made/worked on whilst at school; a tool that lets you zoom into the Mandelbrot Set within a text terminal.

 ---

Written solely in rust; you will need to have cargo installed to compile and run this.

## Building:
* Download this repo and open a terminal window in its directory.
* Run this command:
  > `cargo run --release`

The project should build and run; make sure the terminal is of an adequate font size and has large enough dimensions; as these are determined only at the start of execution.

It's reccomended that you increase the size of your terminal window for a higher resolution and frankly better looking output.

## Controls:
* '`q`' will quit the program.
* Arrow keys <sup>(and `WASD`)</sup> will do the following: 
  * `DOWN` will vertically squeeze the image.
  * `UP` will vertically stretch the image.
  
  <sup>*(these are to compensate for letters being taller than they are wide.)*<sup>

  * `LEFT` will decrement the number of mandelbrot iterations by 10.
  * `RIGHT` will increment the number of iterations by 10.
  
  <sup>*(starts at the minimum 70; just turn this up when the image starts to lack detail without being pixellated)*</sup>
  
* Scrolling will zoom in and out.
* Left clicking will shift perspective towards the location that was clicked.

##### **Disclaimer:** The zooming is not infinite; due to the finite accuracy of the variables used for calculations the image will eventually go pixelated.
