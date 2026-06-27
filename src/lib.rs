use solana_program::{account_info::AccountInfo,
    entrypoint::{ProgramResult},
    entrypoint,
    pubkey::PubKey
}

entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize)]
struct OnChainData {
    count: u32,
};

fn process_instruction(
    program_id: &PubKey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let data_acc = next_account_info(&mut iter)?;

    // match data_acc {
    //     Ok(data_acc) => (),
    //     Err(err) => {return Err(err)}
    // }

    let counter = OnChainData::try_from_slice(&data_acc.data.borrow_mut())?;

    if counter.count == 0 {
        counter.count = 1;
    } else {
        counter.count = counter.count * 2;
    }

    counter.serialize(&mut *data_acc.data.borrow_mut());

    Ok(())

}