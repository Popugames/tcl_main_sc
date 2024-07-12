use multiversx_sc::imports::*;
use multiversx_sc::derive_imports::*;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Clone)]
pub enum EquipSlot {
    Armor,
    Weapon,
    Shield,
    Helmet,
    Necklace,
    Bracelet,
    Shoes,
    Earrings,
    Boost,
    None,
}

impl EquipSlot {
    pub fn from_i32(value: i32) -> Option<EquipSlot> {
        match value {
            0 => Some(EquipSlot::Armor),
            1 => Some(EquipSlot::Weapon),
            2 => Some(EquipSlot::Shield),
            3 => Some(EquipSlot::Helmet),
            4 => Some(EquipSlot::Necklace),
            5 => Some(EquipSlot::Bracelet),
            6 => Some(EquipSlot::Shoes),
            7 => Some(EquipSlot::Earrings),
            8 => Some(EquipSlot::Boost),
            _ => Some(EquipSlot::None),
        }
    }
}
