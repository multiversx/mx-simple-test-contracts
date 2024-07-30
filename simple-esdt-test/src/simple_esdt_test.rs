#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub const ESDT_NFT_CREATE_FUNC_NAME: &str = "ESDTNFTCreate";

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
            .single_esdt(&token_id, nonce, &amount)
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
        token_id: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        creator: ManagedBuffer,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();

        arg_buffer.push_arg(&token_id);
        arg_buffer.push_arg(amount);
        arg_buffer.push_arg(&ManagedBuffer::new()); // name
        arg_buffer.push_arg(&BigUint::zero()); // royalties
        arg_buffer.push_arg(&ManagedBuffer::new()); // hash
        arg_buffer.push_arg(&ManagedBuffer::new()); //attributes
        arg_buffer.push_arg(&ManagedVec::<Self::Api, ManagedBuffer>::new()); // uris

        arg_buffer.push_arg(nonce);
        arg_buffer.push_arg(creator);

        let _ = self.send_raw().call_local_esdt_built_in_function(
            self.blockchain().get_gas_left(),
            &ManagedBuffer::from(ESDT_NFT_CREATE_FUNC_NAME),
            &arg_buffer,
        );
    }
}
