multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage;
use crate::utils;

#[multiversx_sc::module]
pub trait PrivateShop: storage::Storage + utils::Utils{

    
    #[payable("*")]
    #[endpoint(addBalancePrivateShop)]
    fn add_balance_private_shop(&self) {

        let caller = self.blockchain().get_caller();
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();
        let payment_token_id = self.payment_token_id().get();
        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(
            self.user_balance_private_shop(&caller).is_empty(),
            "another payment is in processing"
        );
        require!(
            payment_token == payment_token_id,
            "Invalid token paid"
        );

        self.user_balance_private_shop(&caller).update(|v| *v += payment_amount);
        self.private_shop_balance_timestamp(&caller).set(current_timestamp);
    }

    #[endpoint(refundPrivateShop)]
    fn refund_private_shop(&self) {

        let caller = self.blockchain().get_caller();
        let payment_token_id = self.payment_token_id().get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let private_shop_balance_timestamp = self.private_shop_balance_timestamp(&caller).get();
        let user_balance_private_shop = self.user_balance_private_shop(&caller).get();

        require!(
            user_balance_private_shop > BigUint::zero(),
            "all purchases have been processed"
        );
        require!(
            private_shop_balance_timestamp + 300u64 < current_timestamp,
            "the purchase processing has not finished"
        );

        self.user_balance_private_shop(&caller).clear();
        self.private_shop_balance_timestamp(&caller).clear();

        self.send().direct(
            &caller,
            &payment_token_id,
            0u64,
            &user_balance_private_shop
        );

    }

    #[endpoint(withdrawBalancePrivateShop)]
    fn withdraw_balance_private_shop(
        &self, 
        buyer_address: ManagedAddress, 
        seller_address: ManagedAddress, 
        amount: BigUint
    ) {

        let payment_token_id = self.payment_token_id().get();
        let user_balance_private_shop = self.user_balance_private_shop(&buyer_address).get();
        let server_wallet = self.blockchain().get_caller();

        require!(
            server_wallet == self.server_wallet().get(),
            "invalid caller"
        );

        require!(
            amount == user_balance_private_shop,
            "insufficient tokens in balance"
        );

        self.user_balance_private_shop(&buyer_address).clear();
        self.private_shop_balance_timestamp(&buyer_address).clear();

        let seller_amount = amount.clone() * BigUint::from(90u8) / BigUint::from(100u8); // 90%
        let distribution_amount = amount.clone() * BigUint::from(10u8) / BigUint::from(100u8); // 10%

        self.distribute_shop_fees(buyer_address, distribution_amount);

        self.send().direct(
            &seller_address,
            &payment_token_id,
            0u64,
            &seller_amount
        );
    }


}
