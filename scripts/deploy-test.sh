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

echo "============ Deploying IBC Faucet ============"
FAUCET_INIT_MSG=$(cat $(pwd)/scripts/$NETWORK/ibc_faucet.json | jq -c)
beaker wasm deploy \
    --raw $FAUCET_INIT_MSG \
    --network $NETWORK \
    --admin "signer" \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    ibc-faucet

DENOMS=("utatom" "utosmo" "utevmos" "utjuno" "utscrt" "utstars" "utakt" "utregen" "utstrd" "utumee")
WEIGHTS=("33.35" "20.24" "12.65" "8.83" "6.97" "4.76" "4.29" "3.37" "2.84" "2.69")
STATES=$([ "$NETWORK" = "local" ] && echo "state.local.json" || echo "state.json")
FAUCET_ADDR=$(cat $(pwd)/.beaker/$STATES | jq -r '.'$NETWORK'["ibc-faucet"].addresses.default')

for denom in "${DENOMS[@]}"; do
    beaker wasm execute ibc-faucet \
        --raw $(printf "{\"create\":{\"denom\":\"$denom\",\"config\":{\"unmanaged\":{}}}}") \
        --network $NETWORK \
        --funds $TOKENFACTORY_FEE \
        $SIGNER_FLAG
done

echo "============ Deploying IBC Compat ============"
COMPAT_INIT_MSG=$(
    cat $(pwd)/scripts/$NETWORK/ibc_compat.json | \
    jq -c '.gov = "'$GOV'"'
)
beaker wasm deploy \
    --raw $COMPAT_INIT_MSG \
    --network $NETWORK \
    --admin "signer" \
    --funds $TOKENFACTORY_FEE \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    ibc-compat
COMPAT_ADDR=$(cat $(pwd)/.beaker/$STATES | jq -r '.'$NETWORK'["ibc-compat"].addresses.default')

echo "============ Deploying IBC Core ============"
CORE_INIT_MSG=$(
    cat $(pwd)/scripts/$NETWORK/ibc_core.json | \
    jq -c '.gov = "'$GOV'"' | \
    jq -c '.compat = "'$COMPAT_ADDR'"' | \
    jq -c '.reserve_denom = "factory/'$FAUCET_ADDR'/utosmo"'
)

for i in "${!DENOMS[@]}"; do
    CORE_INIT_MSG=$(echo "$CORE_INIT_MSG" | jq -c '.initial_assets += [["factory/'$FAUCET_ADDR'/'${DENOMS[$i]}'","'${WEIGHTS[$i]}'"]]')
done

beaker wasm deploy \
    --raw $CORE_INIT_MSG \
    --network $NETWORK \
    --admin "signer" \
    --funds $TOKENFACTORY_FEE \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    ibc-core

echo "============ Deploying IBC Periphery ============"
PERIPHERY_INIT_MSG=$(cat $(pwd)/scripts/$NETWORK/ibc_periphery.json | jq -c)
beaker wasm deploy \
    --raw $PERIPHERY_INIT_MSG \
    --network $NETWORK \
    --admin "signer" \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    ibc-periphery

echo "============ Deploying IBC Airdrop ============"
AIRDROP_INIT_MSG=$(cat $(pwd)/scripts/$NETWORK/ibc_airdrop.json | jq -c)
beaker wasm deploy \
    --raw $AIRDROP_INIT_MSG \
    --network $NETWORK \
    --admin "signer" \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    ibc-airdrop
