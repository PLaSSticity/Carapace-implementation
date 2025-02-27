#!/bin/sh
#Meant to be run in a directory holding info-flow-library, servo-ifc.nosync, and mk48-ifc.nosync
set -e
bsdtar -czvf carapace-code.tar.gz.nosync \
    --numeric-owner \
    --no-mac-metadata \
    --exclude=info*/.gitignore \
    --exclude=info*/.gitmodules \
    --exclude=info*/.git \
    --exclude=info*/.github \
    --exclude=info*/target \
    --exclude=info*/ifc_examples/avail/chrono_copy \
    --exclude=*.txt \
    --exclude=servo-ifc.nosync/.gitignore \
    --exclude=servo-ifc.nosync/.gitmodules \
    --exclude=servo-ifc.nosync/.git \
    --exclude=servo-ifc.nosync/.github \
    --exclude=servo-ifc.nosync/target \
    --exclude=servo-ifc.nosync/info*/.gitignore \
    --exclude=servo-ifc.nosync/info*/.gitmodules \
    --exclude=servo-ifc.nosync/info*/.git \
    --exclude=servo-ifc.nosync/info*/.github \
    --exclude=mk48-ifc.nosync/.gitignore \
    --exclude=mk48-ifc.nosync/.gitmodules \
    --exclude=mk48-ifc.nosync/.git \
    --exclude=mk48-ifc.nosync/.github \
    --exclude=mk48-ifc.nosync/info*/.gitignore \
    --exclude=mk48-ifc.nosync/info*/.gitmodules \
    --exclude=mk48-ifc.nosync/info*/.git \
    --exclude=mk48-ifc.nosync/info*/.github \
    --exclude=mk48-ifc.nosync/target \
    --exclude=mk48-ifc.nosync/client/target \
    --exclude=mk48-ifc.nosync/engine/target \
    --exclude=mk48-ifc.nosync/common/target \
    --exclude=mk48-ifc.nosync/macros/target \
    --exclude=mk48-ifc.nosync/sprite_sheet_packer/target \
    --exclude=mk48-ifc.nosync/server/target \
    --exclude=servo-ifc.nosync/.gitignore \
    --exclude=servo-ifc.nosync/.gitmodules \
    --exclude=servo.nosync/.git \
    --exclude=servo.nosync/.github \
    --exclude=servo.nosync/target \
    --exclude=mk48-5.nosync/.gitignore \
    --exclude=mk48-5.nosync/.gitmodules \
    --exclude=mk48-5.nosync/.git \
    --exclude=mk48-5.nosync/.github \
    --exclude=mk48-5.nosync/target \
    --exclude=mk48-5.nosync/client/target \
    --exclude=mk48-5.nosync/engine/target \
    --exclude=mk48-5.nosync/common/target \
    --exclude=mk48-5.nosync/macros/target \
    --exclude=mk48-5.nosync/sprite_sheet_packer/target \
    --exclude=mk48-5.nosync/server/target \
    --exclude=avail-ifc.nosync/.gitignore \
    --exclude=avail-ifc.nosync/.gitmodules \
    --exclude=avail-ifc.nosync/.git \
    --exclude=avail-ifc.nosync/.github \
    --exclude=avail-ifc.nosync/target \
    info-flow-library \
    avail-ifc.nosync \
    avail_runner.sh \
    avail_calculator.py \
    README.md \
    servo_runner.sh \
    servo_helper.sh \
    servo_parser.py #\
    #servo-ifc.nosync \
    #servo.nosync \
    #mk48-ifc.nosync \
    #mk48-5.nosync \

echo "tar built"

mkdir -p carapace-code.nosync

echo "directory made"

bsdtar -xzf carapace-code.tar.gz.nosync -C carapace-code.nosync

echo "tar unloaded"

docker image build -t carapace carapace-code.nosync/ -f Dockerfile

echo "docker built"

docker save carapace:latest | gzip > carapace-docker-image.tar.gz.nosync

echo "docker saved"

bsdtar -czvf carapace.tar.gz.nosync carapace-docker-image.tar.gz.nosync carapace-code.tar.gz.nosync

echo "artifact made"