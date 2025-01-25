use std::sync::Arc;

use super::repository::UserRepository;

trait UniqueIdentifier {
    fn identify(identification_token: String) -> Option<String>;
}

struct EMailIdentifier {
    user_repository: Arc<dyn UserRepository>
}

impl UniqueIdentifier for EMailIdentifier {
    fn identify(identification_token: String) -> Option<String> {
        if ()
    }
}
