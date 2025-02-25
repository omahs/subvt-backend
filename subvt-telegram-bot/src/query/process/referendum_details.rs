use crate::query::Query;
use crate::{MessageType, Messenger, TelegramBot, CONFIG};
use subvt_governance::polkassembly;
use subvt_substrate_client::SubstrateClient;
use subvt_types::substrate::democracy::ReferendumVote;
use subvt_types::telegram::TelegramChatValidator;

impl<M: Messenger + Send + Sync> TelegramBot<M> {
    pub(crate) async fn process_referendum_details_query(
        &self,
        chat_id: i64,
        original_message_id: Option<i32>,
        query: &Query,
    ) -> anyhow::Result<()> {
        if let Some(message_id) = original_message_id {
            self.messenger.delete_message(chat_id, message_id).await?;
        }
        if let Some(id_str) = &query.parameter {
            let referendum_index: u32 = id_str.parse()?;
            if let Some(post) = polkassembly::fetch_referendum_details(referendum_index).await? {
                let chat_validators = self.network_postgres.get_chat_validators(chat_id).await?;
                let mut chat_validator_votes: Vec<(TelegramChatValidator, Option<ReferendumVote>)> =
                    vec![];
                let substrate_client = SubstrateClient::new(&CONFIG).await?;
                for chat_validator in &chat_validators {
                    let vote = substrate_client
                        .get_account_referendum_vote(
                            &chat_validator.account_id,
                            referendum_index,
                            None,
                        )
                        .await?;
                    chat_validator_votes.push((chat_validator.clone(), vote));
                }
                self.messenger
                    .send_message(
                        &self.app_postgres,
                        &self.network_postgres,
                        chat_id,
                        Box::new(MessageType::ReferendumDetails {
                            post,
                            chat_validator_votes,
                        }),
                    )
                    .await?;
            } else {
                self.messenger
                    .send_message(
                        &self.app_postgres,
                        &self.network_postgres,
                        chat_id,
                        Box::new(MessageType::ReferendumNotFound(referendum_index)),
                    )
                    .await?;
            }
        }
        Ok(())
    }
}
