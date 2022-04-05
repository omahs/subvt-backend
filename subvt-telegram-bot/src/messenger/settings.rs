use crate::query::{QueryType, SettingsEditQueryType, SettingsSubSection};
use crate::{Messenger, Query};
use frankenstein::{InlineKeyboardButton, InlineKeyboardMarkup};
use subvt_types::app::{NotificationPeriodType, NotificationTypeCode, UserNotificationRule};
use tera::Context;

impl Messenger {
    fn get_settings_button(
        &self,
        template_file_name: &str,
        query_type: QueryType,
    ) -> anyhow::Result<Vec<InlineKeyboardButton>> {
        Ok(vec![InlineKeyboardButton {
            text: self.renderer.render(template_file_name, &Context::new())?,
            url: None,
            login_url: None,
            callback_data: Some(serde_json::to_string(&Query {
                query_type,
                parameter: None,
            })?),
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }])
    }

    pub fn get_settings_keyboard(&self) -> anyhow::Result<InlineKeyboardMarkup> {
        let rows = vec![
            self.get_settings_button(
                "settings_validator_activity.html",
                QueryType::SettingsNavigate(SettingsSubSection::ValidatorActivity),
            )?,
            self.get_settings_button(
                "settings_nominations.html",
                QueryType::SettingsNavigate(SettingsSubSection::Nominations),
            )?,
            self.get_settings_button(
                "settings_democracy.html",
                QueryType::SettingsNavigate(SettingsSubSection::Democracy),
            )?,
            self.get_settings_button(
                "settings_onekv.html",
                QueryType::SettingsNavigate(SettingsSubSection::OneKV),
            )?,
            self.get_settings_button("cancel.html", QueryType::Cancel)?,
        ];
        Ok(InlineKeyboardMarkup {
            inline_keyboard: rows,
        })
    }
}

impl Messenger {
    fn get_notification_on_off_button(
        &self,
        notification_type_code: NotificationTypeCode,
        template_file_name: &str,
        edit_type: SettingsEditQueryType,
        notification_rules: &[UserNotificationRule],
    ) -> anyhow::Result<Option<Vec<InlineKeyboardButton>>> {
        if let Some(rule) = notification_rules
            .iter()
            .find(|rule| rule.notification_type.code == notification_type_code.to_string())
        {
            let is_on = rule.period_type == NotificationPeriodType::Immediate;
            let mut context = Context::new();
            context.insert("is_on", &is_on);
            Ok(Some(vec![InlineKeyboardButton {
                text: self.renderer.render(template_file_name, &context)?,
                url: None,
                login_url: None,
                callback_data: Some(serde_json::to_string(&Query {
                    query_type: QueryType::SettingsEdit(edit_type),
                    parameter: Some(serde_json::to_string(&!is_on)?),
                })?),
                switch_inline_query: None,
                switch_inline_query_current_chat: None,
                callback_game: None,
                pay: None,
            }]))
        } else {
            Ok(None)
        }
    }

    fn get_notification_period_button(
        &self,
        notification_type_code: NotificationTypeCode,
        edit_type: SettingsEditQueryType,
        period_type: NotificationPeriodType,
        period: u16,
        notification_rules: &[UserNotificationRule],
    ) -> anyhow::Result<Option<Vec<InlineKeyboardButton>>> {
        if let Some(rule) = notification_rules
            .iter()
            .find(|rule| rule.notification_type.code == notification_type_code.to_string())
        {
            let is_selected = rule.period_type == period_type && rule.period == period;
            let mut context = Context::new();
            context.insert("is_selected", &is_selected);
            context.insert("period_type", &period_type.to_string());
            context.insert("period", &period);
            let parameter = (period_type, period);
            Ok(Some(vec![InlineKeyboardButton {
                text: self
                    .renderer
                    .render("settings_item_notification_period.html", &context)?,
                url: None,
                login_url: None,
                callback_data: Some(serde_json::to_string(&Query {
                    query_type: QueryType::SettingsEdit(edit_type),
                    parameter: Some(serde_json::to_string(&parameter)?),
                })?),
                switch_inline_query: None,
                switch_inline_query_current_chat: None,
                callback_game: None,
                pay: None,
            }]))
        } else {
            Ok(None)
        }
    }

    pub fn get_validator_activity_settings_keyboard(
        &self,
        notification_rules: &[UserNotificationRule],
    ) -> anyhow::Result<InlineKeyboardMarkup> {
        let mut rows = vec![self.get_settings_button(
            "settings_item_block_authorship.html",
            QueryType::SettingsNavigate(SettingsSubSection::BlockAuthorship),
        )?];
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::ChainValidatorChilled,
            "settings_item_chilled.html",
            SettingsEditQueryType::Chilled,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::ChainValidatorSetController,
            "settings_item_set_controller.html",
            SettingsEditQueryType::SetController,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::ChainValidatorIdentityChanged,
            "settings_item_id_changed.html",
            SettingsEditQueryType::IdentityChanged,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::ChainValidatorActive,
            "settings_item_active.html",
            SettingsEditQueryType::Active,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::ChainValidatorActiveNextSession,
            "settings_item_active_next_session.html",
            SettingsEditQueryType::ActiveNextSession,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::ChainValidatorInactive,
            "settings_item_inactive.html",
            SettingsEditQueryType::Inactive,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::ChainValidatorInactiveNextSession,
            "settings_item_inactive_next_session.html",
            SettingsEditQueryType::InactiveNextSession,
            notification_rules,
        )? {
            rows.push(item);
        }

        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::ChainValidatorOfflineOffence,
            "settings_item_offline_offence.html",
            SettingsEditQueryType::OfflineOffence,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::ChainValidatorPayoutStakers,
            "settings_item_payout_stakers.html",
            SettingsEditQueryType::PayoutStakers,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::ChainValidatorSessionKeysChanged,
            "settings_item_session_keys_changed.html",
            SettingsEditQueryType::SessionKeysChanged,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::ChainValidatorUnclaimedPayout,
            "settings_item_unclaimed_payout.html",
            SettingsEditQueryType::UnclaimedPayout,
            notification_rules,
        )? {
            rows.push(item);
        }

        rows.push(self.get_settings_button(
            "back.html",
            QueryType::SettingsNavigate(SettingsSubSection::Root),
        )?);
        rows.push(self.get_settings_button("cancel.html", QueryType::Cancel)?);
        Ok(InlineKeyboardMarkup {
            inline_keyboard: rows,
        })
    }

    pub fn get_period_settings_keyboard(
        &self,
        edit_type: SettingsEditQueryType,
        notification_type_code: NotificationTypeCode,
        back_target: SettingsSubSection,
        notification_rules: &[UserNotificationRule],
    ) -> anyhow::Result<InlineKeyboardMarkup> {
        let mut rows = vec![];
        if let Some(button) = self.get_notification_period_button(
            notification_type_code,
            edit_type,
            NotificationPeriodType::Off,
            0,
            notification_rules,
        )? {
            rows.push(button);
        };
        if let Some(button) = self.get_notification_period_button(
            notification_type_code,
            edit_type,
            NotificationPeriodType::Immediate,
            0,
            notification_rules,
        )? {
            rows.push(button);
        };
        if let Some(button) = self.get_notification_period_button(
            notification_type_code,
            edit_type,
            NotificationPeriodType::Hour,
            1,
            notification_rules,
        )? {
            rows.push(button);
        };
        if let Some(button) = self.get_notification_period_button(
            notification_type_code,
            edit_type,
            NotificationPeriodType::Hour,
            2,
            notification_rules,
        )? {
            rows.push(button);
        };
        if let Some(button) = self.get_notification_period_button(
            notification_type_code,
            edit_type,
            NotificationPeriodType::Epoch,
            3,
            notification_rules,
        )? {
            rows.push(button);
        };
        if let Some(button) = self.get_notification_period_button(
            notification_type_code,
            edit_type,
            NotificationPeriodType::Era,
            1,
            notification_rules,
        )? {
            rows.push(button);
        };

        rows.push(self.get_settings_button("back.html", QueryType::SettingsNavigate(back_target))?);
        rows.push(self.get_settings_button("cancel.html", QueryType::Cancel)?);
        Ok(InlineKeyboardMarkup {
            inline_keyboard: rows,
        })
    }

    pub fn get_nomination_settings_keyboard(&self) -> anyhow::Result<InlineKeyboardMarkup> {
        let rows = vec![
            self.get_settings_button(
                "settings_new_nomination.html",
                QueryType::SettingsNavigate(SettingsSubSection::NewNomination),
            )?,
            self.get_settings_button(
                "settings_lost_nomination.html",
                QueryType::SettingsNavigate(SettingsSubSection::LostNomination),
            )?,
            self.get_settings_button(
                "back.html",
                QueryType::SettingsNavigate(SettingsSubSection::Root),
            )?,
            self.get_settings_button("cancel.html", QueryType::Cancel)?,
        ];
        Ok(InlineKeyboardMarkup {
            inline_keyboard: rows,
        })
    }

    pub fn get_democracy_settings_keyboard(
        &self,
        notification_rules: &[UserNotificationRule],
    ) -> anyhow::Result<InlineKeyboardMarkup> {
        let mut rows = vec![];
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::DemocracyProposed,
            "settings_item_democracy_proposed.html",
            SettingsEditQueryType::DemocracyProposed,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::DemocracySeconded,
            "settings_item_democracy_seconded.html",
            SettingsEditQueryType::DemocracySeconded,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::DemocracyStarted,
            "settings_item_democracy_started.html",
            SettingsEditQueryType::DemocracyStarted,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::DemocracyCancelled,
            "settings_item_democracy_cancelled.html",
            SettingsEditQueryType::DemocracyCancelled,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::DemocracyPassed,
            "settings_item_democracy_passed.html",
            SettingsEditQueryType::DemocracyPassed,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::DemocracyNotPassed,
            "settings_item_democracy_not_passed.html",
            SettingsEditQueryType::DemocracyNotPassed,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::DemocracyVoted,
            "settings_item_democracy_voted.html",
            SettingsEditQueryType::DemocracyVoted,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::DemocracyDelegated,
            "settings_item_democracy_delegated.html",
            SettingsEditQueryType::DemocracyDelegated,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::DemocracyUndelegated,
            "settings_item_democracy_undelegated.html",
            SettingsEditQueryType::DemocracyUndelegated,
            notification_rules,
        )? {
            rows.push(item);
        }

        rows.push(self.get_settings_button(
            "back.html",
            QueryType::SettingsNavigate(SettingsSubSection::Root),
        )?);
        rows.push(self.get_settings_button("cancel.html", QueryType::Cancel)?);
        Ok(InlineKeyboardMarkup {
            inline_keyboard: rows,
        })
    }

    pub fn get_onekv_settings_keyboard(
        &self,
        notification_rules: &[UserNotificationRule],
    ) -> anyhow::Result<InlineKeyboardMarkup> {
        let mut rows = vec![];
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::OneKVValidatorRankChange,
            "settings_item_onekv_rank_change.html",
            SettingsEditQueryType::OneKVRankChange,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::OneKVValidatorBinaryVersionChange,
            "settings_item_onekv_binary_version_change.html",
            SettingsEditQueryType::OneKVBinaryVersionChange,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::OneKVValidatorValidityChange,
            "settings_item_onekv_validity_change.html",
            SettingsEditQueryType::OneKVValidityChange,
            notification_rules,
        )? {
            rows.push(item);
        }
        if let Some(item) = self.get_notification_on_off_button(
            NotificationTypeCode::OneKVValidatorLocationChange,
            "settings_item_onekv_location_change.html",
            SettingsEditQueryType::OneKVLocationChange,
            notification_rules,
        )? {
            rows.push(item);
        }

        rows.push(self.get_settings_button(
            "back.html",
            QueryType::SettingsNavigate(SettingsSubSection::Root),
        )?);
        rows.push(self.get_settings_button("cancel.html", QueryType::Cancel)?);
        Ok(InlineKeyboardMarkup {
            inline_keyboard: rows,
        })
    }
}
