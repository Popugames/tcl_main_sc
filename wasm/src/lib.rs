// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                          137
// Async Callback (empty):               1
// Total number of exported functions: 139

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    tcl_main_sc
    (
        init => init
        upgrade => upgrade
        addInfinityStaking => add_infinity_staking
        addReserve => add_reserve
        claimRewards => claim_rewards
        claimLendingRewards => claim_lending_rewards
        claimBorrowingRewards => claim_borrowing_rewards
        claimInfinityRewards => claim_infinity_rewards
        setReinvestInfinity => set_reinvest_infinity
        addDaysAutoClaim => add_days_auto_claim
        resetIndex => resetIndex
        equipNft => equip_nft
        unequipNft => unequip_nft
        getEquippedNfts => get_equipped_nfts
        getNftsData => get_nfts_data
        addTcl => add_tcl
        setGlobalProps => set_global_props
        setCollection => set_collection
        setSft => set_sft
        setNewPrice => set_new_price
        pauseMinting => pause_minting
        startMinting => start_minting
        addBoostStaking => add_boost_staked
        removeBoostStaking => remove_boost_staked
        getRoles => get_roles
        buySft => buy_sft
        mintNft => mint_nft
        upgradeNft => upgrade_nft
        addBonus => add_bonus
        changeBonus => change_bonus
        addCrystal => add_crystal
        changeCrystal => change_crystal
        addSocket => add_socket
        addRefinement => add_refinement
        addStorage => add_storage
        setCollectionData => set_collection_data
        changeReferralCode => change_referral_code
        setReferralCodeOwner => set_referral_code_owner
        setReferralCodeInvitee => set_referral_code_invitee
        getNftName => nft_name
        getNftPrice => nft_price
        getCollectionNonce => collection_nonce
        getNftMax => nft_max
        getNeedSocket => need_socket
        isPaused => paused
        getImageCid => image_cid
        getMetadataCid => metadata_cid
        getPaymentTokenId => payment_token_id
        getNftWave => nft_wave
        getHasBonus => has_bonus
        getHasCrystal => has_crystal
        getNftQuality => nft_quality
        getNftCrystalVariant => nft_crystal_variant
        getNftBonusVariant => nft_bonus_variant
        getBonusCount => bonus_count
        getCrystalCount => crystal_count
        getSocketCount => socket_count
        getTclCount => tcl_count
        getTclMax => tcl_max
        getRefinementTimestamp => refinement_timestamp
        getSftPrice => sft_price
        getSftSold => sft_sold
        getSftMax => sft_max
        getNftUpgradePrice => nft_upgrade_price
        getAddBonusPrice => add_bonus_price
        getChangeBonusPrice => change_bonus_price
        getAddCrystalPrice => add_crystal_price
        getChangeCrystalPrice => change_crystal_price
        getAddSocketPrice => add_socket_price
        getAddRefinementPrice => add_refinement_price
        getUpgradeBonusBhance => upgrade_bonus_chance
        getTclUsdPrice => tcl_usd_price
        getReferralOwner => referral_owner
        getReferralCode => referral_code
        getReferralEarned => referral_earned
        getReferralCodeInvitee => referral_code_invitee
        getReferralInvitees => referral_invitees
        getTransactionsInvitees => transactions_invitees
        getTeamWallet => team_wallet
        getTotalEarnedAllReferrals => total_earned_all_referrals
        getTotalInviteesAllReferrals => total_invitees_all_referrals
        getActiveReferralCodes => active_referral_codes
        getTotalReferralCodes => total_referral_codes
        getTransactionsAllInvitees => total_transactions_all_invitees
        getTotalReserveAmount => total_reserve_amount
        getTotalRewardsReleased => total_rewards_released
        getAprMax => apr_max
        getRewardTokenId => reward_token_id
        getTotalStakedAmount => total_staked_amount
        getServerWallet => server_wallet
        getUserStakedAmount => user_staked_amount
        getUserBoostStakedAmount => user_boost_staked_amount
        getUserLoanedAmount => user_loaned_amount
        getLastClaimedLendingEpoch => last_claimed_lending_epoch
        getUserInfinityStakedAmount => user_infinity_staked_amount
        lastClaimedInfinityEpoch => last_claimed_infinity_epoch
        totalUserInfinityRewards => total_user_infinity_rewards
        getTotalInfinityStakedAmount => total_infinity_staked_amount
        getUserBorrowedAmount => user_borrowed_amount
        getUserBalancePrivateShop => user_balance_private_shop
        getPrivateShopBalanceTimestamp => private_shop_balance_timestamp
        getAutoClaimSubscribers => auto_claim_subscribers
        getSubscriberIndex => subscriber_index
        getReinvestInfinity => reinvest_infinity
        getPriceEgldAutoclaim => price_egld_autoclaim
        getEndSubscriptionEpoch => end_subscription_epoch
        getBatchAutoClaim => batch_auto_claim
        getAutoClaimedCount => auto_claimed_count
        calculateStorage => calculate_storage
        calculateAdditionalStorage => calculate_additional_storage
        calculateReward => calculate_reward
        getApr => get_apr
        getBoost => get_boost
        buildUrisVec => build_uris_vec
        getAttributesBuffer => build_attributes_buffer
        getAttributesRoute => build_attributes_route
        getCurrentWave => get_current_wave
        getNewNftQuality => get_new_nft_quality
        getUpgradeChance => get_upgrade_chance
        isNftInCollection => is_collection_set
        getRandomNumber => get_random_number
        getNftsMinted => get_nfts_minted
        getPrices => get_prices
        getSftData => get_sft_data
        getLendingData => get_lending_data
        getLastClaimedEpoch => get_last_claimed_epoch
        getRewardsData => rewards_data
        getHistoryPurchases => history_purchases
        getReferralData => referral_data
        loanNft => loan_nft
        unloanNft => unloan_nft
        borrowNft => borrow_nft
        setCoinPacks => set_coin_packs
        buyCoins => buy_coins
        getCoinPacks => get_coin_packs
        addBalancePrivateShop => add_balance_private_shop
        refundPrivateShop => refund_private_shop
        withdrawBalancePrivateShop => withdraw_balance_private_shop
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
