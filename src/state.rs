use std::sync::Arc;

use crate::repository::Repository;
#[derive(Clone, Debug)]
pub struct AppState {
    pub repository: Arc<dyn Repository>,
}
