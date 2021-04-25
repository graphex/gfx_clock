# configures cargo for cross compilation

if [ ! -z "$(uname -a | grep -o Darwin)" ]; then
    echo "You seem to be running this from OS X."
    echo "This is meant to be run in an Ubuntu container. Exiting."
    exit 1
fi

mkdir -p /root/workspace
mkdir -p /root/.cargo
cat >>/root/.cargo/config <<EOF
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF