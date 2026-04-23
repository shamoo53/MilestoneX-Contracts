# Asset Issuing Guide

This guide explains how to create and manage custom assets on the Stellar network for StellarAid.

## Overview

StellarAid can operate with:
- **XLM** (native Stellar token)
- **USDC** (USD Coin on Stellar)
- **Custom assets** (your own tokens)

This guide focuses on creating and issuing custom assets.

## Step 1: Generate Issuing Keypair

The issuing account is used to create and distribute your custom asset.

### Using Soroban CLI

```bash
# For testnet
soroban keys generate stellaraid_issuing --network testnet

# Display the keys
soroban keys list
```

### Using Stellar Laboratory

1. Visit: https://laboratory.stellar.org/#account-creator?network=testnet
2. Click "Create KeyPair"
3. Save both the **Public Key** and **Secret Key** securely
4. Fund the account with testnet XLM (minimum 1 XLM)

### Configure Environment

Add to your `.env` file:

```bash
SOROBAN_ISSUING_SECRET_KEY=SXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
SOROBAN_ISSUING_PUBLIC_KEY=GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
ASSET_CODE=STAID
ASSET_NAME=StellarAid Token
```

## Step 2: Fund the Issuing Account

The issuing account needs XLM for:
- Account reserve (0.5 XLM minimum)
- Transaction fees (0.00001 XLM per operation)
- Trustline reserves (0.5 XLM per trustline created)

### Testnet Funding

Visit: https://laboratory.stellar.org/#account-creator?network=testnet

Enter your issuing public key and click "Fund Account".

### Mainnet Funding

Transfer XLM from an exchange or another wallet to your issuing account.

## Step 3: Verify Asset Configuration

```bash
# Check asset configuration
cargo run -p stellaraid-tools -- asset config

# Verify readiness
cargo run -p stellaraid-tools -- asset check
```

## Step 4: Establish Trustlines

Before you can issue assets to someone, they must establish a trustline to your asset.

### What is a Trustline?

A trustline is a relationship that allows an account to hold a specific asset. Users must:
1. Explicitly trust the asset issuer
2. Pay a reserve of 0.5 XLM per trustline

### Establish Trustline via CLI

```bash
# User establishes trustline to your asset
cargo run -p stellaraid-tools -- asset trustline <USER_PUBLIC_KEY>

# Example
cargo run -p stellaraid-tools -- asset trustline GABJ2O4OYNDKJPQZ5IXBM7LQNXQWJ3KQHZQXQZ5IXBM7LQNXQWJ3KQ
```

### Establish Trustline via Stellar Laboratory

1. Visit: https://laboratory.stellar.org/#txbuilder
2. Select "Change Trust" operation
3. Set:
   - Asset Code: `STAID` (or your asset code)
   - Asset Issuer: Your issuing public key
   - Limit: Maximum amount user wants to hold (or 0 for unlimited)
4. Sign with user's secret key
5. Submit transaction

## Step 5: Issue Assets

Once trustlines are established, you can issue assets.

### Issue via CLI

```bash
# Issue assets to a recipient
cargo run -p stellaraid-tools -- asset issue <RECIPIENT_PUBLIC_KEY> <AMOUNT>

# Example: Issue 1000 STAID tokens
cargo run -p stellaraid-tools -- asset issue GABJ2O4OYNDKJPQZ5IXBM7LQNXQWJ3KQHZQXQZ5IXBM7LQNXQWJ3KQ 1000
```

### Issue via Stellar Laboratory

1. Visit: https://laboratory.stellar.org/#txbuilder
2. Select "Payment" operation
3. Set:
   - Source Account: Your issuing account
   - Destination: Recipient's public key
   - Asset: `STAID:YOUR_ISSUING_PUBLIC_KEY`
   - Amount: Amount to issue
4. Sign with issuing secret key
5. Submit transaction

## Step 6: Verify Asset Issuance

### Check Balance

```bash
# Using Horizon API
curl "https://horizon-testnet.stellar.org/accounts/<USER_PUBLIC_KEY>"

# Look for balances array with your asset
```

### Using Stellar Explorer

- Testnet: https://stellar.expert/explorer/testnet/account/<PUBLIC_KEY>
- Mainnet: https://stellar.expert/explorer/public/account/<PUBLIC_KEY>

## Asset Configuration Reference

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `ASSET_CODE` | Asset identifier (1-12 chars) | `STAID` |
| `ASSET_NAME` | Human-readable name | `StellarAid Token` |
| `SOROBAN_ISSUING_SECRET_KEY` | Issuing account secret | `SABC...` |
| `SOROBAN_ISSUING_PUBLIC_KEY` | Issuing account public | `GABC...` |
| `SOROBAN_DISTRIBUTOR_PUBLIC_KEY` | Optional distributor account | `GABC...` |

### Asset Code Rules

- **Length**: 1-12 characters
- **Characters**: Alphanumeric (A-Z, 0-9)
- **Case**: Uppercase recommended
- **Examples**: `STAID`, `USD`, `GOLD`, `AID123`

## Multi-Signature Issuing (Advanced)

For enhanced security, consider using multi-signature accounts:

```bash
# Create multi-sig issuing account
# Requires multiple signatures to issue assets
# Setup via Stellar Laboratory or CLI
```

## Revoking Assets

As the issuer, you can revoke assets from any account:

```bash
# Using Stellar Laboratory
1. Go to txbuilder
2. Select "Allow Trust" operation
3. Set authorized: false
4. Submit transaction
```

## Best Practices

### Security

1. **Separate Accounts**: Use different accounts for:
   - Issuing (cold storage, rarely used)
   - Distribution (hot wallet for daily operations)
   - Admin (contract management)

2. **Backup Keys**: Store secret keys securely offline
   - Hardware wallets
   - Encrypted storage
   - Paper wallets in safe

3. **Monitor Activity**: Regularly check issuing account activity

### Compliance

1. **Legal Review**: Ensure compliance with local regulations
2. **KYC/AML**: Implement if required for your use case
3. **Documentation**: Maintain clear records of asset issuance

### Technical

1. **Test First**: Always test on testnet before mainnet
2. **Reserve XLM**: Keep extra XLM in issuing account
3. **Monitor Trustlines**: Track how many trustlines exist
4. **Rate Limiting**: Implement limits on issuance if needed

## Troubleshooting

### "Trustline does not exist"

The recipient hasn't established a trustline yet. Have them create one first.

### "Insufficient balance"

The issuing account doesn't have enough XLM for reserves or fees.

### "Invalid asset code"

Asset code must be 1-12 alphanumeric characters.

### "Authorization required"

The asset requires issuer authorization. Use "Allow Trust" operation.

## CLI Commands Summary

```bash
# Show asset configuration
cargo run -p stellaraid-tools -- asset config

# Generate issuing keypair
cargo run -p stellaraid-tools -- asset generate

# Check issuing readiness
cargo run -p stellaraid-tools -- asset check

# Establish trustline
cargo run -p stellaraid-tools -- asset trustline <PUBLIC_KEY>

# Issue assets
cargo run -p stellaraid-tools -- asset issue <PUBLIC_KEY> <AMOUNT>
```

## Need Help?

- Stellar Asset Documentation: https://developers.stellar.org/docs/tokens
- Stellar Laboratory: https://laboratory.stellar.org
- Stellar Developers Discord: https://discord.gg/stellardev
