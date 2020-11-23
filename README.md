# mcrestool

## Building

You need the following set up:

 - a **Rust** development environment (cargo)
 - a **C++** development environment (cmake)
 - an up to date version of **Qt 5**
 - **extra-cmake-modules** from your distro's repositories
 - [**corrosion**][1] (for Gentoo: *dev-util/corrosion::2xsaiko*)
 - the **cxxbridge** command from [my cxx fork][2]

Then, run:

    $ mkdir build
    $ cd build
    $ cmake ..
    $ make

That's it! The binary should be output at build/mcrestool.

[1]: https://github.com/AndrewGaspar/corrosion
[2]: https://github.com/2xsaiko/cxx
