# PiDefi API Reference

## Overview
PiDefi provides APIs for contracts (Soroban), backend services (REST/WebSocket), SDK (Rust), and bots. All APIs support hyper-tech features like AI recommendations, quantum signing, and holographic data.

## Authentication
- Use JWT tokens from `/login`.
- Quantum-signed requests for sensitive ops.

## Smart Contracts (Soroban)

### StableCoin
- **transfer(to, amount)**: Transfer Pi Coin.
  - Params: `to` (Address), `amount` (i128)
  - Response: bool
  - Example: `soroban contract invoke --id <id> --method transfer --args <args>`

- **stabilize()**: Adjust supply for peg.
  - AI: Uses predictions for adjustment.
  - Holographic: Returns stabilization layers.

### DeFiExtensions
- **ai_yield_farm(user, amount)**: Stake with AI optimization.
  - Params: `user` (Address), `amount` (i128)
  - Response: bool
  - Quantum: Requires signed request.

## Backend APIs (REST)

### User Management
- **POST /register**: Register user.
  - Body: `{username, password}`
  - Response: `{message}`

- **POST /login**: Authenticate.
  - Body: `{username, password}`
  - Response: `{token}`

### Portfolio
- **GET /portfolio/:userId**: Get portfolio.
  - Headers: `Authorization: Bearer <token>`
  - Response: `{balance, yield, ai_recommendation}`
  - AI: Includes personalized suggestions.
  - Holographic: `/holographic/:userId` returns 3D layers.

### Analytics
- **GET /analytics/:userId**: Analytics data.
  - Response: Holographic JSON.

## WebSocket APIs
- **/stream**: Real-time price updates.
  - Message: `{price, timestamp}`

## SDK (Rust)
- **transfer_pi(to, amount)**: High-level transfer.
  - AI: Suggests optimal timing.
  - Quantum: Signs txn.

## Bots
- **StabilizationBot**: No direct API; monitors autonomously.
- **ArbitrageBot**: Triggers via backend alerts.

## Error Codes
- 400: Invalid request
- 401: Unauthorized
- 500: Internal error (with AI diagnostics)

## Examples
```javascript
// SDK Usage
const sdk = new PiDefiSDK();
sdk.transfer_pi(recipient, 1000);
```
## Rate Limits
100 requests/min per IP.

For code, see lib/sdk/pi_defi_sdk.rs.
