# IBCX Airdrop

## Instaintiation

| There's no need to put some values into instantiate message. Just put empty json object.

Here's the example of InstantiateMsg

```json
{}
```

## Features

- Bearer airdrop (execute airdrop to who received particular value from project)
- Airdrop labeling
- Claim multiple airdrops at once

## Queries

- Latest airdrop id
- Check Qualification
- Airdrop
  - Get single item
  - Iterate airdrops
- Claim
  - Get single item
  - Iterate claims

## Testing

1. Launch Osmosis localnet via run
   - `make localnet-start`
2. Deploy contract
   - `beaker wasm deploy ibcx-airdrop --raw '{}'`
3. Generate test merkle root
   - `cd ./ts/tools`
   - `yarn`
   - `npx ts-node ./src/index.ts 0 ../testdata/input-bearer.json ./testdata/output-bearer.json`
4. Register airdrop
   - `beaker wasm execute ibcx-airdrop --raw '{"merkle_root":"{MERKLE_ROOT}","denom":"{DENOM}"}'`
5. Claim airdrop

   - ```bash
     beaker wasm execute ibcx-airdrop \
        --raw '{ \
            "claim":{ \
                "id":{"id":"{AIRDROP_ID}"}}, \
                "amount":"{AMOUNT}", \
                "claim_proof":{"account":"{CLAIM_PROOF}"}, \
                "merkle_proof":["{MERKLE_PROOF}"] \
            } \
        }'
     ```
