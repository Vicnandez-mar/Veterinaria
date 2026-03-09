use anchor_lang::prelude::*;

declare_id!("Au38Sv27yiAJpH4PXop6Mjv4ewn58fdd3ArvymAh4oXL");

#[program]
pub mod veterinaria {
    use super::*;

    // Crear un nuevo registro
    pub fn create_pet(
        ctx: Context<CreatePet>,
        nombre: String,
        raza: String,
        especie: String,
        edad: u8,
        vacunacion: String,
        dueno: String,
        enfermedades: String,
    ) -> Result<()> {
        let pet_account = &mut ctx.accounts.pet_account;
        pet_account.nombre = nombre;
        pet_account.raza = raza;
        pet_account.especie = especie;
        pet_account.edad = edad;
        pet_account.vacunacion = vacunacion;
        pet_account.dueno = dueno;
        pet_account.enfermedades = enfermedades;
        Ok(())
    }

    // Modificar un registro existente
    pub fn update_pet(
        ctx: Context<UpdatePet>,
        nombre: String,
        raza: String,
        especie: String,
        edad: u8,
        vacunacion: String,
        dueno: String,
        enfermedades: String,
    ) -> Result<()> {
        let pet_account = &mut ctx.accounts.pet_account;
        pet_account.nombre = nombre;
        pet_account.raza = raza;
        pet_account.especie = especie;
        pet_account.edad = edad;
        pet_account.vacunacion = vacunacion;
        pet_account.dueno = dueno;
        pet_account.enfermedades = enfermedades;
        Ok(())
    }

    // Borrar un registro
    pub fn delete_pet(_ctx: Context<DeletePet>) -> Result<()> {
        // Anchor automáticamente cierra la cuenta cuando se usa `close`
        Ok(())
    }
}

// Contextos
#[derive(Accounts)]
pub struct CreatePet<'info> {
    #[account(init, payer = user, space = 8 + PetAccount::MAX_SIZE)]
    pub pet_account: Account<'info, PetAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePet<'info> {
    #[account(mut, has_one = user)]
    pub pet_account: Account<'info, PetAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeletePet<'info> {
    #[account(mut, close = user, has_one = user)]
    pub pet_account: Account<'info, PetAccount>,
    pub user: Signer<'info>,
}

// Datos de la mascota
#[account]
pub struct PetAccount {
    pub nombre: String,
    pub raza: String,
    pub especie: String,
    pub edad: u8,
    pub vacunacion: String,
    pub dueno: String,
    pub enfermedades: String,
    pub user: Pubkey,
}

impl PetAccount {
    pub const MAX_SIZE: usize = 
        4 + 50 + // nombre
        4 + 50 + // raza
        4 + 50 + // especie
        1 +      // edad
        4 + 100 + // vacunacion
        4 + 50 + // dueño
        4 + 200 + // enfermedades
        32;      // user pubkey
}
