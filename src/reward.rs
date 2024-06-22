multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage;

#[multiversx_sc::module]

pub trait RewardModule: storage::Storage{

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

    #[endpoint(claimRewards)]
    fn claim_rewards(&self,wallet_address: &ManagedAddress) {
        let server_wallet = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();
        let last_claimed_epoch = self.last_claimed_epoch(&wallet_address).get();

        require!(
            server_wallet == self.server_wallet().get(),
            "invalid caller"
        );
        require!(
            current_epoch > last_claimed_epoch,
            "rewards already claimed in this epoch"
        );
        require!(
            self.user_staked_amount(&wallet_address).get()>0,
            "no token staked"
        );
        require!(
            self.apr_max().get()>0,
            "apr_max not set"
        );

        let user_reward = self.calculate_reward(&wallet_address);

        //GLOBAL
        self.total_rewards_released().update(|v| *v += &user_reward);

        //LOCAL
        self.last_claimed_epoch(wallet_address).set(current_epoch);

        //SEND TOKENS
        self.send().direct(
            &wallet_address,
            &self.reward_token_id().get(),
            0u64,
            &user_reward
        );

    }

    #[endpoint(calculateReward)]
    fn calculate_reward(&self,wallet_address: &ManagedAddress) -> BigUint {

        let total_reserve = self.total_reserve_amount().get();
        let user_staked_amount = self.user_staked_amount(wallet_address).get();
        let total_staked_amount = self.total_staked_amount().get();
        let apr_max = self.apr_max().get();

        if &user_staked_amount == &BigUint::zero()
        {
            return BigUint::zero();
        }

        let global_daily_reward = (total_reserve/BigUint::from(100u64))/BigUint::from(30u64); 
        let user_reward =  (global_daily_reward*(&user_staked_amount *10000000000u64/ total_staked_amount))/10000000000u64;
        let max_reward = (user_staked_amount * BigUint::from(apr_max)/BigUint::from(100u64))/BigUint::from(365u64);

        if user_reward>max_reward
        {
            return max_reward;
        }
        else
        {
            return user_reward;
        }
    }

}