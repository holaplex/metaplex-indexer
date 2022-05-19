use borsh::BorshDeserialize;
use indexer_core::db::{insert_into, models::BuyInstruction, tables::buy_instructions};

use super::Client;
use crate::prelude::*;

#[derive(BorshDeserialize, Debug, Clone)]
pub struct InstructionParameters {
    trade_state_bump: u8,
    escrow_payment_bump: u8,
    buyer_price: u64,
    token_size: u64,
}

pub(crate) async fn process(client: &Client, data: &[u8], accounts: &[Pubkey]) -> Result<()> {
    let params = InstructionParameters::try_from_slice(data).context("failed to deserialize")?;

    let row = BuyInstruction {
        wallet: Owned(
            accounts
                .get(0)
                .context("failed to get wallet pubkey")?
                .to_string(),
        ),
        payment_account: Owned(
            accounts
                .get(1)
                .context("failed to get payment account pubkey")?
                .to_string(),
        ),
        transfer_authority: Owned(
            accounts
                .get(2)
                .context("failed to get transfer authority pubkey")?
                .to_string(),
        ),
        treasury_mint: Owned(
            accounts
                .get(3)
                .context("failed to get treasury mint pubkey")?
                .to_string(),
        ),
        token_account: Owned(
            accounts
                .get(4)
                .context("failed to get token account pubkey")?
                .to_string(),
        ),
        metadata: Owned(
            accounts
                .get(5)
                .context("failed to get metadata pubkey")?
                .to_string(),
        ),
        escrow_payment_account: Owned(
            accounts
                .get(6)
                .context("failed to get escrow payment account pubkey")?
                .to_string(),
        ),
        authority: Owned(
            accounts
                .get(7)
                .context("failed to get authority pubkey")?
                .to_string(),
        ),
        auction_house: Owned(
            accounts
                .get(8)
                .context("failed to get auction house pubkey")?
                .to_string(),
        ),
        auction_house_fee_account: Owned(
            accounts
                .get(9)
                .context("failed to get auction house fee account pubkey")?
                .to_string(),
        ),
        buyer_trade_state: Owned(
            accounts
                .get(10)
                .context("failed to get buyer trade state pubkey")?
                .to_string(),
        ),
        trade_state_bump: params.trade_state_bump.try_into()?,
        escrow_payment_bump: params.escrow_payment_bump.try_into()?,
        buyer_price: params.buyer_price.try_into()?,
        token_size: params.token_size.try_into()?,
        created_at: Utc::now().naive_utc(),
    };

    dbg!("{:?}", &row);

    client
        .db()
        .run(move |db| {
            insert_into(buy_instructions::table)
                .values(&row)
                .execute(db)
        })
        .await
        .context("failed to insert buy instruction ")?;
    Ok(())
}
