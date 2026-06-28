use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint,
    account_info::{
        next_account_info, 
        AccountInfo
    },
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize)]
struct OnChainData {
    count: u32,
}

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let data_acc = next_account_info(&mut iter)?;

    // match data_acc {
    //     Ok(data_acc) => (),
    //     Err(err) => {return Err(err)}
    // }

    let mut counter = OnChainData::try_from_slice(&data_acc.data.borrow())?;

    if counter.count == 0 {
        counter.count = 1;
    } else {
        counter.count = counter.count * 2;
    }

    counter.serialize(&mut *data_acc.data.borrow_mut())?;

    Ok(())
}