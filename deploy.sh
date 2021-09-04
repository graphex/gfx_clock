#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly PROJECT_NAME=gfx_clock
readonly TARGET_HOST=pi@clockpi
readonly TARGET_PATH=/home/pi/gfx_clock
readonly ARGS=NCS3148C
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
#readonly TARGET_ARCH=arm-unknown-linux-musleabi
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/gfx_clock

docker run -v "$(pwd)":"/root/workspace" -a stdout -a stderr rustpi
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH} ${ARGS}