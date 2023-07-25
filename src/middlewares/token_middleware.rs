use std::{
  rc::Rc,
  future::{ready, Ready}
};
use futures_util::future::LocalBoxFuture;
use actix_web::{
  Error,
  error::ErrorInternalServerError,
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}
};

use crate::structs::auth_struct::TokenStruct;

pub struct CheckToken;

impl<S, B> Transform<S, ServiceRequest> for CheckToken
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = CheckTokenMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(CheckTokenMiddleware {
      service: Rc::new(service)
    }))
  }
}

pub struct CheckTokenMiddleware<S> {
  service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CheckTokenMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let srv = self.service.clone();

    Box::pin(async move {
      if req.path() == "/" || req.path() == "/login" || req.path() == "/register" {
        let res = srv.call(req).await?;
  
        return Ok(res)
      }

      let user_details = req.headers()
        .get("Authorization")
        .ok_or(())
        .and_then(|h| h.to_str().map_err(|_| ()))
        .ok()
        .and_then(|s| s.split_whitespace().nth(1))
        .ok_or(())
        .and_then(|t| TokenStruct::from_jwt(t).map_err(|_| ()))
        .ok();

      if let Some(_) = user_details {
        let res = srv.call(req).await?;
  
        Ok(res)
      } else {
        Err(ErrorInternalServerError("please identify yourself as a user by providing a valid token"))
      }
    })
  }
}