use axum::Router;

pub trait HttpService {
    fn get_router(&self) -> Router;
}
