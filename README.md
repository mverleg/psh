
Psh
===============================

Psh is a mix of Python and shell. It adds some tricks to standard Python 3, to make it more convenient as a replacement for shell scripts.

Features include:

* Get argument values from different sources with `?`
* Call external commands easily with `$`
* Built-in utilities to prevent boilerplate.

Usage
-------------------------------

Downloadable binaries are planned, but for now you can easily build using either Docker::

    docker build -t psh .
    docker run --rm -it "$(pwd)":/code psh example/fac.psh

or with Cargo::

    cargo +nightly run --release -- example/fac.psh

Examples
-------------------------------

Will be added later! Maybe!
