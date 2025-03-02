use anchor_lang::prelude::*;

declare_id!("9DrVUc9XJKWWJcw8ra3SmtWMJGudCzd2YHmSMZfskjZP");

// Anchor programs always use 8 bits for the discriminator
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
pub const LENTH: usize = MAX_WALLET_NAME_LEN + MAX_TOKEN_LEN + MAX_DATA_ENTRIES + MAX_KEY_LEN;

const MAX_WALLET_NAME_LEN: usize = 50;
const MAX_TOKEN_LEN: usize = 10;
const MAX_DATA_ENTRIES: usize = 10;
const MAX_KEY_LEN: usize = 20;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorits(
        ctx: Context<SetFavorits>,
        walt_name: String,
        tkn_name: String,
        blc: u64,
    ) -> Result<()> {
        let user_pub_key = ctx.accounts.user.key();
        msg!("hello mother fucker's, the program is starting ... ");
        msg!(
            "Hello {}, you added data to wallet: {}",
            user_pub_key,
            walt_name
        );
        let mut vec = Vec::new();
        let entry = DataEntry {
            key: tkn_name.clone(),
            value: blc,
        };
        vec.push(entry);
        ctx.accounts.favorits.set_inner(Myfavorites {
            wallet_name: walt_name.clone(),
            token: tkn_name.clone(),
            balance: blc,
            data: vec.clone(),
        });
        msg!("Succsesfly update acount ... ");
        msg!("your new data updated is {:?} ", vec);
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DataEntry {
    pub key: String,
    pub value: u64,
}

// this is a data structure of an account
#[account]
pub struct Myfavorites {
    pub wallet_name: String,

    pub token: String,

    pub balance: u64,

    pub data: Vec<DataEntry>,
}

//this will be inject on blockchain
//this struct should have signer/seed/datato inject ==> this is the program
// *** user type of sugne evry transaction on this program should have a signe***
// *** favorit  is a data that will be injected that hold favorit ***
// *** favorits is of type Account it take<life time,data> it should be under derive macro account
// *** evry eccount shoild be have
//-> payer : type user : signer
//-> space : declare the space of your program + your data structure space
//-> seed  : is a program derives adress PDA generated : this adres is managed only by the system program  who give an adress on blockchain
//         take (key,value)==> key is the adress of bytes maded of struct, value is the user pub_key
//->bump : calculate the seed
//->systeme_program : xith type program take life time and system
//
#[derive(Accounts)]
pub struct SetFavorits<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + LENTH,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorits: Account<'info, Myfavorites>,
    pub system_program: Program<'info, System>,
}
