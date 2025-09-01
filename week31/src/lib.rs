use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    entrypoint,
    pubkey::Pubkey,
    msg
};

#[derive(BorshSerialize, BorshDeserialize)]
enum InstructionType {
    Increment(u32),
    Decrement(u32)
}

#[derive(BorshSerialize, BorshDeserialize)]
struct Counter {
    count: u32,
}

entrypoint!(counter_program);

pub fn counter_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let account = next_account_info(&mut accounts.iter())?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;    // converting array of bytes i.e., instrction_data to InstructionType enum
    let mut counter_data = Counter::try_from_slice(&account.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(value) => {
            counter_data.count += value;
        },
        InstructionType::Decrement(value) => {
            counter_data.count -= value;
        }
    }

    counter_data.serialize(&mut *account.data.borrow_mut())?;
    msg!("Counter updated to {}", counter_data.count);

    Ok(())
}