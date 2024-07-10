multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::referral;
use crate::reward;
use crate::nft;
use crate::types::EquipSlot;
use crate::storage;
use crate::utils;
use crate::equip;

#[multiversx_sc::module]
pub trait LendingModule:  nft::NftModule + referral::ReferralModule + reward::RewardModule + storage::Storage + utils::Utils + equip::EquipModule{


    // Procesează împrumutul unui NFT
    #[payable("*")]
    #[endpoint(loanNft)]
    fn loan_nft(&self) {
        // Obține adresa apelantului și epoca curentă
        let caller = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();

        // Colectează toate transferurile ESDT în timpul apelului
        let transfers = self.call_value().all_esdt_transfers();
        let mut payment_amount = BigUint::zero();
        let mut nft_transfer: Option<EsdtTokenPayment<Self::Api>> = None;
    
        // Identifică transferul NFT și tokenul de plată
        for transfer in transfers.iter() {
            if transfer.token_identifier == self.payment_token_id().get() {
                payment_amount = transfer.amount.clone();
            }
            if self.is_collection_set(&transfer.token_identifier) {
                nft_transfer = Some(transfer.clone());
            }
        }
    
        // Verifică prezența transferului NFT
        let nft_transfer = nft_transfer.expect("NFT not provided");

        // Extrage și validează detaliile NFT
        let nft_nonce = nft_transfer.token_nonce;
        let nft_id = nft_transfer.token_identifier;
        let nft_amount = nft_transfer.amount;
        require!(nft_amount == BigUint::from(1u64), "Only one NFT can be sent at a time");

        // Calculează și verifică stocarea necesară pentru tranzacție
        let collection_id = nft_id.clone();
        let tcl_max = self.tcl_max(&collection_id, &nft_nonce).get();
        let storage = self.calculate_storage(tcl_max);
        require!(self.is_collection_set(&collection_id), "Collection not set");
        require!(&payment_amount > &BigUint::zero(), "Invalid amount");
        require!(&payment_amount <= &storage, "Insufficient storage");

        // Actualizează contoarele pentru NFT și staking
        self.tcl_count(&collection_id, &nft_nonce).update(|v| *v += &payment_amount);
        self.stake_loaned(&caller, &payment_amount);

        // Adaugă NFT-ul în lista disponibilă și în cea de împrumut
        self.available_borrow_nfts(&current_epoch).insert((collection_id.clone(), nft_nonce.clone()));
        self.loaned_nfts(&caller).insert((collection_id.clone(), nft_nonce.clone()));

        // Setează ultima epocă pentru claim pentru a preveni retrageri premature
        self.last_nft_claimed_epoch(&collection_id, &nft_nonce).set(&current_epoch);
    }

    // Procesează returnarea unui NFT împrumutat
    #[endpoint(unloanNft)]
    fn unloan_nft(&self, collection_id: TokenIdentifier, nonce: u64) {
        let caller = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();
        let last_nft_claimed_epoch = self.last_nft_claimed_epoch(&collection_id, &nonce).get();

        // Verifică dacă NFT-ul este împrumutat și dacă colecția este validă
        require!(self.is_collection_set(&collection_id), "Collection not set");
        require!(self.loaned_nfts(&caller).contains(&(collection_id.clone(), nonce.clone())), "NFT not loaned");

        // Verifică condițiile de returnare
        require!(last_nft_claimed_epoch < current_epoch, "Rewards claimed or nft loaned in current epoch");

        // Anulează staking-ul și returnează NFT-ul
        let tcl_count = self.tcl_count(&collection_id, &nonce).get();
        self.tcl_count(&collection_id, &nonce).set(BigUint::zero());
        self.unstake_loaned(&caller, &tcl_count);
        self.available_borrow_nfts(&current_epoch).swap_remove(&(collection_id.clone(), nonce.clone()));
        self.loaned_nfts(&caller).swap_remove(&(collection_id.clone(), nonce.clone()));

        //Send NFT
        self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::esdt(collection_id), nonce, &BigUint::from(1u64));
        
        //Send tokens
        let payment_token_id = self.payment_token_id().get();
        self.send().direct(
            &caller,
            &payment_token_id,
            0u64,
            &tcl_count
        );
    }

}