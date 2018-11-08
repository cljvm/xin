use actix_web::middleware::{Middleware, Started};
use actix_web::{Error, HttpRequest, HttpResponse};

use actix_web::error::ParseError;
use actix_web::http::header::AUTHORIZATION;
use actix_web::HttpMessage;
use chrono::Utc;

use jwt::{decode, TokenData, Validation};

use error::ErrorMessage;

// 鉴权
pub struct Auth;

#[derive(Deserialize)]
pub enum Flag {
    Teacher,
    Student,
}

#[derive(Deserialize)]
pub struct Claims {
    // pub iss: String, // (Issuser)：代表这个JWT的签发主体
    // pub sub: String, // (Subject)：代表这个JWT的主题
    // pub aud: Vec<String>, // (Audience)：代表这个JWT的接收对象
    pub iat: i64, // (Issued at)：是一个时间戳，代表这个JWT的签发时间
    pub exp: i64, // (Expiration time)：是一个时间戳，代表这个JWT的过期时间
    // pub nbf: i64, // (Not Before)：是一个时间戳，代表这个JWT生效的开始时间，意味着在这个时间之前验证JWT是会失败的
    // pub jti: String, // (JWT ID)：是JWT的唯一标识，主要用来作为一次性token,从而回避重放攻击
    pub user: u32,
    pub flag: Flag,
}

impl Claims {
    pub fn new(user: u32, flag: Flag) -> Claims {
        let now = Utc::now().timestamp();
        Claims {
            // iss: "sender".into(),
            // sub: "subject".into(),
            // aud: vec!("receiver".into()),
            iat: now,
            exp: now + (1000 * 60 * 24 * 30),
            // nbf: now,
            // jti: "".into(),
            user: user,
            flag: flag,
        }
    }
}

impl<S> Middleware<S> for Auth {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started, Error> {
        // TODO 是否需要登录才能访问

        if let Ok(token_data) = self.validate(req) {
            // TODO 用户访问权限
            Ok(Started::Done)
        } else {
            Ok(Started::Response(HttpResponse::Unauthorized().json(
                ErrorMessage {
                    code: 401,
                    msg: "Invalid authentication token.".to_owned(),
                    detail: None,
                },
            )))
        }
    }
}

impl Auth {
    fn validate<S>(&self, req: &HttpRequest<S>) -> Result<TokenData<Claims>, ParseError> {
        if let Some(bearer) = req.headers().get(AUTHORIZATION) {
            let mut parts = bearer.to_str().unwrap().splitn(2, ' ');
            match parts.next() {
                Some(scheme) if scheme == "Bearer" => (),
                _ => return Err(ParseError::Header),
            }
            let token = parts.next().ok_or(ParseError::Header)?;
            let secret = b"lx*star";
            let validation = Validation { validate_iat: true, ..Validation::default() };
            let token_data = decode::<Claims>(&token, secret, &validation);
            token_data.map_err(|_e| ParseError::Header)
        } else {
            Err(ParseError::Header)
        }
    }
}
