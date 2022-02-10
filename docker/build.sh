#!/bin/bash
#This builds the docker image and tags it to your local docker repo as "rustpi" for use in the deploy script
set -o errexit
set -o nounset
set -o pipefail
set -o xtrace
cd "$(dirname "$0")"
cp ../Cargo.lock ./Cargo.lock
cp ../Cargo.toml ./Cargo.toml
docker build . -t rustpi
rm ./Cargo.lock ./Cargo.toml
