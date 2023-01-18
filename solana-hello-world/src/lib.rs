use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

// Programın okunmaya başlayacağı satırı ayarla
entrypoint!(process_instruction);

// Programın okunmaya başlayacağı fonksiyon
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // blockchain'e mesaj kaydet
    msg!("Merhaba, Solana!");

    // programı sonlandır
    Ok(())
}
