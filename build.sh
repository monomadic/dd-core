#!/bin/bash

DYLIB_FILE=./target/release/examples/libtest.dylib
VST_NAME=DDPlugTest

rm -rf ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst 2> /dev/null
rm -rf $DYLIB_FILE 2> /dev/null

cargo build --release --example test
mv ./target/release/examples/*.dylib ./target/release/examples/libtest.dylib

if [ -f $DYLIB_FILE ]; then
    vst-bundler $VST_NAME $DYLIB_FILE &&
    mv -v ./$VST_NAME.vst ~/Library/Audio/Plug-Ins/VST/

    du -sh ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst
    echo "done."
    osascript -e 'display notification "Build successful." with title "DDOverdriveGUI"'
else
    echo "lib not found!"
    osascript -e 'display notification "Build failed." with title "DDOverdriveGUI"'
fi
