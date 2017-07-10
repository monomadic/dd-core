#!/bin/bash

echo $PATH

VST_NAME=DDOverdrive
EXAMPLE=overdrive
DYLIB_FILE=./target/release/examples/lib$EXAMPLE.dylib

rm -rf ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst 2> /dev/null
rm -rf $DYLIB_FILE 2> /dev/null

cargo build --release --example $EXAMPLE
mv ./target/release/examples/*.dylib $DYLIB_FILE 2> /dev/null

if [ -f $DYLIB_FILE ]; then
    bash ./scripts/vst-bundler.sh $VST_NAME $DYLIB_FILE &&
    mv -v ./$VST_NAME.vst ~/Library/Audio/Plug-Ins/VST/

    du -sh ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst
    echo "done."
    osascript -e 'display notification "Build successful." with title "DDOverdriveGUI"'
else
    echo "build failed."
    osascript -e 'display notification "Build failed." with title "DDOverdriveGUI"'
fi
