# dd-core

Simple library for developing VST2 plugins in 100% rust. The Steinberg SDK is not required (thankfully! that thing is a garbage pile). Most complexity is abstracted away and the user need only focus on what's important - processing dsp and accessing a simple immediate mode UI via conrod.

Support for audiounit and lv2 will be coming in the future.

This lib could not have been done without overdrivenpotato's hard work on the `rust-vst2` crate.

Currently the code compiles on Mac OS and Windows, with Mac being the first-class citizen over Windows. Increased and ongoing support for Linux and Windows would be nice but I currently only use Mac OS.

You could look at build.sh for an example of how to compile and package a plugin on mac - the vst-bundler script from rust-vst2 is included in the scripts directory.

More examples coming soon!

![Screenshot](screenshot.png)
