//
//

//!

use crate::models::prelude::*;

mod status;
pub use self::status::Status;

pub struct Profile;

#[derive(Clone, Debug, PartialEq, PartialOrd, Builder,
  Deserialize, Serialize)]
pub struct User<'re> {
  avatar: Option<CowStr<'re>>,
  #[serde(default)]
  bot: bool,
  discriminator: u16,
  email: Option<CowStr<'re>>,
  id: Snowflake,
  mfa_enabled: bool,
  username: CowStr<'re>,
  verified: bool
}

impl<'re> User<'re> {
  pub fn avatar_url(&self) -> Option<CowStr<'re>> {
    unimplemented!();
  }

  pub fn default_avatar_url(&self) -> CowStr<'re> {
    unimplemented!();
  }

  pub fn edit<F>(&mut self, http: impl AsRef<HttpClient<'re>>, f: F)
    -> Result<()> where F: FnOnce(&mut Profile) -> Result<&mut Profile>
  {
    let mut m = HashMap::new();
    m.insert("username", Value::String(self.username.to_string()));

    if let Some(email) = self.email {
      m.insert("email", Value::String(email.to_string()));
    }

    let mut new_profile = Profile::from(m);

    f(&mut new_profile)?;

    // let m = ;
    match http.as_ref().edit_profile(&m) {
      Ok(new_profile_recv) => {
        let _ = std::mem::replace(self, new_profile_recv);
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}
