#!/bin/bash

GOV=${GOV:-"osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks"}
BEAKER=${BEAKER:-"beaker"}
DAEMON=${DAEMON:-"osmosisd"}
MNEMONIC=${SIGNER_MNEMONIC:-"notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius"}

SIGNER="deployer-test"
beaker key set "$SIGNER" "$MNEMONIC" -y

function check {
    if [ -z "$1" ]
    then
      echo "\$$2 is not defined"
      exit 1
    fi
}

check "$NETWORK" "NETWORK"
check "$NODE" "NODE"
check "$CHAIN_ID" "CHAIN_ID"

TOKENFACTORY_FEE=$(
    $DAEMON query tokenfactory params \
        --output=json \
        --node=$NODE | \
    jq -r '.params.denom_creation_fee[0] | .amount + .denom'
)
SIGNER_FLAG="--signer-keyring=$SIGNER"
OPTIMIZE_FLAG=$([ "$NETWORK" = "local" ] && echo "--no-wasm-opt")

echo "============ Deploying IBC Core ============"
CORE_INIT_MSG=$(cat $(pwd)/scripts/$NETWORK/ibc_core.json | jq -c '.gov = "'$GOV'"')
beaker wasm deploy \
    --raw $CORE_INIT_MSG \
    --network $NETWORK \
    --funds $TOKENFACTORY_FEE \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    ibc-core

echo "============ Deploying IBC Periphery ============"
PERIPHERY_INIT_MSG=$(cat $(pwd)/scripts/$NETWORK/ibc_periphery.json | jq -c)
beaker wasm deploy \
    --raw $PERIPHERY_INIT_MSG \
    --network $NETWORK \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    ibc-periphery

echo "============ Deploying IBC Airdrop ============"
AIRDROP_INIT_MSG=$(cat $(pwd)/scripts/$NETWORK/ibc_airdrop.json | jq -c)
beaker wasm deploy \
    --raw $AIRDROP_INIT_MSG \
    --network $NETWORK \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    ibc-airdrop

echo "============ Deploying IBC Faucet ============"
FAUCET_INIT_MSG=$(cat $(pwd)/scripts/$NETWORK/ibc_faucet.json | jq -c)
beaker wasm deploy \
    --raw $FAUCET_INIT_MSG \
    --network $NETWORK \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    ibc-faucet
