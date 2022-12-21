#!/bin/bash

GOV=${GOV:-"osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks"}
COLLECTOR=${COLLECTOR:-"osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks"}
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
# DENOMS=("utatom" "utosmo")
WEIGHTS=(33.35 20.24 12.65 8.83 6.97 4.76 4.29 3.37 2.84 2.69)

DECIMAL=1000000
PRICES=(11.6405 1 0.4366 1.6 0.8002 0.0359 0.2715 0.2275 0.3528 0.0079) # in OSMO

STATES=$([ "$NETWORK" = "local" ] && echo "state.local.json" || echo "state.json")
FAUCET_ADDR=$(cat $(pwd)/.beaker/$STATES | jq -r '.'$NETWORK'["ibc-faucet"].addresses.default')
RESERVE_DENOM="factory/$FAUCET_ADDR/utosmo"

# CREATE & MINT
for denom in "${DENOMS[@]}"; do
    echo "=========== CREATING $denom ==========="
    beaker wasm execute ibc-faucet \
        --raw $(printf "{\"create\":{\"denom\":\"$denom\",\"config\":{\"unmanaged\":{}}}}") \
        --network $NETWORK \
        --funds $TOKENFACTORY_FEE \
        $SIGNER_FLAG
    
    L_MINT_MSG=$(printf "{\"mint\":{\"denom\":\"$denom\",\"amount\":\""$(echo "10^30" | bc)"\"}}")
    beaker wasm execute ibc-faucet \
        --raw $L_MINT_MSG \
        --network $NETWORK \
        $SIGNER_FLAG
done

# CREATE POOLS
for i in "${!DENOMS[@]}"; do
    L_TARGET_DENOM="factory/$FAUCET_ADDR/${DENOMS[$i]}"
    if [ "$L_TARGET_DENOM" = "$RESERVE_DENOM" ]; then
        continue
    fi

    L_WEIGHTS=$(echo "1$L_TARGET_DENOM,1$RESERVE_DENOM")
    L_RESERVE_DEPOSIT=$(echo "$(($DECIMAL * $DECIMAL))$RESERVE_DENOM")
    L_TARGET_DEPOSIT=$(printf "%d$L_TARGET_DENOM" "$(echo ''$DECIMAL' / '${PRICES[$i]}' * '$DECIMAL'' | bc)")
    L_DEPOSITS=$(echo "$L_TARGET_DEPOSIT,$L_RESERVE_DEPOSIT")
    echo "$RESERVE_DENOM::$L_TARGET_DENOM =>"
    echo "==== DEPOSITS : $L_DEPOSITS"
    echo "==== WEIGHTS  : $L_WEIGHTS"

    L_POOL_CONFIG=$(
        cat $(pwd)/scripts/pool_config.json | \
        jq -c '.weights = "'$L_WEIGHTS'"' | \
        jq -c '."initial-deposit" = "'$L_DEPOSITS'"'
    )
    
    echo "$L_POOL_CONFIG" > $(pwd)/scripts/pool_config.json

    L_POOL_ID=$(
        $DAEMON tx gamm create-pool \
            --pool-file $(pwd)/scripts/pool_config.json \
            --from $SIGNER \
            --node $NODE \
            --chain-id $CHAIN_ID \
            --yes --output=json -b "block" | \
        jq -r '.logs[0].events[4].attributes[0].value'
    )
    echo "==== POOL_ID  : $L_POOL_ID"

    ## GAUGE CREATION
    # $DAEMON tx incentives create-gauge "gamm/pool/$POOL_ID" "1000000uosmo" \
    #     --epochs 10 \
    #     --from $SIGNER \
    #     --chain-id $CHAIN_ID \
    #     --node $NODE \
    #     --yes --output=json -b "block"
done

echo "Creating uosmo <-> utosmo pool"
L_POOL_CONFIG=$(
    cat $(pwd)/scripts/pool_config.json | \
    jq -c '.weights = "1uosmo,1'$RESERVE_DENOM'"' | \
    jq -c '."initial-deposit" = "1000000uosmo,10000000000000000'$RESERVE_DENOM'"'
)

echo "$L_POOL_CONFIG" > $(pwd)/scripts/pool_config.json

L_POOL_ID=$(
    $DAEMON tx gamm create-pool \
        --pool-file $(pwd)/scripts/pool_config.json \
        --from $SIGNER \
        --node $NODE \
        --chain-id $CHAIN_ID \
        --yes --output=json -b "block" | \
    jq -r '.logs[0].events[4].attributes[0].value'
)
echo "==== POOL_ID  : $L_POOL_ID"

echo "============ Deploying IBC Compat ============"
COMPAT_INIT_MSG=$(
    cat $(pwd)/scripts/$NETWORK/ibc_compat.json | \
    jq -c '.gov = "'$GOV'"' | \
    jq -c '.fee.collector = "'$COLLECTOR'"'
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
    L_WEIGHT=$(echo "${WEIGHTS[$i]} ${PRICES[$i]}" | awk '{print $1 / $2}')
    CORE_INIT_MSG=$(
        echo "$CORE_INIT_MSG" | \
        jq -c '.initial_assets += [["factory/'$FAUCET_ADDR'/'${DENOMS[$i]}'","'$L_WEIGHT'"]]'
    )
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
