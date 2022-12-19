
deploy-local:
	NETWORK="local" \
	CHAIN_ID="localosmosis" \
	NODE="http://localhost:26657" \
	./scripts/deploy-test.sh

deploy-testnet:
	NETWORK="testnet" \
	CHAIN_ID="osmo-test-4" \
	NODE="https://rpc-test.osmosis.zone:443" \
	./scripts/deploy-test.sh

localnet-start:
	./scripts/localnet-start.sh

localnet-startd:
	BACKGROUND=1 ./scripts/localnet-start.sh

localnet-stop:
	./scripts/localnet-stop.sh

schema:
	ls ./contracts | xargs -n 1 -t beaker wasm ts-gen
