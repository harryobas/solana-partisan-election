use anchor_lang::prelude::*;
use std::mem;

declare_id!("44YZnv9ppP2kAaMQLXkTkEKQjjoH9xpazd5um6vTT6G7");

mod constants;

use constants::*;

#[program]
pub mod partisan_election {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
       let election = &mut ctx.accounts.election;
       election.name = name.as_bytes().to_vec();
       election.vote_count = 0;
       election.voter_register = vec![];
       election.party_register = vec![];
       election.ballot_box = vec![];
       election.result_sheet = std::collections::HashMap::new();
       election.voting_open = false;

        Ok(())
    }
    pub fn register_voter(ctx: Context<RegisterVoter>) -> Result<()> {

    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = mem::size_of::<>() + 8,
        seeds = [ELECTION_SEED.as_bytes()],
        bump         
    )]
    pub election: Account<'info, Election>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    
}


#[account]
pub struct Election {
    pub name: Vec<u8>,
    pub vote_count: u64,
    pub voter_register: Vec<Pubkey>,
    pub party_register: Vec<Box<[u8]>>,
    pub ballot_box: Vec<Box<[u8]>>,
    pub result_sheet: std::collections::HashMap<String, u64>,
    voting_open: bool,

}


