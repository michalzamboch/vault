#!/bin/bash

binaryFolders=("../target" 
    "../src/app_controller/target" 
    "../src/app_model/target" 
    "../src/app_view/target" 
    "../src/app_common/target" 
    "../src/app_tests/target")

for DIR in ${binaryFolders[@]}; do
    if [ -d "$DIR" ];
    then
        echo "Deleting binaries: $DIR"
        rm -rf $DIR
    else
        echo "Already cleaned: $DIR"
    fi
done

echo "Binaries cleaned..."
