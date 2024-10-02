use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info,AccountInfo},
    system_instruction,
    sysvar::{rent::Rent,Sysvar},
    program::{invoke_signed},
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
};

use std::convert::TryInto;
use borsh::{BorshSerialize,BorshDeserialize};

use crate::error::ReviewError;
use crate::instruction::MovieInstruction;
use crate::state::{MovieAccountState, MovieCommentCounter, MovieComment};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction: &[u8],
) -> ProgramResult{
    //check why the error is not defined
    let instruction = MovieInstruction::unpack(instruction)?;
    match instruction{
        MovieInstruction::AddMovieReview{title, rating, description} => {
            add_movie_review(program_id, accounts, title,rating,description)
        }

        MovieInstruction::UpdateMovieReview{title,rating,description} =>{
            UpdateMovieReview(program_id,accounts,title,rating,description)
        }
        MovieInstruction::AddComment{comment} => {
            add_comment(program_id,accounts,comment)
        }

    }
}

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String
) -> ProgramResult{

    //Rating Check
    if rating >= 6{
        msg!("Rating Cant be higher than 5");
        return Err(ReviewError::InvalidRating.into());
    }

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let pda_counter = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    //Used Program Error not the custom Derieved Error
    if !initializer.is_signer{
        msg!("Missing Required Signature");
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    let (pda,bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(),title.as_bytes().as_ref(),], program_id);
    if pda != *pda_account.key{
        msg!("Invalid Seeds for PDA");
        return Err(ReviewError::invalidPDA.into());
    }

   // let total_len: usize = 1 + 1 + (4 + title.len()) + (4 + description.len());

    // if total_len > 1000{
    //     msg!("Data length Larger thank 1000 bytes");
    //     return Err(ReviewError::InvalidDataLenght.into());
    // }

    let account_len = 1000;

    if MovieAccountState::get_account_size(title.clone(),description.clone()) > account_len {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLenght.into());
    }


    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[initializer.clone(),pda_account.clone(),system_program.clone()],
        &[&[initializer.key.as_ref(),title.as_bytes().as_ref(),&[bump_seed]]],
    )?;

    msg!("PDA created: {}",pda);

    msg!("unpackaing State Account");
    let mut account_data = MovieAccountState::try_from_slice(&pda_account.data.borrow())?;
    //let mut account_data = try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("Borrowed account data");

    //Checking if the movie Account is already initialised or not
    if account_data.is_initialized{
        msg!("Account Data already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.descriminator = MovieAccountState::DISCRIMINATOR.to_string();
    account_data.reviewer = *initializer.key;
    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.is_initialized = true;

    msg!("Serializing Account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;


    //Logic To initialize the Counter Account
    msg!("Create comment Counter");
    let rent = Rent::get()?;
    let counter_rent_lamports = rent.minimum_balance(MovieCommentCounter::SIZE);

    let (counter, counter_bump) = Pubkey::find_program_address(&[pda.as_ref(), "comment".as_ref()],program_id);
    if counter != *pda_counter.key {
        msg!("Invalid seeds for PDA");
        return Err(ReviewError::InvalidArguments.into());
    }

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_counter.key,
            counter_rent_lamports,
            MovieCommentCounter::SIZE.try_into().unwrap(),
            program_id,
        ),
        &[initializer.clone(),pda_counter.clone(),system_program.clone(),],

        &[&[pda.as_ref(),"comment".as_ref(),&[counter_bump]]],
    )?;
    msg!("Comment Counter Created");

    let mut counter_data = MovieCommentCounter::try_from_slice(&pda_counter.data.borrow()).unwrap();

    msg!("Checking If counter is already initialised");
    if counter_data.is_initialized{
        msg!("account Already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    counter_data.discriminator = MovieCommentCounter::DISCRIMINATOR.to_string();
    counter_data.counter = 0;
    counter_data.is_initialized = true;
    msg!("Comment Count: {}", counter_data.counter);
    counter_data.serialize(&mut &mut pda_counter.data.borrow_mut()[..])?;
    Ok(())
}

pub fn UpdateMovieReview(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String
) -> ProgramResult{
    //Rating check
    if rating >= 6 || rating < 0 {
        msg!("Rating Cant be Higher than 5");
        return Err(ReviewError::InvalidRating.into());
    }

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let pda_counter = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if pda_account.owner != program_id{
        return Err(ReviewError::InvalidOwner.into());
    }

    if !initializer.is_signer{
        msg!("Missing Required Signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda,bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), title.as_bytes().as_ref(),],program_id);
    if pda != *pda_account.key{
        msg!("Invalid Seeds for PDA");
        return Err(ReviewError::invalidPDA.into());
    }

    let total_len = 1 + 1 + (4 + title.len()) + (4 + description.len());
    if total_len > 1000{
        msg!("Data length Larger than 1000 bytes");
    }

    let mut account_data = MovieAccountState::try_from_slice(&pda_account.data.borrow())?;
    if !account_data.is_initialized{
        msg!("Account Data is not initialized");
        return Err(ReviewError::AccountNotInitialised.into());
    }

    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.is_initialized = true;

    msg!("SErializing Account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    Ok(())
}

pub fn add_comment(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    comment: String,
) -> ProgramResult{
    msg!("Adding Comment...");
    msg!("comment: {}",comment);

    let account_info_iter = &mut accounts.iter();

    let commenter = next_account_info(account_info_iter)?;
    let pda_review = next_account_info(account_info_iter)?;
    let pda_counter = next_account_info(account_info_iter)?;
    let pda_comment = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let mut counter_data = MovieCommentCounter::try_from_slice(&pda_counter.data.borrow()).unwrap();
    let account_len = MovieComment::get_account_size(comment.clone());

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[pda_review.key.as_ref(),counter_data.counter.to_be_bytes().as_ref(),],
        program_id,
    );

    if pda != *pda_comment.key {
        msg!("Invalid Seeds for PDA");
        return Err(ReviewError::invalidPDA.into());
    }

    invoke_signed(
        &system_instruction::create_account(
            commenter.key,
            pda_comment.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[commenter.clone(),pda_comment.clone(),system_program.clone(),],
        &[&[pda_review.key.as_ref(),counter_data.counter.to_be_bytes().as_ref(),&[bump_seed],]],
    )?;

    msg!("Created Comment Account");

    let mut comment_data = MovieComment::try_from_slice(&pda_comment.data.borrow())?;

    msg!("checking if comment account is already initialised ");

    if comment_data.is_initialized{
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    comment_data.discriminator = MovieComment::DISCRIMINATOR.to_string();
    comment_data.review = *pda_review.key;
    comment_data.commentor = *commenter.key;
    comment_data.comment = comment;
    comment_data.is_initialized = true;

    comment_data.serialize(&mut &mut pda_comment.data.borrow_mut()[..])?;

    msg!("comment count : {}",counter_data.counter);
    counter_data.counter +=1;
    counter_data.serialize(&mut &mut pda_counter.data.borrow_mut()[..])?;

    Ok(())
}





