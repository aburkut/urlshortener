pub trait IDProvider {
    fn provide(&self) -> String;
}

pub struct NanoIDProvider {
    length: usize,
}

impl NanoIDProvider {
    pub fn new(length: usize) -> Self {
        Self { length }
    }
}

impl Default for NanoIDProvider {
    fn default() -> Self {
        Self { length: 7 }
    }
}

impl IDProvider for NanoIDProvider {
    fn provide(&self) -> String {
        nanoid::format(
            nanoid::rngs::default,
            &nanoid::alphabet::SAFE,
            self.length
        )
    }
}

pub struct FakeIDProvider {
    id: String,
}

impl FakeIDProvider {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
}

impl IDProvider for FakeIDProvider {
    fn provide(&self) -> String {
        self.id.clone()
    }
}