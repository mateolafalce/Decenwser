use anchor_lang::prelude::*;

#[account]
pub struct MainAccount {
    pub bump_original: u8,      // 1
    pub authority: Pubkey,      // 32
    pub web_name: String,       // 4 + 32
    pub html: Vec<u16>,         // 4 + 2
    pub js: Vec<u16>,           // 4 + 2
    pub len: u16                // 2
}
#[account]
pub struct StoreAccount {
    pub content: String,       // 931
    pub bump_original: u8   // 1
}
impl MainAccount {
    pub const SIZE: usize = 1 + 32 + 4 + 32 + (( 4 + 2 )*2) + 2;
}
impl StoreAccount {
    pub const SIZE: usize = 931 + 1;
}

