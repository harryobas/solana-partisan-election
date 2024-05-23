use anchor_lang::prelude::*;


declare_id!("44YZnv9ppP2kAaMQLXkTkEKQjjoH9xpazd5um6vTT6G7");

mod constants;
mod errors;

use constants::*;
use errors::*;

#[program]
pub mod election {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
       let election = &mut ctx.accounts.election;
       election.name = name;
       election.vote_count = 0;
       election.voter_register = std::collections::HashSet::new();
       election.party_register = std::collections::HashSet::new();
       election.ballot_box = vec![];
       election.result_sheet = std::collections::HashMap::new();
       election.authority = ctx.accounts.payer.key();
       election.voting_open = false;

        Ok(())
    }
    pub fn register_voter(ctx: Context<RegisterVoter>) -> Result<()> {
        let election = &mut ctx.accounts.election;
        let voter_id = ctx.accounts.authority.key();

        election.voter_register.insert(voter_id);

        Ok(())

    }
    pub fn register_party(ctx: Context<RegisterParty>, party_name: String) -> Result<()> {
        
        let election = &mut ctx.accounts.election;
        election.party_register.insert(party_name);

        Ok(())

    }
    pub fn cast_vote(ctx: Context<CastVote>, ballot: String) -> Result<()> {
        
        let election = &mut ctx.accounts.election;
        let voter_id = ctx.accounts.authority.key();

        require!(
            election.voting_open == true,
            ElectionError::VotingNotOpen
        );

        require!(
            election.voter_register.contains(&voter_id),
            ElectionError::NotRegisteredToVote
        );

        require!(
            election.party_register.contains(&ballot),
            ElectionError::InvalidArgument
        );
    
        election.ballot_box.push(ballot);
        election.vote_count += 1;

        Ok(())
    }
    pub fn close_voting(ctx: Context<CloseVoting>) -> Result<()> {
        let election = &mut ctx.accounts.election;

        require!(
            election.authority == ctx.accounts.authority.key(),
            ElectionError::Unauthorized 
        );

        require!(
            election.voting_open == true,
            ElectionError::VotingAlreadyClosed
        );

        election.voting_open = false;

        let mut result_sheet = election.result_sheet.clone();

        for ballot in election.ballot_box.iter() {
            let count = result_sheet.entry(ballot.to_string()).or_insert(0);
            *count += 1;
        };

        election.result_sheet = result_sheet;

        Ok(())
    }

    pub fn open_voting(ctx: Context<OpenVoting>) -> Result<()> {
        let election = &mut ctx.accounts.election;
        require!(
            election.authority == ctx.accounts.authority.key(),
            ElectionError::Unauthorized 
        );
        require!(
            election.voting_open == false,
            ElectionError::VotingAlreadyOpen
        );

        election.voting_open = true;

        Ok(())
    }
    
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Election>(),
        seeds = [ELECTION_SEED.as_bytes()],
        bump         
    )]
    pub election: Box<Account<'info, Election>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    
}

#[derive(Accounts)]
pub struct RegisterVoter<'info> {
    #[account(mut)]
    pub election: Box<Account<'info, Election>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterParty<'info> {
    #[account(mut)]
    pub election: Box<Account<'info, Election>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub election: Box<Account<'info, Election>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseVoting<'info> {
    #[account(mut)]
    pub election: Box<Account<'info, Election>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OpenVoting<'info> {
    #[account(mut)]
    pub election: Box<Account<'info, Election>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}



#[account]
pub struct Election {
    pub name: String,
    pub vote_count: u64,
    pub voter_register: std::collections::HashSet<Pubkey>,
    pub party_register: std::collections::HashSet<String>,
    pub ballot_box: Vec<String>,
    pub result_sheet: std::collections::HashMap<String, u64>,
    pub authority: Pubkey,
    pub voting_open: bool,

}




