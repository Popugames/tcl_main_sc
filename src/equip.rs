multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::referral;
use crate::reward;
use crate::nft;
use crate::types::EquipSlot;
use crate::storage;
use crate::utils;

#[multiversx_sc::module]
pub trait EquipModule: nft::NftModule + referral::ReferralModule + reward::RewardModule + storage::Storage + utils::Utils{


    #[payable("*")]
    #[endpoint(equipNft)]
    fn equip_nft(&self) {

        // Verifică dacă ID-ul tokenului de plată este setat
        require!(!self.payment_token_id().is_empty(), "Payment token ID not set");
    
        // Obține adresa apelantului
        let caller = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();
        let not_borrowed = self.user_borrowed_amount(&caller, &current_epoch).is_empty();

        require!(not_borrowed, "can t equip when borrowed");
        
        // Obține toate transferurile ESDT efectuate în timpul acestui apel
        let transfers = self.call_value().all_esdt_transfers();
        let mut payment_amount = BigUint::zero();
        let mut nft_transfer: Option<EsdtTokenPayment<Self::Api>> = None;
    
        // Parcurge transferurile pentru a identifica tokenul de plată și transferul NFT
        for transfer in transfers.iter() {
            if transfer.token_identifier == self.payment_token_id().get() {
                payment_amount = transfer.amount.clone();
            }
            if self.is_collection_set(&transfer.token_identifier) {
                nft_transfer = Some(transfer.clone());
            }
        }
    
        // Verifică dacă transferul NFT a fost furnizat
        let nft_transfer = nft_transfer.expect("NFT not provided");
    
        // Extrage detaliile transferului NFT
        let nft_nonce = nft_transfer.token_nonce;
        let nft_id = nft_transfer.token_identifier;
        let nft_amount = nft_transfer.amount;
    
        // Verifică dacă doar un NFT a fost trimis
        require!(nft_amount == BigUint::from(1u64), "Only one NFT can be sent at a time");
    
        // Obține ID-ul colecției și calculează stocarea necesară
        let collection_id = nft_id.clone();
        let tcl_max = self.tcl_max(&collection_id, &nft_nonce).get();
        let storage = self.calculate_storage(tcl_max);

        // Obține slotul de echipare pentru colecția specificată
        let nft_slot = self.equip_slot(&collection_id).get();

        // Verifică dacă utilizatorul deja are un NFT echipat în acest slot
        require!(
            !self.equipped_nfts(&caller).contains_key(&nft_slot),
            "Already have an NFT equipped on this slot"
        );

        // Verifică dacă colecția este setată
        require!(self.is_collection_set(&collection_id), "Collection not set");
    
        //Verifica daca suma platita este mai mica sau egala cu capacitatea de stocare
        require!(
            &payment_amount <= &storage,
            "Insufficient storage"
        );
    
        // Dacă suma de plată este mai mare decât zero, actualizează tokenii din stocarea NFT-ului si pe cei din stake
        if &payment_amount > &BigUint::zero() {
            self.tcl_count(&collection_id, &nft_nonce).update(|v| *v += &payment_amount);
            self.stake(&caller, &payment_amount);
        }
    
        // Echiparea NFT-ului în slotul specificat pentru apelant
        self.equipped_nfts(&caller).insert(nft_slot, (collection_id.clone(), nft_nonce.clone()));
    }
    
    #[endpoint(unequipNft)]
    fn unequip_nft(&self, nft_slot: EquipSlot) {
        let caller = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();
    
        // Verificăm dacă există un NFT echipat în slotul specificat pentru apelant
        require!(
            self.equipped_nfts(&caller).contains_key(&nft_slot),
            "No NFT equipped on this slot"
        );
        
        // Obținem detaliile NFT-ului echipat
        match self.equipped_nfts(&caller).get(&nft_slot) {
            Some((collection_id, nonce)) => {
                // Verificăm dacă suma TCL este zero sau dacă ultima revendicare a fost făcută acum două epoci
                require!(
                    self.tcl_count(&collection_id, &nonce).get() == BigUint::zero() || self.last_claimed_epoch(&caller).get() < current_epoch,
                    "Cannot unequip if claimed in the same epoch"
                );
    
                // Obținem suma TCL asociată NFT-ului
                let tcl_count = self.tcl_count(&collection_id, &nonce).get();
                
                // Dacă suma TCL este mai mare decât zero, unstake suma
                if &tcl_count > &BigUint::zero() {
                    self.tcl_count(&collection_id, &nonce).set(BigUint::zero());
                    self.unstake(&caller, &tcl_count);
                    let payment_token_id = self.payment_token_id().get();
                    self.send().direct(
                        &caller,
                        &payment_token_id,
                        0u64,
                        &tcl_count
                    );
                }
        
                // Scoatem NFT-ul echipat din slotul specificat pentru apelant
                self.equipped_nfts(&caller).remove(&nft_slot);
    
                // Returnăm NFT-ul către apelant
                self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::esdt(collection_id), nonce, &BigUint::from(1u64));
            },
            None => {}
        }
    }
    
    #[endpoint(getEquippedNfts)]
    fn get_equipped_nfts(&self, wallet_address: ManagedAddress) -> ManagedBuffer {
        let mut equipped_nfts_str = ManagedBuffer::new_from_bytes(b"");
        let current_epoch = self.blockchain().get_block_epoch();
        let is_borrowed =  !self.user_borrowed_amount(&wallet_address, &current_epoch).is_empty();
        let is_borrowed_buffer =  if is_borrowed
        {
            ManagedBuffer::from("1")
        }else{
            ManagedBuffer::from("0")
        };
    
        //equipped_nfts_str.append(&is_borrowed);
        //equipped_nfts_str.append(&ManagedBuffer::new_from_bytes(b","));

        //EQUIPPED
        for slot_index in 0..9 {
            let equip_slot = EquipSlot::from_i32(slot_index).unwrap();
            match self.equipped_nfts(&wallet_address).get(&equip_slot) {
                Some((collection_id, nonce)) => {
                    let tcl_count = self.tcl_count(&collection_id, &nonce).get();
    
                    //Construim bufferul pentru NFT echipat
                    equipped_nfts_str.append(&collection_id.into_managed_buffer());
                    equipped_nfts_str.append(&ManagedBuffer::new_from_bytes(b" "));
                    equipped_nfts_str.append(&self.decimal_to_ascii((nonce as u32).try_into().unwrap()));
                    equipped_nfts_str.append(&ManagedBuffer::new_from_bytes(b" "));
                    equipped_nfts_str.append(&self.biguint_to_ascii(&tcl_count));
                    equipped_nfts_str.append(&ManagedBuffer::new_from_bytes(b" "));
                    equipped_nfts_str.append(&is_borrowed_buffer);
                    equipped_nfts_str.append(&ManagedBuffer::new_from_bytes(b","));
                },
                None => {}
            }
        }

        //BORROWED
        if is_borrowed
        {
            let (collection_id, nonce) = self.borrowed_nft(&wallet_address, &current_epoch).get();
            let tcl_count = self.tcl_count(&collection_id, &nonce).get();

            equipped_nfts_str.append(&collection_id.into_managed_buffer());
            equipped_nfts_str.append(&ManagedBuffer::new_from_bytes(b" "));
            equipped_nfts_str.append(&self.decimal_to_ascii((nonce as u32).try_into().unwrap()));
            equipped_nfts_str.append(&ManagedBuffer::new_from_bytes(b" "));
            equipped_nfts_str.append(&self.biguint_to_ascii(&tcl_count));
            equipped_nfts_str.append(&ManagedBuffer::new_from_bytes(b" "));
            equipped_nfts_str.append(&is_borrowed_buffer);
            equipped_nfts_str.append(&ManagedBuffer::new_from_bytes(b","));

        }

        equipped_nfts_str
    }

    #[endpoint(getNftsData)]
    fn get_nfts_data(&self, wallet_address: ManagedAddress, identifiers: MultiValueEncoded<MultiValue2<TokenIdentifier, u64>>) -> ManagedBuffer {
        let mut nfts_data_str = ManagedBuffer::new_from_bytes(b"");
        let current_epoch = self.blockchain().get_block_epoch();

        let is_borrowed =  !self.user_borrowed_amount(&wallet_address, &current_epoch).is_empty();
        let is_borrowed_buffer =  if is_borrowed
        {
            ManagedBuffer::from("1")
        }else{
            ManagedBuffer::from("0")
        };
    
        for identifier in identifiers.into_iter() {
            let (collection_id, nonce) = identifier.into_tuple();
            nfts_data_str.append(&is_borrowed_buffer); // is_borrowed
            nfts_data_str.append(&ManagedBuffer::new_from_bytes(b" "));

            nfts_data_str.append(&ManagedBuffer::new_from_bytes(b"0")); // is_lend - MARIUSTODO
            nfts_data_str.append(&ManagedBuffer::new_from_bytes(b" "));

            let tcl_max = self.tcl_max(&collection_id, &nonce).get();
            let storage = self.calculate_storage(tcl_max);
            nfts_data_str.append(&self.biguint_to_ascii(&storage)); // tcl_storage
            nfts_data_str.append(&ManagedBuffer::new_from_bytes(b" "));

            let crystal_count = self.crystal_count(&collection_id, &nonce).get();
            nfts_data_str.append(&self.decimal_to_ascii((crystal_count as u32).try_into().unwrap())); // crystals
            nfts_data_str.append(&ManagedBuffer::new_from_bytes(b" "));

            let socket_count = self.socket_count(&collection_id, &nonce).get();
            nfts_data_str.append(&self.decimal_to_ascii((socket_count as u32).try_into().unwrap())); // sockets
            nfts_data_str.append(&ManagedBuffer::new_from_bytes(b" "));

            let bonus_count = self.bonus_count(&collection_id, &nonce).get();
            nfts_data_str.append(&self.decimal_to_ascii((bonus_count as u32).try_into().unwrap())); // bonuses
            nfts_data_str.append(&ManagedBuffer::new_from_bytes(b" "));

            let refinement_timestamp = self.refinement_timestamp(&collection_id, &nonce).get();
            nfts_data_str.append(&self.biguint_to_ascii(&BigUint::from(refinement_timestamp)));
            nfts_data_str.append(&ManagedBuffer::new_from_bytes(b" ")); 

            let nft_wave = self.nft_wave(&collection_id, &nonce).get();
            let max_socket = if self.max_socket(&collection_id).len() > 0{
                self.max_socket(&collection_id).get(nft_wave as usize)
            }else{
                0u8
            };


            nfts_data_str.append(&self.decimal_to_ascii((max_socket as u32).try_into().unwrap())); // max_socket
            //nfts_data_str.append(&ManagedBuffer::new_from_bytes(b" ")); 

            //----------------------------------------------------------------------end
            nfts_data_str.append(&ManagedBuffer::new_from_bytes(b","));
        }
    
        nfts_data_str
    }

    #[payable("*")]
    #[endpoint(addTcl)]
    fn add_tcl(&self, wallet_address: &ManagedAddress, collection_id: &TokenIdentifier, nonce: &u64)
    {
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();
        let payment_token_id = self.payment_token_id().get();
        let tcl_count = self.tcl_count(&collection_id, &nonce).get();
        let tcl_max = self.tcl_max(&collection_id, &nonce).get();
        let storage = self.calculate_storage(tcl_max);

        require!(
            self.is_collection_set(&collection_id),
            "Collection not set"
        );

        require!(
            payment_token == payment_token_id, 
            "invalid token paid"
        );

        require!(
            &payment_amount + &tcl_count <= storage, 
            "insufficient storage"
        );

        require!(
            self.is_equipped(&wallet_address, &collection_id, &nonce) ||
            self.loaned_nfts(&wallet_address).contains(&(collection_id.clone(), nonce.clone())), 
            "nft not equipped or loaned"
        );

        //UPDATE NFT
        self.tcl_count(&collection_id, &nonce).update(|v| *v += &payment_amount);

        if self.is_equipped(&wallet_address, &collection_id, &nonce){
            //STAKE EQUIPPED
            self.stake(&wallet_address, &payment_amount);
        }
        else{
            //STAKE LOANED
            self.stake_loaned(&wallet_address, &payment_amount);
        }
        
        
        
    }
    
}
