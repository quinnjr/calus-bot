use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod _api;
pub use self::_api::{ DefaultApi, DefaultApiClient };
mod app_api;
pub use self::app_api::{ AppApi, AppApiClient };
mod community_content_api;
pub use self::community_content_api::{ CommunityContentApi, CommunityContentApiClient };
mod content_api;
pub use self::content_api::{ ContentApi, ContentApiClient };
mod destiny2_api;
pub use self::destiny2_api::{ Destiny2Api, Destiny2ApiClient };
mod fireteam_api;
pub use self::fireteam_api::{ FireteamApi, FireteamApiClient };
mod forum_api;
pub use self::forum_api::{ ForumApi, ForumApiClient };
mod group_v2_api;
pub use self::group_v2_api::{ GroupV2Api, GroupV2ApiClient };
mod preview_api;
pub use self::preview_api::{ PreviewApi, PreviewApiClient };
mod tokens_api;
pub use self::tokens_api::{ TokensApi, TokensApiClient };
mod trending_api;
pub use self::trending_api::{ TrendingApi, TrendingApiClient };
mod user_api;
pub use self::user_api::{ UserApi, UserApiClient };

pub mod configuration;
pub mod client;
