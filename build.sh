#!/bin/bash

DYLIB_FILE=libdd_core.dylib
VST_NAME=DDOverdriveGUI

rm -rf ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst
# rm ./target/release/*.d
rm ./target/release/*.rlib
rm ./target/release/*.dylib

cargo build --release

vst-bundler $VST_NAME ./target/release/$DYLIB_FILE &&
mv -v ./$VST_NAME.vst ~/Library/Audio/Plug-Ins/VST/

du -sh ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst