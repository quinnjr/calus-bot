/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokensPartnerOfferHistoryResponse {
  #[serde(rename = "PartnerOfferKey")]
  partner_offer_key: Option<String>,
  #[serde(rename = "MembershipId")]
  membership_id: Option<i64>,
  #[serde(rename = "MembershipType")]
  membership_type: Option<i32>,
  #[serde(rename = "LocalizedName")]
  localized_name: Option<String>,
  #[serde(rename = "LocalizedDescription")]
  localized_description: Option<String>,
  #[serde(rename = "IsConsumable")]
  is_consumable: Option<bool>,
  #[serde(rename = "QuantityApplied")]
  quantity_applied: Option<i32>,
  #[serde(rename = "ApplyDate")]
  apply_date: Option<String>
}

impl TokensPartnerOfferHistoryResponse {
  pub fn new() -> TokensPartnerOfferHistoryResponse {
    TokensPartnerOfferHistoryResponse {
      partner_offer_key: None,
      membership_id: None,
      membership_type: None,
      localized_name: None,
      localized_description: None,
      is_consumable: None,
      quantity_applied: None,
      apply_date: None
    }
  }

  pub fn set_partner_offer_key(&mut self, partner_offer_key: String) {
    self.partner_offer_key = Some(partner_offer_key);
  }

  pub fn with_partner_offer_key(mut self, partner_offer_key: String) -> TokensPartnerOfferHistoryResponse {
    self.partner_offer_key = Some(partner_offer_key);
    self
  }

  pub fn partner_offer_key(&self) -> Option<&String> {
    self.partner_offer_key.as_ref()
  }

  pub fn reset_partner_offer_key(&mut self) {
    self.partner_offer_key = None;
  }

  pub fn set_membership_id(&mut self, membership_id: i64) {
    self.membership_id = Some(membership_id);
  }

  pub fn with_membership_id(mut self, membership_id: i64) -> TokensPartnerOfferHistoryResponse {
    self.membership_id = Some(membership_id);
    self
  }

  pub fn membership_id(&self) -> Option<&i64> {
    self.membership_id.as_ref()
  }

  pub fn reset_membership_id(&mut self) {
    self.membership_id = None;
  }

  pub fn set_membership_type(&mut self, membership_type: i32) {
    self.membership_type = Some(membership_type);
  }

  pub fn with_membership_type(mut self, membership_type: i32) -> TokensPartnerOfferHistoryResponse {
    self.membership_type = Some(membership_type);
    self
  }

  pub fn membership_type(&self) -> Option<&i32> {
    self.membership_type.as_ref()
  }

  pub fn reset_membership_type(&mut self) {
    self.membership_type = None;
  }

  pub fn set_localized_name(&mut self, localized_name: String) {
    self.localized_name = Some(localized_name);
  }

  pub fn with_localized_name(mut self, localized_name: String) -> TokensPartnerOfferHistoryResponse {
    self.localized_name = Some(localized_name);
    self
  }

  pub fn localized_name(&self) -> Option<&String> {
    self.localized_name.as_ref()
  }

  pub fn reset_localized_name(&mut self) {
    self.localized_name = None;
  }

  pub fn set_localized_description(&mut self, localized_description: String) {
    self.localized_description = Some(localized_description);
  }

  pub fn with_localized_description(mut self, localized_description: String) -> TokensPartnerOfferHistoryResponse {
    self.localized_description = Some(localized_description);
    self
  }

  pub fn localized_description(&self) -> Option<&String> {
    self.localized_description.as_ref()
  }

  pub fn reset_localized_description(&mut self) {
    self.localized_description = None;
  }

  pub fn set_is_consumable(&mut self, is_consumable: bool) {
    self.is_consumable = Some(is_consumable);
  }

  pub fn with_is_consumable(mut self, is_consumable: bool) -> TokensPartnerOfferHistoryResponse {
    self.is_consumable = Some(is_consumable);
    self
  }

  pub fn is_consumable(&self) -> Option<&bool> {
    self.is_consumable.as_ref()
  }

  pub fn reset_is_consumable(&mut self) {
    self.is_consumable = None;
  }

  pub fn set_quantity_applied(&mut self, quantity_applied: i32) {
    self.quantity_applied = Some(quantity_applied);
  }

  pub fn with_quantity_applied(mut self, quantity_applied: i32) -> TokensPartnerOfferHistoryResponse {
    self.quantity_applied = Some(quantity_applied);
    self
  }

  pub fn quantity_applied(&self) -> Option<&i32> {
    self.quantity_applied.as_ref()
  }

  pub fn reset_quantity_applied(&mut self) {
    self.quantity_applied = None;
  }

  pub fn set_apply_date(&mut self, apply_date: String) {
    self.apply_date = Some(apply_date);
  }

  pub fn with_apply_date(mut self, apply_date: String) -> TokensPartnerOfferHistoryResponse {
    self.apply_date = Some(apply_date);
    self
  }

  pub fn apply_date(&self) -> Option<&String> {
    self.apply_date.as_ref()
  }

  pub fn reset_apply_date(&mut self) {
    self.apply_date = None;
  }

}



