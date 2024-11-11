use anchor_lang::prelude::*;

declare_id!("5fgrj1KVQF4PHoAg1s9dSgkobZ8udxmHZW3ENn6SRjLx");

#[program]
pub mod anchor_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
