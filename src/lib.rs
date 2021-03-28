use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub struct Id {
    uuid_as_128: u128,
    c: u8,
}

impl Id {
    pub fn get(&self) -> (u128, u8) {        
        (self.uuid_as_128, self.c)
    }
}

pub struct HyperId {
    uuid: Uuid,
    c: u8,
}

impl HyperId {
    pub fn new () -> Self {
        let uuid = Uuid::new_v4();
        let c: u8 = 0;

        Self{
            uuid,
            c,
        }
    }

    pub fn get(&self) -> Id {
        Id {
            uuid_as_128: self.uuid.as_u128(),
            c: self.c,
        }
    }

    pub fn generate(&mut self) -> Id {
        self.c = self.c.checked_add(1)
            .unwrap_or(0);
        if self.c == 0 {
            self.uuid = Uuid::new_v4();
        }

        Id {
            uuid_as_128: self.uuid.as_u128(),
            c: self.c,
        }
    }
}

impl Default for HyperId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_from_zero() {
        let hyperid = HyperId::default();
        assert_eq!(hyperid.c, 0);
    }

    #[test]
    fn get_return_equal_id() {
        let hyperid = HyperId::default();
        assert_eq!(hyperid.get(), hyperid.get());
    }

    #[test]
    fn generate_change_internal_state() {
        let mut hyperid = HyperId::default();

        let c = hyperid.c;
        hyperid.generate();
        assert_ne!(hyperid.c, c);
    }

    #[test]
    fn generate_returns_different_id() {
        let mut hyperid = HyperId::default();

        let previous_id = hyperid.get();
        let next_id = hyperid.generate();
        assert_ne!(previous_id, next_id);
    }

    #[test]
    fn on_255_generate_a_new_base() {
        let mut hyperid = HyperId::default();

        let base = hyperid.uuid;

        for _ in 0..255 {
            hyperid.generate();
        }
        let new_base = hyperid.uuid;
        assert_eq!(base, new_base);

        hyperid.generate();

        let new_base = hyperid.uuid;
        assert_ne!(base, new_base);
    }

    #[test]
    fn different_instances_have_different_base() {
        let hyperid1 = HyperId::default();
        let hyperid2 = HyperId::default();
        assert_ne!(hyperid1.uuid, hyperid2.uuid);
    }
}
