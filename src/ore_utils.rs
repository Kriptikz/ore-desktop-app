use orz::{
    instruction,
    state::{Hash, Proof, Treasury},
    utils::AccountDeserialize,
    BUS_ADDRESSES, EPOCH_DURATION, ID as ORE_ID, MINT_ADDRESS, PROOF, TOKEN_DECIMALS,
    TREASURY_ADDRESS,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::ReadableAccount, clock::Clock, instruction::Instruction, pubkey::Pubkey, sysvar,
};
use spl_associated_token_account::get_associated_token_address;

pub fn get_mine_ix(signer: Pubkey, next_hash: Hash, nonce: u64) -> Instruction {
    instruction::mine(signer, BUS_ADDRESSES[0], next_hash.into(), nonce)
}

pub fn get_register_ix(signer: Pubkey) -> Instruction {
    instruction::register(signer)
}

pub fn get_reset_ix(signer: Pubkey) -> Instruction {
    instruction::reset(signer)
}

pub fn get_claim_ix(signer: Pubkey, beneficiary: Pubkey, claim_amount: u64) -> Instruction {
    instruction::claim(signer, beneficiary, claim_amount)
}

pub fn get_ore_mint() -> Pubkey {
    MINT_ADDRESS
}

pub fn get_ore_epoch_duration() -> i64 {
    EPOCH_DURATION
}

pub fn get_ore_decimals() -> u8 {
    TOKEN_DECIMALS
}

pub fn get_proof_and_treasury(
    client: &RpcClient,
    authority: Pubkey,
) -> (Result<Proof, ()>, Result<Treasury, ()>) {
    let account_pubkeys = vec![TREASURY_ADDRESS, proof_pubkey(authority)];
    let datas = client.get_multiple_accounts(&account_pubkeys);
    if let Ok(datas) = datas {
        let treasury = if let Some(data) = &datas[0] {
            Ok(*Treasury::try_from_bytes(data.data()).expect("Failed to parse treasury account"))
        } else {
            Err(())
        };

        let proof = if let Some(data) = &datas[1] {
            Ok(*Proof::try_from_bytes(data.data()).expect("Failed to parse treasury account"))
        } else {
            Err(())
        };

        (proof, treasury)
    } else {
        (Err(()), Err(()))
    }
}

pub fn get_treasury(client: &RpcClient) -> Result<Treasury, ()> {
    let data = client.get_account_data(&TREASURY_ADDRESS);
    if let Ok(data) = data {
        Ok(*Treasury::try_from_bytes(&data).expect("Failed to parse treasury account"))
    } else {
        Err(())
    }
}

pub fn get_proof(client: &RpcClient, authority: Pubkey) -> Result<Proof, String> {
    let proof_address = proof_pubkey(authority);
    let data = client.get_account_data(&proof_address);
    match data {
        Ok(data) => return Ok(*Proof::try_from_bytes(&data).unwrap()),
        Err(_) => return Err("Failed to get miner account".to_string()),
    }
}

pub fn proof_pubkey(authority: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[PROOF, authority.as_ref()], &ORE_ID).0
}

pub fn treasury_tokens_pubkey() -> Pubkey {
    get_associated_token_address(&TREASURY_ADDRESS, &MINT_ADDRESS)
}

pub fn get_clock_account(client: &RpcClient) -> Clock {
    let data = client
        .get_account_data(&sysvar::clock::ID)
        .expect("Failed to get miner account");
    bincode::deserialize::<Clock>(&data).expect("Failed to deserialize clock")
}
