multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage;

#[multiversx_sc::module]
pub trait ReferralModule: storage::Storage{

    #[only_owner]
    #[endpoint(changeReferralCode)]
    fn change_referral_code(&self, referral_code: ManagedBuffer,wallet_address: &ManagedAddress) {
        require!(
            !referral_code.is_empty(),
            "referral code cannot be empty"
        );
        require!(
            !self.referral_code(wallet_address).is_empty(),
            "referral code must be set"
        );
        require!(
            self.referral_owner(&referral_code).is_empty(),
            "the code has been taken"
        );

        //INDIVIDUAL REFERRAL
        self.referral_owner(&referral_code).set(wallet_address);
        self.referral_code(wallet_address).set(&referral_code);
    }

    #[endpoint(setReferralCodeOwner)]
    fn set_referral_code_owner(&self, referral_code: ManagedBuffer) {
        require!(
            !referral_code.is_empty(),
            "referral code cannot be empty"
        );
        let owner = self.blockchain().get_caller();
        require!(
            self.referral_code(&owner).is_empty(),
            "already has a referral code"
        );
        require!(
            self.referral_owner(&referral_code).is_empty(),
            "the code has been taken"
        );

        //INDIVIDUAL REFERRAL
        self.referral_owner(&referral_code).set(&owner);
        self.referral_code(&owner).set(&referral_code);

        //STATISTICS
        self.total_referral_codes().update(|v| *v += 1);
    }

    #[endpoint(setReferralCodeInvitee)]
    fn set_referral_code_invitee(&self, referral_code: ManagedBuffer) {
        require!(
            !&referral_code.is_empty(),
            "referral code cannot be empty"
        );
        let invitee = self.blockchain().get_caller();
        require!(
            !self.referral_owner(&referral_code).is_empty(),
            "invalid referral code"
        );
        require!(
            &self.referral_code(&invitee).get() != &referral_code,
            "you can't add your own code"
        );
        require!(
            self.referral_code_invitee(&invitee).is_empty(),
            "you have already set a code"
        );

        //INDIVIDUAL REFERRAL
        self.referral_code_invitee(&invitee).set(&referral_code);
        self.referral_invitees(&referral_code).update(|v| *v += 1);

        //STATISTICS
        self.total_invitees_all_referrals().update(|v| *v += 1);
        if self.referral_invitees(&referral_code).get() == 25
        {
            self.active_referral_codes().update(|v| *v += 1);
        }

    }

}