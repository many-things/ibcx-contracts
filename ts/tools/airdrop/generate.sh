#!/bin/bash

ID_FROM=${ID_FROM:-0}
BEAKER=${BEAKER:-"beaker"}
DAEMON=${DAEMON:-"osmosisd"}
MNEMONIC=${SIGNER_MNEMONIC:-"notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius"}

SIGNER="deployer-test"
beaker key set "$SIGNER" "$MNEMONIC" -y
(echo "y"; echo "$MNEMONIC") | $DAEMON keys add --recover "$SIGNER"

function check {
    if [ -z "$1" ]
    then
      echo "\$$2 is not defined"
      exit 1
    fi
}

check "$NETWORK" "NETWORK"

SIGNER_FLAG="--signer-keyring=$SIGNER"

# generate osmo staker airdrop
yarn start $(($ID_FROM + 0)) ./airdrop/$NETWORK/osmo-staker.json ./airdrop/$NETWORK-out/osmo-staker.json
yarn start $(($ID_FROM + 1)) ./airdrop/$NETWORK/osmo-mission-1.json ./airdrop/$NETWORK-out/osmo-mission-1.json
yarn start $(($ID_FROM + 2)) ./airdrop/$NETWORK/osmo-mission-2.json ./airdrop/$NETWORK-out/osmo-mission-2.json

# generate ion staker airdrop
yarn start $(($ID_FROM + 3)) ./airdrop/$NETWORK/ion-staker.json ./airdrop/$NETWORK-out/ion-staker.json
yarn start $(($ID_FROM + 4)) ./airdrop/$NETWORK/ion-mission-1.json ./airdrop/$NETWORK-out/ion-mission-1.json
yarn start $(($ID_FROM + 5)) ./airdrop/$NETWORK/ion-mission-2.json ./airdrop/$NETWORK-out/ion-mission-2.json

# # register airdrops
# beaker wasm execute ibc-airdrop \
#     --raw '{"register": {"merkle_root":"", "denom": "}}' \
#     --network $NETWORK \
#     $SIGNER_FLAG
