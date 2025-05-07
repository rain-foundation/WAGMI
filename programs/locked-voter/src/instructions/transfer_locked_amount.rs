use crate::*;

// cannot access pubkey! macro without solana_program crate.
// Have to use `declared_id` trick.
mod admin_pubkey {
    use super::*;
    declare_id!("11111111111111111111111111111111");
}

#[account]
#[derive(Debug, Default)]
pub struct TransferRequest {
    /// The owner of the [Escrow] source.
    pub owner: Pubkey,
    /// The [Escrow] source pubkey.
    pub escrow_source: Pubkey,
    /// The [Escrow] destination pubkey.
    pub escrow_destination: Pubkey,
    /// Amount to transfer
    pub amount: u64,
}

impl TransferRequest {
    pub const LEN: usize = std::mem::size_of::<Pubkey>() * 3 + 8;
}

#[derive(Accounts)]
pub struct NewTransferRequest<'info> {
    /// Payer of the initialization.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The owner of the [Escrow] source.
    pub owner: Signer<'info>,

    /// [TransferRequest].
    #[account(
        init,
        payer = payer,
        space = 8 + TransferRequest::LEN
    )]
    pub request: Account<'info, TransferRequest>,

    /// [Locker].
    pub locker: Box<Account<'info, Locker>>,

    /// [Escrow].
    #[account(mut, has_one = locker, has_one = owner)]
    pub escrow_source: Box<Account<'info, Escrow>>,

    /// [Escrow].
    #[account(mut, has_one = locker)]
    pub escrow_destination: Box<Account<'info, Escrow>>,

    /// System program.
    pub system_program: Program<'info, System>,
}

impl<'info> NewTransferRequest<'info> {
    /// Creates a new [TransferRequest].
    pub fn new_transfer_request(&mut self) -> Result<()> {
        let request = &mut self.request;

        request.owner = self.owner.key();
        request.escrow_source = self.escrow_source.key();
        request.escrow_destination = self.escrow_destination.key();
        request.amount = self.escrow_source.amount;

        Ok(())
    }
}

impl<'info> Validate<'info> for NewTransferRequest<'info> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Accounts for [voter::transfer_locked_amount].
#[derive(Accounts)]
pub struct TransferLockedAmount<'info> {
    /// Admin pubkey
    #[account(address = ADMIN_PUBKEY::ID)]
    pub admin: Signer<'info>,

    /// The owner of the [Escrow] source.
    /// CHECK: using has_one constraints
    #[account(mut)]
    pub owner: UncheckedAccount<'info>,

    /// [Locker].
    pub locker: Box<Account<'info, Locker>>,

    /// [Escrow].
    #[account(mut, has_one = locker, has_one = owner)]
    pub escrow_source: Box<Account<'info, Escrow>>,

    /// [Escrow].
    #[account(mut, has_one = locker)]
    pub escrow_destination: Box<Account<'info, Escrow>>,

    /// [TransferRequest].
    #[account(mut, has_one = owner, has_one = escrow_source, has_one = escrow_destination)]
    pub request: Account<'info, TransferRequest>,
}

impl<'info> TransferLockedAmount<'info> {
    /// Creates a new [Escrow].
    pub fn transfer_locked_amount(&mut self) -> Result<()> {
        let escrow_source = &mut self.escrow_source;
        let escrow_destination = &mut self.escrow_destination;

        require!(escrow_source.amount > 0, crate::ErrorCode::AmountIsZero);

        escrow_source.amount = 0;
        escrow_destination.amount = unwrap_int!(escrow_destination.amount.checked_add(escrow_source.amount));

        Ok(())
    }
}

impl<'info> Validate<'info> for TransferLockedAmount<'info> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

