pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        Some(Self {
            health: 100,
            mana: self.mana.map(|_v| 100),
            ..*self
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            self.health = self.health.saturating_sub(mana_cost);
        }

        if mana_cost > self.mana.unwrap_or(0) {
            return 0;
        }

        self.mana = self.mana.map(|m| m - mana_cost);

        mana_cost * 2
    }
}
