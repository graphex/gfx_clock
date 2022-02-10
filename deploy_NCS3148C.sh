#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly PROJECT_NAME=gfx_clock
readonly TARGET_HOST=pi@192.168.168.206
readonly TARGET_PATH=/home/pi/gfx_clock
readonly ARGS=NCS3148C
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
#readonly TARGET_ARCH=arm-unknown-linux-musleabi
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/gfx_clock
readonly SOURCE_SERVICE=./service/gfx_clock_${ARGS}.service
readonly TARGET_SERVICE=/lib/systemd/system/gfx_clock.service

#set up the rustpi image using /docker/build.sh and update with each Cargo.toml change for speed
docker run -v "$(pwd)":"/root/workspace" -a stdout -a stderr rustpi
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}

# put the /service/gfx_clock_${ARGS}.service in the pi's /lib/systemd/system/ dir
rsync --rsync-path="sudo rsync" ${SOURCE_SERVICE} ${TARGET_HOST}:${TARGET_SERVICE}
ssh -t ${TARGET_HOST} "sudo chmod 644 ${TARGET_SERVICE} && sudo systemctl daemon-reload && sudo systemctl enable gfx_clock.service && sudo systemctl restart gfx_clock.service"
ssh -t ${TARGET_HOST} journalctl -f -u gfx_clock.service --no-pager

# ssh -t ${TARGET_HOST} ${TARGET_PATH} ${ARGS}