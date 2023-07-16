use std::sync::Arc;
use std::collections::HashSet;
use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;
use actix_web::{Error, body::EitherBody, dev::{self, Service, ServiceRequest, ServiceResponse, Transform}};

use crate::{helpers::response::response_json, structs::auth_struct::TokenStruct};

pub struct CheckToken;

impl<S, B> Transform<S, ServiceRequest> for CheckToken
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type InitError = ();
  type Transform = CheckTokenMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(CheckTokenMiddleware { service }))
  }
}
pub struct CheckTokenMiddleware<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for CheckTokenMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  dev::forward_ready!(service);

  fn call(&self, request: ServiceRequest) -> Self::Future {
    let path = request.path().to_string();

    if path == "/" || path == "/login" || path == "/register" {
      let fut = self.service.call(request);

      return Box::pin(async move {
        fut.await.map(ServiceResponse::map_into_left_body)
      })
    }

    let whitelist_routes: HashSet<String> = vec![
      "/forum".to_owned(),
      "/student".to_owned(),
      "/university".to_owned(),
      "/scholarship".to_owned(),
    ].into_iter().collect();
  
    let whitelist_routes = Arc::new(whitelist_routes);
  
    let user_details = request
      .headers()
      .get("Authorization")
      .ok_or(())
      .and_then(|h| h.to_str().map_err(|_| ()))
      .ok()
      .and_then(|s| s.split_whitespace().nth(1))
      .ok_or(())
      .and_then(|t| TokenStruct::from_jwt(t).map_err(|_| ()))
      .ok();

    if let Some(user) = user_details {
      if whitelist_routes.contains(&path) && user.role == "user" {
        let method = request.method().to_string();

        if method == "POST" || method == "PUT" || method == "DELETE" {
          let req = request.into_parts().0;
          let response = response_json(
            "unauthorize".to_string(),
            "you are not allowed to access this route".to_string(),
            vec![]
          ).map_into_right_body();
      
          return Box::pin(async { Ok(ServiceResponse::new(req, response)) })
        } else {
          let fut = self.service.call(request);
      
          return Box::pin(async move {
            fut.await.map(ServiceResponse::map_into_left_body)
          })
        }
      }

      let fut = self.service.call(request);
  
      return Box::pin(async move {
        fut.await.map(ServiceResponse::map_into_left_body)
      })
    }

    let req = request.into_parts().0;
    let response = response_json(
      "unauthorize".to_string(),
      "please authorize your self as user".to_string(),
      vec![]
    ).map_into_right_body();

    return Box::pin(async { Ok(ServiceResponse::new(req, response)) })
  }
}