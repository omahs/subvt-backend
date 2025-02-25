use crate::{NotificationGenerator, CONFIG};
use std::sync::Arc;
use subvt_persistence::postgres::app::PostgreSQLAppStorage;
use subvt_persistence::postgres::network::PostgreSQLNetworkStorage;
use subvt_types::app::{notification::NotificationTypeCode, Block};

impl NotificationGenerator {
    pub(crate) async fn inspect_set_controller_extrinsics(
        &self,
        network_postgres: Arc<PostgreSQLNetworkStorage>,
        app_postgres: Arc<PostgreSQLAppStorage>,
        block: &Block,
    ) -> anyhow::Result<()> {
        log::debug!(
            "Inspect block #{} for controller change extrinsics.",
            block.number
        );
        for extrinsic in network_postgres
            .get_set_controller_extrinsics_in_block(&block.hash)
            .await?
        {
            let rules = app_postgres
                .get_notification_rules_for_validator(
                    &NotificationTypeCode::ChainValidatorSetController.to_string(),
                    CONFIG.substrate.network_id,
                    &extrinsic.caller_account_id,
                )
                .await?;
            self.generate_notifications(
                app_postgres.clone(),
                &rules,
                block.number,
                &Some(extrinsic.caller_account_id),
                Some(&extrinsic.clone()),
            )
            .await?;
        }
        Ok(())
    }
}
