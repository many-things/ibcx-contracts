#!/bin/bash

HOME=$PWD/localnet
DAEMON=${DAEMON:-$PWD/osmosis/build/osmosisd}

cd osmosis
[ ! -f $DAEMON ] && make build

echo "y\n" | HOME=$HOME make localnet-init

[ -z $BACKGROUND ] && \
    HOME=$HOME make localnet-start || \
    HOME=$HOME make localnet-startd

cd ..
