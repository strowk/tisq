#!/bin/bash

THESYSTEMIS="unknown-linux-gnu"
POSTFIX=""

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    THESYSTEMIS="unknown-linux-gnu"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    THESYSTEMIS="apple-darwin"
elif [[ "$OSTYPE" == "cygwin" ]]; then
    THESYSTEMIS="pc-windows-gnu"
elif [[ "$OSTYPE" == "msys" ]]; then
    THESYSTEMIS="pc-windows-gnu"
elif [[ "$OSTYPE" == "win32" ]]; then
    THESYSTEMIS="pc-windows-gnu"
fi

if [[ "$THESYSTEMIS" == "unknown-linux-gnu" ]]; then
    libc=$(ldd /bin/ls | grep 'musl' | head -1 | cut -d ' ' -f1)
    if ! [ -z $libc ]; then
        THESYSTEMIS="unknown-linux-musl"
    fi
fi

if [[ "$THESYSTEMIS" == "pc-windows-gnu" ]]; then
    POSTFIX=".exe"
fi

echo "The system is $THESYSTEMIS"
ARCH="$(uname -m)"
echo "architecture is $ARCH"

BUILD="${ARCH}-${THESYSTEMIS}"
DOWNLOAD_URL="$(curl https://api.github.com/repos/strowk/tisq/releases/latest | grep browser_download_url | grep ${BUILD} | cut -d '"' -f 4 )"

if [[ -z "$DOWNLOAD_URL" ]]; then
    echo "No prebuilt binary found for $BUILD"
    echo "Check out existing builds in https://github.com/strowk/tisq/releases/latest"
    echo "Or you could build from source"
    echo "Refer to README in https://github.com/strowk/tisq#from-sources for more information"
    exit 1
fi

echo "Downloading from $DOWNLOAD_URL"
curl "$DOWNLOAD_URL" -Lo ./tisq-archive.tgz
mkdir -p ./tisq-install
tar -xzf ./tisq-archive.tgz -C ./tisq-install

INSTALL_SOURCE="./tisq-install/target/${BUILD}/release/tisq${POSTFIX}"
INSTALL_TARGET="/usr/local/bin/tisq"

chmod +x "${INSTALL_SOURCE}"

SUDO_PREFIX="sudo"

if [[ "$THESYSTEMIS" == "pc-windows-gnu" ]]; then
    mkdir -p /usr/local/bin
    SUDO_PREFIX=""
fi

# if set environment variable NO_SUDO, then don't use sudo
if [[ "$NO_SUDO" == "1" ]]; then
    SUDO_PREFIX=""
fi

${SUDO_PREFIX} mv "${INSTALL_SOURCE}" "${INSTALL_TARGET}${POSTFIX}"

rm tisq-install/ -r
rm tisq-archive.tgz

echo "tisq is installed, checking by running 'tisq --version'"
tisq --version
