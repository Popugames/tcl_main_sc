multiversx_sc::imports!();
multiversx_sc::derive_imports!();




use crate::referral;
use crate::reward;
use crate::types::EquipSlot;
use crate::storage;
use crate::utils;

#[multiversx_sc::module]
pub trait NftModule: referral::ReferralModule + reward::RewardModule + storage::Storage + utils::Utils{

//OWNER FUNCTIONS 
   
    #[only_owner]
    #[endpoint(setGlobalProps)]
    fn set_global_props(
        &self,
        payment_token_id: EgldOrEsdtTokenIdentifier,
        nft_upgrade_price: BigUint,
        add_bonus_price: BigUint,
        change_bonus_price: BigUint,
        add_socket_price: BigUint,
        add_crystal_price: BigUint,
        change_crystal_price: BigUint,
        upgrade_bonus_chance: u32,
        add_refinement_price: BigUint,
        tcl_usd_price: BigUint,
        team_wallet: ManagedAddress,
        server_wallet: ManagedAddress,
        apr_max: u16
    ) {
        self.payment_token_id().set(payment_token_id);
        self.nft_upgrade_price().set(nft_upgrade_price);
        self.add_bonus_price().set(add_bonus_price);
        self.change_bonus_price().set(change_bonus_price);
        self.add_socket_price().set(add_socket_price);
        self.add_crystal_price().set(add_crystal_price);
        self.change_crystal_price().set(change_crystal_price);
        self.upgrade_bonus_chance().set(upgrade_bonus_chance);
        self.add_refinement_price().set(add_refinement_price);
        self.tcl_usd_price().set(tcl_usd_price);
        self.team_wallet().set(team_wallet);
        self.server_wallet().set(server_wallet);
        self.apr_max().set(apr_max);
    }


    #[only_owner]
    #[endpoint(setCollection)]
    fn set_collection(
        &self,
        collection_id: TokenIdentifier,
        nft_name: ManagedBuffer,
        nft_price: BigUint,
        nft_max: u64,
        royalties: BigUint,
        need_socket: bool,
        equip_slot: EquipSlot,
        collection_data: MultiValueEncoded<MultiValue6<ManagedBuffer,ManagedBuffer, u16, u8, u16, u32>> //(image_cid, metadata_cid, nft_count, max_socket, max_crystal_variants, max_bonus_variants)
    )  {
        let collection_nonce = self.collection_nonce(&collection_id);
        if collection_nonce.is_empty() {
            collection_nonce.set(0u64)
        }

        self.collection_ids().push(&collection_id);
        self.nft_name(&collection_id).set(nft_name);
        self.nft_price(&collection_id).set(nft_price);
        self.nft_max(&collection_id).set(nft_max);
        self.royalties(&collection_id).set(royalties);
        self.paused(&collection_id).set(true);
        self.need_socket(&collection_id).set(need_socket);
        self.equip_slot(&collection_id).set(equip_slot);
        self.set_collection_data(&collection_id, collection_data);
    }

    #[only_owner]
    #[endpoint(setSft)]
    fn set_sft(
        &self,
        collection_id: TokenIdentifier,
        nonce: u64,
        sft_price: BigUint,
        sft_sold: u32,
        sft_max: u32
    )
    {
        self.sft_price(&collection_id, &nonce).set(sft_price);
        self.sft_sold(&collection_id, &nonce).set(sft_sold);
        self.sft_max(&collection_id, &nonce).set(sft_max);
    }

    #[only_owner]
    #[endpoint(setNewPrice)]
    fn set_new_price(&self,collection_id:TokenIdentifier,nft_price:BigUint)
    {
        self.nft_price(&collection_id).set(nft_price);
    }

    #[only_owner]
    #[endpoint(pauseMinting)]
    fn pause_minting(&self,collection_id:TokenIdentifier){
        self.paused(&collection_id).set(true);
    }

    #[only_owner]
    #[endpoint(startMinting)]
    fn start_minting(&self,collection_id:TokenIdentifier){
        self.paused(&collection_id).set(false);
    }
//END

//PUBLIC FUNCTIONS

    #[endpoint(getRoles)] //TODO DELETE IN PRODUCTION
    fn get_roles(&self, collection_id: &TokenIdentifier) -> ManagedBuffer {
        let roles = self.blockchain().get_esdt_local_roles(&collection_id);

        // Creează un buffer care să conțină toate rolurile
        let mut buffer = ManagedBuffer::new();
        
        for role in roles.iter_roles() {
        
            buffer.append(&ManagedBuffer::from(role.as_role_name()));
            buffer.append(&ManagedBuffer::from(&b", "[..])); // Adaugă o virgulă și un spațiu pentru separare
        }
        
    
        //Returnează bufferul
        buffer
        //roles.bits()
    }


    #[payable("*")]
    #[endpoint(buySft)]
    fn buy_sft(&self, collection_id: TokenIdentifier, nonce: u64) {

        let caller = self.blockchain().get_caller();
        let sft_price = self.sft_price(&collection_id, &nonce).get();
        let sft_sold = self.sft_sold(&collection_id, &nonce).get();
        let sft_max = self.sft_max(&collection_id, &nonce).get();
        let payment_token_id = self.payment_token_id().get();
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();

        require!(!self.payment_token_id().is_empty(), "Payment token ID not set");
        require!(!self.sft_price(&collection_id, &nonce).is_empty(), "sft not set");
        require!(&sft_sold < &sft_max, "sold out");
        require!(payment_token == payment_token_id,"invalid token paid");
        require!(
            &payment_amount == &sft_price,
            "invalid amount"
        );

        //UPDATE SFTs SOLD
        self.sft_sold(&collection_id, &nonce).update(|v| *v += 1);

        //SEND NFT TO CALLER
        self.send().direct_esdt(
            &caller,
            &collection_id,
            nonce,
            &BigUint::from(1u32),
        );

        //DISTRIBUTE TOKENS
        self.distribute_tokens(&caller,payment_amount);
    }


    #[payable("*")]
    #[endpoint(mintNft)]
    fn mint_nft(&self,collection_id: &TokenIdentifier) 
    {
        require!(!self.payment_token_id().is_empty(), "payment_token_id not set");
        require!(self.is_collection_set(&collection_id), "collection not set");
        let roles = self.blockchain().get_esdt_local_roles(&collection_id);

        //require!(roles.has_role(&EsdtLocalRole::NftUpdateAttributes), "NftUpdateAttributes not set");
        require!(roles.has_role(&EsdtLocalRole::NftCreate), "NFTCreateRole not set");

        let caller = self.blockchain().get_caller();
        let nonce = self.collection_nonce(&collection_id).get() + 1;
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();
        let payment_token_id = self.payment_token_id().get();
        let default_quality = self.get_random_number(0u32, 1u32, true);
        let default_json_index = 0u32;
        let current_wave = self.get_current_wave(&collection_id);
        let metadata_cid = self.metadata_cid(&collection_id).get(current_wave.clone());

        
        

        require!(
            payment_token == payment_token_id,
            "invalid token paid"
        );
        require!(
            self.paused(&collection_id).get()==false,
            "minting not started or paused"
        );
        require!(
            &payment_amount == &self.nft_price(&collection_id).get(),
            "invalid amount"
        );
        require!(
            nonce <= self.nft_max(&collection_id).get(),
            "sold out"
        );

        self.collection_nonce(&collection_id).update(|v| *v += 1);

        let name = self.nft_name(&collection_id).get();
        let royalties = self.royalties(&collection_id).get();
        let attributes = self.build_attributes_buffer(metadata_cid, false, false, default_quality.clone(), 0u32, default_json_index);
        let hash_buffer = self.crypto().sha256(&attributes);
        let attributes_hash = hash_buffer.as_managed_buffer();

        let uris = self.build_uris_vec(&collection_id, current_wave.clone()); //(1-based index)

        let one = BigUint::from(1u32);

       
        //SET DEFAULT NFT PROPERTIES
        self.nft_wave(&collection_id, &nonce).set(current_wave as u32);//(1-based index)
        self.has_bonus(&collection_id, &nonce).set(false);
        self.has_crystal(&collection_id, &nonce).set(false);
        self.nft_bonus_variant(&collection_id, &nonce).set(0u32);
        self.nft_crystal_variant(&collection_id, &nonce).set(0u32);
        self.nft_quality(&collection_id, &nonce).set(&default_quality);
        self.bonus_count(&collection_id, &nonce).set(0u8);
        self.socket_count(&collection_id, &nonce).set(0u8);
        self.tcl_count(&collection_id, &nonce).set(BigUint::zero());

        //UPDATE TCL MAX
        let additional_storage = self.calculate_storage(payment_amount.clone() * BigUint::from(2u8));
        self.tcl_max(&collection_id, &nonce).update(|v| *v += additional_storage);

        //CREATE NFT
        self.send().esdt_nft_create(
            &collection_id,
            &one,
            &name,
            &royalties,
            &attributes_hash,
            &attributes,
            &uris,
        );

        //SEND NFT TO CALLER
        self.send().direct_esdt(
            &caller,
            &collection_id,
            nonce,
            &one,
        );

        //DISTRIBUTE TOKENS
        self.distribute_tokens(&caller,payment_amount);
    }

    #[payable("*")]
    #[endpoint(upgradeNft)]
    fn upgrade_nft(&self, collection_id: TokenIdentifier, nonce: u64) {
        let caller = self.blockchain().get_caller();
        let is_equipped = self.is_equipped(&caller, &collection_id, &nonce);
    
        // Verificări inițiale pentru a asigura că ID-ul tokenului de plată și prețul de upgrade sunt setate
        require!(!self.payment_token_id().is_empty(), "Payment token ID not set");
        require!(!self.nft_upgrade_price().is_empty(), "Upgrade price not set");
    
        // Obținere transferuri ESDT din apelul curent
        let transfers = self.call_value().all_esdt_transfers();
        let mut payment_amount = BigUint::zero();
        let mut nft_transfer: Option<EsdtTokenPayment<Self::Api>> = None;
    
        // Iterăm prin transferuri pentru a găsi tokenul de plată și, dacă nu este echipat, NFT-ul
        for transfer in transfers.iter() {
            if transfer.token_identifier == self.payment_token_id().get() {
                payment_amount = transfer.amount.clone();
            }
    
            if self.is_collection_set(&transfer.token_identifier) {
                nft_transfer = Some(transfer.clone());
            }
        }
    
        // Verificăm dacă am primit suma corectă de plată
        let expected_amount = self.nft_upgrade_price().get();
        require!(payment_amount == expected_amount, "Incorrect payment amount");
    
        // Dacă NFT-ul nu este echipat, verificăm și extragem detaliile transferului NFT
        if !is_equipped {
            // Verificăm dacă NFT-ul a fost furnizat
            require!(nft_transfer.is_some(), "NFT not provided");
    
            // Extragem detaliile necesare din transferul NFT
            let nft_transfer = nft_transfer.unwrap();
            let nft_nonce = nft_transfer.token_nonce;
            let nft_id = nft_transfer.token_identifier;
            let nft_amount = nft_transfer.amount;
    
            // Verificăm că doar un NFT a fost trimis și că acesta corespunde detaliilor așteptate
            require!(nft_amount == BigUint::from(1u64), "Only one NFT can be sent at a time");
            require!(nft_id == collection_id && nft_nonce == nonce, "NFT mismatch");
        }
    
        // Obținem și verificăm calitatea actuală a NFT-ului
        let nft_quality = self.nft_quality(&collection_id, &nonce).get();
        let new_nft_quality = self.get_new_nft_quality(nft_quality);
        let nft_quality_max = 9u32;
    
        // Verificăm că colecția este setată și că NFT-ul nu are deja calitatea maximă
        require!(self.is_collection_set(&collection_id), "Collection not set");
        require!(nft_quality < nft_quality_max, "This NFT already has the maximum quality");
        // TODO: check if has role NftUpdateAttributes
    
        // Actualizăm metadata și proprietățile NFT-ului cu noua calitate
        self.nft_quality(&collection_id, &nonce).set(&new_nft_quality);
        self.update_attributes(&collection_id, nonce.clone(), new_nft_quality);
       
    
        // Actualizăm capacitatea de stocare (TCL MAX) pentru NFT
        let additional_storage = self.calculate_storage(payment_amount.clone() * BigUint::from(2u8));
        self.tcl_max(&collection_id, &nonce).update(|v| *v += additional_storage);
    
        // Distribuim tokenii de plată
        self.distribute_tokens(&caller, payment_amount);
    
        // Dacă NFT-ul nu era echipat, îl returnăm utilizatorului
        if !is_equipped {
            self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::esdt(collection_id.clone()), nonce, &BigUint::from(1u64));
        }
    }
    
    #[payable("*")]
    #[endpoint(addBonus)]
    fn add_bonus(&self, collection_id: TokenIdentifier, nonce: u64) {
        // Verificări inițiale pentru a asigura că ID-ul tokenului de plată și prețul pentru bonus sunt setate
        require!(!self.payment_token_id().is_empty(), "Payment token ID not set");
        require!(!self.add_bonus_price().is_empty(), "add_bonus_price not set");
    
        // Obținere transferuri ESDT din apelul curent
        let transfers = self.call_value().all_esdt_transfers();
        let mut payment_amount = BigUint::zero();
        let mut nft_transfer: Option<EsdtTokenPayment<Self::Api>> = None;
    
        // Iterăm prin transferuri pentru a găsi tokenul de plată și, dacă nu este echipat, NFT-ul
        for transfer in transfers.iter() {
            if transfer.token_identifier == self.payment_token_id().get() {
                payment_amount = transfer.amount.clone();
            }
    
            if self.is_collection_set(&transfer.token_identifier) {
                nft_transfer = Some(transfer.clone());
            }
        }
    
        // Verificăm dacă am primit suma corectă de plată
        let expected_amount = self.add_bonus_price().get();
        require!(payment_amount == expected_amount, "Incorrect payment amount");
    
        let caller = self.blockchain().get_caller();
        let is_equipped = self.is_equipped(&caller, &collection_id, &nonce);
    
        // Dacă NFT-ul nu este echipat, verificăm și extragem detaliile transferului NFT
        if !is_equipped {
            // Verificăm dacă NFT-ul a fost furnizat
            require!(nft_transfer.is_some(), "NFT not provided");
    
            // Extragem detaliile necesare din transferul NFT
            let nft_transfer = nft_transfer.unwrap();
            let nft_nonce = nft_transfer.token_nonce;
            let nft_id = nft_transfer.token_identifier;
            let nft_amount = nft_transfer.amount;
    
            // Verificăm că doar un NFT a fost trimis și că acesta corespunde detaliilor așteptate
            require!(nft_amount == BigUint::from(1u64), "Only one NFT can be sent at a time");
            require!(nft_id == collection_id && nft_nonce == nonce, "NFT mismatch");
        }
    
        // Obținem și verificăm calitatea actuală a NFT-ului și numărul de bonusuri
        let nft_quality = self.nft_quality(&collection_id, &nonce).get();
        let nft_quality_max = 9u32;
        let bonus_count = self.bonus_count(&collection_id, &nonce).get();
    
        // Verificări suplimentare pentru colecție și numărul maxim de bonusuri
        require!(self.is_collection_set(&collection_id), "Collection not set");
        require!(bonus_count < 5u8, "Already has the maximum number of bonuses");
        // TODO: check if has role NftUpdateAttributes
    
        // Generăm un număr aleator pentru a determina dacă se adaugă un bonus
        let random_number = self.get_random_number(1u32, 100u32, true);
        if random_number <= 50u32 {
            self.bonus_count(&collection_id, &nonce).set(bonus_count + 1);
        }
    
        // Actualizăm capacitatea de stocare (TCL MAX) pentru NFT
        let additional_storage = self.calculate_storage(payment_amount.clone() * BigUint::from(2u8));
        self.tcl_max(&collection_id, &nonce).update(|v| *v += additional_storage);
    
        // Dacă numărul maxim de bonusuri a fost atins și numărul aleator este în intervalul corect
        if bonus_count == 4u8 && random_number <= 50u32 {
            let nft_wave = self.nft_wave(&collection_id, &nonce).get();
            let max_bonusses_variants = self.max_bonusses_variants(&collection_id).get(nft_wave as usize);
            let bonus_variant = self.get_random_number(0u32, max_bonusses_variants, false);
            self.has_bonus(&collection_id, &nonce).set(true);
            self.nft_bonus_variant(&collection_id, &nonce).set(bonus_variant);
    
            // Actualizăm metadata NFT-ului
            self.update_attributes(&collection_id, nonce.clone(), nft_quality);
        }
    
        // Dacă NFT-ul nu era echipat, îl returnăm utilizatorului
        if !is_equipped {
            self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::esdt(collection_id.clone()), nonce, &BigUint::from(1u64));
        }
    
        // Distribuim tokenii de plată
        self.distribute_tokens(&caller, payment_amount);
    }
    
    #[payable("*")]
    #[endpoint(changeBonus)]
    fn change_bonus(&self, collection_id: TokenIdentifier, nonce: u64) {
        // Verificări inițiale pentru a asigura că ID-ul tokenului de plată și prețul pentru schimbarea bonusului sunt setate
        require!(!self.payment_token_id().is_empty(), "Payment token ID not set");
        require!(!self.change_bonus_price().is_empty(), "change_bonus_price not set");
    
        // Obținere transferuri ESDT din apelul curent
        let transfers = self.call_value().all_esdt_transfers();
        let mut payment_amount = BigUint::zero();
        let mut nft_transfer: Option<EsdtTokenPayment<Self::Api>> = None;
    
        // Iterăm prin transferuri pentru a găsi tokenul de plată și, dacă nu este echipat, NFT-ul
        for transfer in transfers.iter() {
            if transfer.token_identifier == self.payment_token_id().get() {
                payment_amount = transfer.amount.clone();
            }
    
            if self.is_collection_set(&transfer.token_identifier) {
                nft_transfer = Some(transfer.clone());
            }
        }
    
        // Verificăm dacă am primit suma corectă de plată
        let expected_amount = self.change_bonus_price().get();
        require!(payment_amount == expected_amount, "Incorrect payment amount");
    
        let caller = self.blockchain().get_caller();
        let is_equipped = self.is_equipped(&caller, &collection_id, &nonce);
    
        // Dacă NFT-ul nu este echipat, verificăm și extragem detaliile transferului NFT
        if !is_equipped {
            // Verificăm dacă NFT-ul a fost furnizat
            require!(nft_transfer.is_some(), "NFT not provided");
    
            // Extragem detaliile necesare din transferul NFT
            let nft_transfer = nft_transfer.unwrap();
            let nft_nonce = nft_transfer.token_nonce;
            let nft_id = nft_transfer.token_identifier;
            let nft_amount = nft_transfer.amount;
    
            // Verificăm că doar un NFT a fost trimis și că acesta corespunde detaliilor așteptate
            require!(nft_amount == BigUint::from(1u64), "Only one NFT can be sent at a time");
            require!(&nft_id == &collection_id && &nft_nonce == &nonce, "NFT mismatch");
        }
    
        // Obținem și verificăm calitatea actuală a NFT-ului și numărul de bonusuri
        let nft_quality = self.nft_quality(&collection_id, &nonce).get();
        let bonus_count = self.bonus_count(&collection_id, &nonce).get();
    
        // Verificări suplimentare pentru colecție și numărul minim de bonusuri pentru schimbare
        require!(self.is_collection_set(&collection_id), "Collection not set");
        require!(bonus_count >= 5u8, "Must have 5 bonuses to change");
    
        // Generăm un nou bonus aleator
        let nft_wave = self.nft_wave(&collection_id, &nonce).get();
        let max_bonusses_variants = self.max_bonusses_variants(&collection_id).get(nft_wave as usize);
        let bonus_variant = self.get_random_number(0u32, max_bonusses_variants, false);
        self.has_bonus(&collection_id, &nonce).set(true);
        self.nft_bonus_variant(&collection_id, &nonce).set(bonus_variant);
    
        // Actualizăm metadata NFT-ului
        self.update_attributes(&collection_id, nonce.clone(), nft_quality);
    
        // Actualizăm capacitatea de stocare (TCL MAX) pentru NFT
        let additional_storage = self.calculate_storage(payment_amount.clone() * BigUint::from(2u8));
        self.tcl_max(&collection_id, &nonce).update(|v| *v += additional_storage);
    
        // Dacă NFT-ul nu era echipat, îl returnăm utilizatorului
        if !is_equipped {
            self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::esdt(collection_id.clone()), nonce, &BigUint::from(1u64));
        }
    
        // Distribuim tokenii de plată
        self.distribute_tokens(&caller, payment_amount);
    }
    
    #[payable("*")]
    #[endpoint(addCrystal)]
    fn add_crystal(&self, collection_id: TokenIdentifier, nonce: u64) {
        // Verificări inițiale pentru a asigura că ID-ul tokenului de plată și prețul pentru adăugarea cristalului sunt setate
        require!(!self.payment_token_id().is_empty(), "Payment token ID not set");
        require!(!self.add_crystal_price().is_empty(), "add_crystal_price not set");
    
        // Obținere transferuri ESDT din apelul curent
        let transfers = self.call_value().all_esdt_transfers();
        let mut payment_amount = BigUint::zero();
        let mut nft_transfer: Option<EsdtTokenPayment<Self::Api>> = None;
    
        // Iterăm prin transferuri pentru a găsi tokenul de plată și, dacă nu este echipat, NFT-ul
        for transfer in transfers.iter() {
            if transfer.token_identifier == self.payment_token_id().get() {
                payment_amount = transfer.amount.clone();
            }
    
            if self.is_collection_set(&transfer.token_identifier) {
                nft_transfer = Some(transfer.clone());
            }
        }
    
        // Verificăm dacă am primit suma corectă de plată
        let expected_amount = self.add_crystal_price().get();
        require!(payment_amount == expected_amount, "Incorrect payment amount");
    
        let caller = self.blockchain().get_caller();
        let is_equipped = self.is_equipped(&caller, &collection_id, &nonce);
    
        // Dacă NFT-ul nu este echipat, verificăm și extragem detaliile transferului NFT
        if !is_equipped {
            // Verificăm dacă NFT-ul a fost furnizat
            require!(nft_transfer.is_some(), "NFT not provided");
    
            // Extragem detaliile necesare din transferul NFT
            let nft_transfer = nft_transfer.unwrap();
            let nft_nonce = nft_transfer.token_nonce;
            let nft_id = nft_transfer.token_identifier;
            let nft_amount = nft_transfer.amount;
    
            // Verificăm că doar un NFT a fost trimis și că acesta corespunde detaliilor așteptate
            require!(nft_amount == BigUint::from(1u64), "Only one NFT can be sent at a time");
            require!(nft_id == collection_id && nft_nonce == nonce, "NFT mismatch");
        }
    
        // Obținem și verificăm calitatea actuală a NFT-ului și numărul de cristale
        let nft_quality = self.nft_quality(&collection_id, &nonce).get();
        let crystal_count = self.crystal_count(&collection_id, &nonce).get();
        let nft_wave = self.nft_wave(&collection_id, &nonce).get();
        let crystal_count_max = self.max_socket(&collection_id).get(nft_wave as usize);
    
        // Verificări suplimentare pentru colecție și numărul maxim de cristale
        require!(self.is_collection_set(&collection_id), "Collection not set");
        require!(crystal_count < crystal_count_max, "Max crystal added");
    
        // Generăm un număr aleator pentru adăugarea cristalului
        let add_crystal = self.get_random_number(1u32, 100u32, true);
    
        if add_crystal <= 50 {
            self.crystal_count(&collection_id, &nonce).set(crystal_count + 1);
            if crystal_count == crystal_count_max - 1 {
                let nft_wave = self.nft_wave(&collection_id, &nonce).get();
                let max_crystal_variants = self.max_crystal_variants(&collection_id).get(nft_wave as usize);
                let crystal_variant = self.get_random_number(0u32, max_crystal_variants as u32, false);
                self.has_crystal(&collection_id, &nonce).set(true);
                self.nft_crystal_variant(&collection_id, &nonce).set(crystal_variant);
                // Actualizăm metadata NFT-ului
                self.update_attributes(&collection_id, nonce.clone(), nft_quality);
            }
        }
    
        // Actualizăm capacitatea de stocare (TCL MAX) pentru NFT
        let additional_storage = self.calculate_storage(payment_amount.clone() * BigUint::from(2u8));
        self.tcl_max(&collection_id, &nonce).update(|v| *v += additional_storage);
    
        // Dacă NFT-ul nu era echipat, îl returnăm utilizatorului
        if !is_equipped {
            self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::esdt(collection_id.clone()), nonce, &BigUint::from(1u64));
        }
    
        // Distribuim tokenii de plată
        self.distribute_tokens(&caller, payment_amount);
    }
    
    #[payable("*")]
    #[endpoint(changeCrystal)]
    fn change_crystal(&self, collection_id: TokenIdentifier, nonce: u64) {
        // Verificări inițiale
        require!(!self.payment_token_id().is_empty(), "Payment token ID not set");
        require!(!self.change_crystal_price().is_empty(), "change_crystal_price not set");
    
        // Obținere transferuri ESDT
        let transfers = self.call_value().all_esdt_transfers();
        let mut payment_amount = BigUint::zero();
        let mut nft_transfer: Option<EsdtTokenPayment<Self::Api>> = None;
    
        for transfer in transfers.iter() {
            if transfer.token_identifier == self.payment_token_id().get() {
                payment_amount = transfer.amount.clone();
            }
    
            if self.is_collection_set(&transfer.token_identifier) {
                nft_transfer = Some(transfer.clone());
            }
        }
    
        // Verificăm dacă am primit suma corectă de plată
        let expected_amount = self.change_crystal_price().get();
        require!(payment_amount == expected_amount, "Incorrect payment amount");
    
        let caller = self.blockchain().get_caller();
        let is_equipped = self.is_equipped(&caller, &collection_id, &nonce);
    
        // Dacă NFT-ul nu este echipat, verificăm și extragem detaliile transferului NFT
        if !is_equipped {
            // Verificăm dacă NFT-ul a fost furnizat
            require!(nft_transfer.is_some(), "NFT not provided");
    
            // Extragem detaliile necesare din transferul NFT
            let nft_transfer = nft_transfer.unwrap();
            let nft_nonce = nft_transfer.token_nonce;
            let nft_id = nft_transfer.token_identifier;
            let nft_amount = nft_transfer.amount;
    
            // Verificăm că doar un NFT a fost trimis și că acesta corespunde detaliilor așteptate
            require!(nft_amount == BigUint::from(1u64), "Only one NFT can be sent at a time");
            require!(nft_id == collection_id && nft_nonce == nonce, "NFT mismatch");
        }
    
        // Obținem și verificăm calitatea actuală a NFT-ului și dacă are cristale
        let nft_quality = self.nft_quality(&collection_id, &nonce).get();
        let has_crystal = self.has_crystal(&collection_id, &nonce).get();
    
        // Verificări suplimentare
        require!(self.is_collection_set(&collection_id), "Collection not set");
        require!(has_crystal, "must have all crystal added");
    
        // Generăm un nou cristal aleator
        let nft_wave = self.nft_wave(&collection_id, &nonce).get();
        let max_crystal_variants = self.max_crystal_variants(&collection_id).get(nft_wave as usize);
        let crystal_variant = self.get_random_number(0u32, max_crystal_variants as u32, false);
        self.nft_crystal_variant(&collection_id, &nonce).set(crystal_variant);
    
        // Actualizăm metadata NFT-ului
        self.update_attributes(&collection_id, nonce.clone(), nft_quality);
    
        // Actualizăm capacitatea de stocare (TCL MAX) pentru NFT
        let additional_storage = self.calculate_storage(payment_amount.clone() * BigUint::from(2u8));
        self.tcl_max(&collection_id, &nonce).update(|v| *v += additional_storage);
    
        // Dacă NFT-ul nu era echipat, îl returnăm utilizatorului
        if !is_equipped {
            self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::esdt(collection_id.clone()), nonce, &BigUint::from(1u64));
        }
    
        // Distribuim tokenii de plată
        self.distribute_tokens(&caller, payment_amount);
    }
    
    #[payable("*")]
    #[endpoint(addSocket)]
    fn add_socket(&self, collection_id: TokenIdentifier, nonce: u64) {
        // Verificări inițiale
        require!(!self.payment_token_id().is_empty(), "Payment token ID not set");
        require!(!self.add_socket_price().is_empty(), "add_socket_price not set");
    
        // Obținere transferuri ESDT
        let transfers = self.call_value().all_esdt_transfers();
        let mut payment_amount = BigUint::zero();
        let mut nft_transfer: Option<EsdtTokenPayment<Self::Api>> = None;
    
        for transfer in transfers.iter() {
            if transfer.token_identifier == self.payment_token_id().get() {
                payment_amount = transfer.amount.clone();
            }
    
            if self.is_collection_set(&transfer.token_identifier) {
                nft_transfer = Some(transfer.clone());
            }
        }
    
        // Verificăm dacă am primit suma corectă de plată
        let expected_amount = self.add_socket_price().get();
        require!(payment_amount == expected_amount, "Incorrect payment amount");
    
        let caller = self.blockchain().get_caller();
        let is_equipped = self.is_equipped(&caller, &collection_id, &nonce);
    
        // Dacă NFT-ul nu este echipat, verificăm și extragem detaliile transferului NFT
        if !is_equipped {
            // Verificăm dacă NFT-ul a fost furnizat
            require!(nft_transfer.is_some(), "NFT not provided");
    
            // Extragem detaliile necesare din transferul NFT
            let nft_transfer = nft_transfer.unwrap();
            let nft_nonce = nft_transfer.token_nonce;
            let nft_id = nft_transfer.token_identifier;
            let nft_amount = nft_transfer.amount;
    
            // Verificăm că doar un NFT a fost trimis și că acesta corespunde detaliilor așteptate
            require!(nft_amount == BigUint::from(1u64), "Only one NFT can be sent at a time");
            require!(nft_id == collection_id && nft_nonce == nonce, "NFT mismatch");
        }
    
        // Obținem și verificăm calitatea actuală a NFT-ului și dacă are nevoie de socket
        let nft_quality = self.nft_quality(&collection_id, &nonce).get();
        let socket_count = self.socket_count(&collection_id, &nonce).get();
        let need_socket = self.need_socket(&collection_id).get();
    
        // Verificări suplimentare
        require!(self.is_collection_set(&collection_id), "Collection not set");
        require!(need_socket, "this nft does not require socket");
        require!(socket_count < 3u8, "already has the maximum number of sockets");
    
        // Actualizăm proprietățile NFT-ului
        self.socket_count(&collection_id, &nonce).update(|v| *v += 1u8);
    
        // Actualizăm capacitatea de stocare (TCL MAX) pentru NFT
        let additional_storage = self.calculate_storage(payment_amount.clone() * BigUint::from(2u8));
        self.tcl_max(&collection_id, &nonce).update(|v| *v += additional_storage);
    
        // Dacă NFT-ul nu era echipat, îl returnăm utilizatorului
        if !is_equipped {
            self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::esdt(collection_id.clone()), nonce, &BigUint::from(1u64));
        }
    
        // Distribuim tokenii de plată
        self.distribute_tokens(&caller, payment_amount);
    }
    
    #[payable("*")]
    #[endpoint(addRefinement)]
    fn add_refinement(&self, collection_id: TokenIdentifier, nonce: u64) {
        // Verificări inițiale
        require!(!self.payment_token_id().is_empty(), "Payment token ID not set");
        require!(!self.add_refinement_price().is_empty(), "add_refinement_price not set");
    
        // Obținere transferuri ESDT
        let transfers = self.call_value().all_esdt_transfers();
        let mut payment_amount = BigUint::zero();
        let mut nft_transfer: Option<EsdtTokenPayment<Self::Api>> = None;
    
        for transfer in transfers.iter() {
            if transfer.token_identifier == self.payment_token_id().get() {
                payment_amount = transfer.amount.clone();
            }
    
            if self.is_collection_set(&transfer.token_identifier) {
                nft_transfer = Some(transfer.clone());
            }
        }
    
        // Verificăm dacă am primit suma corectă de plată
        let expected_amount = self.add_refinement_price().get();
        require!(payment_amount == expected_amount, "Incorrect payment amount");
    
        let caller = self.blockchain().get_caller();
        let is_equipped = self.is_equipped(&caller, &collection_id, &nonce);
    
        // Dacă NFT-ul nu este echipat, verificăm și extragem detaliile transferului NFT
        if !is_equipped {
            // Verificăm dacă NFT-ul a fost furnizat
            require!(nft_transfer.is_some(), "NFT not provided");
    
            // Extragem detaliile necesare din transferul NFT
            let nft_transfer = nft_transfer.unwrap();
            let nft_nonce = nft_transfer.token_nonce;
            let nft_id = nft_transfer.token_identifier;
            let nft_amount = nft_transfer.amount;
    
            // Verificăm că doar un NFT a fost trimis și că acesta corespunde detaliilor așteptate
            require!(nft_amount == BigUint::from(1u64), "Only one NFT can be sent at a time");
            require!(nft_id == collection_id && nft_nonce == nonce, "NFT mismatch");
        }
    
        // Obținem și verificăm calitatea actuală a NFT-ului și cristalizarea
        let nft_quality = self.nft_quality(&collection_id, &nonce).get();
        let crystal_count = self.crystal_count(&collection_id, &nonce).get();
        let nft_wave = self.nft_wave(&collection_id, &nonce).get();
        let crystal_count_max = self.max_socket(&collection_id).get(nft_wave as usize);
    
        // Verificări suplimentare
        require!(self.is_collection_set(&collection_id), "Collection not set");
        require!(crystal_count < crystal_count_max, "max refinement added");
    
        // Actualizăm numărul de cristale pentru NFT
        self.crystal_count(&collection_id, &nonce).set(crystal_count + 1);
    
        // Dacă numărul maxim de cristale este atins, actualizăm varianta de cristal
        if crystal_count == crystal_count_max - 1 {
            self.has_crystal(&collection_id, &nonce).set(true);
            self.nft_crystal_variant(&collection_id, &nonce).set(0u32);
            let current_timestamp = self.blockchain().get_block_timestamp();
            self.refinement_timestamp(&collection_id, &nonce).set(current_timestamp);
    
            // Actualizăm metadata NFT-ului
            self.update_attributes(&collection_id, nonce, nft_quality);
        }
    
        // Actualizăm capacitatea de stocare (TCL MAX) pentru NFT
        let additional_storage = self.calculate_storage(payment_amount.clone() * BigUint::from(2u8));
        self.tcl_max(&collection_id, &nonce).update(|v| *v += additional_storage);
    
        // Dacă NFT-ul nu era echipat, îl returnăm utilizatorului
        if !is_equipped {
            self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::esdt(collection_id.clone()), nonce, &BigUint::from(1u64));
        }
    
        // Distribuim tokenii de plată
        self.distribute_tokens(&caller, payment_amount);
    }
    
    #[payable("*")]
    #[endpoint(addStorage)]
    fn add_storage(&self, collection_id: TokenIdentifier, nonce: u64) {
        // Verificări inițiale
        require!(!self.payment_token_id().is_empty(), "Payment token ID not set");
    
        // Obținere transferuri ESDT
        let transfers = self.call_value().all_esdt_transfers();
        let mut payment_amount = BigUint::zero();
        let mut nft_transfer: Option<EsdtTokenPayment<Self::Api>> = None;
    
        for transfer in transfers.iter() {
            if transfer.token_identifier == self.payment_token_id().get() {
                payment_amount = transfer.amount.clone();
            }
    
            if self.is_collection_set(&transfer.token_identifier) {
                nft_transfer = Some(transfer.clone());
            }
        }
    
        let caller = self.blockchain().get_caller();
        let is_equipped = self.is_equipped(&caller, &collection_id, &nonce);
    
        // Dacă NFT-ul nu este echipat, verificăm și extragem detaliile transferului NFT
        if !is_equipped {
            // Verificăm dacă NFT-ul a fost furnizat
            require!(nft_transfer.is_some(), "NFT not provided");
    
            // Extragem detaliile necesare din transferul NFT
            let nft_transfer = nft_transfer.unwrap();
            let nft_nonce = nft_transfer.token_nonce;
            let nft_id = nft_transfer.token_identifier;
            let nft_amount = nft_transfer.amount;
    
            // Verificăm că doar un NFT a fost trimis și că acesta corespunde detaliilor așteptate
            require!(nft_amount == BigUint::from(1u64), "Only one NFT can be sent at a time");
            require!(nft_id == collection_id && nft_nonce == nonce, "NFT mismatch");
        }
    
        // Obținem și verificăm calitatea actuală a NFT-ului și cristalizarea
        let nft_quality = self.nft_quality(&collection_id, &nonce).get();
        let crystal_count = self.crystal_count(&collection_id, &nonce).get();
        let nft_wave = self.nft_wave(&collection_id, &nonce).get();
        let crystal_count_max = self.max_socket(&collection_id).get(nft_wave as usize);
    
        // Verificări suplimentare
        require!(self.is_collection_set(&collection_id), "Collection not set");
    
        // Calculăm și actualizăm capacitatea de stocare (TCL MAX) pentru NFT
        let additional_storage = self.calculate_storage(payment_amount.clone() * BigUint::from(2u8));
        self.tcl_max(&collection_id, &nonce).update(|v| *v += additional_storage);
    
        // Dacă NFT-ul nu era echipat, îl returnăm utilizatorului
        if !is_equipped {
            self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::esdt(collection_id.clone()), nonce, &BigUint::from(1u64));
        }
    
        // Distribuim tokenii de plată
        self.distribute_tokens(&caller, payment_amount);
    }
    

//END

//SETERS
    #[endpoint(setCollectionData)] //(image_CID, metadata_CID, nft_count, max_socket, max_crystal_variants, max_bonusses_variants)
    fn set_collection_data(&self, collection_id: &TokenIdentifier, collection_data: MultiValueEncoded<MultiValue6<ManagedBuffer, ManagedBuffer, u16, u8, u16, u32>>) {
        self.image_cid(&collection_id).clear();
        self.metadata_cid(&collection_id).clear();
        self.nft_count(&collection_id).clear();
        self.max_socket(&collection_id).clear();
        self.max_crystal_variants(&collection_id).clear();
        self.max_bonusses_variants(&collection_id).clear();

        for value in collection_data.into_iter() {
            let (image_cid, metadata_cid, nft_count, max_socket, max_crystal_variants, max_bonusses_variants) = value.into_tuple();
            self.image_cid(&collection_id).push(&image_cid);
            self.metadata_cid(&collection_id).push(&metadata_cid);
            self.nft_count(&collection_id).push(&nft_count);
            self.max_socket(&collection_id).push(&max_socket);
            self.max_crystal_variants(&collection_id).push(&max_crystal_variants);
            self.max_bonusses_variants(&collection_id).push(&max_bonusses_variants);
        }
    }
//END



}