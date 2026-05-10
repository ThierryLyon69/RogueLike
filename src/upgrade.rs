#[derive(Clone, Copy)]
pub enum Upgrade {
    Damage,
    Speed,
    FireRate,
    MaxHp,
}

pub struct UpgradeState {
    pub current: Option<Upgrade>,
}

impl UpgradeState {
    pub fn new() -> Self {
        Self { current: None }
    }

    pub fn label(&self) -> &'static str {
        match self.current {
            Some(Upgrade::Damage) => "Damage",
            Some(Upgrade::Speed) => "Speed",
            Some(Upgrade::FireRate) => "FireRate",
            Some(Upgrade::MaxHp) => "MaxHp",
            None => "None",
        }
    }
}
