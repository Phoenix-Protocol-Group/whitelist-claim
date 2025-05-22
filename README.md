# whitelist-claim
A contract that lets approved users claim their airdropped tokens securely.

## Features

- Deposit tokens into a claimable balance with multiple claimants.
- Each claimant can claim their portion once.
- Supports multiple deposits and combines claimants.
- Only the admin can deposit.

---

## Initialization

To initialize and deploy the contract, use the Soroban CLI:

```bash docci-background docci-delay-after=2
soroban contract deploy \
    --wasm whitelist_claim.optimized.wasm \
    --source $YOUR_WALLET \
    --network $NETWORK \
    -- \
    --admin $ADMIN_KEY
```

## Deposit

Admin-only. Adds (appends) claimants and transfers tokens into the contract.

```bash docci-background docci-delay-after=3
soroban contract invoke \
  --id $CONTRACT_ID \
  --source $YOUR_WALLET \
  --network $NETWORK \
  --fn deposit \
  -- \
  --from $ADMIN_ADDRESS \
  --token $TOKEN_CONTRACT_ADDRESS \
  --claimants "[{\"claimant\":\"$CLAIMANT1\",\"amount\":100}, {\"claimant\":\"$CLAIMANT2\",\"amount\":200}]"
```

## Claim

Called by a claimant to receive their tokens. Is removed from the list afterwards.

```bash docci-background docci-delay-after=3
soroban contract invoke \
  --id $CONTRACT_ID \
  --source $CLAIMANT1 \
  --network $NETWORK \
  --fn claim \
  -- \
  --sender $CLAIMANT1 \
  --token $TOKEN_CONTRACT_ADDRESS
```
