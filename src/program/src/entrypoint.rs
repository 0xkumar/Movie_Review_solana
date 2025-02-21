use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    entrypoint
};

use crate::processor;

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult{
    msg!("Process Instruction: {} : {} accounts, data={:?}",
        program_id,accounts.len(),instruction_data);

    processor::process_instruction(program_id,accounts,instruction_data)?;
    Ok(())
}

