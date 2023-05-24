pub mod configuration;
pub mod routers;
pub mod startup;
pub mod models;
pub mod services;
pub mod app;
pub mod utils;
pub mod middleware;
pub const IGNORE_ROUTES: [&str;4] = ["/api/testping", "/api/auth/signup", "/api/auth/login","/api/auth/logout"];
