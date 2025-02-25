use crate::postgres::network::PostgreSQLNetworkStorage;
use subvt_types::crypto::AccountId;
use subvt_types::substrate::{Balance, Era};

impl PostgreSQLNetworkStorage {
    pub async fn get_validator_all_era_rewards(
        &self,
        validator_account_id: &AccountId,
    ) -> anyhow::Result<Vec<(Era, Balance)>> {
        let era_rewards: Vec<(i64, i64, i64, i64)> = sqlx::query_as(
            r#"
            SELECT E.index, E.start_timestamp, E.end_timestamp, SUM(EV.amount::bigint)::bigint
            FROM sub_event_rewarded EV
            INNER JOIN sub_extrinsic_payout_stakers EX
                ON EV.block_hash = EX.block_hash
                AND EV.extrinsic_index = EX.extrinsic_index
                AND COALESCE(EV.nesting_index, '') = COALESCE(EX.nesting_index, '')
                AND EX.validator_account_id = EV.rewardee_account_id
            INNER JOIN sub_era E
                ON E.index = EX.era_index
            WHERE EV.rewardee_account_id = $1
            GROUP BY E.index
            ORDER BY E.index ASC;
            "#,
        )
        .bind(validator_account_id.to_string())
        .fetch_all(&self.connection_pool)
        .await?;
        let mut result = vec![];
        for era_reward in era_rewards {
            result.push((
                Era {
                    index: era_reward.0 as u32,
                    start_timestamp: era_reward.1 as u64,
                    end_timestamp: era_reward.1 as u64,
                },
                era_reward.3 as Balance,
            ))
        }
        Ok(result)
    }
}
