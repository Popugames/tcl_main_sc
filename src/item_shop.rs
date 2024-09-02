multiversx_sc::imports!();
multiversx_sc::derive_imports!();


use crate::storage;
use crate::utils;

#[multiversx_sc::module]
pub trait ItemShop: storage::Storage + utils::Utils{


    #[only_owner]
    #[payable("*")]
    #[endpoint(setCoinPacks)]
    fn set_coin_packs(&self, coin_packs: MultiValueEncoded<MultiValue2<u32, BigUint>>){
        self.set_packs(coin_packs);
    }

    #[payable("*")]
    #[endpoint(buyCoins)]
    fn buy_coins(&self, coins_count: u32) {
        let caller = self.blockchain().get_caller();
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();
        let payment_token_id = self.payment_token_id().get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        
        require!(
            payment_token == payment_token_id,
            "Invalid token paid"
        );
        require!(
            self.coin_packs().contains(&(coins_count.clone(), payment_amount.clone())),
            "No coin pack available for the requested coin amount"
        );

        // Add transaction to user purchase history
        self.user_purchase_history(&caller).insert((current_timestamp, coins_count));

        // Increase total_packs_purchased
        self.total_packs_purchased().update(|v| *v += 1);

        // Update total_tcl_paid_for_packs
        self.total_tcl_paid_for_packs().update(|v| *v += payment_amount.clone());

        // Distribuim tokenii de plată
        self.distribute_tokens(&caller, payment_amount);
    }

//SETERS 
    //(coins_count, tcl_price)
    fn set_packs(&self, coin_packs: MultiValueEncoded<MultiValue2<u32, BigUint>>) {
        self.coin_packs().clear();

        for value in coin_packs.into_iter() {
            let (coins_count, tcl_price) = value.into_tuple();
            self.coin_packs().insert((coins_count, tcl_price));
        }
    }
//END  

    #[endpoint(getCoinPacks)]
    fn get_coin_packs(&self) -> ManagedBuffer {

        let mut return_buffer = ManagedBuffer::new();

        for (coins_count, tcl_price) in self.coin_packs().iter() {
            
            return_buffer.append(&self.decimal_to_ascii((coins_count as u32).try_into().unwrap())); // Coins Count
            return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
            return_buffer.append(&self.biguint_to_ascii(&tcl_price)); //TCL Price
            return_buffer.append(&ManagedBuffer::new_from_bytes(b","));
        }

        return_buffer
    }
}
