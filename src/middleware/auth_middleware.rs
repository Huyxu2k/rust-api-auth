use crate::{models::response::ResponseBody, utils::token_utils, IGNORE_ROUTES};
//use actix_service::{Service, Transform};
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::{
        header::{HeaderName, HeaderValue},
        Method,
    },
    web::Data,
    Error, HttpResponse,
};
use futures::{
    executor::block_on,
    future::{ok, ready, Ready},
};
use futures_util::future::LocalBoxFuture;
use log::{error, info};
use sqlx::{MySqlPool};
use std::{
    task::{Poll},
};

pub struct AuthenticationMiddleware<S> {
    service: S,
}
impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Response = ServiceResponse<EitherBody<B>>;
    type Future =LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;// Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>; //LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    //dev::forward_ready!(service);

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self,mut req: ServiceRequest) -> Self::Future {
        let mut authenticate_pas = false;

        let headers = req.headers_mut();
        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );
        if Method::OPTIONS == *req.method() {
            authenticate_pas = true;
        } else {
            for router in IGNORE_ROUTES {
                if req.path().starts_with(router) {
                    authenticate_pas = true;
                    break;
                }
            }
            if !authenticate_pas {
                if let Some(pool) = req.app_data::<Data<MySqlPool>>() {
                    info!("Connecting to database...");
                    if let Some(authen_header) = req.headers().get("Authorization") {
                        info!("Parsing authorization header...");
                        if let Ok(authen_str) = authen_header.to_str() {
                            if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer")
                            {
                                info!("Parsing token...");
                                let token = authen_str[6..authen_str.len()].trim();
                                if let Ok(token_data) = token_utils::decode_token(token.to_string())
                                {
                                    info!("Decoding token...");
                                    //let token_result=token_utils::verify_token(&token_data, pool);
                                    //block_on(token_result);
                                    if block_on(token_utils::verify_token(&token_data, pool.get_ref().to_owned()))
                                        .is_ok()
                                    {
                                        info!("Valid token");
                                        authenticate_pas = true;
                                    } else {
                                        error!("Invalid token");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if authenticate_pas {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
                //fut.await.map(ServiceResponse::map_into_left_body)
            })
        } else {
            let res = HttpResponse::Unauthorized()
                .json(ResponseBody::new("Invalid Token".to_string(), ""))
                .map_into_right_body();
            Box::pin(async move { 
                Ok(req.into_response(res))
             })
        }
    }
}

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type InitError = ();
    type Response = ServiceResponse<EitherBody<B>>;
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}
