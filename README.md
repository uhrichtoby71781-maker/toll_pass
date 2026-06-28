# toll_pass

## Project Title
toll_pass

## Project Description
toll_pass is a Soroban smart contract that turns a paper highway sticker into a transparent, prepaid on-chain pass. Drivers top up credits on their pass, registered toll stations deduct the exact fee whenever a vehicle crosses, and every trip is permanently logged so users can audit, verify, or dispute any charge directly on the Stellar ledger.

## Project Vision
Our vision is to remove the friction, opacity, and double-charging that plague today's highway toll systems. By anchoring balances, station registrations, and crossing events on Stellar, toll_pass enables borderless interoperability between national toll operators, real-time settlement for road authorities, and verifiable proof of every cent a driver pays — building the foundation for an open, programmable, cross-border mobility network.

## Key Features
- **Prepaid driver pass** — `topup` lets any wallet load credits onto its own pass, signed by the holder via `require_auth()`.
- **Operator-owned stations** — `register_station` lets a highway operator publish a uniquely-keyed toll booth with a human-readable location.
- **Authenticated crossings** — `cross` requires the station operator's signature, verifies their ownership of the station, deducts the fee atomically, and emits an incremental crossing id.
- **On-chain trip history** — every crossing is stored as a `Crossing` struct (user, station, fee, timestamp) and indexed per driver for instant lookup.
- **Driver-side dispute** — `dispute` lets the affected driver flag a wrongful charge with a written reason, locking an immutable complaint record on-chain.
- **Public read views** — `get_balance`, `get_crossing_count`, `get_station`, and `get_crossing` give wallets and explorers free, gas-free reads.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** travel dApp — see `contracts/toll_pass/src/lib.rs` for the full toll_pass business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CBRGBIJAOCGODRLJQASSEWU6IYCJTDIEEZC2QJ5Y5F4LOCJY3UBBYPQG`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/78b1051de2114a0d6a379af68183f76c351f28904e5a836305be865abca24b6b`

## Future Scope
- Replace the internal `u32` credit ledger with a real Stellar asset (USDC or XLM) using SAC transfers, so top-ups settle in stablecoins.
- Add an arbitration role that can resolve disputed crossings and refund credits back to the driver's balance.
- Introduce time-of-day and vehicle-class dynamic pricing, plus discount tiers for frequent commuters.
- Issue a soulbound NFT "pass" per driver to anchor identity and unlock loyalty rewards across multiple highway operators.
- Cross-border roaming: let operators in different countries accept the same pass via a shared registry contract.
- Ship a Freighter-powered web dApp and an in-vehicle SDK that auto-signs crossings via short-range beacons.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `toll_pass` (travel)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
