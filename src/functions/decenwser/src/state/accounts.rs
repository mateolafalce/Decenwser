use anchor_lang::prelude::*;

#[account]
pub struct MainAccount {
    pub bump_original: u8,      // 1
    pub authority: Pubkey,      // 32
    pub web_name: String,       // 4 + 32
    pub html: Vec<u64>,         // 4 + 8
    pub css: Vec<u64>,          // 4 + 8
    pub js: Vec<u64>,           // 4 + 8
    pub len: u16,               // 2
}
#[account]
pub struct DecenwserAccount {
    pub pages_online: u64,
    pub total_updates: u64,
    pub bump_original: u8
}
#[account]
pub struct HTML {
    pub html: String
}
#[account]
pub struct CSS {
    pub css: String      
}
#[account]
pub struct JS {
    pub js: String    
}
impl MainAccount {
    pub const SIZE: usize = 1 + 4 + 32 + 4 + 32 + 8 + 4 + 8 + 4 + 8 + 2;
}
impl DecenwserAccount {
    pub const SIZE: usize = 8 + 8 + 1;
}
impl HTML {
    pub const SIZE: usize = 932;
}
impl CSS {
    pub const SIZE: usize = 932;
}
impl JS {
    pub const SIZE: usize = 932;
}
