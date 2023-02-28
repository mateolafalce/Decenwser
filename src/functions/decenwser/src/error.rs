use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("You are not the authority of this web domain")]AuthorityError, 
    #[msg("The website name is too long. 32 utf-8 characters or less is allowed")]TooLong, 
    #[msg("The maximum content saved by a PDA is 9900")]Max9900, 
}
