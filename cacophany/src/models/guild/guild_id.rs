//
//

//!

use crate::{
  http::Client,
  models::prelude::*
};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Id<'re> {
  gid: usize,
  http_client: Option<&'re Client<'re>>
}

impl<'re> Id<'re> {
  pub fn ban(self, client: AsRef<()>, user: impl Into<UserId>,
    options: BanOptions) -> Result<()>
  {
    let (dmd, reason) = options;

    if dmd > 7 {

    }

    if reason.len() > 512 {

    }


  }

  pub fn bans(self, client: impl AsRef<()>) -> Result<()> {
    Ok(())
  }

  pub fn audit_logs(self, client: impl AsRef<()>,
    action: Option<u8>, user_id: Option<UserId>,
    before: Option<()>, limit: Option<u8>) -> Result<AuditLogs>
  {

  }
}
