use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("You are not the authority of this web domain")]AuthorityError, 
    #[msg("The website name is too long. 32 utf-8 characters or less is allowed")]TooLong, 
    #[msg("The secure check is true. Only the admin of the web will be able to modify it")]SecureCheckError,
}
