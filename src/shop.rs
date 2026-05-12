use rand::seq::SliceRandom;

use crate::player::Player;

#[derive(Clone, Copy)]
pub enum ShopItemId {
    Damage,
    Speed,
    FireRate,
    MaxHp,
    Heal,
    Range,
}

#[derive(Clone, Copy)]
pub struct ShopItem {
    pub id: ShopItemId,
    pub name: &'static str,
    pub cost: i32,
}

impl ShopItem {
    pub fn apply(&self, player: &mut Player) {
        match self.id {
            ShopItemId::Damage => player.add_damage(1),
            ShopItemId::Speed => player.add_speed(20.0),
            ShopItemId::FireRate => player.reduce_fire_interval(0.02),
            ShopItemId::MaxHp => player.add_max_hp(1),
            ShopItemId::Heal => player.heal(1),
            ShopItemId::Range => player.add_attack_range(20.0),
        }
    }
}

pub fn catalog() -> &'static [ShopItem] {
    static ITEMS: [ShopItem; 6] = [
        ShopItem {
            id: ShopItemId::Damage,
            name: "Damage +1",
            cost: 5,
        },
        ShopItem {
            id: ShopItemId::Speed,
            name: "Speed +20",
            cost: 4,
        },
        ShopItem {
            id: ShopItemId::FireRate,
            name: "FireRate +",
            cost: 6,
        },
        ShopItem {
            id: ShopItemId::MaxHp,
            name: "Max HP +1",
            cost: 7,
        },
        ShopItem {
            id: ShopItemId::Heal,
            name: "Heal +1",
            cost: 3,
        },
        ShopItem {
            id: ShopItemId::Range,
            name: "Range +20",
            cost: 4,
        },
    ];
    &ITEMS
}

pub fn roll_offers(count: usize) -> Vec<ShopItem> {
    let mut items = catalog().to_vec();
    let mut rng = rand::thread_rng();
    items.shuffle(&mut rng);
    let take = count.min(items.len());
    items.truncate(take);
    items
}
