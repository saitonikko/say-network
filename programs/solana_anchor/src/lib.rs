pub mod utils;
use {
    crate::utils::*,
    anchor_lang::{
        prelude::*,
        // solana_program::{
        //     program_pack::Pack,
        // }
    },
    // spl_token::state,
};

declare_id!("2YmQ2gMwntFMYLM2MEiaHC3x93HV7G2w7stFgEkMbdqo");
pub const PRESALE : &str = "presale";
pub const CLIENT_DATA_SIZE : usize = 32*2+8*19+1;
pub const PRESALE_DATA_SIZE : usize = 32+8*9+2+1;
// pub const SECONDS_PER_MONTH : u64 = 30 * 24 * 60 * 60;
pub const SECONDS_PER_MONTH : u64 = 5 * 60;

#[program]
pub mod solana_anchor {
    use super::*;

    pub fn initialize_presale(
        ctx : Context<InitPresale>,
        _min_allocation : u64,
        _max_allocation : u64,
        _hardcap : u64,
        _first_vesting : u64,
        _every_vesting : u64,
        _start_time : u64,
        _end_time : u64,
        _round : u16,
        _is_whitelist : bool,
        ) -> ProgramResult {
        let presale = &mut ctx.accounts.presale;
        presale.owner = ctx.accounts.owner.key();
        presale.min_allocation = _min_allocation;
        presale.max_allocation = _max_allocation;
        presale.hardcap = _hardcap;
        presale.first_vesting = _first_vesting;
        presale.every_vesting = _every_vesting;
        presale.start_time = _start_time;
        presale.end_time = _end_time;
        presale.round = _round;
        presale.is_whitelist = _is_whitelist;
        presale.total_raised = 0;
        presale.participants = 0;
        Ok(())
    }

    pub fn initialize_client(ctx : Context<InitClient>) -> ProgramResult{
        let client = &mut ctx.accounts.client;

        client.owner = *ctx.accounts.authority.key;

        client.bought_amount_1 = 0;
        client.bought_amount_2 = 0;
        client.bought_amount_3 = 0;
        client.bought_amount_4 = 0;
        client.bought_amount_5 = 0;
        client.bought_amount_6 = 0;
        client.bought_time_1 = 0;
        client.bought_time_2 = 0;
        client.bought_time_3 = 0;
        client.bought_time_4 = 0;
        client.bought_time_5 = 0;
        client.bought_time_6 = 0;
        client.already_claimed_1 = 0;
        client.already_claimed_2 = 0;
        client.already_claimed_3 = 0;
        client.already_claimed_4 = 0;
        client.already_claimed_5 = 0;
        client.already_claimed_6 = 0;
        client.pending_amount = 0;
        client.is_whitelisted = false;
        Ok(())
    }

    pub fn add_to_whitelist(ctx : Context<SetWhitelist>) -> ProgramResult {

        let client = &mut ctx.accounts.client;

        client.is_whitelisted = true;
        Ok(())
    }

    pub fn remove_from_whitelist(ctx : Context<SetWhitelist>) -> ProgramResult {

        let client = &mut ctx.accounts.client;

        client.is_whitelisted = false;
        Ok(())
    }

    pub fn buy(ctx : Context<Buy>, _amount : u64, _now_time : u64 ) -> ProgramResult {
        let client = &mut ctx.accounts.client;

        // ///////////////////////////////////////////////////////////////////
        let presale = &mut ctx.accounts.presale;

        match presale.round {
            1 => {
                if client.bought_amount_1 > 0 {
                    return Err(PresaleError::AlreadyBought.into());
                }
            }
            2 => {
                if client.bought_amount_2 > 0 {
                    return Err(PresaleError::AlreadyBought.into());
                }
            }
            3 => {
                if client.bought_amount_3 > 0 {
                    return Err(PresaleError::AlreadyBought.into());
                }
            }
            4 => {
                if client.bought_amount_4 > 0 {
                    return Err(PresaleError::AlreadyBought.into());
                }
            }
            5 => {
                if client.bought_amount_5 > 0 {
                    return Err(PresaleError::AlreadyBought.into());
                }
            }
            6 => {
                if client.bought_amount_6 > 0 {
                    return Err(PresaleError::AlreadyBought.into());
                }
            }
            _ => {}
        }

        if presale.is_whitelist==true && client.is_whitelisted==false {
            return Err(PresaleError::NotWhitelisted.into());
        }

        match presale.round {
            1 => {
                client.bought_amount_1 = client.bought_amount_1 + _amount;
                client.bought_time_1 = _now_time;
            }
            2 => {
                client.bought_amount_2 = client.bought_amount_2 + _amount;
                client.bought_time_2 = _now_time;
            }
            3 => {
                client.bought_amount_3 = client.bought_amount_3 + _amount;
                client.bought_time_3 = _now_time;
            }
            4 => {
                client.bought_amount_4 = client.bought_amount_4 + _amount;
                client.bought_time_4 = _now_time;
            }
            5 => {
                client.bought_amount_5 = client.bought_amount_5 + _amount;
                client.bought_time_5 = _now_time;
            }
            6 => {
                client.bought_amount_6 = client.bought_amount_6 + _amount;
                client.bought_time_6 = _now_time;
            }
            _ => {}
        }
        
        presale.total_raised = presale.total_raised + _amount;
        presale.participants = presale.participants + 1;

        Ok(())
    }

    pub fn claim(ctx : Context<Claim>, _now_time : u64) -> ProgramResult {
        let client = &mut ctx.accounts.client;

        let presale = &mut ctx.accounts.presale;

        match presale.round {
            1 => {
                if client.bought_amount_1 == 0 {
                    return Err(PresaleError::NotBoughtYet.into());
                }
            }
            2 => {
                if client.bought_amount_2 == 0 {
                    return Err(PresaleError::NotBoughtYet.into());
                }
            }
            3 => {
                if client.bought_amount_3 == 0 {
                    return Err(PresaleError::NotBoughtYet.into());
                }
            }
            4 => {
                if client.bought_amount_4 == 0 {
                    return Err(PresaleError::NotBoughtYet.into());
                }
            }
            5 => {
                if client.bought_amount_5 == 0 {
                    return Err(PresaleError::NotBoughtYet.into());
                }
            }
            6 => {
                if client.bought_amount_6 == 0 {
                    return Err(PresaleError::NotBoughtYet.into());
                }
            }
            _ => {}
        }

        let mut bought_time: u64 = 0;
        let mut bought_amount: u64 = 0;
        let mut already_claimed: u64 = 0;

        match presale.round {
            1 => {
                bought_time = client.bought_time_1;
                bought_amount = client.bought_amount_1;
                already_claimed = client.already_claimed_1;
            }
            2 => {
                bought_time = client.bought_time_2;
                bought_amount = client.bought_amount_2;
                already_claimed = client.already_claimed_2;
            }
            3 => {
                bought_time = client.bought_time_3;
                bought_amount = client.bought_amount_3;
                already_claimed = client.already_claimed_3;
            }
            4 => {
                bought_time = client.bought_time_4;
                bought_amount = client.bought_amount_4;
                already_claimed = client.already_claimed_4;
            }
            5 => {
                bought_time = client.bought_time_5;
                bought_amount = client.bought_amount_5;
                already_claimed = client.already_claimed_5;
            }
            6 => {
                bought_time = client.bought_time_6;
                bought_amount = client.bought_amount_6;
                already_claimed = client.already_claimed_6;
            }
            _ => {}
        }

        let mut need_claimed_pecentage = presale.first_vesting + ((_now_time - bought_time) / SECONDS_PER_MONTH ) * presale.every_vesting;
            if need_claimed_pecentage > 100 {
                need_claimed_pecentage = 100;
            }
        let pending_amount = bought_amount * need_claimed_pecentage / 100 - already_claimed;
        
        // spl_token_transfer_without_seed(
        //     TokenTransferParamsWithoutSeed{
        //         source : ctx.accounts.auth_token.clone(),
        //         destination : ctx.accounts.client_pot.clone(),
        //         authority : ctx.accounts.authority.clone(),
        //         token_program : ctx.accounts.token_program.clone(),
        //         amount : pending_amount,
        //     }
        // )?;

        match presale.round {
            1 => {
                client.already_claimed_1 = client.already_claimed_1 + pending_amount;
            }
            2 => {
                client.already_claimed_2 = client.already_claimed_2 + pending_amount;
            }
            3 => {
                client.already_claimed_3 = client.already_claimed_3 + pending_amount;
            }
            4 => {
                client.already_claimed_4 = client.already_claimed_4 + pending_amount;
            }
            5 => {
                client.already_claimed_5 = client.already_claimed_5 + pending_amount;
            }
            6 => {
                client.already_claimed_6 = client.already_claimed_6 + pending_amount;
            }
            _ => {}
        }

        client.pending_amount = pending_amount;
        
        Ok(())
    }

    pub fn set_authority(ctx : Context<SetAuthority>) -> ProgramResult {
        let presale = &mut ctx.accounts.presale;

        spl_token_set_authority(
            TokenSetAuthorityParams{
                authority : ctx.accounts.owner.clone(),
                new_authority : ctx.accounts.new_authority.clone(),
                account : ctx.accounts.presale_pot.clone(),
                token_program : ctx.accounts.token_program.clone(),
            }
        )?;

        presale.owner = *ctx.accounts.new_authority.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(mut, signer)]
    /// CHECK: checked in program
    owner : AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: checked in program
    new_authority : AccountInfo<'info>,

    #[account(mut, owner=spl_token::id())]
    /// CHECK: checked in program
    presale_pot : AccountInfo<'info>,

    #[account(mut,has_one = owner)]
    presale : ProgramAccount<'info,PresaleData>,

    #[account(address=spl_token::id())]
    /// CHECK: checked in program
    token_program : AccountInfo<'info>,    
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut, signer)]
    /// CHECK: checked in program
    authority : AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: checked in program
    auth_token : AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: checked in program
    client_pot : AccountInfo<'info>,

    #[account(mut)]
    presale : ProgramAccount<'info,PresaleData>,
    
    #[account(mut)]
    client : ProgramAccount<'info,ClientData>,

    #[account(address=spl_token::id())]
    /// CHECK: checked in program
    token_program : AccountInfo<'info>, 
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    presale : ProgramAccount<'info,PresaleData>,
    
    #[account(mut)]
    client : ProgramAccount<'info,ClientData>, 
}

#[derive(Accounts)]
pub struct SetWhitelist<'info> {
    #[account(mut)]
    client : ProgramAccount<'info,ClientData>,
}

#[derive(Accounts)]
pub struct InitClient<'info> {
    #[account(init, payer = payer, space=8+CLIENT_DATA_SIZE)]
    client : ProgramAccount<'info,ClientData>,

    #[account(mut)]
    /// CHECK: checked in program
    authority : AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: checked in program
    payer : AccountInfo<'info>,

    system_program : Program<'info,System>
}

#[derive(Accounts)]
pub struct InitPresale<'info> {
    #[account(init, payer = owner, space=8+PRESALE_DATA_SIZE)]
    presale : ProgramAccount<'info,PresaleData>,

    #[account(mut)]
    owner : Signer<'info>,
    
    system_program : Program<'info,System>,
}

#[account]
pub struct PresaleData {
    pub owner : Pubkey,
    pub min_allocation : u64,
    pub max_allocation : u64,
    pub hardcap : u64,
    pub total_raised : u64,
    pub first_vesting : u64,
    pub every_vesting : u64,
    pub start_time : u64,
    pub end_time : u64,
    pub participants: u64,
    pub round : u16,
    pub is_whitelist : bool
}

#[account]
pub struct ClientData {
    pub owner : Pubkey,
    pub bought_amount_1 : u64,
    pub bought_amount_2 : u64,
    pub bought_amount_3 : u64,
    pub bought_amount_4 : u64,
    pub bought_amount_5 : u64,
    pub bought_amount_6 : u64,
    pub bought_time_1 : u64,
    pub bought_time_2 : u64,
    pub bought_time_3 : u64,
    pub bought_time_4 : u64,
    pub bought_time_5 : u64,
    pub bought_time_6 : u64,
    pub already_claimed_1 : u64,
    pub already_claimed_2 : u64,
    pub already_claimed_3 : u64,
    pub already_claimed_4 : u64,
    pub already_claimed_5 : u64,
    pub already_claimed_6 : u64,
    pub pending_amount : u64,
    pub is_whitelisted : bool
}

#[error]
pub enum PresaleError {
    #[msg("Account does not have correct owner")]
    IncorrectOwner,

    #[msg("Derived key is invalid")]
    DerivedKeyInvalid,

    #[msg("Invalid authority")]
    InvalidAuthority,

    #[msg("Presale has already started")]
    AlreadyStarted,

    #[msg("Data type mismatch")]
    DataTypeMismatch,

    #[msg("Already stopped")]
    AlreadyStopped,

    #[msg("Invalid client owner")]
    InvalidClientOwner,

    #[msg("Invalid presale account")]
    InvalidPresaleAccount,

    #[msg("Invalid token program")]
    InvalidTokenProgram,

    #[msg("Not match presale address")]
    NotMatchPresale,

    #[msg("Preslae is not active yet")]
    NotActiveYet,

    #[msg("Amount is invalid")]
    InvalidAmount,

    #[msg("Not match token address")]
    NotMatchTokenAddress,

    #[msg("Balance too low")]
    BalanceTooLow,

    #[msg("Hardcap has been reached")]
    HardcapReached,

    #[msg("You will be going over the hardcap")]
    WillOverHardcap,

    #[msg("You cant buy more than the max allocation")]
    MoreThanMaxAllocation,

    #[msg("You are not whitelisted")]
    NotWhitelisted,

    #[msg("Token transfer failed")]
    TokenTransferFailed,

    #[msg("Already distributed 100% of tokens")]
    AlreadyDistributedOverflow,

    #[msg("Not match mint address")]
    NotMatchMintAddress,

    #[msg("Not match owner address")]
    NotMatchOwnerAddress,

    #[msg("Not match presale address")]
    NotMatchPresaleAddress,

    #[msg("Not match presale pot address")]
    NotMatchPresalePotAddress,

    #[msg("Already paid")]
    AlreadyPaied,

    #[msg("Presale is not stopped yet")]
    NotStoppedYet,

    #[msg("Token set authority failed")]
    TokenSetAuthorityFailed,

    #[msg("Sale not started or already ended")]
    SaleNotActive,

    #[msg("You have not bought tokens yet")]
    NotBoughtYet,
    
    #[msg("You have already bought tokens in this round")]
    AlreadyBought,
}