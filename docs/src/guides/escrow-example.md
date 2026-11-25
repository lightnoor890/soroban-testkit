# Worked Example: Escrow with Arbiter Dispute Resolution

This example builds on the bounty escrow state machine to show token custody,
a two-party submission and a designated arbiter. The first reviewer supplied at
creation is the arbiter. Either the buyer or submitted seller can raise a
dispute; only that arbiter can settle it, paying the seller or refunding the
buyer. The existing deadline refund remains available when no dispute is open.

## Run the example

```sh
cargo test -p soroban-testkit-example-escrow
```

The contract is `#![no_std]` in `examples/escrow/src/lib.rs`. Contract tests
cover validation and lifecycle behavior; integration tests exercise the
fixtures, assertion, core budget and storage, and generator crates.

## Dispute lifecycle

```text
Open ──submit──▶ Submitted ──approval quorum──▶ Released
                       │
                       ├──party raises──▶ Disputed ──arbiter awards──▶ Released
                       │                                  └──────────▶ Refunded
                       └──deadline──▶ Refunded
```

`raise_dispute` authenticates the caller and checks that the caller is one of
the escrow parties. `resolve_dispute` authenticates the arbiter, checks the
pending state, and transfers the escrow exactly once. Tests also check that a
non-arbiter cannot resolve a dispute and compare the dispute-call budget with
a committed ceiling.

## Testkit coverage

- `tests/fixtures.rs` sets up identities, token balances, and deadlines.
- `tests/assert.rs` checks lifecycle results, events, and authorization.
- `tests/budget_storage.rs` measures invocation cost and storage.
- `tests/proptest.rs` checks safety properties over generated amounts.

The generated third-party property asserts that unauthorized addresses cannot
claim escrowed funds. Timeout tests advance the ledger clock instead of waiting.
