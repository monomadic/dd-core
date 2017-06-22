#!/bin/bash

DYLIB_FILE=libdd_core.dylib
VST_NAME=DDOverdriveGUI

m -rf ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst 2> /dev/null
rm ./target/release/*.d 2> /dev/null
rm ./target/release/*.rlib 2> /dev/null
rm ./target/release/*.dylib 2> /dev/null

cargo build --release

if [ -f ./target/release/$DYLIB_FILE ]; then
    vst-bundler $VST_NAME ./target/release/$DYLIB_FILE &&
    mv -v ./$VST_NAME.vst ~/Library/Audio/Plug-Ins/VST/

    du -sh ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst
    echo "done."
    osascript -e 'display notification "Build successful." with title "DDOverdriveGUI"'
else
    echo "error."
    osascript -e 'display notification "Build failed." with title "DDOverdriveGUI"'
fi
