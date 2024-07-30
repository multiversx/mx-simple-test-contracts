#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait SimpleEsdtTest {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint]
    fn local_mint(&self, token_id: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send().esdt_local_mint(&token_id, nonce, &amount);
        self.tx()
            .to(ToCaller)
            .single_esdt(&token_id, 0, &amount)
            .transfer();
    }

    #[payable("*")]
    #[endpoint]
    fn burn(&self) {
        let esdt = self.call_value().single_esdt();
        self.send()
            .esdt_local_burn(&esdt.token_identifier, esdt.token_nonce, &esdt.amount);
    }

    #[endpoint]
    fn nft_create(
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
}
