#!/bin/bash

image_directory="pre_images"
default_image_name="master_icon.png"
image_default=$(find $image_directory -name "$default_image_name" -print -quit 2>/dev/null)

output_directory="extension/icons"

image_sizes="16,32,48,128,256"

if [ ! -d "$image_directory" ]; then
    echo -e "Directory '$image_directory' wasn't avaiable and has been created!"
    mkdir $image_directory
elif [ "$image_default" ]; then
    echo -e "Found image '$image_default'!"

    cd icon-gen || exit 1
    
    cargo run --release -- \
        --input ../"$image_default" \
        --output ../"$output_directory" \
        --name icon \
        --sizes $image_sizes
else
    echo -e "image '$default_image_name' wasn't found inside '$image_directory/'!\nChange name given in image-gen.sh line 4 if the name is different"
    exit 1    
fi

