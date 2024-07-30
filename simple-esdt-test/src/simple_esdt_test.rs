#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait SimpleEsdtTest {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(ESDTLocalMint)]
    fn esdt_local_mint(&self, token_id: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send().esdt_local_mint(&token_id, nonce, &amount);
    }

    #[endpoint(ESDTLocalBurn)]
    fn esdt_local_burn(&self, token_id: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send().esdt_local_burn(&token_id, nonce, &amount);
    }

    #[endpoint(ESDTNFTCreate)]
    fn esdt_nft_create(
        &self,
        token_id: &TokenIdentifier,
        amount: &BigUint,
        name: &ManagedBuffer,
        royalties: &BigUint,
        hash: &ManagedBuffer,
        attributes: &ManagedBuffer,
        uris: &ManagedVec<ManagedBuffer>,
    ) {
        self.send()
            .esdt_nft_create(&token_id, &amount, name, royalties, hash, attributes, uris);
    }

    #[endpoint(ESDTNFTAddQuantity)]
    fn esdt_nft_add_quantity(&self, token_id: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send().esdt_local_mint(&token_id, nonce, &amount);
    }

    #[endpoint(ESDTNFTBurn)]
    fn esdt_nft_burn(&self, token_id: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send().esdt_local_burn(&token_id, nonce, &amount);
    }
}
