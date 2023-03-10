use {
    crate::PresaleError,
    anchor_lang::{
        prelude::{AccountInfo, ProgramResult,},
        solana_program::{
            program::{invoke_signed, invoke},
        },
    },
};

///TokenTransferParams
pub struct TokenTransferParams<'a: 'b, 'b> {
    /// CHECK: checked in program
    /// source
    pub source: AccountInfo<'a>,
    /// CHECK: checked in program
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// CHECK: checked in program
    /// authority
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [u8]],
    /// CHECK: checked in program
    /// token_program
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_transfer(params: TokenTransferParams<'_, '_>) -> ProgramResult {
    let TokenTransferParams {
        source,
        destination,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;

    let result = invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination, authority, token_program],
        &[authority_signer_seeds],
    );

    result.map_err(|_| PresaleError::TokenTransferFailed.into())
}

///TokenTransferParams
pub struct TokenTransferParamsWithoutSeed<'a> {
    /// CHECK: checked in program
    /// source
    pub source: AccountInfo<'a>,
    /// CHECK: checked in program
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// CHECK: checked in program
    /// authority
    pub authority: AccountInfo<'a>,
    /// CHECK: checked in program
    /// token_program
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_transfer_without_seed(params: TokenTransferParamsWithoutSeed<'_>) -> ProgramResult {
    let TokenTransferParamsWithoutSeed {
        source,
        destination,
        authority,
        token_program,
        amount,
    } = params;

    let result = invoke(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination ,authority, token_program],
    );

    result.map_err(|_| PresaleError::TokenTransferFailed.into())
}

pub struct TokenSetAuthorityParams<'a>{
    /// CHECK: checked in program
    pub authority : AccountInfo<'a>,
    /// CHECK: checked in program
    pub new_authority : AccountInfo<'a>,
    /// CHECK: checked in program
    pub account : AccountInfo<'a>,
    /// CHECK: checked in program
    pub token_program : AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_set_authority(params : TokenSetAuthorityParams<'_>) -> ProgramResult {
    let TokenSetAuthorityParams {
        authority,
        new_authority,
        account,
        token_program,
    } = params;

    let result = invoke(
        &spl_token::instruction::set_authority(
            token_program.key,
            account.key,
            Some(new_authority.key),
            spl_token::instruction::AuthorityType::AccountOwner,
            authority.key,
            &[],
        )?,
        &[authority,new_authority,account,token_program],
    );
    result.map_err(|_| PresaleError::TokenSetAuthorityFailed.into())
}