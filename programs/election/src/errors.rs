use anchor_lang::prelude::*;

#[error_code]
pub enum ElectionError {
    #[msg("you are not authorized to perform this action")]
    Unauthorized,
    #[msg("invalid argument")]
    InvalidArgument,
    #[msg("voting is not open")]
    VotingNotOpen,
    #[msg("voting is already open")]
    VotingAlreadyOpen,
    #[msg("voting is already closed")]
    VotingAlreadyClosed,
    #[msg("you are not registered to vote in this election")]
    NotRegisteredToVote,

}