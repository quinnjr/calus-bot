use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  _api: Box<::apis::DefaultApi>,
  app_api: Box<::apis::AppApi>,
  community_content_api: Box<::apis::CommunityContentApi>,
  content_api: Box<::apis::ContentApi>,
  destiny2_api: Box<::apis::Destiny2Api>,
  fireteam_api: Box<::apis::FireteamApi>,
  forum_api: Box<::apis::ForumApi>,
  group_v2_api: Box<::apis::GroupV2Api>,
  preview_api: Box<::apis::PreviewApi>,
  tokens_api: Box<::apis::TokensApi>,
  trending_api: Box<::apis::TrendingApi>,
  user_api: Box<::apis::UserApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      _api: Box::new(::apis::DefaultApiClient::new(rc.clone())),
      app_api: Box::new(::apis::AppApiClient::new(rc.clone())),
      community_content_api: Box::new(::apis::CommunityContentApiClient::new(rc.clone())),
      content_api: Box::new(::apis::ContentApiClient::new(rc.clone())),
      destiny2_api: Box::new(::apis::Destiny2ApiClient::new(rc.clone())),
      fireteam_api: Box::new(::apis::FireteamApiClient::new(rc.clone())),
      forum_api: Box::new(::apis::ForumApiClient::new(rc.clone())),
      group_v2_api: Box::new(::apis::GroupV2ApiClient::new(rc.clone())),
      preview_api: Box::new(::apis::PreviewApiClient::new(rc.clone())),
      tokens_api: Box::new(::apis::TokensApiClient::new(rc.clone())),
      trending_api: Box::new(::apis::TrendingApiClient::new(rc.clone())),
      user_api: Box::new(::apis::UserApiClient::new(rc.clone())),
    }
  }

  pub fn _api(&self) -> &::apis::DefaultApi{
    self._api.as_ref()
  }

  pub fn app_api(&self) -> &::apis::AppApi{
    self.app_api.as_ref()
  }

  pub fn community_content_api(&self) -> &::apis::CommunityContentApi{
    self.community_content_api.as_ref()
  }

  pub fn content_api(&self) -> &::apis::ContentApi{
    self.content_api.as_ref()
  }

  pub fn destiny2_api(&self) -> &::apis::Destiny2Api{
    self.destiny2_api.as_ref()
  }

  pub fn fireteam_api(&self) -> &::apis::FireteamApi{
    self.fireteam_api.as_ref()
  }

  pub fn forum_api(&self) -> &::apis::ForumApi{
    self.forum_api.as_ref()
  }

  pub fn group_v2_api(&self) -> &::apis::GroupV2Api{
    self.group_v2_api.as_ref()
  }

  pub fn preview_api(&self) -> &::apis::PreviewApi{
    self.preview_api.as_ref()
  }

  pub fn tokens_api(&self) -> &::apis::TokensApi{
    self.tokens_api.as_ref()
  }

  pub fn trending_api(&self) -> &::apis::TrendingApi{
    self.trending_api.as_ref()
  }

  pub fn user_api(&self) -> &::apis::UserApi{
    self.user_api.as_ref()
  }


}
