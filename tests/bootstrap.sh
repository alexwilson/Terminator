#!/bin/bash

# Copy configuration file.
cp tests/terminator-test.yml  ~/.terminator.yml
sed -i.bak "s#__HOME__#$HOME#"g ~/.terminator.yml

# Generate some totally top secret data.
mkdir -p ~/IMPORTANT-SSH-KEYS
touch ~/IMPORTANT-SSH-KEYS/{id_rsa,id_dsa,id_nsa}

mkdir -p ~/PRODUCTION-ASSETS
touch ~/PRODUCTION-ASSETS/{copyrighted_image1.jpg,copyrighted_image2.jpg}
