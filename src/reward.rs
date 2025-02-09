multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage;
use crate::utils;
use core::cmp;

#[multiversx_sc::module]

pub trait RewardModule: storage::Storage + utils::Utils{

    
    #[payable("*")]
    #[endpoint(addInfinityStaking)]
    fn add_infinity_staking(&self)
    {
        let caller = self.blockchain().get_caller();
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();
        let payment_token_id = self.payment_token_id().get();

        require!(
            payment_token == payment_token_id, 
            "invalid token paid"
        );

        require!(
            &payment_amount >= &BigUint::from(10u128.pow(18)),
            "too small amount"
        );

        //add to infinity stake
        self.stake_infinity(&caller, &payment_amount);
    }

    fn stake_infinity(&self, wallet_address: &ManagedAddress, amount: &BigUint){

        //GLOBAL
        self.total_staked_amount().update(|v| *v += amount.clone());
        self.total_infinity_staked_amount().update(|v| *v += amount.clone());
 
        //LOCAL
        self.user_infinity_staked_amount(&wallet_address).update(|v| *v += amount.clone());
         
        // BURN
        self.send().esdt_local_burn(
            &self.payment_token_id().get().unwrap_esdt(),
            0u64,
            &amount
        );

    }

    #[payable("*")]
    #[endpoint(addReserve)]
    fn add_reserve(&self) {
        
        let reward_token = self.reward_token_id().get();
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();

        require!(payment_token == reward_token, "Invalid token input");
        self.total_reserve_amount().update(|v| *v += &payment_amount);
    }

    fn stake(&self,wallet_address: &ManagedAddress, amount: &BigUint) {
        //GLOBAL
        self.total_staked_amount().update(|v| *v += amount.clone());

        //LOCAL
        self.user_staked_amount(wallet_address).update(|v| *v += amount.clone());
    }

    fn unstake(&self,wallet_address: &ManagedAddress, amount: &BigUint) {
        //GLOBAL
        self.total_staked_amount().update(|v| *v -= amount.clone());

        //LOCAL
        self.user_staked_amount(wallet_address).update(|v| *v -= amount.clone());
    }

    fn stake_loaned(&self,wallet_address: &ManagedAddress, amount: &BigUint) {

        let penalised_amount = amount.clone() * BigUint::from(80u64) / BigUint::from(100u64);

        //GLOBAL
        self.total_staked_amount().update(|v| *v += amount.clone());

        //LOCAL
        self.user_loaned_amount(wallet_address).update(|v| *v += penalised_amount);
    }

    fn unstake_loaned(&self,wallet_address: &ManagedAddress, amount: &BigUint) {

        let penalised_amount = amount.clone() * BigUint::from(80u64) / BigUint::from(100u64);

        //GLOBAL
        self.total_staked_amount().update(|v| *v -= amount.clone());

        //LOCAL
        self.user_loaned_amount(wallet_address).update(|v| *v -= penalised_amount.clone());
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self,wallet_address: ManagedAddress) {
        let server_wallet = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();
        let last_claimed_epoch = self.last_claimed_epoch(&wallet_address).get();
        let user_staked_amount = self.user_staked_amount(&wallet_address).get();

        require!(
            server_wallet == self.server_wallet().get(),
            "invalid caller"
        );
        require!(
            current_epoch > last_claimed_epoch,
            "rewards already claimed in this epoch"
        );
        require!(
            &user_staked_amount > &0,
            "no token staked"
        );
        require!(
            self.apr_max().get()>0,
            "apr_max not set"
        );

        //Calculate rewards
        let user_reward = self.calculate_reward(&wallet_address, user_staked_amount, true);


        //GLOBAL
        self.total_reserve_amount().update(|v| *v -= &user_reward);
        self.total_rewards_released().update(|v| *v += &user_reward);

        //LOCAL
        self.last_claimed_epoch(&wallet_address).set(current_epoch);

        //SEND TOKENS
        self.send().direct(
            &wallet_address,
            &self.reward_token_id().get(),
            0u64,
            &user_reward
        );

        //AUTOCLAIM
        self.auto_claim_list();
    }

    #[endpoint(claimLendingRewards)]
    fn claim_lending_rewards(&self, wallet_address: ManagedAddress) {
        // Obține informații despre apelant și starea curentă
        let caller = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();
        let next_epoch = &current_epoch + &1u64; 
        let last_claimed_lending_epoch = self.last_claimed_lending_epoch(&caller).get();
        let user_loaned_amount = self.user_loaned_amount(&caller).get();
        let server_wallet = self.blockchain().get_caller();

        require!(
            server_wallet == self.server_wallet().get() || &caller == &wallet_address,
            "invalid caller"
        );
        // Verifică condițiile pentru revendicare
        require!(
            &current_epoch > &last_claimed_lending_epoch,
            "rewards already claimed in this epoch"
        );
        require!(
            &user_loaned_amount > &0,
            "no token staked"
        );
        require!(
            self.apr_max().get()>0,
            "apr_max not set"
        );
        
        //Calculate rewards and boost rewards
        let user_reward = self.calculate_reward(&wallet_address, user_loaned_amount, false);
        let min_amount_to_borrow = self.min_amount_to_borrow().get();

        // Actualizează recompensele totale eliberate
        self.total_reserve_amount().update(|v| *v -= &user_reward);
        self.total_rewards_released().update(|v| *v += &user_reward);
    
        // Actualizează ultima epocă de revendicare pentru utilizator
        self.last_claimed_lending_epoch(&caller).set(&current_epoch);
    
        //Setam epoca curenta pentru fiecare NFT dat imprumut
        for (collection_id, nonce) in self.loaned_nfts(&caller).iter() {

            self.last_nft_claimed_epoch(&collection_id, &nonce).set(&current_epoch);
            let tcl_count = self.tcl_count(&collection_id, &nonce).get();

            if &tcl_count > &min_amount_to_borrow
            {
                // Adaugă NFT-ul în lista disponibilă pentru urmatoarea epoca
                self.available_borrow_nfts(&next_epoch).insert((collection_id.clone(), nonce.clone()));

                //Adaugă NFT-ul în lista disponibilă pentru epoca curenta daca nu exista sau nu a fost imprumutat
                let is_borrowed = self.borrowed_nfts(&current_epoch).contains(&(collection_id.clone(), nonce.clone()));
                let is_available = self.available_borrow_nfts(&current_epoch).contains(&(collection_id.clone(), nonce.clone()));
                let last_borrowed_nft_claimed_epoch = self.last_borrowed_nft_claimed_epoch(&collection_id, &nonce).get();

                if &last_borrowed_nft_claimed_epoch < &current_epoch && !is_available && !is_borrowed{
                self.available_borrow_nfts(&current_epoch).insert((collection_id.clone(), nonce.clone()));
                }
            }
        }
    
        // Trimite recompensa către portofelul utilizatorului
        self.send().direct(
            &caller,
            &self.reward_token_id().get(),
            0u64,
            &user_reward
        );
    }

    #[endpoint(claimBorrowingRewards)]
    fn claim_borrowing_rewards(&self,wallet_address: ManagedAddress) {
        let server_wallet = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();
        let user_borrowed_amount = self.user_borrowed_amount(&wallet_address, &current_epoch).get();
        let is_borrowed = !self.user_borrowed_amount(&wallet_address, &current_epoch).is_empty();
        let user_borrowed_amount = self.user_borrowed_amount(&wallet_address, &current_epoch).get();
        let (collection_id, nonce) = self.borrowed_nft(&wallet_address,&current_epoch).get();


        require!(
            server_wallet == self.server_wallet().get(),
            "invalid caller"
        );
        require!(
            is_borrowed,
            "not borrowed"
        );
        require!(
            self.apr_max().get()>0,
            "apr_max not set"
        );

        let user_reward = self.calculate_reward(&wallet_address, user_borrowed_amount, false);

        //GLOBAL
        self.total_reserve_amount().update(|v| *v -= &user_reward);
        self.total_rewards_released().update(|v| *v += &user_reward);

        self.last_borrowed_claimed_epoch(&wallet_address).set(&current_epoch);
        self.borrowed_nft(&wallet_address, &current_epoch).clear();
        self.user_borrowed_amount(&wallet_address, &current_epoch).clear();
        self.last_borrowed_nft_claimed_epoch(&collection_id, &nonce).set(&current_epoch);
       
        //SEND TOKENS
        self.send().direct(
            &wallet_address,
            &self.reward_token_id().get(),
            0u64,
            &user_reward
        );

        //AUTOCLAIM
        self.auto_claim_list();
    }

    #[endpoint(claimInfinityRewards)]
    fn claim_infinity_rewards(&self, wallet_address: ManagedAddress) {
        // Obține informații despre apelant și starea curentă
        let caller = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();
        let next_epoch = &current_epoch + &1u64; 
        let last_claimed_infinity_epoch = self.last_claimed_infinity_epoch(&caller).get();
        let user_infinity_staked_amount = self.user_infinity_staked_amount(&caller).get();

        require!(
            &caller == &self.server_wallet().get() || &caller == &wallet_address,
            "invalid caller"
        );
        // Verifică condițiile pentru revendicare
        require!(
            &current_epoch > &last_claimed_infinity_epoch,
            "infinity rewards already claimed in this epoch"
        );
        require!(
            &user_infinity_staked_amount > &0,
            "no token staked"
        );
        require!(
            self.apr_max().get()>0,
            "apr_max not set"
        );
        
        //Calculate rewards and boost rewards
        let user_reward = self.calculate_reward(&wallet_address, user_infinity_staked_amount, false);

        // Actualizează recompensele totale eliberate
        self.total_reserve_amount().update(|v| *v -= &user_reward);
        self.total_rewards_released().update(|v| *v += &user_reward);

        // Actualizează recompensele totale ale utilizatorului
        self.total_user_infinity_rewards(&caller).update(|v| *v += &user_reward);
    
        // Actualizează ultima epocă de revendicare pentru utilizator
        self.last_claimed_infinity_epoch(&caller).set(&current_epoch);
    
        if self.reinvest_infinity(&caller).get(){
            //add to infinity stake(reinvest)
            self.stake_infinity(&caller, &user_reward);
        }else{
            // Trimite recompensa către portofelul utilizatorului
            self.send().direct(
                &caller,
                &self.reward_token_id().get(),
                0u64,
                &user_reward
            );
        }
    }

    fn subscribe_auto_claim(&self,wallet_address: &ManagedAddress) {
        let auto_claim_subscribers = self.auto_claim_subscribers();
       
        if self.subscriber_index(&wallet_address).get() == 0usize{
            self.auto_claim_subscribers().push(&wallet_address);
            self.subscriber_index(wallet_address).set(self.auto_claim_subscribers().len());
            self.process_subscriber(wallet_address);
        }
    }

    fn unsubscribe_auto_claim(&self,index_to_unsubscribe: &usize) {
        let current_epoch = self.blockchain().get_block_epoch();
        let last_index = self.auto_claim_subscribers().len();
        let last_subscriber = self.auto_claim_subscribers().get(last_index.clone());
        self.auto_claim_subscribers().swap_remove(index_to_unsubscribe.clone());
        let unsubscribe_wallet = self.auto_claim_subscribers().get(index_to_unsubscribe.clone());
        self.subscriber_index(&unsubscribe_wallet).set(0);

        if &last_index > index_to_unsubscribe{
            self.process_subscriber(&last_subscriber);
            self.subscriber_index(&last_subscriber).set(index_to_unsubscribe);
            self.auto_claimed_count(&current_epoch).update(|v| *v += 1);
        }
    }

    #[endpoint(setReinvestInfinity)]
    fn set_reinvest_infinity(&self, is_reinvest:bool) {
        let caller = self.blockchain().get_caller();
        self.reinvest_infinity(&caller).set(is_reinvest);
    }

    #[payable("EGLD")]
    #[endpoint(addDaysAutoClaim)]
    fn add_days_auto_claim(&self,#[payment_amount] payment_amount: BigUint) {
        let caller = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();
        let end_subscription_epoch = self.end_subscription_epoch(&caller).get();
        let price_egld_autoclaim = self.price_egld_autoclaim().get();
    
        // Verificăm dacă plata este suficientă
        require!(
            &payment_amount >= &price_egld_autoclaim,
            "insufficient payment"
        );
    
        // Verificăm dacă plata este un multiplu al prețului
        require!(
            &payment_amount % &price_egld_autoclaim == BigUint::zero(),
            "payment must be a multiple of the price"
        );
    
        // Calculăm numărul de epochs plătite
        let paid_epochs = (payment_amount.clone() / price_egld_autoclaim).to_u64().unwrap();
    
        // Actualizăm timpul de expirare al subscripției
        if &end_subscription_epoch == &0u64 || &end_subscription_epoch <= &current_epoch {
            self.end_subscription_epoch(&caller).set(current_epoch + paid_epochs);
        } else {
            self.end_subscription_epoch(&caller).set(end_subscription_epoch + paid_epochs);
        }

        //add do subscribers list
        self.subscribe_auto_claim(&caller);

        //SEND EGLD TO OWNER
        let server_wallet = self.server_wallet().get();
        self.send().direct(
            &server_wallet,
            &EgldOrEsdtTokenIdentifier::egld(),
            0,
            &payment_amount
        );
    }

    fn auto_claim_list(&self) {
        let current_epoch = self.blockchain().get_block_epoch();
        let batch_auto_claim = self.batch_auto_claim().get();
        let auto_claimed_count = self.auto_claimed_count(&current_epoch).get();
        let auto_claim_subscribers = self.auto_claim_subscribers();
        let subscribers_len = auto_claim_subscribers.len(); // 1-based length
    
        if subscribers_len > 0 && auto_claimed_count < subscribers_len {
            // exemplu: dacă auto_claimed_count=0, batch=20, subscribers_len=3
            // => end_index = 3
            let end_index = cmp::min(auto_claimed_count + batch_auto_claim, subscribers_len);
            let start_index = auto_claimed_count + 1; // trecem pe 1-based
    
            let mut unsubscribe_list = self.temporary_unsubscribe_list();
            unsubscribe_list.clear();
    
            // parcurgem inclusiv end_index
            // => pentru exemplul dat, i = 1..=3 => i = 1,2,3
            for i in start_index..=end_index {
                let subscriber = auto_claim_subscribers.get(i);
    
                if self.end_subscription_epoch(&subscriber).get() >= current_epoch {
                    self.process_subscriber(&subscriber);
                    self.auto_claimed_count(&current_epoch).update(|v| *v += 1);
                } else {
                    unsubscribe_list.push(&i);
                }
            }
    
            // procesăm unsubscribe_list
            for index in unsubscribe_list.into_iter() {
                self.unsubscribe_auto_claim(&index);
            }
        }
    }
    
    fn process_subscriber(&self, wallet_address: &ManagedAddress ){
        let current_epoch = self.blockchain().get_block_epoch();

        //NFT LOAN
        let last_claimed_lending_epoch = self.last_claimed_lending_epoch(&wallet_address).get();
        let user_loaned_amount = self.user_loaned_amount(&wallet_address).get();

        if &user_loaned_amount > &BigUint::zero() && &last_claimed_lending_epoch < &current_epoch{
            let next_epoch = &current_epoch + &1u64; 
            //Calculate rewards and boost rewards
            let user_reward = self.calculate_reward(&wallet_address, user_loaned_amount, false);
            let min_amount_to_borrow = self.min_amount_to_borrow().get();

            // Actualizează recompensele totale eliberate
            self.total_reserve_amount().update(|v| *v -= &user_reward);
            self.total_rewards_released().update(|v| *v += &user_reward);
        
            // Actualizează ultima epocă de revendicare pentru utilizator
            self.last_claimed_lending_epoch(&wallet_address).set(&current_epoch);
        
            //Setam epoca curenta pentru fiecare NFT dat imprumut
            for (collection_id, nonce) in self.loaned_nfts(&wallet_address).iter() {

                self.last_nft_claimed_epoch(&collection_id, &nonce).set(&current_epoch);
                let tcl_count = self.tcl_count(&collection_id, &nonce).get();

                if &tcl_count > &min_amount_to_borrow
                {
                    // Adaugă NFT-ul în lista disponibilă pentru urmatoarea epoca
                    self.available_borrow_nfts(&next_epoch).insert((collection_id.clone(), nonce.clone()));

                    //Adaugă NFT-ul în lista disponibilă pentru epoca curenta daca nu exista sau nu a fost imprumutat
                    let is_borrowed = self.borrowed_nfts(&current_epoch).contains(&(collection_id.clone(), nonce.clone()));
                    let is_available = self.available_borrow_nfts(&current_epoch).contains(&(collection_id.clone(), nonce.clone()));
                    let last_borrowed_nft_claimed_epoch = self.last_borrowed_nft_claimed_epoch(&collection_id, &nonce).get();

                    if &last_borrowed_nft_claimed_epoch < &current_epoch && !is_available && !is_borrowed{
                    self.available_borrow_nfts(&current_epoch).insert((collection_id.clone(), nonce.clone()));
                    }
                }
            }
        
            // Trimite recompensa către portofelul utilizatorului
            self.send().direct(
                &wallet_address,
                &self.reward_token_id().get(),
                0u64,
                &user_reward
            );
        }

        //INFINITY STAKING
        let last_claimed_infinity_epoch = self.last_claimed_infinity_epoch(&wallet_address).get();
        let user_infinity_staked_amount = self.user_infinity_staked_amount(&wallet_address).get();

        if &user_infinity_staked_amount > &BigUint::zero() && &last_claimed_infinity_epoch < &current_epoch{
            //Calculate rewards and boost rewards
             let user_reward = self.calculate_reward(&wallet_address, user_infinity_staked_amount, false);

            // Actualizează recompensele totale eliberate
            self.total_reserve_amount().update(|v| *v -= &user_reward);
            self.total_rewards_released().update(|v| *v += &user_reward);

            // Actualizează recompensele totale ale utilizatorului
            self.total_user_infinity_rewards(&wallet_address).update(|v| *v += &user_reward);
    
            // Actualizează ultima epocă de revendicare pentru utilizator
            self.last_claimed_infinity_epoch(&wallet_address).set(&current_epoch);
    
            if self.reinvest_infinity(&wallet_address).get(){
                //add to infinity stake(reinvest)
                self.stake_infinity(&wallet_address, &user_reward);
            }else{
                // Trimite recompensa către portofelul utilizatorului
                self.send().direct(
                    &wallet_address,
                    &self.reward_token_id().get(),
                    0u64,
                    &user_reward
                );
            }
        }
    }

    // TO DELETE TO DELETE TO DELETE TO DELETE TO DELETE TO DELETE TO DELETE TO DELETE TO DELETE TO DELETE TO DELETE 
    #[only_owner]
    #[endpoint(resetIndex)]
    fn resetIndex(&self, wallet_address: ManagedAddress) {

        let subscriber_index = self.subscriber_index(&wallet_address).get();
        let subscriber = if subscriber_index <= self.auto_claim_subscribers().len(){    
            self.auto_claim_subscribers().get(subscriber_index)
        }else{
            ManagedAddress::default()
        };

        if subscriber != wallet_address{
            self.subscriber_index(&wallet_address).set(0);
            let current_epoch = self.blockchain().get_block_epoch();
            let end_subscription_epoch = self.end_subscription_epoch(&wallet_address).get();
    
            if end_subscription_epoch >= current_epoch{
                //add do subscribers list
                self.subscribe_auto_claim(&wallet_address);
            }
        }
    }

}