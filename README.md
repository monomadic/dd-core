# dd-core

DD-Core provides enough functionality to develop VST2 plugins in 100% rust. Most of the grunt work is abstracted away and the user need only focus on what's important - processing dsp and accessing a simple immediate mode UI via conrod.

Currently the code does compile in Mac OS and Windows, with Mac being the first-class citizen over Windows. Increased and ongoing support for Linux and Windows would be nice but I currently only use Mac OS.

You could look at build.sh for an example of how to compile and package a plugin on mac - the vst-bundler script from rust-vst2 is included in the scripts directory.

More examples coming soon!

![Screenshot](screenshot.png)
