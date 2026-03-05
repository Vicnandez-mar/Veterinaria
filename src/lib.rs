use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod biblioteca {
    use super::*;

    pub fn crear_biblioteca() -> Result<()> {
        //Código ........
    }
}

#[account]
#[derive(InitSpace)]
pub struct Biblioteca{
    owner: Pubkey,

    #[max_len(60)]
    name: String,

    #[max_len(20)]
    books: Vec<libro>,
} 

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, portialEq, Debug)]
pub struct libro {
    #[max_len(60)]
    name: String,

    page: u16,

    available: bool,
}

#[derive(Accounts)]
pub struct NuevaBiblioteca {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Biblioteca::INIT_SPACE + 8,
        seeds = [b"biblioteca", owner.key().as_ref{}],
        bump
    )]
    pub biblioteca: Account<'info, Biblioteca>,

    pub system_program: Program<'info, System>,
}


pub struct NuevoLibro {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,
}
