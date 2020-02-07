/// Copied from https://github.com/petertrotman/actix-web-middleware-redirect-https with an
/// additional enabled property added, and support X-Forwarded-Proto header.
///
/// Provides a middleware for `actix-web` to redirect all `http` requests to `https`.

use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http, Error, HttpResponse,
};
use futures::future::{ok, Either, Ready};
use std::task::{Context, Poll};

/// Middleware for `actix-web` which redirects all `http` requests to `https` with optional url
/// string replacements.
///
/// ## Usage
/// ```
/// use actix_web::{App, web, HttpResponse};
/// use actix_web_middleware_redirect_https::RedirectHTTPS;
///
/// App::new()
///     .wrap(RedirectHTTPS::default())
///     .route("/", web::get().to(|| HttpResponse::Ok()
///                                     .content_type("text/plain")
///                                     .body("Always HTTPS!")));
/// ```
#[derive(Debug, Default, Clone)]
pub struct RedirectHTTPS {
    replacements: Vec<(String, String)>,
    enabled: bool
}

#[allow(dead_code)]
impl RedirectHTTPS {
    /// Creates a RedirectHTTPS middleware which also performs string replacement on the final url.
    /// This is useful when not running on the default web and ssl ports (80 and 443) since we will
    /// need to change the development web port in the hostname to the development ssl port.
    ///
    /// ## Usage
    /// ```
    /// use actix_web::{App, web, HttpResponse};
    /// use actix_web_middleware_redirect_https::RedirectHTTPS;
    ///
    /// App::new()
    ///     .wrap(RedirectHTTPS::with_replacements(&[(":8080".to_owned(), ":8443".to_owned())]))
    ///     .route("/", web::get().to(|| HttpResponse::Ok()
    ///                                     .content_type("text/plain")
    ///                                     .body("Always HTTPS on non-default ports!")));
    /// ```
    pub fn with_replacements(replacements: &[(String, String)]) -> Self {
        RedirectHTTPS {
            replacements: replacements.to_vec(),
            enabled: false
        }
    }

    pub fn enable(&self, config: bool) -> Self {
        RedirectHTTPS {
            replacements: self.clone().replacements,
            enabled: config
        }
    }
}

impl<S, B> Transform<S> for RedirectHTTPS
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RedirectHTTPSService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectHTTPSService {
            service,
            replacements: self.replacements.clone(),
            enabled: self.enabled
        })
    }
}

pub struct RedirectHTTPSService<S> {
    service: S,
    replacements: Vec<(String, String)>,
    enabled: bool
}

impl<S, B> Service for RedirectHTTPSService<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {

        let scheme =  req.headers().get("X-Forwarded-Proto")
            .map(|header| header.to_str().unwrap())
            .unwrap_or(req.connection_info().scheme())
            .to_lowercase();

        if !self.enabled || scheme == "https" {
            Either::Left(self.service.call(req))
        } else {
            let host = req.connection_info().host().to_owned();
            let uri = req.uri().to_owned();
            let mut url = format!("https://{}{}", host, uri);
            for (s1, s2) in self.replacements.iter() {
                url = url.replace(s1, s2);
            }
            Either::Right(ok(req.into_response(
                HttpResponse::MovedPermanently()
                    .header(http::header::LOCATION, url)
                    .finish()
                    .into_body(),
            )))
        }
    }
}
