use crate::domain::object::card::Card;
use crate::domain::object::card::CardId;
use crate::domain::repository::{account::AccountRepository, card::CardRepository};
use anyhow;
pub fn post_card(
    card_repository: &mut impl CardRepository,
    account_repository: &mut impl AccountRepository,
    card: &Card,
) -> anyhow::Result<()> {
    let account_id = &card.account_id;
    match account_repository.find_by_id(account_id) {
        Ok(_) => {
            card_repository.insert(card)?;
            Ok(())
        }
        Err(err) => Err(anyhow::anyhow!("Failed to find account: {}", err)),
    }
}
