use anchor_lang::prelude::*;

declare_id!("Dw9Lc4NFEkZxvDVCS1S3vJASZmWqheWa14QbauzSex32");

#[program]
pub mod biblioteca {
    use super::*;

    pub fn crear_biblioteca(context: Context<NuevaBiblioteca>, name: String) -> Result<()> {
        let owner: Pubkey = context.accounts.owner.key();
        let books: Vec<Book> = Vec::new();

        context.accounts.biblioteca.set_inner(Biblioteca {
            owner,
            name,
            books,
        });

        Ok(())

    }

    pub fn agregar_libro(context: Context<NewBook>, name: String, page: u16) -> Result<()> {
        let book: Book = Book {
            name,
            page,
            available: true,
        };

        context.accounts.biblioteca.books.push(book);

        Ok(())
    }

    pub fn eliminar_libro(context: Context<NewBook>, name:String) -> Result<()> {
        let books: &mut Vec<Book> = &mut context.accounts.biblioteca.books;
        
        for book in 0..books.len() {
            if books[book].name == name {
                books.remove(book);
                msg!("Libro {name} eliminado!");
                return Ok(());
            }
        }

        Err(Errores::LibroNoExiste.into())
    }

    pub fn ver_libros(context: Context<NewBook>) -> Result<()> {
       msg!("The Book List is: {:#?}", context.accounts.biblioteca.books);

        Ok(())
    }

    pub fn alternar_estado(context: Context<NewBook>, name:String) -> Result<()> {
        let books= &mut context.accounts.biblioteca.books;

         for book in 0..books.len() {
            let estado = books[book].available;

            if books[book].name == name {
                let nuevo_estado = !estado;
                books[book].available = nuevo_estado;

                msg!("El libro: {} ahora tiene un valor de disponibilidad {}", name, nuevo_estado);
                return Ok(());
            }
        }

        Err(Errores::LibroNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propiedtario de la cuenta.")]
    Youarenottheowner,

    #[msg("Error, el libro proporcionado no existe.")]
    LibroNoExiste,

}

#[account]
#[derive(InitSpace)]
pub struct Biblioteca{
    owner: Pubkey,

    #[max_len(60)]
    name: String,

    #[max_len(20)]
    books: Vec<Book>,
} 

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Book {
    #[max_len(60)]
    name: String,

    page: u16,

    available: bool,
}

#[derive(Accounts)]
pub struct NuevaBiblioteca<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Biblioteca::INIT_SPACE + 8,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump
    )]
    pub biblioteca: Account<'info, Biblioteca>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct NewBook<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,
}
