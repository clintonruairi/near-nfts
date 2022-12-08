use crate::*;
use near_sdk::ext_contract;

pub trait NonFungibleTokenCore {
    // approve an account ID to transfer a token on your behalf
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>);

    // check if passed in account has access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;

    // revoke a specific accoutn from tranferring token on my behalf
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);

    // revoke all accounts from transgferring token on my behalf
    fn nft_revoke_all(&mut self, token_id: TokenId);
}

trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}
