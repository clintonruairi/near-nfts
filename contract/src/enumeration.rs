impl Contract {
    // get total supply of NFTs on the contract
    pub fn nft_total_supply(&self) -> u128 {
        todo!();
    }

    // Query for nft tokens on the contract regardless of the owner using pagination

    pub fn nft_tokens(&self, from_index: Option<u128>, limit: Option<u64>) -> Vec<JsonToken> {
        todo!();
    }

    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> u128 {
        todo!();
    }

    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<u128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        todo!();
    }
}
