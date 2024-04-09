solana_program::declare_id!("MyProgram111111111111111111111111111111111111111");
#[solana_program::program]
pub mod my_program {
    use solana_program::pubkey::Pubkey;

    #[solana_program::entry]
    pub fn entrypoint(_ctx: Context, _accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
        // Program logic here
        Ok(())
    }
}