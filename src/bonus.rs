multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait BonusModule {
    fn add_bonus(&self) {
        /*
        itemType = <get item type> { any of "Weapon" | "Armor" }
        itemSubType = <get item subtype> { any of "Sword" | "Dagger" | "Bow" | "Fan" | "Staff" | "Armor" | "Helmet" | "Shield" | "Shoes" | "Earrings" | "Necklaces" | "Bracelet" }

        dynamicStats = <get item dynamic stats> { Dictionary<String, Integer> }
        defaultStats = <get item default stats> { Dictionary<String, Integer> }

        maxStatsCount = Min(dynamicStatsSubtypesCount[itemSubType], dynamicStatsCount) { dynamicStatsSubtypesCount - Dictionary<String, Integer> | dynamicStatsCount - const }
        if (dynamicStats.size() < maxStatsCount) {
            Pair<String, Integer> randomBonus = get_random_bonus(dynamicStats, itemType, itemSubType, defaultStats, addBonusPercent); { addBonusPercent - const }
            if (newBonus != null && random(1, 100) <= addBonusChance[dynamicStats.size()]) {
                dynamicStats.Add(randomBonus);

                < save attribute >
            }
        }
        */
    }

    fn change_bonus(&self) {
        /*
        itemType = <get item type> { any of "Weapon" | "Armor" }
        itemSubType = <get item subtype> { any of "Sword" | "Dagger" | "Bow" | "Fan" | "Staff" | "Armor" | "Helmet" | "Shield" | "Shoes" | "Earrings" | "Necklaces" | "Bracelet" }

        dynamicStats = <get item dynamic stats> { Dictionary<String, Integer> }
        defaultStats = <get item default stats> { Dictionary<String, Integer> }

        maxStatsCount = Min(dynamicStatsSubtypesCount[itemSubType], dynamicStatsCount) { dynamicStatsSubtypesCount - Dictionary<String, Integer> | dynamicStatsCount - const }
        if (dynamicStats.size() > 0 && dynamicStats.size() <= maxStatsCount) {
            foreach (bonus in dynamicStats) {
                Pair<String, Integer> randomBonus = get_random_bonus(dynamicStats, itemType, itemSubType, defaultStats, addBonusPercent); { addBonusPercent - const }
                if (randomBonus == null)
                    return "You cannot do that.";

                newDynamicStats.Add(randomBonus);
            }
            
            < save attribute >
        } else
            return "You cannot do that.";
        */
    }

    fn get_random_bonus(&self) {
        /*

        */
    }

    // BONUS STORAGE

    #[endpoint(setBonusData)]
    fn set_bonus_data(&self) {
        let mut dynamicStatsCount = 5u8;
        self.set_dynamic_stats_count(dynamicStatsCount);

        let mut dynamicStatsTypes = MultiValueEncoded::new();
        dynamicStatsTypes.push(ManagedBuffer::from("Weapon"));
        dynamicStatsTypes.push(ManagedBuffer::from("Armor"));
        self.set_dynamic_stats_types(dynamicStatsTypes);

        let mut dynamicStatsSubTypes = MultiValueEncoded::new();
        dynamicStatsSubTypes.push(ManagedBuffer::from("Sword"));
        dynamicStatsSubTypes.push(ManagedBuffer::from("Dagger"));
        dynamicStatsSubTypes.push(ManagedBuffer::from("Staff"));
        dynamicStatsSubTypes.push(ManagedBuffer::from("Fan"));
        dynamicStatsSubTypes.push(ManagedBuffer::from("Bow"));
        dynamicStatsSubTypes.push(ManagedBuffer::from("Armor"));
        dynamicStatsSubTypes.push(ManagedBuffer::from("Helmet"));
        dynamicStatsSubTypes.push(ManagedBuffer::from("Shield"));
        dynamicStatsSubTypes.push(ManagedBuffer::from("Shoes"));
        dynamicStatsSubTypes.push(ManagedBuffer::from("Earrings"));
        dynamicStatsSubTypes.push(ManagedBuffer::from("Necklaces"));
        dynamicStatsSubTypes.push(ManagedBuffer::from("Bracelet"));
        self.set_dynamic_stats_available_subtypes(dynamicStatsSubTypes);

        let mut dynamicStatsBonuses = MultiValueEncoded::new();
        dynamicStatsBonuses.push(ManagedBuffer::from("Max HP"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Max SP"));
        //dynamicStatsBonuses.push(ManagedBuffer::from("Max SP")); // necklace
        dynamicStatsBonuses.push(ManagedBuffer::from("Vitality"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Intelligence"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Strength"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Dexterity"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Attack Speed"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Movement Speed"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Spell Speed"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Damage will be absorbed by HP"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Damage will be absorbed by SP"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Chance to take SP from the enemy"));
        dynamicStatsBonuses.push(ManagedBuffer::from("HP Regeneration"));
        dynamicStatsBonuses.push(ManagedBuffer::from("SP Regeneration"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Poisoning Chance"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Chance for Blackout"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Slowing Chance"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Chance of Critical Hit"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Chance of Piercing Hit"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Strong Against Half Humans"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Strong Against Monsters"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Chance to block physical attacks"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Chance to reflect physical attacks"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Chance of Evading Arrow"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Sword Defence"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Dagger Defence"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Fan Defence"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Staff Defence"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Arrow Resistance"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Magic Resistance"));
        dynamicStatsBonuses.push(ManagedBuffer::from( "Poison Resistance"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Chance of double EXP"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Chance of double Curse drop"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Chance of double Item drop"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Defence against Blackouts"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Defence against Slowing"));
        dynamicStatsBonuses.push(ManagedBuffer::from("Attack Value"));
        //dynamicStatsBonuses.push(ManagedBuffer::from("Average Damage"));
        //dynamicStatsBonuses.push(ManagedBuffer::from("Spell Damage"));
        self.set_dynamic_stats_bonuses(dynamicStatsBonuses);

        let mut addBonusChance = MultiValueEncoded::new();
        addBonusChance.push(100u8);
        addBonusChance.push(80u8);
        addBonusChance.push(60u8);
        addBonusChance.push(50u8);
        addBonusChance.push(30u8);
        addBonusChance.push(0u8);
        addBonusChance.push(0u8);
        self.set_add_bonus_chance(addBonusChance);

        let mut addBonusPercent = MultiValueEncoded::new();
        addBonusPercent.push(40u8);
        addBonusPercent.push(50u8);
        addBonusPercent.push(10u8);
        addBonusPercent.push(0u8);
        addBonusPercent.push(0u8);
        self.set_add_bonus_percent(addBonusPercent);

        let mut changeBonusPercent = MultiValueEncoded::new();
        changeBonusPercent.push(0u8);
        changeBonusPercent.push(10u8);
        changeBonusPercent.push(40u8);
        changeBonusPercent.push(35u8);
        changeBonusPercent.push(15u8);
        self.set_change_bonus_percent(changeBonusPercent);

        let mut dynamicStatsChances = MultiValueEncoded::new();
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Max HP"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Max SP"), 0u8)));
        //dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Max SP"), 0u8))); // necklace
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Vitality"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Intelligence"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Strength"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Dexterity"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Attack Speed"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Movement Speed"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Spell Speed"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Damage will be absorbed by HP"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Damage will be absorbed by SP"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Chance to take SP from the enemy"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("HP Regeneration"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("SP Regeneration"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Poisoning Chance"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Chance for Blackout"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Slowing Chance"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Chance of Critical Hit"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Chance of Piercing Hit"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Strong Against Half Humans"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Strong Against Monsters"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Chance to block physical attacks"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Chance to reflect physical attacks"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Chance of Evading Arrow"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Sword Defence"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Dagger Defence"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Fan Defence"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Staff Defence"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Arrow Resistance"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Magic Resistance"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from( "Poison Resistance"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Chance of double EXP"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Chance of double Curse drop"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Chance of double Item drop"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Defence against Blackouts"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Defence against Slowing"), 0u8)));
        dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Attack Value"), 0u8)));
        //dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Average Damage"), 0u8)));
        //dynamicStatsChances.push(MultiValue2::from((ManagedBuffer::from("Spell Damage"), 0u8)));
        self.set_dynamic_stats_chances(dynamicStatsChances);

        let mut dynamicBonusesValues = MultiValueEncoded::new();

        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Max HP"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Max SP"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        /*
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Max SP"), 0u8))); // necklace
        */

        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Vitality"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Intelligence"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Strength"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Dexterity"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Attack Speed"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Movement Speed"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Spell Speed"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Damage will be absorbed by HP"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Damage will be absorbed by SP"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Chance to take SP from the enemy"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("HP Regeneration"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("SP Regeneration"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Poisoning Chance"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Chance for Blackout"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Slowing Chance"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Chance of Critical Hit"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Chance of Piercing Hit"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Strong Against Half Humans"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Strong Against Monsters"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Chance to block physical attacks"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Chance to reflect physical attacks"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Chance of Evading Arrow"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Sword Defence"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Dagger Defence"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Fan Defence"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Staff Defence"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Arrow Resistance"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Magic Resistance"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from( "Poison Resistance"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Chance of double EXP"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Chance of double Curse drop"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Chance of double Item drop"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Defence against Blackouts"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Defence against Slowing"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Attack Value"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        /*
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Average Damage"), MultiValueManagedVecCounted::from(dynamicBonusesValuesList))));
        
        let mut dynamicBonusesValuesList = ManagedVec::new();
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValuesList.push(0u16);
        dynamicBonusesValues.push(MultiValue2::from((ManagedBuffer::from("Spell Damage"), 0u8)));
        */

        self.set_dynamic_stats_values(dynamicBonusesValues);

        let mut dynamicBonusesSubtypes = MultiValueEncoded::new();

        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Max HP"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Max SP"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        /*
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Max SP"), 0u8))); // necklace
        */

        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Vitality"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Intelligence"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Strength"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Dexterity"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Attack Speed"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Movement Speed"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Spell Speed"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Damage will be absorbed by HP"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Damage will be absorbed by SP"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Chance to take SP from the enemy"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("HP Regeneration"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("SP Regeneration"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Poisoning Chance"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Chance for Blackout"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Slowing Chance"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Chance of Critical Hit"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Chance of Piercing Hit"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Strong Against Half Humans"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Strong Against Monsters"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Chance to block physical attacks"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Chance to reflect physical attacks"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Chance of Evading Arrow"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Sword Defence"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Dagger Defence"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Fan Defence"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Staff Defence"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Arrow Resistance"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Magic Resistance"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from( "Poison Resistance"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Chance of double EXP"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Chance of double Curse drop"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Chance of double Item drop"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Defence against Blackouts"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Defence against Slowing"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Attack Value"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        /*
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Average Damage"), MultiValueManagedVecCounted::from(dynamicBonusesSubtypesList))));
        
        let mut dynamicBonusesSubtypesList = ManagedVec::new();
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypesList.push(ManagedBuffer::from("Weapon"));
        dynamicBonusesSubtypes.push(MultiValue2::from((ManagedBuffer::from("Spell Damage"), 0u8)));
        */

        self.set_dynamic_stats_subtypes(dynamicBonusesSubtypes);
    }

    //#region

    // DYNAMIC_STATS_COUNT
    #[view(getDynamicStatsCount)]
    #[storage_mapper("dynamic_stats_count")]
    fn dynamic_stats_count(&self) -> SingleValueMapper<u8>; // 5

    #[endpoint(setDynamicStatsCount)]
    fn set_dynamic_stats_count(&self, value: u8) {
        self.dynamic_stats_count().update(|v| *v = value);
    }

    // DYNAMIC_STATS_TYPES
    #[view(getDynamicStatsTypes)]
    #[storage_mapper("dynamic_stats_types")]
    fn dynamic_stats_types(&self) -> SetMapper<ManagedBuffer>; // [ "Weapon", "Armor" ]

    #[endpoint(setDynamicStatsTypes)]
    fn set_dynamic_stats_types(&self, values: MultiValueEncoded<ManagedBuffer>) {
        self.dynamic_stats_types().clear();
        for value in values {
            self.dynamic_stats_types().insert(value);
        }
    }

    // DYNAMIC_STATS_SUBTYPES

    #[view(getDynamicStatsAvailableSubtypes)]
    #[storage_mapper("dynamic_stats_available_subtypes")]
    fn dynamic_stats_available_subtypes(&self) -> SetMapper<ManagedBuffer>; // [ "Sword", "Dagger", "Staff", "Fan", "Bow", "Armor", "Helmet", "Shield", "Shoes", "Earrings", "Necklaces", "Bracelet" ]

    #[endpoint(setDynamicStatsAvailableSubtypes)]
    fn set_dynamic_stats_available_subtypes(&self, values: MultiValueEncoded<ManagedBuffer>) {
        self.dynamic_stats_available_subtypes().clear();
        for value in values {
            self.dynamic_stats_available_subtypes().insert(value);
        }
    }

    // DYNAMIC_STATS_BONUSES

    #[view(getDynamicStatsBonuses)]
    #[storage_mapper("dynamic_stats_bonuses")]
    fn dynamic_stats_bonuses(&self) -> SetMapper<ManagedBuffer>; // [ "Max HP", "Max SP", "Max SP", "Vitality", "Intelligence", "Strength", "Dexterity", "Attack Speed", "Movement Speed", "Spell Speed", "Damage will be absorbed by HP", "Damage will be absorbed by SP", "Chance to take SP from the enemy", "HP Regeneration", "SP Regeneration", "Poisoning Chance", "Chance for Blackout", "Slowing Chance", "Chance of Critical Hit", "Chance of Piercing Hit", "Strong Against Half Humans", "Strong Against Monsters", "Chance to block physical attacks", "Chance to reflect physical attacks", "Chance of Evading Arrow", "Sword Defence", "Dagger Defence", "Fan Defence", "Staff Defence", "Arrow Resistance", "Magic Resistance",  "Poison Resistance", "Chance of double EXP", "Chance of double Curse drop", "Chance of double Item drop", "Defence against Blackouts", "Defence against Slowing", "Attack Value", "Average Damage", "Spell Damage" ]

    #[endpoint(setDynamicStatsBonuses)]
    fn set_dynamic_stats_bonuses(&self, values: MultiValueEncoded<ManagedBuffer>) {
        self.dynamic_stats_bonuses().clear();
        for value in values {
            self.dynamic_stats_bonuses().insert(value);
        }
    }

    // ADD_BONUS_CHANCE

    #[view(getAddBonusChance)]
    #[storage_mapper("add_bonus_chance")]
    fn add_bonus_chance(&self) -> SetMapper<u8>; // [ 100, 80, 60, 50, 30, 0, 0 ]

    #[endpoint(setAddBonusChance)]
    fn set_add_bonus_chance(&self, values: MultiValueEncoded<u8>) {
        self.add_bonus_chance().clear();
        for value in values {
            self.add_bonus_chance().insert(value);
        }
    }

    // ADD_BONUS_PERCENT

    #[view(getAddBonusPercent)]
    #[storage_mapper("add_bonus_percent")]
    fn add_bonus_percent(&self) -> SetMapper<u8>; // [ 40, 50, 10, 0, 0 ]

    #[endpoint(setAddBonusPercent)]
    fn set_add_bonus_percent(&self, values: MultiValueEncoded<u8>) {
        self.add_bonus_percent().clear();
        for value in values {
            self.add_bonus_percent().insert(value);
        }
    }

    // CHANGE_BONUS_PERCENT

    #[view(getChangeBonusPercent)]
    #[storage_mapper("change_bonus_percent")]
    fn change_bonus_percent(&self) -> SetMapper<u8>; // [ 0, 10, 40, 35, 15 ]

    #[endpoint(setChangeBonusPercent)]
    fn set_change_bonus_percent(&self, values: MultiValueEncoded<u8>) {
        self.change_bonus_percent().clear();
        for value in values {
            self.change_bonus_percent().insert(value);
        }
    }

    // DYNAMIC_STATS_CHANCES

    #[view(getDynamicStatsChances)]
    #[storage_mapper("dynamic_stats_chances")]
    fn dynamic_stats_chances(&self) -> MapMapper<ManagedBuffer, u8>; // [ ["Max HP", 100], ["Max SP", 50] ]

    #[endpoint(getDynamicStatsChance)]
    fn get_dynamic_stats_chance(&self, bonus: ManagedBuffer) -> u8 {
        return self.dynamic_stats_chances().get(&bonus).unwrap_or_default();
    }

    #[endpoint(setDynamicStatsChances)]
    fn set_dynamic_stats_chances(&self, chances: MultiValueEncoded<MultiValue2<ManagedBuffer, u8>>) {
        self.dynamic_stats_chances().clear();
        for chance in chances.into_iter() {
            let (key, value) = chance.into_tuple();
            self.dynamic_stats_chances().insert(key, value);
        }
    }

    // DYNAMIC_STATS_VALUES
    #[view(getDynamicStatsValues)]
    #[storage_mapper("dynamic_stats_values")]
    fn dynamic_stats_values(&self) -> MapMapper<ManagedBuffer, ManagedVec<u16>>; // [ ["Max HP", [500, 1000, 1500, 2000] ], ["Max SP", [50, 100, 200] ] ]

    #[endpoint(getDynamicStatsValue)]
    fn get_dynamic_stats_value(&self, bonus: ManagedBuffer, index: usize) -> u16 {
        let get_bonus = self.dynamic_stats_values().get(&bonus).unwrap_or_default();
        return get_bonus.get(index);
    }

    #[endpoint(setDynamicStatsValues)]
    fn set_dynamic_stats_values(&self, values: MultiValueEncoded<MultiValue2<ManagedBuffer, MultiValueManagedVecCounted<u16>>>) {
        let keys: ManagedVec<ManagedBuffer> = self.dynamic_stats_values().keys().collect();
        for key in &keys {
            self.dynamic_stats_values().remove(&key);
        }
        
        for value in values.into_iter() {
            let (key, bonus_values) = value.into_tuple();
            self.dynamic_stats_values().insert(key, bonus_values.into_vec());
        }
    }

    // DYNAMIC_STATS_SUBTYPES

    #[view(getDynamicStatsSubtypes)]
    #[storage_mapper("dynamic_stats_subtypes")]
    fn dynamic_stats_subtypes(&self) -> MapMapper<ManagedBuffer, ManagedVec<ManagedBuffer>>; // [ ["Max HP", [ "Armor", "Necklace", "Shoes" ] ], ["Max SP", [ "Necklace" ] ] ]

    #[endpoint(getDynamicStatsSubtype)]
    fn get_dynamic_stats_subtype(&self, bonus: ManagedBuffer, index: usize) -> ManagedBuffer {
        let get_subtype = self.dynamic_stats_subtypes().get(&bonus).unwrap_or_default();
        return (*get_subtype.get(index)).clone();
    }
    
    #[endpoint(setDynamicStatsSubtypes)]
    fn set_dynamic_stats_subtypes(&self, values: MultiValueEncoded<MultiValue2<ManagedBuffer, MultiValueManagedVecCounted<ManagedBuffer>>>) {
        let keys: ManagedVec<ManagedBuffer> = self.dynamic_stats_subtypes().keys().collect();
        for key in &keys {
            self.dynamic_stats_subtypes().remove(&key);
        }

        for value in values.into_iter() {
            let (key, bonus_values) = value.into_tuple();
            self.dynamic_stats_subtypes().insert(key.clone(), bonus_values.into_vec());

            for sub_type in self.dynamic_stats_subtypes().get(&key).unwrap_or_default().iter() {
                if !self.dynamic_stats_subtypes_count().contains_key(&sub_type) {
                    self.dynamic_stats_subtypes_count().insert((*sub_type).clone(), 0u8);
                }
        
                let current_count = self.dynamic_stats_subtypes_count().get(&sub_type).unwrap();
                self.dynamic_stats_subtypes_count().insert((*sub_type).clone(), current_count + 1);
            }
        }
    }

    // DYNAMIC_STATS_SUBTYPES_COUNT

    #[view(getDynamicStatsSubtypesCount)]
    #[storage_mapper("dynamic_stats_subtypes_count")]
    fn dynamic_stats_subtypes_count(&self) -> MapMapper<ManagedBuffer, u8>; // [ ["Armor", 5], ["Shoes", 6] ]

    #[endpoint(getDynamicStatsSubtypeCount)]
    fn dynamic_stats_subtype_count(&self, sub_type: ManagedBuffer) -> u8 {
        return self.dynamic_stats_subtypes_count().get(&sub_type).unwrap_or_default();
    }

    //#endregion */
}
