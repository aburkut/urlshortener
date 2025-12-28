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
        nanoid::format(nanoid::rngs::default, &nanoid::alphabet::SAFE, self.length)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nanoid_provider_default_length() {
        let provider = NanoIDProvider::default();
        let id = provider.provide();
        assert_eq!(id.len(), 7, "Default NanoID length should be 7");
    }

    #[test]
    fn test_nanoid_provider_custom_length() {
        let provider = NanoIDProvider::new(10);
        let id = provider.provide();
        assert_eq!(id.len(), 10, "Custom NanoID length should be 10");
    }

    #[test]
    fn test_nanoid_provider_uniqueness() {
        let provider = NanoIDProvider::default();
        let id1 = provider.provide();
        let id2 = provider.provide();
        assert_ne!(id1, id2, "NanoID should generate unique IDs on each call");
    }

    #[test]
    fn test_nanoid_provider_safe_alphabet() {
        let provider = NanoIDProvider::default();
        let id = provider.provide();
        // Safe alphabet should only contain URL-safe characters
        assert!(
            id.chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-'),
            "NanoID should only contain URL-safe characters"
        );
    }

    #[test]
    fn test_fake_id_provider() {
        let provider = FakeIDProvider::new("test123".to_string());
        assert_eq!(provider.provide(), "test123");
        assert_eq!(
            provider.provide(),
            "test123",
            "FakeIDProvider should return the same ID"
        );
    }

    #[test]
    fn test_fake_id_provider_set_id() {
        let mut provider = FakeIDProvider::new("initial".to_string());
        assert_eq!(provider.provide(), "initial");

        provider.set_id("updated".to_string());
        assert_eq!(
            provider.provide(),
            "updated",
            "FakeIDProvider should return updated ID"
        );
    }
}
