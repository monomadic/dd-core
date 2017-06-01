#!/bin/bash

cargo build --release
rm -rf ~/Library/Audio/Plug-Ins/VST/DDGui.vst

DYLIB_FILE=libdd_gui.dylib
VST_NAME=DDGui

rm -rf ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst
vst-bundler $VST_NAME ./target/release/$DYLIB_FILE &&
mv -v ./$VST_NAME.vst ~/Library/Audio/Plug-Ins/VST/

du -sh ~/Library/Audio/Plug-Ins/VST/DDGui.vst
