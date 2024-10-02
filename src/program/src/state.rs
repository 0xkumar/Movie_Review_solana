use borsh::{BorshSerialize, BorshDeserialize};
//use solana_program::program_pack::{IsInitialized,Sealed};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize,BorshSerialize)]
pub struct MovieAccountState{
    pub descriminator: String,
    pub is_initialized: bool,
    pub reviewer: Pubkey,
    pub rating: u8,
    pub title: String,
    pub description: String,
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct MovieCommentCounter{
    pub discriminator: String,
    pub is_initialized: bool,
    pub counter: u64,
}

#[derive(BorshDeserialize,BorshSerialize)]
pub struct MovieComment{
    pub discriminator: String,
    pub is_initialized: bool,
    pub review: Pubkey,
    pub commentor: Pubkey,
    pub comment : String,
    pub count:u64,
}

//Sealed is Solana's version of Rust's Sized trait
//This simply specifies that MovieAccountState has a known size and provides for some compiler optimizations.
// impl Sealed for MovieAccountState{}

impl MovieAccountState{
    pub const DISCRIMINATOR: &'static str = "review";

    pub fn get_account_size(title: String, description: String) -> usize{
        return (4 + MovieAccountState::DISCRIMINATOR.len()) + 1 + 1 + (4 + title.len()) + (4 + description.len())    }
}


impl MovieCommentCounter{
    pub const DISCRIMINATOR: &'static str = "counter";
    pub const SIZE: usize = (4 + MovieCommentCounter::DISCRIMINATOR.len()) + 1 + 8;
}

impl MovieComment{
    pub const DISCRIMINATOR: &'static str = "comment";

    pub fn get_account_size(comment: String) -> usize{
        return (4 + MovieComment::DISCRIMINATOR.len()) + 1 + 32 + 32 + (4 + comment.len()) + 8;
    }
}
