multiversx_sc::imports!();
multiversx_sc::derive_imports!();



use crate::types::EquipSlot;

#[multiversx_sc::module]
pub trait Storage {

//EQUIP
    #[storage_mapper("equipped_nfts")]
    fn equipped_nfts(&self, wallet_address: &ManagedAddress) -> MapMapper<EquipSlot, (TokenIdentifier, u64)>;
//END  

//COLLECTION PROPERTIES
    #[storage_mapper("collectionIds")]// Storage to hold the list of collection IDs
    fn collection_ids(&self) -> VecMapper<TokenIdentifier>;

    #[view(getNftName)]
    #[storage_mapper("nft_name")]
    fn nft_name(&self,collection_id: &TokenIdentifier) -> SingleValueMapper<ManagedBuffer>;

    #[view(getNftPrice)]
    #[storage_mapper("nft_price")]
    fn nft_price(&self,collection_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getCollectionNonce)]//number of NFTs minted for specific collection
    #[storage_mapper("collection_nonce")]
    fn collection_nonce(&self,collection_id: &TokenIdentifier) -> SingleValueMapper<u64>;

    #[view(getNftMax)]//the maximum number of NFTs that can be minted for specific collection
    #[storage_mapper("nft_max")]
    fn nft_max(&self,collection_id: &TokenIdentifier) -> SingleValueMapper<u64>;

    #[view(getNeedSocket)]
    #[storage_mapper("need_socket")]
    fn need_socket(&self,collection_id: &TokenIdentifier) -> SingleValueMapper<bool>;

    #[storage_mapper("royalties")]
    fn royalties(&self,collection_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(isPaused)]//used to temporary stop nft minting for a specific collection
    #[storage_mapper("paused")]
    fn paused(&self,collection_id: &TokenIdentifier) -> SingleValueMapper<bool>;

    #[view(getImageCid)]
    #[storage_mapper("image_cid")]
    fn image_cid(&self,collection_id: &TokenIdentifier) -> VecMapper<ManagedBuffer>;// ["cid1", "cid2"]

    #[view(getMetadataCid)]
    #[storage_mapper("metadata_cid")]
    fn metadata_cid(&self,collection_id: &TokenIdentifier) -> VecMapper<ManagedBuffer>;// ["cid1", "cid2"]

    #[storage_mapper("nft_count")]// number of NFTs per Wave
    fn nft_count(&self,collection_id: &TokenIdentifier) -> VecMapper<u16>;

    #[storage_mapper("max_socket")]
    fn max_socket(&self,collection_id: &TokenIdentifier) -> VecMapper<u8>;

    #[storage_mapper("max_crystal_variants")]
    fn max_crystal_variants(&self,collection_id: &TokenIdentifier) -> VecMapper<u16>;

    #[storage_mapper("max_bonusses_variants")]
    fn max_bonusses_variants(&self,collection_id: &TokenIdentifier) -> VecMapper<u32>;

    #[storage_mapper("equip_slot")]
    fn equip_slot(&self,collection_id: &TokenIdentifier) -> SingleValueMapper<EquipSlot>;

//END

//NFT PROPERTIES
    #[view(getPaymentTokenId)] //token used to mint nfts
    #[storage_mapper("payment_token_id")]
    fn payment_token_id(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getNftWave)]
    #[storage_mapper("nft_wave")]
    fn nft_wave(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u32>;

    #[view(getHasBonus)]
    #[storage_mapper("has_bonus")]
    fn has_bonus(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<bool>;

    #[view(getHasCrystal)]
    #[storage_mapper("has_crystal")]
    fn has_crystal(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<bool>;

    #[view(getNftQuality)] //nft quality. quality 0 represents an GameItem+0
    #[storage_mapper("nft_quality")]
    fn nft_quality(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u32>;

    #[view(getNftCrystalVariant)]
    #[storage_mapper("nft_crystal_variant")]
    fn nft_crystal_variant(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u32>;

    #[view(getNftBonusVariant)]
    #[storage_mapper("nft_bonus_variant")]
    fn nft_bonus_variant(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u32>;

    #[view(getBonusCount)] //the number of bonuses added to the NFT
    #[storage_mapper("bonus_count")]
    fn bonus_count(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u8>;

    #[view(getCrystalCount)] //the number of crystals added to the NFT
    #[storage_mapper("crystal_count")]
    fn crystal_count(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u8>;

    #[view(getSocketCount)] //the number of sockets filled with crystals.
    #[storage_mapper("socket_count")]
    fn socket_count(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u8>;

    #[view(getTclCount)] //the number of TCL staked on specific nft
    #[storage_mapper("tcl_count")]
    fn tcl_count(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<BigUint>;

    #[view(getTclMax)] //"Storage" the maximum number of TCL that can be stacked on a specific nft
    #[storage_mapper("tcl_max")]
    fn tcl_max(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<BigUint>;

    #[view(getRefinementTimestamp)] //the time when last refinement was added
    #[storage_mapper("refinement_timestamp")]
    fn refinement_timestamp(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u64>;

    #[view(getSftPrice)]
    #[storage_mapper("sft_price")]
    fn sft_price(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<BigUint>;

    #[view(getSftSold)]
    #[storage_mapper("sft_sold")]
    fn sft_sold(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u32>;

    #[view(getSftMax)]
    #[storage_mapper("sft_max")]
    fn sft_max(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u32>;

//END

//GLOBAL STORAGE
    #[view(getNftUpgradePrice)]
    #[storage_mapper("nft_upgrade_price")]
    fn nft_upgrade_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getAddBonusPrice)]
    #[storage_mapper("add_bonus_price")]
    fn add_bonus_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getChangeBonusPrice)]
    #[storage_mapper("change_bonus_price")]
    fn change_bonus_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getAddCrystalPrice)]
    #[storage_mapper("add_crystal_price")]
    fn add_crystal_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getChangeCrystalPrice)]
    #[storage_mapper("change_crystal_price")]
    fn change_crystal_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getAddSocketPrice)]
    #[storage_mapper("add_socket_price")]
    fn add_socket_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getAddRefinementPrice)]
    #[storage_mapper("add_refinement_price")]
    fn add_refinement_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getUpgradeBonusBhance)]
    #[storage_mapper("upgrade_bonus_chance")] // default = 0  upgrade bonus chance; 10 = +10% upgrade chance
    fn upgrade_bonus_chance(&self) -> SingleValueMapper<u32>;

    #[storage_mapper("private_seed")]// used to generate unpredictible random number. update after each generation.
    fn private_seed(&self) -> SingleValueMapper<u64>;

    #[view(getTclUsdPrice)]
    #[storage_mapper("tcl_usd_price")]//price in usd expressed in 18 decimals. 1$ = 1_000_000_000_000_000_000
    fn tcl_usd_price(&self) -> SingleValueMapper<BigUint>;
//END

//REFERRAL
    #[view(getReferralOwner)]
    #[storage_mapper("referral_owner")]
    fn referral_owner(&self, referral_code: &ManagedBuffer) -> SingleValueMapper<ManagedAddress>;

    #[view(getReferralCode)]
    #[storage_mapper("referral_code")]
    fn referral_code(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<ManagedBuffer>;

    #[view(getReferralEarned)]
    #[storage_mapper("referral_earned")]
    fn referral_earned(&self, referral_code: &ManagedBuffer) -> SingleValueMapper<BigUint>;

    #[view(getReferralCodeInvitee)]
    #[storage_mapper("referral_code_invitee")]
    fn referral_code_invitee(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<ManagedBuffer>;

    #[view(getReferralInvitees)]
    #[storage_mapper("referral_invitees")]
    fn referral_invitees(&self, referral_code: &ManagedBuffer) -> SingleValueMapper<u32>;

    #[view(getTransactionsInvitees)]
    #[storage_mapper("transactions_invitees")]
    fn transactions_invitees(&self, referral_code: &ManagedBuffer) -> SingleValueMapper<u64>;

    #[view(getTeamWallet)]
    #[storage_mapper("team_wallet")]
    fn team_wallet(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getTotalEarnedAllReferrals)]
    #[storage_mapper("total_earned_all_referrals")]
    fn total_earned_all_referrals(&self) -> SingleValueMapper<BigUint>;

    #[view(getTotalInviteesAllReferrals)]
    #[storage_mapper("total_invitees_all_referrals")]
    fn total_invitees_all_referrals(&self) -> SingleValueMapper<u32>;

    #[view(getActiveReferralCodes)]
    #[storage_mapper("active_referral_codes")]
    fn active_referral_codes(&self) -> SingleValueMapper<u32>;

    #[view(getTotalReferralCodes)]
    #[storage_mapper("total_referral_codes")]
    fn total_referral_codes(&self) -> SingleValueMapper<u32>;

    #[view(getTransactionsAllInvitees)]
    #[storage_mapper("total_transactions_all_invitees")]
    fn total_transactions_all_invitees(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("referral_code_invitees")]
    fn referral_code_invitees(&self,referral_code: &ManagedBuffer) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("active_referral_code_list")]
    fn active_referral_code_list(&self) -> UnorderedSetMapper<ManagedBuffer>;

//END

//REWARD
    #[view(getTotalReserveAmount)]
    #[storage_mapper("total_reserve_amount")]
    fn total_reserve_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getTotalRewardsReleased)]
    #[storage_mapper("total_rewards_released")]
    fn total_rewards_released(&self) -> SingleValueMapper<BigUint>;

    #[view(getAprMax)]
    #[storage_mapper("apr_max")]
    fn apr_max(&self) -> SingleValueMapper<u16>;

    #[view(getRewardTokenId)]
    #[storage_mapper("reward_token_id")]
    fn reward_token_id(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getTotalStakedAmount)]
    #[storage_mapper("total_staked_amount")]
    fn total_staked_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getServerWallet)]
    #[storage_mapper("server_wallet")]
    fn server_wallet(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getUserStakedAmount)]
    #[storage_mapper("user_staked_amount")]
    fn user_staked_amount(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getUserBoostStakedAmount)]
    #[storage_mapper("user_boost_staked_amount")]
    fn user_boost_staked_amount(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getUserLoanedAmount)]
    #[storage_mapper("user_loaned_amount")]
    fn user_loaned_amount(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("last_claimed_epoch")]
    fn last_claimed_epoch(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getLastClaimedLendingEpoch)]
    #[storage_mapper("last_claimed_lending_epoch")]
    fn last_claimed_lending_epoch(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getUserInfinityStakedAmount)]
    #[storage_mapper("user_infinity_staked_amount")]
    fn user_infinity_staked_amount(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(lastClaimedInfinityEpoch)]
    #[storage_mapper("last_claimed_infinity_epoch")]
    fn last_claimed_infinity_epoch(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(totalUserInfinityRewards)]
    #[storage_mapper("total_user_infinity_rewards")]
    fn total_user_infinity_rewards(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getTotalInfinityStakedAmount)]
    #[storage_mapper("total_infinity_staked_amount")]
    fn total_infinity_staked_amount(&self) -> SingleValueMapper<BigUint>;


//END

//LENDING

    #[storage_mapper("available_borrow_nfts")]
    fn available_borrow_nfts(&self,epoch: &u64) -> UnorderedSetMapper<(TokenIdentifier, u64)>;
    #[storage_mapper("borrowed_nfts")]
    fn borrowed_nfts(&self,epoch: &u64) -> UnorderedSetMapper<(TokenIdentifier, u64)>;

    #[storage_mapper("loaned_nfts")]
    fn loaned_nfts(&self,wallet_address: &ManagedAddress) -> UnorderedSetMapper<(TokenIdentifier, u64)>;

    #[storage_mapper("last_nft_claimed_epoch")]
    fn last_nft_claimed_epoch(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u64>;

    #[storage_mapper("last_borrowed_nft_claimed_epoch")]
    fn last_borrowed_nft_claimed_epoch(&self,collection_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u64>;

    #[storage_mapper("last_borrowed_claimed_epoch")]
    fn last_borrowed_claimed_epoch(&self,wallet_address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getUserBorrowedAmount)]
    #[storage_mapper("user_borrowed_amount")]
    fn user_borrowed_amount(&self, wallet_address: &ManagedAddress, epoch: &u64) -> SingleValueMapper<BigUint>;

    #[storage_mapper("borrowed_nft")]
    fn borrowed_nft(&self,wallet_address: &ManagedAddress, epoch: &u64) -> SingleValueMapper<(TokenIdentifier, u64)>;

    #[storage_mapper("min_amount_to_borrow")]
    fn min_amount_to_borrow(&self) -> SingleValueMapper<BigUint>;
//

//ITEM SHOP
    #[storage_mapper("coin_packs")]
    fn coin_packs(&self) -> SetMapper<(u32, BigUint)>; //(coins_count, tcl_price)

    #[storage_mapper("user_purchase_history")]
    fn user_purchase_history(&self, wallet_address: &ManagedAddress) -> SetMapper<(u64, u32)>; //(timestamp, coins_count)

    #[storage_mapper("total_packs_purchased")]
    fn total_packs_purchased(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("total_tcl_paid_for_packs")]
    fn total_tcl_paid_for_packs(&self) -> SingleValueMapper<BigUint>;

//

//PRIVATE SHOP
    #[view(getUserBalancePrivateShop)]
    #[storage_mapper("user_balance_private_shop")]
    fn user_balance_private_shop(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getPrivateShopBalanceTimestamp)]
    #[storage_mapper("private_shop_balance_timestamp")]
    fn private_shop_balance_timestamp(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<u64>;
//

//AUTO CLAIM
    #[view(getAutoClaimSubscribers)]
    #[storage_mapper("auto_claim_subscribers")]
    fn auto_claim_subscribers(&self) -> VecMapper<ManagedAddress>;

    #[view(getSubscriberIndex)]
    #[storage_mapper("subscriber_index")]
    fn subscriber_index(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<usize>;

    #[view(getReinvestInfinity)]
    #[storage_mapper("reinvest_infinity")]
    fn reinvest_infinity(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<bool>;

    #[view(getPriceEgldAutoclaim)]
    #[storage_mapper("price_egld_autoclaim")]
    fn price_egld_autoclaim(&self) -> SingleValueMapper<BigUint>;

    #[view(getEndSubscriptionEpoch)]
    #[storage_mapper("end_subscription_epoch")]
    fn end_subscription_epoch(&self, wallet_address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getBatchAutoClaim)]
    #[storage_mapper("batch_auto_claim")]
    fn batch_auto_claim(&self) -> SingleValueMapper<usize>;

    #[view(getAutoClaimedCount)]
    #[storage_mapper("auto_claimed_count")]
    fn auto_claimed_count(&self, epoch: &u64) -> SingleValueMapper<usize>;

    #[storage_mapper("temporary_unsubscribe_list")]
    fn temporary_unsubscribe_list(&self) -> VecMapper<usize>;

//

} 