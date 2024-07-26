multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const IPFS_GATEWAY_HOST: &[u8] = "https://ipfs.io/ipfs/".as_bytes();
const METADATA_KEY_NAME: &[u8] = "metadata:".as_bytes();
const METADATA_FILE_EXTENSION: &[u8] = ".json".as_bytes();
const IMAGE_FILE: &[u8] = "0.jpg".as_bytes();
const ATTR_SEPARATOR: &[u8] = ";".as_bytes();
const URI_SLASH: &[u8] = "/".as_bytes();
const TAGS_KEY_NAME: &[u8] = "tags:tcl,mmorpg,thecursedland,game,play&earn".as_bytes();
const NORMALIZATION_COEFFICIENT: u8 = 200; //această valoare este un coeficient de normalizare pentru tcl_storage.
const DEFAULT_REFERRAL: &[u8] = "LANDER23".as_bytes();

use crate::storage;
use crate::types::EquipSlot;

#[multiversx_sc::module]
pub trait Utils: storage::Storage {

    #[view(calculateStorage)]
    fn calculate_storage(&self, amount: BigUint) -> BigUint {

        let tcl_usd_price = self.tcl_usd_price().get();

        // Verifică dacă oricare dintre valori este zero
        if &amount == &BigUint::zero() || &tcl_usd_price == &BigUint::zero() {
            return BigUint::zero();
        }

        // Efectuează calculul doar dacă ambele valori sunt diferite de zero
        let normalization_coefficient = BigUint::from(NORMALIZATION_COEFFICIENT);
        let factor = BigUint::from(10u128.pow(18));
        let tcl_storage = (amount * factor) / (tcl_usd_price * normalization_coefficient);

        tcl_storage
    }

    #[view(calculateAdditionalStorage)]
    fn calculate_additional_storage(&self, amount: BigUint) -> BigUint {

        let tcl_usd_price = self.tcl_usd_price().get();

        // Verifică dacă oricare dintre valori este zero
        if &amount == &BigUint::zero() || &tcl_usd_price == &BigUint::zero() {
            return BigUint::zero();
        }

        let additional_storage = amount * BigUint::from(2u8) * tcl_usd_price / BigUint::from(5000000000000000u64);

        additional_storage
    }

    #[endpoint(calculateReward)]
    fn calculate_reward(&self,wallet_address: &ManagedAddress, user_staked_amount: BigUint) -> BigUint {

        let total_reserve = self.total_reserve_amount().get();
        let total_staked_amount = self.total_staked_amount().get();
        let apr_max = self.apr_max().get();

        if &user_staked_amount == &BigUint::zero()
        {
            return BigUint::zero();
        }

        let global_daily_reward = (total_reserve/BigUint::from(100u64))/BigUint::from(30u64); 
        let normal_reward =  (global_daily_reward*(&user_staked_amount *10000000000u64/ total_staked_amount))/10000000000u64;
        let max_reward = (user_staked_amount * BigUint::from(apr_max)/BigUint::from(100u64))/BigUint::from(365u64);


        let user_reward = if normal_reward > max_reward
        {
            max_reward
        }
        else
        {
            normal_reward
        };

        //Calculate boost rewards
        let total_rewards = if self.equipped_nfts(&wallet_address).contains_key(&EquipSlot::Boost)
        {
            let (collection_id, nonce) = self.equipped_nfts(&wallet_address).get(&EquipSlot::Boost).unwrap();
            if nonce == 1
            {
                user_reward.clone() + user_reward.clone() * BigUint::from(10u64) / BigUint::from(100u64)
            }else if nonce == 2
            {
                user_reward.clone() + user_reward.clone() * BigUint::from(7u64) / BigUint::from(100u64)
            }else
            {
                user_reward.clone() + user_reward.clone() * BigUint::from(5u64) / BigUint::from(100u64)
            }
        }
        else
        {
            user_reward
        };

        total_rewards
    }

    #[view(buildUrisVec)]
    fn build_uris_vec(&self, collection_id: &TokenIdentifier, current_wave: usize) -> ManagedVec<ManagedBuffer>{

        let mut uris = ManagedVec::new();

        let image_cid = self.image_cid(&collection_id).get(current_wave);
        let uri_slash = ManagedBuffer::new_from_bytes(URI_SLASH);
        let image_file = ManagedBuffer::new_from_bytes(IMAGE_FILE);

        let mut uri_buffer = ManagedBuffer::new_from_bytes(IPFS_GATEWAY_HOST);
        uri_buffer.append(&image_cid);
        uri_buffer.append(&uri_slash);
        uri_buffer.append(&image_file);

        uris.push(uri_buffer);

        uris
    }

    // NO_BONUS + NO_CRYSTAL = "metadata:metadata_cid/without_bonus/without_crystal/quality/crystal_variant/0.json"
    // NO_BONUS + CRYSTAL = "metadata:metadata_cid/without_bonus/with_crystal/quality/crystal_variant/0.json"
    // BONUS + NO_CRYSTAL = "metadata:metadata_cid/with_bonus/without_crystal/quality/crystal_variant/bonus_variant.json"
    // BONUS + CRYSTAL =  "metadata:metadata_cid/with_bonus/with_crystal/quality/crystal_variant/bonus_variant.json"
    #[view(getAttributesBuffer)]
    fn build_attributes_buffer(
        &self,
        metadata_cid: ManagedBuffer,
        with_bonus: bool,
        with_crystal: bool,
        quality: u32,
        crystal_variant: u32,
        bonus_variant: u32,
    ) -> ManagedBuffer {

        let metadata_slash = ManagedBuffer::new_from_bytes(URI_SLASH);
        let with_bonus_buffer = if with_bonus {
            ManagedBuffer::new_from_bytes(b"with_bonus")
        } else {
            ManagedBuffer::new_from_bytes(b"without_bonus")
        };
        let with_crystal_buffer = if with_crystal {
            ManagedBuffer::new_from_bytes(b"with_crystal")
        } else {
            ManagedBuffer::new_from_bytes(b"without_crystal")
        };
        let quality_buffer = self.decimal_to_ascii(quality.try_into().unwrap());
        let crystal_variant_buffer = self.decimal_to_ascii(crystal_variant.try_into().unwrap());
        let bonus_variant_buffer = self.decimal_to_ascii(bonus_variant.try_into().unwrap());
        let metadata_file_extension = ManagedBuffer::new_from_bytes(METADATA_FILE_EXTENSION);
        let separator = ManagedBuffer::new_from_bytes(ATTR_SEPARATOR);
        let tags_key_name = ManagedBuffer::new_from_bytes(TAGS_KEY_NAME);
    
        let mut attributes = ManagedBuffer::new_from_bytes(METADATA_KEY_NAME);
        attributes.append(&metadata_cid);
        attributes.append(&metadata_slash);
        attributes.append(&with_bonus_buffer);
        attributes.append(&metadata_slash);
        attributes.append(&with_crystal_buffer);
        attributes.append(&metadata_slash);
        attributes.append(&quality_buffer);
        attributes.append(&metadata_slash);
        attributes.append(&crystal_variant_buffer);
        attributes.append(&metadata_slash);
        attributes.append(&bonus_variant_buffer);
        attributes.append(&metadata_file_extension);
        attributes.append(&separator);
        attributes.append(&tags_key_name);
    
        attributes
        //ManagedBuffer::new_from_bytes(b"metadata:QmUUKNNMF1hDJ1cx67RcQvbLLHWQyagZbNYZqZ4EYDTafk/with_bonus/with_crystal/0/0/30166.json;tags:song,beautiful,music")
    }

    fn decimal_to_ascii(&self, mut number: u32) -> ManagedBuffer {
        const MAX_NUMBER_CHARACTERS: usize = 10;
        const ZERO_ASCII: u8 = b'0';
  
        let mut as_ascii = [0u8; MAX_NUMBER_CHARACTERS];
        let mut nr_chars = 0;
  
        loop {
            unsafe {
                let reminder: u8 = (number % 10).try_into().unwrap_unchecked();
                number /= 10;
  
                as_ascii[nr_chars] = ZERO_ASCII + reminder;
                nr_chars += 1;
            }
  
            if number == 0 {
                break;
            }
        }
  
        let slice = &mut as_ascii[..nr_chars];
        slice.reverse();
  
        ManagedBuffer::new_from_bytes(slice)
    }

    fn biguint_to_ascii(&self, number: &BigUint) -> ManagedBuffer {
        let zero_ascii: u8 = b'0';

        if number == &BigUint::zero() {
            return ManagedBuffer::new_from_bytes(&[zero_ascii]);
        }

        let mut as_ascii = [0u8; 256];
        let mut current_number = number.clone();
        let mut index = 0;

        while current_number != BigUint::zero() {
            let remainder = current_number.clone().rem(&BigUint::from(10u32));
            let digit: u8 = remainder.to_u64().unwrap() as u8;
            current_number = current_number.div(&BigUint::from(10u32));
            as_ascii[index] = zero_ascii + digit;
            index += 1;
        }

        let mut final_buffer = ManagedBuffer::new();
        for i in (0..index).rev() {
            final_buffer.append_bytes(&[as_ascii[i]]);
        }

        final_buffer
    }


    fn update_attributes(&self, collection_id: &TokenIdentifier, nonce: u64, nft_quality: u32) {
        let nft_wave = self.nft_wave(&collection_id, &nonce).get();
        let metadata_cid = self.metadata_cid(&collection_id).get(nft_wave as usize);
        let has_bonus = self.has_bonus(&collection_id, &nonce).get();
        let has_crystal = self.has_crystal(&collection_id, &nonce).get();
        let crystal_variant = self.nft_crystal_variant(&collection_id, &nonce).get();
        let bonus_variant = self.nft_bonus_variant(&collection_id, &nonce).get();

        let new_attributes = self.build_attributes_buffer(metadata_cid, has_bonus, has_crystal, nft_quality, crystal_variant, bonus_variant);

        self.send().nft_update_attributes(
            collection_id,
            nonce,
            &new_attributes
        );
    }

    fn is_available(&self,wallet_address: &ManagedAddress, collection_id: &TokenIdentifier, nonce: &u64) -> bool{

        let current_epoch = self.blockchain().get_block_epoch();
        let is_available_borrowing = self.available_borrow_nfts(&current_epoch).contains(&(collection_id.clone(), nonce.clone()));
        let claimed_this_epoch = &self.last_nft_claimed_epoch(&collection_id, &nonce).get() == &current_epoch;

        if is_available_borrowing || !claimed_this_epoch{
            return true;
        }

        return false;
    }

    #[view(getCurrentWave)]
    fn get_current_wave(&self, collection_id: &TokenIdentifier) -> usize {
        // Retrieve the total number of NFTs already minted
        let nft_minted: u64 = self.collection_nonce(collection_id).get();

        let mut total_nfts = 0u64;
        for (index, count) in self.nft_count(collection_id).iter().enumerate() {
            total_nfts += count as u64;
            if nft_minted < total_nfts {
                return index + 1; // Current wave (1-based index)
            }
        }

        // If all waves are completed
        self.nft_count(collection_id).len()
    }

    //Use random function and upgrade chance to define if nft is upgraded or downgraded
    #[view(getNewNftQuality)]
    fn get_new_nft_quality(&self, nft_quality: u32) -> u32 {

        let upgrade_chance = self.get_upgrade_chance(&nft_quality);
        let random_number = self.get_random_number(1u32, 100u32, true);

        if random_number <= upgrade_chance
        {
            return nft_quality+1;
        }
        if nft_quality == 0
        {
            return nft_quality;
        }
        return nft_quality-1;
    }

    #[view(getUpgradeChance)]
    fn get_upgrade_chance(&self, item_quality: &u32) -> u32 {

        let bonus_chance = self.upgrade_bonus_chance().get();
        let upgrade_chance = 100 - (item_quality * 10) + bonus_chance;

        if upgrade_chance > 100
        {
            return 100;
        }

        return upgrade_chance;
    }

    // Method to check if a collection has set by owner
    #[view(isNftInCollection)]
    fn is_collection_set(&self, nft_id: &TokenIdentifier) -> bool {
        for id in self.collection_ids().iter() {
            if &id == nft_id {
                return true;
            }
        }
        false
    }

    fn distribute_tokens(&self, caller: &ManagedAddress, amount: BigUint) {

        let payment_token_id = self.payment_token_id().get();
        let referral_code = if self.referral_code(&caller).is_empty() {
            ManagedBuffer::new_from_bytes(DEFAULT_REFERRAL)
        } else {
            self.referral_code(&caller).get()
        };

        let cashback_wallet = if self.referral_code(&caller).is_empty() {
            self.team_wallet().get()
        } else {
            caller.clone()
        };

        require!(
            !self.referral_owner(&referral_code).is_empty(),
            "referral not set"
        );

        let referral_owner = if self.referral_invitees(&referral_code).get() >= 5u32 {
            self.referral_owner(&referral_code).get()
        } else {
            self.referral_owner(&ManagedBuffer::new_from_bytes(DEFAULT_REFERRAL)).get()
        };
        
        let reward_pool = amount.clone() * BigUint::from(70u8) / BigUint::from(100u8); // 70%
        let burn_amount = amount.clone() * BigUint::from(20u8) / BigUint::from(100u8); // 20%
        let referral_amount = amount.clone() * BigUint::from(5u8) / BigUint::from(100u8); // 5%
        let cashback_amount = amount * BigUint::from(5u8) / BigUint::from(100u8); // 5%
    
        // REWARD POOL
        self.total_reserve_amount().update(|v| *v += reward_pool);
    
        // BURN
        self.send().esdt_local_burn(
            &payment_token_id.clone().unwrap_esdt(),
            0u64,
            &burn_amount
        );
    
        // REFERRAL
        self.send().direct(
            &referral_owner,
            &payment_token_id,
            0u64,
            &referral_amount
        );
    
        // CASHBACK
        self.send().direct(
            &cashback_wallet,
            &payment_token_id,
            0u64,
            &cashback_amount
        );
    }

    //generates a random number between $min and $max included
    #[endpoint(getRandomNumber)]
    fn get_random_number(&self, min: u32, max: u32, max_included: bool) -> u32 {
        // Verifică dacă min este mai mic sau egal cu max
        require!(min <= max, "Min must be less than or equal to Max");
    
        // Obține seed-ul privat
        let mut private_seed = self.private_seed().get();
    
        // Obține entropie suplimentară din RandomnessSource
        let mut randomness_source = RandomnessSource::new();
        let additional_entropy: u64 = randomness_source.next_u64();
    
        // Combină seed-ul privat cu entropia suplimentară
        let combined_seed = private_seed ^ additional_entropy;
    
        // Creează un număr aleatoriu în intervalul [min, max] sau [min, max-1]
        let range = if max_included {
            max - min + 1 // Include max
        } else {
            max - min // Exclude max
        };
        
        let random_value = min + (combined_seed % range as u64) as u32;
    
        // Actualizează seed-ul privat pentru a preveni predictibilitatea
        private_seed = private_seed.wrapping_add(random_value.clone() as u64);
        self.private_seed().set(private_seed);
    
        random_value
    }  
    
    fn is_equipped(&self, wallet_address: &ManagedAddress, collection_id: &TokenIdentifier, nonce: &u64) -> bool{

        let nft_slot = self.equip_slot(&collection_id).get();
        let mut is_equipped = false;

        if !self.equipped_nfts(&wallet_address).contains_key(&nft_slot) {
            return false;
        }

        // Obținem detaliile NFT-ului echipat
        match self.equipped_nfts(&wallet_address).get(&nft_slot) {
            Some((equipped_id, equipped_nonce)) => {

                if &equipped_id == collection_id && &equipped_nonce == nonce{
                    is_equipped=true;
                }
            },
            None => {}
        }

        return is_equipped;
    }

    #[endpoint(getNftsMinted)]
    fn get_nfts_minted(&self, identifiers: MultiValueEncoded<TokenIdentifier>) -> ManagedBuffer {
        let mut nfts_minted_str = ManagedBuffer::new_from_bytes(b"");
    
        for identifier in identifiers.into_iter() {

            let nft_count = self.collection_nonce(&identifier).get();
            let nft_max = self.nft_max(&identifier).get();
            nfts_minted_str.append(&self.decimal_to_ascii((nft_count as u32).try_into().unwrap()));
            nfts_minted_str.append(&ManagedBuffer::new_from_bytes(b" "));
            nfts_minted_str.append(&self.decimal_to_ascii((nft_max as u32).try_into().unwrap()));
            nfts_minted_str.append(&ManagedBuffer::new_from_bytes(b","));
        }
    
        nfts_minted_str
    }

    #[endpoint(getPrices)]
    fn get_prices(&self, collection_id: &TokenIdentifier) -> ManagedBuffer {
        let mut prices_str = ManagedBuffer::new_from_bytes(b"");
    
        let mint_price = self.nft_price(&collection_id).get();
        let nft_upgrade_price = self.nft_upgrade_price().get();
        let add_bonus_price = self.add_bonus_price().get();
        let change_bonus_price = self.change_bonus_price().get();
        let add_socket_price = self.add_socket_price().get();
        let add_crystal_price = self.add_crystal_price().get();
        let change_crystal_price = self.change_crystal_price().get();
        let add_refinement_price = self.add_refinement_price().get();

        prices_str.append(&self.biguint_to_ascii(&mint_price));
        prices_str.append(&ManagedBuffer::new_from_bytes(b","));
        prices_str.append(&self.biguint_to_ascii(&nft_upgrade_price));
        prices_str.append(&ManagedBuffer::new_from_bytes(b","));
        prices_str.append(&self.biguint_to_ascii(&add_bonus_price));
        prices_str.append(&ManagedBuffer::new_from_bytes(b","));
        prices_str.append(&self.biguint_to_ascii(&change_bonus_price));
        prices_str.append(&ManagedBuffer::new_from_bytes(b","));
        prices_str.append(&self.biguint_to_ascii(&add_socket_price));
        prices_str.append(&ManagedBuffer::new_from_bytes(b","));
        prices_str.append(&self.biguint_to_ascii(&add_crystal_price));
        prices_str.append(&ManagedBuffer::new_from_bytes(b","));
        prices_str.append(&self.biguint_to_ascii(&change_crystal_price));
        prices_str.append(&ManagedBuffer::new_from_bytes(b","));
        prices_str.append(&self.biguint_to_ascii(&add_refinement_price));
    
        prices_str
    }

    #[endpoint(getSftData)]
    fn get_sft_data(&self, identifiers: MultiValueEncoded<MultiValue2<TokenIdentifier, u64>>) -> ManagedBuffer {
        let mut sfts_data_str = ManagedBuffer::new_from_bytes(b"");
    
        for identifier in identifiers.into_iter() {
            let (collection_id, nonce) = identifier.into_tuple();

            let sft_sold = self.sft_sold(&collection_id, &nonce).get();
            let sft_max = self.sft_max(&collection_id, &nonce).get();

            sfts_data_str.append(&self.decimal_to_ascii((sft_sold).try_into().unwrap()));
            sfts_data_str.append(&ManagedBuffer::new_from_bytes(b" "));
            sfts_data_str.append(&self.decimal_to_ascii((sft_max).try_into().unwrap()));
            //----------------------------------------------------------------------end
            sfts_data_str.append(&ManagedBuffer::new_from_bytes(b","));
        }
    
        sfts_data_str
    }

    #[endpoint(getLendingData)]
    fn get_lending_data(&self, wallet_address: ManagedAddress) -> ManagedBuffer {
        let mut lending_data_str = ManagedBuffer::new_from_bytes(b"");

        let current_epoch = self.blockchain().get_block_epoch();
        let available_borrow_nfts =  self.available_borrow_nfts(&current_epoch).len();
    
        //NFT AVAILABLE 
        lending_data_str.append(&self.decimal_to_ascii((available_borrow_nfts as u32).try_into().unwrap()));
        lending_data_str.append(&ManagedBuffer::new_from_bytes(b","));

        //LONED NFTs
        if self.loaned_nfts(&wallet_address).len() > 0
        {
            for (collection_id, nonce) in self.loaned_nfts(&wallet_address).iter() {
                let tcl_count = self.tcl_count(&collection_id, &nonce).get();

                lending_data_str.append(&collection_id.into_managed_buffer());
                lending_data_str.append(&ManagedBuffer::new_from_bytes(b" "));
                lending_data_str.append(&self.decimal_to_ascii((nonce as u32).try_into().unwrap()));
                lending_data_str.append(&ManagedBuffer::new_from_bytes(b" "));
                lending_data_str.append(&self.biguint_to_ascii(&tcl_count));
                lending_data_str.append(&ManagedBuffer::new_from_bytes(b","));
            }
        }
       
        lending_data_str
    }

    //TODO Sterge dupa ce implementeaza Yuris getRewardsData in loc de getLastClaimedEpoch
    #[endpoint(getLastClaimedEpoch)]
    fn get_last_claimed_epoch(&self, wallet_address: ManagedAddress) -> ManagedBuffer {
        let mut return_buffer = ManagedBuffer::new_from_bytes(b"");
    
        let current_epoch = self.blockchain().get_block_epoch();
        let last_claimed_epoch = self.last_claimed_epoch(&wallet_address).get();
        let last_claimed_lending_epoch = self.last_claimed_lending_epoch(&wallet_address).get();

        return_buffer.append(&self.decimal_to_ascii((current_epoch as u32).try_into().unwrap())); //Current Epoch
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.decimal_to_ascii((last_claimed_epoch as u32).try_into().unwrap())); //Last Claimed Epoch
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.decimal_to_ascii((last_claimed_lending_epoch as u32).try_into().unwrap())); //Last Claimed Lending Epoch
    
        return_buffer
    }

    #[endpoint(getRewardsData)]
    fn rewards_data(&self, wallet_address: ManagedAddress) -> ManagedBuffer {
        let mut return_buffer = ManagedBuffer::new_from_bytes(b"");
    
        let total_reserve_amount = self.total_reserve_amount().get();
        let total_rewards_released = self.total_rewards_released().get();
        let apr_max = self.apr_max().get();
        let total_staked_amount = self.total_staked_amount().get();
        let user_staked_amount = self.user_staked_amount(&wallet_address).get();
        let user_loaned_amount = self.user_loaned_amount(&wallet_address).get();
        let current_epoch = self.blockchain().get_block_epoch();
        let last_claimed_epoch = self.last_claimed_epoch(&wallet_address).get();
        let last_claimed_lending_epoch = self.last_claimed_lending_epoch(&wallet_address).get();

        let daily_rewads = if self.user_borrowed_amount(&wallet_address, &current_epoch).is_empty(){
            self.calculate_reward(&wallet_address, user_staked_amount.clone())
        }else{
            self.calculate_reward(&wallet_address, self.user_borrowed_amount(&wallet_address, &current_epoch).get())
        };

        let daily_loaned_rewads = self.calculate_reward(&wallet_address, user_loaned_amount.clone());
        let available_nfts_count = self.available_borrow_nfts(&current_epoch).len();
        let last_borrowed_claimed_epoch = self.last_borrowed_claimed_epoch(&wallet_address).get();

        return_buffer.append(&self.biguint_to_ascii(&total_reserve_amount)); //Total Reserve Amount
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.biguint_to_ascii(&total_rewards_released)); //Total Rewards Released
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.decimal_to_ascii((apr_max as u32).try_into().unwrap())); //APR Max
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.biguint_to_ascii(&total_staked_amount)); //Total Staked Amount
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.biguint_to_ascii(&user_staked_amount)); //Used Staked Amount
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.biguint_to_ascii(&user_loaned_amount)); //Used Loaned Amount
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.decimal_to_ascii((current_epoch as u32).try_into().unwrap())); //Current Epoch
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.decimal_to_ascii((last_claimed_epoch as u32).try_into().unwrap())); //Last Claimed Epoch
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.decimal_to_ascii((last_claimed_lending_epoch as u32).try_into().unwrap())); //Last Claimed Lending Epoch
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.biguint_to_ascii(&daily_rewads)); //Daily Rewards
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.biguint_to_ascii(&daily_loaned_rewads)); //Daily Loaned Rewards
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.decimal_to_ascii((available_nfts_count as u32).try_into().unwrap())); //Available borrow count
        return_buffer.append(&ManagedBuffer::new_from_bytes(b" "));
        return_buffer.append(&self.decimal_to_ascii((last_borrowed_claimed_epoch as u32).try_into().unwrap())); //Last Claimed Borrowing Epoch
        
    
        return_buffer
    }

}