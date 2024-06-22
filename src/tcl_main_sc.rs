#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod reward;
pub mod nft;
pub mod referral;
pub mod equip;
pub mod types;
pub mod storage;
pub mod utils;


#[multiversx_sc::contract]
pub trait Contract : reward::RewardModule + equip::EquipModule + nft::NftModule + referral::ReferralModule + storage::Storage + utils::Utils{

    #[init]
    fn init(&self,token_id:EgldOrEsdtTokenIdentifier) {
    }
    #[upgrade]
    fn upgrade(&self,token_id:EgldOrEsdtTokenIdentifier) {
        self.reward_token_id().set(token_id);
    }

}