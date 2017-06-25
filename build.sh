#!/bin/bash

DYLIB_FILE=./target/release/libdd_core.dylib
VST_NAME=DDOverdriveGUI

rm -rf ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst 2> /dev/null
rm -rf $DYLIB_FILE 2> /dev/null

cargo build --release

if [ -f $DYLIB_FILE ]; then
    vst-bundler $VST_NAME $DYLIB_FILE &&
    mv -v ./$VST_NAME.vst ~/Library/Audio/Plug-Ins/VST/

    du -sh ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst
    echo "done."
    osascript -e 'display notification "Build successful." with title "DDOverdriveGUI"'
else
    echo "error."
    osascript -e 'display notification "Build failed." with title "DDOverdriveGUI"'
fi
