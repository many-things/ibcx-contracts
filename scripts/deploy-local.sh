#!/bin/bash

GOV=${GOV:-"osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks"}
BEAKER=${BEAKER:-"beaker"}
DAEMON=${DAEMON:-"osmosisd"}
NODE=${NODE:-"http://localhost:26657"}
SIGNER=${SIGNER:-"test1"}

TOKENFACTORY_FEE=$($DAEMON query tokenfactory params --output=json --node=$NODE | jq -r '.params.denom_creation_fee[0] | .amount + .denom')

echo "============ Deploying IBC Core ============"
CORE_INIT_MSG=$(cat $(pwd)/scripts/local/ibc_core.json | jq -c '.gov = "'$GOV'"')
beaker wasm deploy \
    --raw $CORE_INIT_MSG \
    --signer-account $SIGNER \
    --no-wasm-opt ibc_core \
    --funds $TOKENFACTORY_FEE

echo "============ Deploying IBC Periphery ============"
PERIPHERY_INIT_MSG=$(cat $(pwd)/scripts/local/ibc_periphery.json | jq -c)
beaker wasm deploy \
    --raw $PERIPHERY_INIT_MSG \
    --signer-account $SIGNER \
    --no-wasm-opt ibc_periphery

echo "============ Deploying IBC Airdrop ============"
AIRDROP_INIT_MSG=$(cat $(pwd)/scripts/local/ibc_airdrop.json | jq -c)
beaker wasm deploy \
    --raw $AIRDROP_INIT_MSG \
    --signer-account $SIGNER \
    --no-wasm-opt ibc_airdrop
