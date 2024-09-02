multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage;
use crate::utils;

#[multiversx_sc::module]

pub trait RewardModule: storage::Storage + utils::Utils{

    

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

    }
    
    

    

}