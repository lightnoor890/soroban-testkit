use soroban_sdk::{testutils::Address as _, token::StellarAssetClient, Address};
use soroban_testkit_example_escrow::{BountyEscrowContract, BountyEscrowContractClient};
use soroban_testkit_fixtures::builder::TestContextBuilder;

#[test]
fn fixture_builds_roles_and_token() {
    let ctx = TestContextBuilder::new().with_users(5).build();
    let contract = ctx.env.register(BountyEscrowContract, ());
    let token = ctx
        .env
        .register_stellar_asset_contract_v2(ctx.admin.clone())
        .address();
    StellarAssetClient::new(&ctx.env, &token).mint(ctx.user(0), &1000);
    assert_eq!(
        soroban_sdk::token::Client::new(&ctx.env, &token).balance(ctx.user(0)),
        1000
    );
    assert!(ctx.user(1) != ctx.user(2));
    let client = BountyEscrowContractClient::new(&ctx.env, &contract);
    assert_eq!(client.next_bounty_id(), 1);
}

#[test]
fn fresh_fixture_uses_distinct_contract_addresses() {
    let ctx = TestContextBuilder::new().with_users(3).build();
    let a = ctx.env.register(BountyEscrowContract, ());
    let b = ctx.env.register(BountyEscrowContract, ());
    assert_ne!(a, b);
}

#[test]
fn identities_can_be_generated_for_additional_parties() {
    let ctx = TestContextBuilder::new().with_users(2).build();
    let external = Address::generate(&ctx.env);
    assert_ne!(external, *ctx.user(0));
}

#[test]
fn fixture_admin_is_available_to_issue_test_tokens() {
    let ctx = TestContextBuilder::new().with_users(2).build();
    let token = ctx
        .env
        .register_stellar_asset_contract_v2(ctx.admin.clone())
        .address();
    StellarAssetClient::new(&ctx.env, &token).mint(ctx.user(0), &7);
    assert_eq!(
        soroban_sdk::token::Client::new(&ctx.env, &token).balance(ctx.user(0)),
        7
    );
}
