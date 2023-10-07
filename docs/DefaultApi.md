# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**absences_absence_reasons_get**](DefaultApi.md#absences_absence_reasons_get) | **GET** /absences/absence-reasons | Get all absence reasons
[**absences_absence_reasons_id_delete**](DefaultApi.md#absences_absence_reasons_id_delete) | **DELETE** /absences/absence-reasons/{id} | Delete absence reason
[**absences_absence_reasons_id_get**](DefaultApi.md#absences_absence_reasons_id_get) | **GET** /absences/absence-reasons/{id} | Get absence reason
[**absences_absence_reasons_id_patch**](DefaultApi.md#absences_absence_reasons_id_patch) | **PATCH** /absences/absence-reasons/{id} | Update absence reason
[**absences_absence_reasons_post**](DefaultApi.md#absences_absence_reasons_post) | **POST** /absences/absence-reasons | Create absence reason
[**absences_get**](DefaultApi.md#absences_get) | **GET** /absences | Get all absences
[**absences_id_approve_post**](DefaultApi.md#absences_id_approve_post) | **POST** /absences/{id}/approve | Approve absence
[**absences_id_delete**](DefaultApi.md#absences_id_delete) | **DELETE** /absences/{id} | Delete absence
[**absences_id_delete_approval_delete**](DefaultApi.md#absences_id_delete_approval_delete) | **DELETE** /absences/{id}/delete-approval | Delete absence approval
[**absences_id_get**](DefaultApi.md#absences_id_get) | **GET** /absences/{id} | Get absence
[**absences_id_patch**](DefaultApi.md#absences_id_patch) | **PATCH** /absences/{id} | Update absence
[**absences_membermember_get**](DefaultApi.md#absences_membermember_get) | **GET** /absences/member:{member} | Get all absences by member
[**absences_post**](DefaultApi.md#absences_post) | **POST** /absences | Create absence
[**administration_support_post**](DefaultApi.md#administration_support_post) | **POST** /administration/support | Create support
[**administration_user_agreement_enabled_get**](DefaultApi.md#administration_user_agreement_enabled_get) | **GET** /administration/user-agreement/enabled | Is user agreement enabled?
[**administration_user_agreement_enabled_post**](DefaultApi.md#administration_user_agreement_enabled_post) | **POST** /administration/user-agreement/enabled | Enable / disable user agreement
[**administration_user_agreement_get**](DefaultApi.md#administration_user_agreement_get) | **GET** /administration/user-agreement | Get user agreement
[**administration_user_agreement_patch**](DefaultApi.md#administration_user_agreement_patch) | **PATCH** /administration/user-agreement | Upload new user agreement
[**administration_user_agreement_status_get**](DefaultApi.md#administration_user_agreement_status_get) | **GET** /administration/user-agreement/status | Get all user agreement statuses
[**administration_user_agreement_status_profile_get**](DefaultApi.md#administration_user_agreement_status_profile_get) | **GET** /administration/user-agreement/status/{profile} | Get user agreement status for a profile
[**applications_application_authorizations_authorized_contexts_get**](DefaultApi.md#applications_application_authorizations_authorized_contexts_get) | **GET** /applications/{application}/authorizations/authorized-contexts | Get all authorized contexts
[**applications_application_authorizations_authorized_rights_delete**](DefaultApi.md#applications_application_authorizations_authorized_rights_delete) | **DELETE** /applications/{application}/authorizations/authorized-rights | Delete authorized right
[**applications_application_authorizations_authorized_rights_get**](DefaultApi.md#applications_application_authorizations_authorized_rights_get) | **GET** /applications/{application}/authorizations/authorized-rights | Get all authorized rights
[**applications_application_authorizations_authorized_rights_patch**](DefaultApi.md#applications_application_authorizations_authorized_rights_patch) | **PATCH** /applications/{application}/authorizations/authorized-rights | Update authorized right
[**applications_application_authorizations_authorized_rights_request_rights_patch**](DefaultApi.md#applications_application_authorizations_authorized_rights_request_rights_patch) | **PATCH** /applications/{application}/authorizations/authorized-rights/request-rights | Request Rights
[**applications_application_authorizations_required_rights_get**](DefaultApi.md#applications_application_authorizations_required_rights_get) | **GET** /applications/{application}/authorizations/required-rights | Get all required rights
[**applications_application_authorizations_required_rights_patch**](DefaultApi.md#applications_application_authorizations_required_rights_patch) | **PATCH** /applications/{application}/authorizations/required-rights | Update required right
[**applications_application_bearer_token_get**](DefaultApi.md#applications_application_bearer_token_get) | **GET** /applications/{application}/bearer-token | Bearer Token
[**applications_application_client_secret_get**](DefaultApi.md#applications_application_client_secret_get) | **GET** /applications/{application}/client-secret | Get client secret
[**applications_application_client_secret_regenerate_post**](DefaultApi.md#applications_application_client_secret_regenerate_post) | **POST** /applications/{application}/client-secret/regenerate | Regenerate app secret
[**applications_application_delete**](DefaultApi.md#applications_application_delete) | **DELETE** /applications/{application} | Delete application
[**applications_application_force_remove_post**](DefaultApi.md#applications_application_force_remove_post) | **POST** /applications/{application}/force-remove | Force-remove application
[**applications_application_get**](DefaultApi.md#applications_application_get) | **GET** /applications/{application} | Get application
[**applications_application_gpg_keys_fingerprint_delete**](DefaultApi.md#applications_application_gpg_keys_fingerprint_delete) | **DELETE** /applications/{application}/gpg-keys/{fingerprint} | Delete GPG key
[**applications_application_gpg_keys_fingerprint_patch**](DefaultApi.md#applications_application_gpg_keys_fingerprint_patch) | **PATCH** /applications/{application}/gpg-keys/{fingerprint} | Revoke GPG key
[**applications_application_gpg_keys_get**](DefaultApi.md#applications_application_gpg_keys_get) | **GET** /applications/{application}/gpg-keys | Get GPG keys
[**applications_application_gpg_keys_post**](DefaultApi.md#applications_application_gpg_keys_post) | **POST** /applications/{application}/gpg-keys | Add GPG key
[**applications_application_last_client_credentials_access_get**](DefaultApi.md#applications_application_last_client_credentials_access_get) | **GET** /applications/{application}/last-client-credentials-access | Get last client credentials access info
[**applications_application_patch**](DefaultApi.md#applications_application_patch) | **PATCH** /applications/{application} | Update application
[**applications_application_permanent_tokens_current_delete**](DefaultApi.md#applications_application_permanent_tokens_current_delete) | **DELETE** /applications/{application}/permanent-tokens/current | Delete current permanent token
[**applications_application_permanent_tokens_get**](DefaultApi.md#applications_application_permanent_tokens_get) | **GET** /applications/{application}/permanent-tokens | Get all permanent tokens
[**applications_application_permanent_tokens_post**](DefaultApi.md#applications_application_permanent_tokens_post) | **POST** /applications/{application}/permanent-tokens | Create permanent token
[**applications_application_permanent_tokens_token_id_delete**](DefaultApi.md#applications_application_permanent_tokens_token_id_delete) | **DELETE** /applications/{application}/permanent-tokens/{tokenId} | Delete permanent token
[**applications_application_permanent_tokens_token_id_patch**](DefaultApi.md#applications_application_permanent_tokens_token_id_patch) | **PATCH** /applications/{application}/permanent-tokens/{tokenId} | Update permanent token
[**applications_application_public_keys_get**](DefaultApi.md#applications_application_public_keys_get) | **GET** /applications/{application}/public-keys | Public Keys
[**applications_application_restore_post**](DefaultApi.md#applications_application_restore_post) | **POST** /applications/{application}/restore | Restore application
[**applications_application_signing_key_get**](DefaultApi.md#applications_application_signing_key_get) | **GET** /applications/{application}/signing-key | Get signing key
[**applications_application_signing_key_regenerate_post**](DefaultApi.md#applications_application_signing_key_regenerate_post) | **POST** /applications/{application}/signing-key/regenerate | Regenerate signing key
[**applications_application_ssh_keys_fingerprint_delete**](DefaultApi.md#applications_application_ssh_keys_fingerprint_delete) | **DELETE** /applications/{application}/ssh-keys/{fingerprint} | Delete SSH key
[**applications_application_ssh_keys_get**](DefaultApi.md#applications_application_ssh_keys_get) | **GET** /applications/{application}/ssh-keys | Get SSH keys
[**applications_application_ssh_keys_post**](DefaultApi.md#applications_application_ssh_keys_post) | **POST** /applications/{application}/ssh-keys | Add SSH key
[**applications_application_ui_extensions_disable_for_everybody_patch**](DefaultApi.md#applications_application_ui_extensions_disable_for_everybody_patch) | **PATCH** /applications/{application}/ui-extensions/disable-for-everybody | Disable application UI
[**applications_application_ui_extensions_disable_for_me_patch**](DefaultApi.md#applications_application_ui_extensions_disable_for_me_patch) | **PATCH** /applications/{application}/ui-extensions/disable-for-me | Disable application UI for me
[**applications_application_ui_extensions_enable_for_everybody_patch**](DefaultApi.md#applications_application_ui_extensions_enable_for_everybody_patch) | **PATCH** /applications/{application}/ui-extensions/enable-for-everybody | Enable application UI
[**applications_application_ui_extensions_enable_for_me_patch**](DefaultApi.md#applications_application_ui_extensions_enable_for_me_patch) | **PATCH** /applications/{application}/ui-extensions/enable-for-me | Enable application UI for me
[**applications_application_ui_extensions_get**](DefaultApi.md#applications_application_ui_extensions_get) | **GET** /applications/{application}/ui-extensions | Get UI extensions
[**applications_application_unfurl_domains_authorize_post**](DefaultApi.md#applications_application_unfurl_domains_authorize_post) | **POST** /applications/{application}/unfurl-domains/authorize | Authorize unfurled domains
[**applications_application_unfurl_domains_get**](DefaultApi.md#applications_application_unfurl_domains_get) | **GET** /applications/{application}/unfurl-domains | Get all unfurl domains
[**applications_application_unfurl_patterns_authorize_post**](DefaultApi.md#applications_application_unfurl_patterns_authorize_post) | **POST** /applications/{application}/unfurl-patterns/authorize | Authorize unfurled patterns
[**applications_application_unfurl_patterns_get**](DefaultApi.md#applications_application_unfurl_patterns_get) | **GET** /applications/{application}/unfurl-patterns | Get all unfurl patterns
[**applications_application_verification_token_get**](DefaultApi.md#applications_application_verification_token_get) | **GET** /applications/{application}/verification-token | Get verification token
[**applications_application_verification_token_regenerate_post**](DefaultApi.md#applications_application_verification_token_regenerate_post) | **POST** /applications/{application}/verification-token/regenerate | Regenerate verification token
[**applications_application_webhooks_get**](DefaultApi.md#applications_application_webhooks_get) | **GET** /applications/{application}/webhooks | Get all webhooks
[**applications_application_webhooks_post**](DefaultApi.md#applications_application_webhooks_post) | **POST** /applications/{application}/webhooks | Create webhook
[**applications_application_webhooks_webhook_id_bearer_token_get**](DefaultApi.md#applications_application_webhooks_webhook_id_bearer_token_get) | **GET** /applications/{application}/webhooks/{webhookId}/bearer-token | Bearer Token
[**applications_application_webhooks_webhook_id_custom_headers_get**](DefaultApi.md#applications_application_webhooks_webhook_id_custom_headers_get) | **GET** /applications/{application}/webhooks/{webhookId}/custom-headers | Get custom header
[**applications_application_webhooks_webhook_id_custom_headers_post**](DefaultApi.md#applications_application_webhooks_webhook_id_custom_headers_post) | **POST** /applications/{application}/webhooks/{webhookId}/custom-headers | Post custom header
[**applications_application_webhooks_webhook_id_delete**](DefaultApi.md#applications_application_webhooks_webhook_id_delete) | **DELETE** /applications/{application}/webhooks/{webhookId} | Delete webhook
[**applications_application_webhooks_webhook_id_patch**](DefaultApi.md#applications_application_webhooks_webhook_id_patch) | **PATCH** /applications/{application}/webhooks/{webhookId} | Update webhook
[**applications_application_webhooks_webhook_id_post**](DefaultApi.md#applications_application_webhooks_webhook_id_post) | **POST** /applications/{application}/webhooks/{webhookId} | Post webhook
[**applications_application_webhooks_webhook_id_signing_key_get**](DefaultApi.md#applications_application_webhooks_webhook_id_signing_key_get) | **GET** /applications/{application}/webhooks/{webhookId}/signing-key | Get signing key
[**applications_application_webhooks_webhook_id_signing_key_regenerate_post**](DefaultApi.md#applications_application_webhooks_webhook_id_signing_key_regenerate_post) | **POST** /applications/{application}/webhooks/{webhookId}/signing-key/regenerate | Regenerate
[**applications_application_webhooks_webhook_id_subscriptions_get**](DefaultApi.md#applications_application_webhooks_webhook_id_subscriptions_get) | **GET** /applications/{application}/webhooks/{webhookId}/subscriptions | Get all subscriptions
[**applications_application_webhooks_webhook_id_subscriptions_post**](DefaultApi.md#applications_application_webhooks_webhook_id_subscriptions_post) | **POST** /applications/{application}/webhooks/{webhookId}/subscriptions | Create subscription
[**applications_application_webhooks_webhook_id_subscriptions_subscription_id_delete**](DefaultApi.md#applications_application_webhooks_webhook_id_subscriptions_subscription_id_delete) | **DELETE** /applications/{application}/webhooks/{webhookId}/subscriptions/{subscriptionId} | Delete subscription
[**applications_application_webhooks_webhook_id_subscriptions_subscription_id_patch**](DefaultApi.md#applications_application_webhooks_webhook_id_subscriptions_subscription_id_patch) | **PATCH** /applications/{application}/webhooks/{webhookId}/subscriptions/{subscriptionId} | Update subscription
[**applications_application_webhooks_webhook_id_subscriptions_subscription_id_request_missing_rights_post**](DefaultApi.md#applications_application_webhooks_webhook_id_subscriptions_subscription_id_request_missing_rights_post) | **POST** /applications/{application}/webhooks/{webhookId}/subscriptions/{subscriptionId}/request-missing-rights | Request Missing Rights
[**applications_authorizations_authorized_applications_get**](DefaultApi.md#applications_authorizations_authorized_applications_get) | **GET** /applications/authorizations/authorized-applications | Get applications authorized in context
[**applications_error_message_post**](DefaultApi.md#applications_error_message_post) | **POST** /applications/error-message | Set error message
[**applications_paged_get**](DefaultApi.md#applications_paged_get) | **GET** /applications/paged | Get all applications
[**applications_parameters_get**](DefaultApi.md#applications_parameters_get) | **GET** /applications/parameters | Get all parameters
[**applications_parameters_key_delete**](DefaultApi.md#applications_parameters_key_delete) | **DELETE** /applications/parameters/{key} | Remove parameter
[**applications_parameters_key_get**](DefaultApi.md#applications_parameters_key_get) | **GET** /applications/parameters/{key} | Get parameter
[**applications_parameters_key_patch**](DefaultApi.md#applications_parameters_key_patch) | **PATCH** /applications/parameters/{key} | Set parameter
[**applications_parameters_profile_get**](DefaultApi.md#applications_parameters_profile_get) | **GET** /applications/parameters/profile | Get all profile parameters
[**applications_parameters_profile_key_delete**](DefaultApi.md#applications_parameters_profile_key_delete) | **DELETE** /applications/parameters/profile/{key} | Remove profile parameter
[**applications_parameters_profile_key_get**](DefaultApi.md#applications_parameters_profile_key_get) | **GET** /applications/parameters/profile/{key} | Get profile parameter
[**applications_parameters_profile_key_patch**](DefaultApi.md#applications_parameters_profile_key_patch) | **PATCH** /applications/parameters/profile/{key} | Set profile parameter
[**applications_post**](DefaultApi.md#applications_post) | **POST** /applications | Create application
[**applications_report_application_as_healthy_post**](DefaultApi.md#applications_report_application_as_healthy_post) | **POST** /applications/report-application-as-healthy | Report application as healthy
[**applications_ui_extensions_patch**](DefaultApi.md#applications_ui_extensions_patch) | **PATCH** /applications/ui-extensions | Set UI extensions
[**applications_unfurls_domains_patch**](DefaultApi.md#applications_unfurls_domains_patch) | **PATCH** /applications/unfurls/domains | Update unfurled domains
[**applications_unfurls_patterns_patch**](DefaultApi.md#applications_unfurls_patterns_patch) | **PATCH** /applications/unfurls/patterns | Update unfurled patterns
[**applications_unfurls_queue_content_post**](DefaultApi.md#applications_unfurls_queue_content_post) | **POST** /applications/unfurls/queue/content | Post unfurls content
[**applications_unfurls_queue_get**](DefaultApi.md#applications_unfurls_queue_get) | **GET** /applications/unfurls/queue | Get unfurl queue items
[**applications_unfurls_queue_request_external_auth_post**](DefaultApi.md#applications_unfurls_queue_request_external_auth_post) | **POST** /applications/unfurls/queue/request-external-auth | Request external system authentication
[**applications_unfurls_queue_reset_external_auth_requests_post**](DefaultApi.md#applications_unfurls_queue_reset_external_auth_requests_post) | **POST** /applications/unfurls/queue/reset-external-auth-requests | Clear external system authentication requests
[**auth_modules_config_delete**](DefaultApi.md#auth_modules_config_delete) | **DELETE** /auth-modules/config | Delete config
[**auth_modules_config_get**](DefaultApi.md#auth_modules_config_get) | **GET** /auth-modules/config | Get config
[**auth_modules_config_put**](DefaultApi.md#auth_modules_config_put) | **PUT** /auth-modules/config | Put config
[**auth_modules_discover_oidc_get**](DefaultApi.md#auth_modules_discover_oidc_get) | **GET** /auth-modules/discover-oidc | Discover OIDC
[**auth_modules_get**](DefaultApi.md#auth_modules_get) | **GET** /auth-modules | Get all auth modules
[**auth_modules_id_delete**](DefaultApi.md#auth_modules_id_delete) | **DELETE** /auth-modules/{id} | Delete auth module
[**auth_modules_id_logins_identifier_change_post**](DefaultApi.md#auth_modules_id_logins_identifier_change_post) | **POST** /auth-modules/{id}/logins/{identifier}/change | Change password
[**auth_modules_id_logins_identifier_delete**](DefaultApi.md#auth_modules_id_logins_identifier_delete) | **DELETE** /auth-modules/{id}/logins/{identifier} | Delete login
[**auth_modules_id_logins_identifier_reset_post**](DefaultApi.md#auth_modules_id_logins_identifier_reset_post) | **POST** /auth-modules/{id}/logins/{identifier}/reset | Reset password
[**auth_modules_id_patch**](DefaultApi.md#auth_modules_id_patch) | **PATCH** /auth-modules/{id} | Update auth module
[**auth_modules_id_saml_metadata_post**](DefaultApi.md#auth_modules_id_saml_metadata_post) | **POST** /auth-modules/{id}/saml-metadata | SAML metadata
[**auth_modules_keykey_get**](DefaultApi.md#auth_modules_keykey_get) | **GET** /auth-modules/key:{key} | Get auth module by key
[**auth_modules_post**](DefaultApi.md#auth_modules_post) | **POST** /auth-modules | Create auth module
[**auth_modules_reorder_post**](DefaultApi.md#auth_modules_reorder_post) | **POST** /auth-modules/reorder | Reorder authentication modules
[**auth_modules_test_built_in_post**](DefaultApi.md#auth_modules_test_built_in_post) | **POST** /auth-modules/test/built-in | Test built-in settings
[**auth_modules_test_ldap_post**](DefaultApi.md#auth_modules_test_ldap_post) | **POST** /auth-modules/test/ldap | Test LDAP settings
[**auth_modules_throttled_logins_delete**](DefaultApi.md#auth_modules_throttled_logins_delete) | **DELETE** /auth-modules/throttled-logins | Reset throttling status
[**auth_modules_throttled_logins_get**](DefaultApi.md#auth_modules_throttled_logins_get) | **GET** /auth-modules/throttled-logins | Get throttled logins
[**auth_modules_throttled_logins_org_status_delete**](DefaultApi.md#auth_modules_throttled_logins_org_status_delete) | **DELETE** /auth-modules/throttled-logins/org-status | Reset organization throttling
[**auth_modules_throttled_logins_org_status_get**](DefaultApi.md#auth_modules_throttled_logins_org_status_get) | **GET** /auth-modules/throttled-logins/org-status | Get organization throttling status
[**auth_modules_usages_get**](DefaultApi.md#auth_modules_usages_get) | **GET** /auth-modules/usages | Get all usages
[**billing_admin_features_get**](DefaultApi.md#billing_admin_features_get) | **GET** /billing-admin/features | Get features
[**billing_admin_overdrafts_get**](DefaultApi.md#billing_admin_overdrafts_get) | **GET** /billing-admin/overdrafts | Get overdrafts
[**billing_admin_overdrafts_put**](DefaultApi.md#billing_admin_overdrafts_put) | **PUT** /billing-admin/overdrafts | Set overdrafts
[**billing_admin_reports_billing_period_get**](DefaultApi.md#billing_admin_reports_billing_period_get) | **GET** /billing-admin/reports/{billingPeriod} | Get billing report
[**billing_admin_reports_today_get**](DefaultApi.md#billing_admin_reports_today_get) | **GET** /billing-admin/reports/today | Get billing report for today
[**billing_admin_trial_put**](DefaultApi.md#billing_admin_trial_put) | **PUT** /billing-admin/trial | Activate trial. Not available for On-Premises installations.
[**blog_aliasalias_get**](DefaultApi.md#blog_aliasalias_get) | **GET** /blog/alias:{alias} | Get blog post by alias
[**blog_external_idid_get**](DefaultApi.md#blog_external_idid_get) | **GET** /blog/external-id:{id} | Get blog post by external ID
[**blog_get**](DefaultApi.md#blog_get) | **GET** /blog | Get all blog posts
[**blog_id_delete**](DefaultApi.md#blog_id_delete) | **DELETE** /blog/{id} | Unpublish blog post
[**blog_id_get**](DefaultApi.md#blog_id_get) | **GET** /blog/{id} | Get blog post
[**blog_id_patch**](DefaultApi.md#blog_id_patch) | **PATCH** /blog/{id} | Update blog post
[**blog_import_post**](DefaultApi.md#blog_import_post) | **POST** /blog/import | Import blog posts
[**blog_post**](DefaultApi.md#blog_post) | **POST** /blog | Publish blog post
[**blog_stats_get**](DefaultApi.md#blog_stats_get) | **GET** /blog/stats | Get stats
[**calendars_absence_events_get**](DefaultApi.md#calendars_absence_events_get) | **GET** /calendars/absence-events | Get all absence events
[**calendars_birthday_events_get**](DefaultApi.md#calendars_birthday_events_get) | **GET** /calendars/birthday-events | Get all birthday events
[**calendars_birthday_events_starred_get**](DefaultApi.md#calendars_birthday_events_starred_get) | **GET** /calendars/birthday-events/starred | Get all starred birthday events
[**calendars_event_participations_id_patch**](DefaultApi.md#calendars_event_participations_id_patch) | **PATCH** /calendars/event-participations/{id} | Update event participation
[**calendars_events_get**](DefaultApi.md#calendars_events_get) | **GET** /calendars/events | Get all events
[**calendars_events_id_get**](DefaultApi.md#calendars_events_id_get) | **GET** /calendars/events/{id} | Get event
[**calendars_holidays_get**](DefaultApi.md#calendars_holidays_get) | **GET** /calendars/holidays | Get all holidays
[**calendars_meetings_get**](DefaultApi.md#calendars_meetings_get) | **GET** /calendars/meetings | Get all meetings
[**calendars_meetings_id_conference_rooms_delete**](DefaultApi.md#calendars_meetings_id_conference_rooms_delete) | **DELETE** /calendars/meetings/{id}/conference-rooms | Remove conference room
[**calendars_meetings_id_conference_rooms_post**](DefaultApi.md#calendars_meetings_id_conference_rooms_post) | **POST** /calendars/meetings/{id}/conference-rooms | Add conference room
[**calendars_meetings_id_delete**](DefaultApi.md#calendars_meetings_id_delete) | **DELETE** /calendars/meetings/{id} | Delete meeting
[**calendars_meetings_id_get**](DefaultApi.md#calendars_meetings_id_get) | **GET** /calendars/meetings/{id} | Get meeting
[**calendars_meetings_id_participation_status_patch**](DefaultApi.md#calendars_meetings_id_participation_status_patch) | **PATCH** /calendars/meetings/{id}/participation-status | Update profile participation status
[**calendars_meetings_id_patch**](DefaultApi.md#calendars_meetings_id_patch) | **PATCH** /calendars/meetings/{id} | Update meeting
[**calendars_meetings_next_occurrence_get**](DefaultApi.md#calendars_meetings_next_occurrence_get) | **GET** /calendars/meetings/next-occurrence | Get next meeting occurrence
[**calendars_meetings_occurrences_by_meeting_get**](DefaultApi.md#calendars_meetings_occurrences_by_meeting_get) | **GET** /calendars/meetings/occurrences-by-meeting | Get meeting occurrences for period for multiple meetings
[**calendars_meetings_occurrences_get**](DefaultApi.md#calendars_meetings_occurrences_get) | **GET** /calendars/meetings/occurrences | Get meeting occurrences for period
[**calendars_meetings_participation_statuses_external_get**](DefaultApi.md#calendars_meetings_participation_statuses_external_get) | **GET** /calendars/meetings/participation-statuses-external | Get RSVP statuses for external users
[**calendars_meetings_participation_statuses_get**](DefaultApi.md#calendars_meetings_participation_statuses_get) | **GET** /calendars/meetings/participation-statuses | Get meeting participation statuses for profiles
[**calendars_meetings_post**](DefaultApi.md#calendars_meetings_post) | **POST** /calendars/meetings | Create meeting
[**calendars_meetings_profile_participation_get**](DefaultApi.md#calendars_meetings_profile_participation_get) | **GET** /calendars/meetings/profile-participation | Get profile participation statuses for meetings
[**calendars_meetings_profile_participation_records_get**](DefaultApi.md#calendars_meetings_profile_participation_records_get) | **GET** /calendars/meetings/profile-participation-records | Get profile participation status records for meetings
[**calendars_membership_events_get**](DefaultApi.md#calendars_membership_events_get) | **GET** /calendars/membership-events | Get all membership events
[**calendars_non_working_days_events_get**](DefaultApi.md#calendars_non_working_days_events_get) | **GET** /calendars/non-working-days-events | Get all non working days events
[**calls_post**](DefaultApi.md#calls_post) | **POST** /calls | Create call
[**chats_channels_all_channels_get**](DefaultApi.md#chats_channels_all_channels_get) | **GET** /chats/channels/all-channels | List all channels
[**chats_channels_channel_administrator_get**](DefaultApi.md#chats_channels_channel_administrator_get) | **GET** /chats/channels/{channel}/administrator | Get channel administrator
[**chats_channels_channel_administrator_patch**](DefaultApi.md#chats_channels_channel_administrator_patch) | **PATCH** /chats/channels/{channel}/administrator | Assign channel administrator
[**chats_channels_channel_archive_delete**](DefaultApi.md#chats_channels_channel_archive_delete) | **DELETE** /chats/channels/{channel}/archive | Archive channel
[**chats_channels_channel_attachments_files_get**](DefaultApi.md#chats_channels_channel_attachments_files_get) | **GET** /chats/channels/{channel}/attachments/files | List file attachments in channel
[**chats_channels_channel_attachments_get**](DefaultApi.md#chats_channels_channel_attachments_get) | **GET** /chats/channels/{channel}/attachments | List attachments in channel
[**chats_channels_channel_attachments_images_get**](DefaultApi.md#chats_channels_channel_attachments_images_get) | **GET** /chats/channels/{channel}/attachments/images | List images in channel
[**chats_channels_channel_attachments_links_get**](DefaultApi.md#chats_channels_channel_attachments_links_get) | **GET** /chats/channels/{channel}/attachments/links | List links in channel
[**chats_channels_channel_attachments_videos_get**](DefaultApi.md#chats_channels_channel_attachments_videos_get) | **GET** /chats/channels/{channel}/attachments/videos | List videos in channel
[**chats_channels_channel_delete**](DefaultApi.md#chats_channels_channel_delete) | **DELETE** /chats/channels/{channel} | Delete channel
[**chats_channels_channel_description_patch**](DefaultApi.md#chats_channels_channel_description_patch) | **PATCH** /chats/channels/{channel}/description | Change channel description
[**chats_channels_channel_get**](DefaultApi.md#chats_channels_channel_get) | **GET** /chats/channels/{channel} | Get channel
[**chats_channels_channel_icon_patch**](DefaultApi.md#chats_channels_channel_icon_patch) | **PATCH** /chats/channels/{channel}/icon | Change channel icon
[**chats_channels_channel_name_patch**](DefaultApi.md#chats_channels_channel_name_patch) | **PATCH** /chats/channels/{channel}/name | Rename channel
[**chats_channels_channel_restore_archived_post**](DefaultApi.md#chats_channels_channel_restore_archived_post) | **POST** /chats/channels/{channel}/restore-archived | Restore archived channel
[**chats_channels_channel_subscribers_teams_delete**](DefaultApi.md#chats_channels_channel_subscribers_teams_delete) | **DELETE** /chats/channels/{channel}/subscribers/teams | Remove teams from channel
[**chats_channels_channel_subscribers_teams_get**](DefaultApi.md#chats_channels_channel_subscribers_teams_get) | **GET** /chats/channels/{channel}/subscribers/teams | List teams subscribed to channel
[**chats_channels_channel_subscribers_teams_post**](DefaultApi.md#chats_channels_channel_subscribers_teams_post) | **POST** /chats/channels/{channel}/subscribers/teams | Add teams to channel
[**chats_channels_channel_subscribers_users_delete**](DefaultApi.md#chats_channels_channel_subscribers_users_delete) | **DELETE** /chats/channels/{channel}/subscribers/users | Remove users from channel
[**chats_channels_channel_subscribers_users_get**](DefaultApi.md#chats_channels_channel_subscribers_users_get) | **GET** /chats/channels/{channel}/subscribers/users | List users subscribed to channel
[**chats_channels_channel_subscribers_users_post**](DefaultApi.md#chats_channels_channel_subscribers_users_post) | **POST** /chats/channels/{channel}/subscribers/users | Add users to channel
[**chats_channels_conversations_channel_convert_post**](DefaultApi.md#chats_channels_conversations_channel_convert_post) | **POST** /chats/channels/conversations/{channel}/convert | Convert conversation to private channel
[**chats_channels_conversations_channel_subject_patch**](DefaultApi.md#chats_channels_conversations_channel_subject_patch) | **PATCH** /chats/channels/conversations/{channel}/subject | Change conversation subject
[**chats_channels_conversations_post**](DefaultApi.md#chats_channels_conversations_post) | **POST** /chats/channels/conversations | Create conversation
[**chats_channels_dm_post**](DefaultApi.md#chats_channels_dm_post) | **POST** /chats/channels/dm | Get or create direct messages channel
[**chats_channels_is_name_free_post**](DefaultApi.md#chats_channels_is_name_free_post) | **POST** /chats/channels/is-name-free | Is name free?
[**chats_channels_post**](DefaultApi.md#chats_channels_post) | **POST** /chats/channels | Add new channel
[**chats_messages_delete_message_post**](DefaultApi.md#chats_messages_delete_message_post) | **POST** /chats/messages/delete-message | Delete message
[**chats_messages_edit_message_post**](DefaultApi.md#chats_messages_edit_message_post) | **POST** /chats/messages/edit-message | Edit message
[**chats_messages_get**](DefaultApi.md#chats_messages_get) | **GET** /chats/messages | Get channel messages
[**chats_messages_import_post**](DefaultApi.md#chats_messages_import_post) | **POST** /chats/messages/import | Import messages
[**chats_messages_message_get**](DefaultApi.md#chats_messages_message_get) | **GET** /chats/messages/{message} | Get message
[**chats_messages_pin_patch**](DefaultApi.md#chats_messages_pin_patch) | **PATCH** /chats/messages/pin | Pin message
[**chats_messages_pinned_messages_get**](DefaultApi.md#chats_messages_pinned_messages_get) | **GET** /chats/messages/pinned-messages | List pinned messages in channel
[**chats_messages_send_message_post**](DefaultApi.md#chats_messages_send_message_post) | **POST** /chats/messages/send-message | Send message
[**chats_messages_send_post**](DefaultApi.md#chats_messages_send_post) | **POST** /chats/messages/send | Send text message
[**chats_messages_sync_batch_current_etag_get**](DefaultApi.md#chats_messages_sync_batch_current_etag_get) | **GET** /chats/messages/sync-batch/current-etag | Get current sync etag
[**chats_messages_sync_batch_get**](DefaultApi.md#chats_messages_sync_batch_get) | **GET** /chats/messages/sync-batch | Get sync batch
[**chats_messages_unpin_patch**](DefaultApi.md#chats_messages_unpin_patch) | **PATCH** /chats/messages/unpin | Unpin message
[**checklists_checklist_items_plan_item_delete**](DefaultApi.md#checklists_checklist_items_plan_item_delete) | **DELETE** /checklists/{checklist}/items/{planItem} | Delete plan item
[**checklists_checklist_items_plan_item_get**](DefaultApi.md#checklists_checklist_items_plan_item_get) | **GET** /checklists/{checklist}/items/{planItem} | Get plan item
[**checklists_checklist_items_plan_item_move_post**](DefaultApi.md#checklists_checklist_items_plan_item_move_post) | **POST** /checklists/{checklist}/items/{planItem}/move | Move plan item
[**checklists_checklist_items_plan_item_patch**](DefaultApi.md#checklists_checklist_items_plan_item_patch) | **PATCH** /checklists/{checklist}/items/{planItem} | Update plan item
[**checklists_checklist_items_post**](DefaultApi.md#checklists_checklist_items_post) | **POST** /checklists/{checklist}/items | Create plan item
[**custom_fields_extended_types_get**](DefaultApi.md#custom_fields_extended_types_get) | **GET** /custom-fields/extended-types | Get all extended types
[**custom_fields_type_key_all_values_get**](DefaultApi.md#custom_fields_type_key_all_values_get) | **GET** /custom-fields/{typeKey}/all-values | Get all values
[**custom_fields_type_key_entity_id_values_get**](DefaultApi.md#custom_fields_type_key_entity_id_values_get) | **GET** /custom-fields/{typeKey}/{entityId}/values | Get value
[**custom_fields_type_key_entity_id_values_patch**](DefaultApi.md#custom_fields_type_key_entity_id_values_patch) | **PATCH** /custom-fields/{typeKey}/{entityId}/values | Update value
[**custom_fields_type_key_enum_values_custom_field_id_get**](DefaultApi.md#custom_fields_type_key_enum_values_custom_field_id_get) | **GET** /custom-fields/{typeKey}/enum-values/{customFieldId} | Get all enum values
[**custom_fields_type_key_enum_values_custom_field_id_post**](DefaultApi.md#custom_fields_type_key_enum_values_custom_field_id_post) | **POST** /custom-fields/{typeKey}/enum-values/{customFieldId} | Create enum value
[**custom_fields_type_key_fields_get**](DefaultApi.md#custom_fields_type_key_fields_get) | **GET** /custom-fields/{typeKey}/fields | Get all fields
[**custom_fields_type_key_fields_id_archive_post**](DefaultApi.md#custom_fields_type_key_fields_id_archive_post) | **POST** /custom-fields/{typeKey}/fields/{id}/archive | Archive field
[**custom_fields_type_key_fields_id_delete**](DefaultApi.md#custom_fields_type_key_fields_id_delete) | **DELETE** /custom-fields/{typeKey}/fields/{id} | Delete field
[**custom_fields_type_key_fields_id_filter_values_get**](DefaultApi.md#custom_fields_type_key_fields_id_filter_values_get) | **GET** /custom-fields/{typeKey}/fields/{id}/filter-values | Get all filter values
[**custom_fields_type_key_fields_id_patch**](DefaultApi.md#custom_fields_type_key_fields_id_patch) | **PATCH** /custom-fields/{typeKey}/fields/{id} | Update field
[**custom_fields_type_key_fields_id_restore_post**](DefaultApi.md#custom_fields_type_key_fields_id_restore_post) | **POST** /custom-fields/{typeKey}/fields/{id}/restore | Restore field
[**custom_fields_type_key_fields_post**](DefaultApi.md#custom_fields_type_key_fields_post) | **POST** /custom-fields/{typeKey}/fields | Create field
[**custom_fields_type_key_fields_reorder_post**](DefaultApi.md#custom_fields_type_key_fields_reorder_post) | **POST** /custom-fields/{typeKey}/fields/reorder | Reorder fields
[**custom_fields_v2_entity_type_fields_custom_field_archive_post**](DefaultApi.md#custom_fields_v2_entity_type_fields_custom_field_archive_post) | **POST** /custom-fields-v2/{entityType}/fields/{customField}/archive | Archive custom field
[**custom_fields_v2_entity_type_fields_custom_field_delete**](DefaultApi.md#custom_fields_v2_entity_type_fields_custom_field_delete) | **DELETE** /custom-fields-v2/{entityType}/fields/{customField} | Delete custom field
[**custom_fields_v2_entity_type_fields_custom_field_enum_values_bulk_update_post**](DefaultApi.md#custom_fields_v2_entity_type_fields_custom_field_enum_values_bulk_update_post) | **POST** /custom-fields-v2/{entityType}/fields/{customField}/enum-values/bulk-update | Bulk update enum values
[**custom_fields_v2_entity_type_fields_custom_field_enum_values_enum_value_to_remove_delete**](DefaultApi.md#custom_fields_v2_entity_type_fields_custom_field_enum_values_enum_value_to_remove_delete) | **DELETE** /custom-fields-v2/{entityType}/fields/{customField}/enum-values/{enumValueToRemove} | Delete enum value
[**custom_fields_v2_entity_type_fields_custom_field_enum_values_get**](DefaultApi.md#custom_fields_v2_entity_type_fields_custom_field_enum_values_get) | **GET** /custom-fields-v2/{entityType}/fields/{customField}/enum-values | Get enum values
[**custom_fields_v2_entity_type_fields_custom_field_enum_values_patch**](DefaultApi.md#custom_fields_v2_entity_type_fields_custom_field_enum_values_patch) | **PATCH** /custom-fields-v2/{entityType}/fields/{customField}/enum-values | Update enum value
[**custom_fields_v2_entity_type_fields_custom_field_enum_values_post**](DefaultApi.md#custom_fields_v2_entity_type_fields_custom_field_enum_values_post) | **POST** /custom-fields-v2/{entityType}/fields/{customField}/enum-values | Create enum value
[**custom_fields_v2_entity_type_fields_custom_field_get**](DefaultApi.md#custom_fields_v2_entity_type_fields_custom_field_get) | **GET** /custom-fields-v2/{entityType}/fields/{customField} | Get single custom field
[**custom_fields_v2_entity_type_fields_custom_field_patch**](DefaultApi.md#custom_fields_v2_entity_type_fields_custom_field_patch) | **PATCH** /custom-fields-v2/{entityType}/fields/{customField} | Update custom field
[**custom_fields_v2_entity_type_fields_custom_field_restore_post**](DefaultApi.md#custom_fields_v2_entity_type_fields_custom_field_restore_post) | **POST** /custom-fields-v2/{entityType}/fields/{customField}/restore | Restore custom field
[**custom_fields_v2_entity_type_fields_get**](DefaultApi.md#custom_fields_v2_entity_type_fields_get) | **GET** /custom-fields-v2/{entityType}/fields | Get custom fields
[**custom_fields_v2_entity_type_fields_post**](DefaultApi.md#custom_fields_v2_entity_type_fields_post) | **POST** /custom-fields-v2/{entityType}/fields | Create custom field
[**custom_fields_v2_entity_type_fields_reorder_post**](DefaultApi.md#custom_fields_v2_entity_type_fields_reorder_post) | **POST** /custom-fields-v2/{entityType}/fields/reorder | Reorder custom fields
[**custom_fields_v2_values_entity_custom_field_get**](DefaultApi.md#custom_fields_v2_values_entity_custom_field_get) | **GET** /custom-fields-v2/values/{entity}/{customField} | Get single value
[**custom_fields_v2_values_entity_custom_field_post**](DefaultApi.md#custom_fields_v2_values_entity_custom_field_post) | **POST** /custom-fields-v2/values/{entity}/{customField} | Set single value
[**custom_fields_v2_values_entity_get**](DefaultApi.md#custom_fields_v2_values_entity_get) | **GET** /custom-fields-v2/values/{entity} | Get all values for entity
[**custom_fields_v2_values_entity_post**](DefaultApi.md#custom_fields_v2_values_entity_post) | **POST** /custom-fields-v2/values/{entity} | Set values for entity
[**emojis_add_post**](DefaultApi.md#emojis_add_post) | **POST** /emojis/add | Add emoji
[**emojis_delete_post**](DefaultApi.md#emojis_delete_post) | **POST** /emojis/delete | Delete emoji
[**emojis_exists_get**](DefaultApi.md#emojis_exists_get) | **GET** /emojis/exists | Check if emoji exists
[**emojis_frequently_used_get**](DefaultApi.md#emojis_frequently_used_get) | **GET** /emojis/frequently-used | Get frequently used emojis
[**emojis_record_usage_post**](DefaultApi.md#emojis_record_usage_post) | **POST** /emojis/record-usage | Record emojis usage
[**emojis_search_get**](DefaultApi.md#emojis_search_get) | **GET** /emojis/search | Search emoji
[**emojis_sync_batch_get**](DefaultApi.md#emojis_sync_batch_get) | **GET** /emojis/sync-batch | Get sync batch
[**external_issues_default_issue_status_for_mr_merge_post**](DefaultApi.md#external_issues_default_issue_status_for_mr_merge_post) | **POST** /external-issues/default-issue-status-for-mr-merge | Set default target issue status for merge request merge
[**external_issues_events_queue_get**](DefaultApi.md#external_issues_events_queue_get) | **GET** /external-issues/events-queue | Get external issue event queue items
[**external_issues_external_tracker_projects_delete**](DefaultApi.md#external_issues_external_tracker_projects_delete) | **DELETE** /external-issues/external-tracker-projects | Disconnect external issue tracker project
[**external_issues_external_tracker_projects_get**](DefaultApi.md#external_issues_external_tracker_projects_get) | **GET** /external-issues/external-tracker-projects | Get all connected external issue tracker projects
[**external_issues_external_tracker_projects_post**](DefaultApi.md#external_issues_external_tracker_projects_post) | **POST** /external-issues/external-tracker-projects | Connect external issue tracker projects
[**external_issues_issue_content_post**](DefaultApi.md#external_issues_issue_content_post) | **POST** /external-issues/issue-content | Post external issue data
[**external_issues_issue_statuses_post**](DefaultApi.md#external_issues_issue_statuses_post) | **POST** /external-issues/issue-statuses | Provide all possible statuses for external issues
[**external_issues_issues_issue_prefix_issue_id_code_reviews_delete**](DefaultApi.md#external_issues_issues_issue_prefix_issue_id_code_reviews_delete) | **DELETE** /external-issues/issues/{issuePrefix}/{issueId}/code-reviews | Unlink code reviews from external issue
[**external_issues_issues_issue_prefix_issue_id_code_reviews_post**](DefaultApi.md#external_issues_issues_issue_prefix_issue_id_code_reviews_post) | **POST** /external-issues/issues/{issuePrefix}/{issueId}/code-reviews | Link code reviews to external issue
[**external_issues_issues_issue_prefix_issue_id_commits_delete**](DefaultApi.md#external_issues_issues_issue_prefix_issue_id_commits_delete) | **DELETE** /external-issues/issues/{issuePrefix}/{issueId}/commits | Unlink commits from external issue
[**external_issues_issues_issue_prefix_issue_id_commits_post**](DefaultApi.md#external_issues_issues_issue_prefix_issue_id_commits_post) | **POST** /external-issues/issues/{issuePrefix}/{issueId}/commits | Link commits to external issue
[**external_issues_mark_issues_as_deleted_post**](DefaultApi.md#external_issues_mark_issues_as_deleted_post) | **POST** /external-issues/mark-issues-as-deleted | Mark external issues as deleted
[**external_link_patterns_delete**](DefaultApi.md#external_link_patterns_delete) | **DELETE** /external-link-patterns | Delete external link pattern
[**external_link_patterns_get**](DefaultApi.md#external_link_patterns_get) | **GET** /external-link-patterns | Get all external link patterns
[**external_link_patterns_post**](DefaultApi.md#external_link_patterns_post) | **POST** /external-link-patterns | Create external link pattern
[**http_api_model_get**](DefaultApi.md#http_api_model_get) | **GET** /http-api-model | Get HTTP API model
[**issues_get**](DefaultApi.md#issues_get) | **GET** /issues | Get issue
[**issues_get_by_ids_post**](DefaultApi.md#issues_get_by_ids_post) | **POST** /issues/get-by-ids | Get issues by identifiers
[**notifications_channel_subscriptions_get**](DefaultApi.md#notifications_channel_subscriptions_get) | **GET** /notifications/channel-subscriptions | Get all channel subscriptions
[**notifications_channel_subscriptions_id_delete**](DefaultApi.md#notifications_channel_subscriptions_id_delete) | **DELETE** /notifications/channel-subscriptions/{id} | Delete channel subscription
[**notifications_channel_subscriptions_id_patch**](DefaultApi.md#notifications_channel_subscriptions_id_patch) | **PATCH** /notifications/channel-subscriptions/{id} | Update channel subscription
[**notifications_channel_subscriptions_id_request_missing_rights_post**](DefaultApi.md#notifications_channel_subscriptions_id_request_missing_rights_post) | **POST** /notifications/channel-subscriptions/{id}/request-missing-rights | Request Missing Rights
[**notifications_channel_subscriptions_post**](DefaultApi.md#notifications_channel_subscriptions_post) | **POST** /notifications/channel-subscriptions | Create channel subscription
[**notifications_get**](DefaultApi.md#notifications_get) | **GET** /notifications | Get all notifications
[**notifications_personal_custom_subscriptions_get**](DefaultApi.md#notifications_personal_custom_subscriptions_get) | **GET** /notifications/personal-custom-subscriptions | Get all personal custom subscriptions
[**notifications_personal_custom_subscriptions_id_delete**](DefaultApi.md#notifications_personal_custom_subscriptions_id_delete) | **DELETE** /notifications/personal-custom-subscriptions/{id} | Delete personal custom subscription
[**notifications_personal_custom_subscriptions_id_patch**](DefaultApi.md#notifications_personal_custom_subscriptions_id_patch) | **PATCH** /notifications/personal-custom-subscriptions/{id} | Update personal custom subscription
[**notifications_personal_custom_subscriptions_post**](DefaultApi.md#notifications_personal_custom_subscriptions_post) | **POST** /notifications/personal-custom-subscriptions | Create personal custom subscription
[**notifications_personal_subscriptions_all_personal_subscription_targets_get**](DefaultApi.md#notifications_personal_subscriptions_all_personal_subscription_targets_get) | **GET** /notifications/personal-subscriptions/all-personal-subscription-targets | All personal subscription targets
[**notifications_personal_subscriptions_personal_subscription_settings_get**](DefaultApi.md#notifications_personal_subscriptions_personal_subscription_settings_get) | **GET** /notifications/personal-subscriptions/personal-subscription-settings | Get personal subscription settings
[**notifications_personal_subscriptions_update_personal_subscription_subject_post**](DefaultApi.md#notifications_personal_subscriptions_update_personal_subscription_subject_post) | **POST** /notifications/personal-subscriptions/update-personal-subscription-subject | Update personal subscription subject
[**notifications_personal_subscriptions_update_personal_subscription_target_post**](DefaultApi.md#notifications_personal_subscriptions_update_personal_subscription_target_post) | **POST** /notifications/personal-subscriptions/update-personal-subscription-target | Update personal subscription target
[**notifications_private_feeds_get**](DefaultApi.md#notifications_private_feeds_get) | **GET** /notifications/private-feeds | Get all private feeds
[**notifications_private_feeds_id_delete**](DefaultApi.md#notifications_private_feeds_id_delete) | **DELETE** /notifications/private-feeds/{id} | Delete private feed
[**notifications_private_feeds_id_patch**](DefaultApi.md#notifications_private_feeds_id_patch) | **PATCH** /notifications/private-feeds/{id} | Update private feed
[**notifications_private_feeds_post**](DefaultApi.md#notifications_private_feeds_post) | **POST** /notifications/private-feeds | Create private feed
[**organization_domains_check_get**](DefaultApi.md#organization_domains_check_get) | **GET** /organization/domains/check | Check domain availability
[**organization_domains_get**](DefaultApi.md#organization_domains_get) | **GET** /organization/domains | Get all domains
[**organization_domains_patch**](DefaultApi.md#organization_domains_patch) | **PATCH** /organization/domains | Update organization domain
[**organization_get**](DefaultApi.md#organization_get) | **GET** /organization | Get organization
[**organization_jet_sales_license_activation_url_get**](DefaultApi.md#organization_jet_sales_license_activation_url_get) | **GET** /organization/jet-sales/license-activation-url | Get license activation url
[**organization_jet_sales_url_get**](DefaultApi.md#organization_jet_sales_url_get) | **GET** /organization/jet-sales/url | Get JetSales URL
[**organization_patch**](DefaultApi.md#organization_patch) | **PATCH** /organization | Update organization
[**permission_roles_create_post**](DefaultApi.md#permission_roles_create_post) | **POST** /permission-roles/create | Create role
[**permission_roles_get_post**](DefaultApi.md#permission_roles_get_post) | **POST** /permission-roles/get | Get roles
[**permission_roles_role_id2_fa_requirement_get**](DefaultApi.md#permission_roles_role_id2_fa_requirement_get) | **GET** /permission-roles/{roleId}/2-fa-requirement | Get 2FA requirement
[**permission_roles_role_id2_fa_requirement_patch**](DefaultApi.md#permission_roles_role_id2_fa_requirement_patch) | **PATCH** /permission-roles/{roleId}/2-fa-requirement | Set 2FA requirement
[**permission_roles_role_id_delete**](DefaultApi.md#permission_roles_role_id_delete) | **DELETE** /permission-roles/{roleId} | Delete role
[**permission_roles_role_id_patch**](DefaultApi.md#permission_roles_role_id_patch) | **PATCH** /permission-roles/{roleId} | Update role
[**permission_roles_role_id_permissions_delete**](DefaultApi.md#permission_roles_role_id_permissions_delete) | **DELETE** /permission-roles/{roleId}/permissions | Revoke role permissions
[**permission_roles_role_id_permissions_get**](DefaultApi.md#permission_roles_role_id_permissions_get) | **GET** /permission-roles/{roleId}/permissions | Get role permissions
[**permission_roles_role_id_permissions_post**](DefaultApi.md#permission_roles_role_id_permissions_post) | **POST** /permission-roles/{roleId}/permissions | Grant role permissions
[**permission_roles_role_id_profiles_get**](DefaultApi.md#permission_roles_role_id_profiles_get) | **GET** /permission-roles/{roleId}/profiles | Get role members
[**permission_roles_role_id_profiles_profile_delete**](DefaultApi.md#permission_roles_role_id_profiles_profile_delete) | **DELETE** /permission-roles/{roleId}/profiles/{profile} | Remove role member
[**permission_roles_role_id_profiles_profile_post**](DefaultApi.md#permission_roles_role_id_profiles_profile_post) | **POST** /permission-roles/{roleId}/profiles/{profile} | Add role member
[**permission_roles_role_id_reset_role_permissions_to_default_post**](DefaultApi.md#permission_roles_role_id_reset_role_permissions_to_default_post) | **POST** /permission-roles/{roleId}/reset-role-permissions-to-default | Reset role permissions to default
[**permission_roles_role_id_teams_get**](DefaultApi.md#permission_roles_role_id_teams_get) | **GET** /permission-roles/{roleId}/teams | Get role teams
[**permission_roles_role_id_teams_team_delete**](DefaultApi.md#permission_roles_role_id_teams_team_delete) | **DELETE** /permission-roles/{roleId}/teams/{team} | Remove role team
[**permission_roles_role_id_teams_team_post**](DefaultApi.md#permission_roles_role_id_teams_team_post) | **POST** /permission-roles/{roleId}/teams/{team} | Add role team
[**permissions_check_permission_post**](DefaultApi.md#permissions_check_permission_post) | **POST** /permissions/check-permission | Check permission
[**permissions_get**](DefaultApi.md#permissions_get) | **GET** /permissions | Get all permissions
[**projects_automation_deployment_targets_audit_log_get**](DefaultApi.md#projects_automation_deployment_targets_audit_log_get) | **GET** /projects/automation/deployment-targets/audit-log | Get audit messages
[**projects_automation_deployment_targets_full_number_id_get**](DefaultApi.md#projects_automation_deployment_targets_full_number_id_get) | **GET** /projects/automation/deployment-targets/{fullNumberId} | Get deployment target by full number id
[**projects_automation_deployment_targets_get**](DefaultApi.md#projects_automation_deployment_targets_get) | **GET** /projects/automation/deployment-targets | Get all deployment targets
[**projects_automation_deployment_targets_search_post**](DefaultApi.md#projects_automation_deployment_targets_search_post) | **POST** /projects/automation/deployment-targets/search | search
[**projects_automation_dsl_evaluations_config_get**](DefaultApi.md#projects_automation_dsl_evaluations_config_get) | **GET** /projects/automation/dsl-evaluations/config | Get DSL evaluation configuration
[**projects_automation_graph_executions_id_get**](DefaultApi.md#projects_automation_graph_executions_id_get) | **GET** /projects/automation/graph-executions/{id} | Get graph execution
[**projects_automation_graph_executions_id_stop_post**](DefaultApi.md#projects_automation_graph_executions_id_stop_post) | **POST** /projects/automation/graph-executions/{id}/stop | Stop execution
[**projects_automation_job_executions_current_get**](DefaultApi.md#projects_automation_job_executions_current_get) | **GET** /projects/automation/job-executions/current | Get current
[**projects_automation_jobs_job_id_get**](DefaultApi.md#projects_automation_jobs_job_id_get) | **GET** /projects/automation/jobs/{jobId} | Get job
[**projects_automation_step_executions_step_exec_id_parameters_key_get**](DefaultApi.md#projects_automation_step_executions_step_exec_id_parameters_key_get) | **GET** /projects/automation/step-executions/{stepExecId}/parameters/{key} | Get parameter
[**projects_automation_step_executions_step_exec_id_parameters_key_patch**](DefaultApi.md#projects_automation_step_executions_step_exec_id_parameters_key_patch) | **PATCH** /projects/automation/step-executions/{stepExecId}/parameters/{key} | Update parameter
[**projects_automation_step_executions_used_parameters_param_parameter_id_get**](DefaultApi.md#projects_automation_step_executions_used_parameters_param_parameter_id_get) | **GET** /projects/automation/step-executions/used-parameters/param/{parameterId} | Get param
[**projects_automation_step_executions_used_parameters_secret_secret_id_get**](DefaultApi.md#projects_automation_step_executions_used_parameters_secret_secret_id_get) | **GET** /projects/automation/step-executions/used-parameters/secret/{secretId} | Get secret
[**projects_automation_subscriptions_legacy_channels_delete**](DefaultApi.md#projects_automation_subscriptions_legacy_channels_delete) | **DELETE** /projects/automation/subscriptions/legacy-channels | Delete legacy channels
[**projects_collaboratorprofile_get**](DefaultApi.md#projects_collaboratorprofile_get) | **GET** /projects/collaborator:{profile} | Get all projects by collaborator
[**projects_get**](DefaultApi.md#projects_get) | **GET** /projects | Get all projects
[**projects_membermember_get**](DefaultApi.md#projects_membermember_get) | **GET** /projects/member:{member} | Get all projects by member
[**projects_params_default_bundle_get**](DefaultApi.md#projects_params_default_bundle_get) | **GET** /projects/params/default-bundle | Get all default bundle
[**projects_params_default_bundle_post**](DefaultApi.md#projects_params_default_bundle_post) | **POST** /projects/params/default-bundle | Create default bundle
[**projects_params_id_delete**](DefaultApi.md#projects_params_id_delete) | **DELETE** /projects/params/{id} | Delete param
[**projects_params_id_patch**](DefaultApi.md#projects_params_id_patch) | **PATCH** /projects/params/{id} | Update param
[**projects_params_in_default_bundle_get**](DefaultApi.md#projects_params_in_default_bundle_get) | **GET** /projects/params/in-default-bundle | Get all in default bundle
[**projects_params_in_default_bundle_post**](DefaultApi.md#projects_params_in_default_bundle_post) | **POST** /projects/params/in-default-bundle | Create in default bundle
[**projects_planning_boards_board_delete**](DefaultApi.md#projects_planning_boards_board_delete) | **DELETE** /projects/planning/boards/{board} | Delete board
[**projects_planning_boards_board_get**](DefaultApi.md#projects_planning_boards_board_get) | **GET** /projects/planning/boards/{board} | Get board
[**projects_planning_boards_board_issues_get**](DefaultApi.md#projects_planning_boards_board_issues_get) | **GET** /projects/planning/boards/{board}/issues | Get all issues on board
[**projects_planning_boards_board_issues_issue_delete**](DefaultApi.md#projects_planning_boards_board_issues_issue_delete) | **DELETE** /projects/planning/boards/{board}/issues/{issue} | Remove issue from board
[**projects_planning_boards_board_issues_issue_post**](DefaultApi.md#projects_planning_boards_board_issues_issue_post) | **POST** /projects/planning/boards/{board}/issues/{issue} | Add issue to board
[**projects_planning_boards_board_patch**](DefaultApi.md#projects_planning_boards_board_patch) | **PATCH** /projects/planning/boards/{board} | Update board
[**projects_planning_boards_sprints_post**](DefaultApi.md#projects_planning_boards_sprints_post) | **POST** /projects/planning/boards/sprints | Create sprint
[**projects_planning_boards_sprints_sprint_archive_delete**](DefaultApi.md#projects_planning_boards_sprints_sprint_archive_delete) | **DELETE** /projects/planning/boards/sprints/{sprint}/archive | Archive sprint
[**projects_planning_boards_sprints_sprint_issues_get**](DefaultApi.md#projects_planning_boards_sprints_sprint_issues_get) | **GET** /projects/planning/boards/sprints/{sprint}/issues | Get all issues in sprint
[**projects_planning_boards_sprints_sprint_issues_issue_delete**](DefaultApi.md#projects_planning_boards_sprints_sprint_issues_issue_delete) | **DELETE** /projects/planning/boards/sprints/{sprint}/issues/{issue} | Remove issue from sprint
[**projects_planning_boards_sprints_sprint_issues_issue_post**](DefaultApi.md#projects_planning_boards_sprints_sprint_issues_issue_post) | **POST** /projects/planning/boards/sprints/{sprint}/issues/{issue} | Add issue to sprint
[**projects_planning_boards_sprints_sprint_launch_post**](DefaultApi.md#projects_planning_boards_sprints_sprint_launch_post) | **POST** /projects/planning/boards/sprints/{sprint}/launch | Launch planned sprint
[**projects_planning_boards_sprints_sprint_patch**](DefaultApi.md#projects_planning_boards_sprints_sprint_patch) | **PATCH** /projects/planning/boards/sprints/{sprint} | Update sprint
[**projects_post**](DefaultApi.md#projects_post) | **POST** /projects | Create project
[**projects_private_projects_get**](DefaultApi.md#projects_private_projects_get) | **GET** /projects/private-projects | Get all private projects
[**projects_private_projects_project_request_access_post**](DefaultApi.md#projects_private_projects_project_request_access_post) | **POST** /projects/private-projects/{project}/request-access | Request access to project
[**projects_project_access_admins_get**](DefaultApi.md#projects_project_access_admins_get) | **GET** /projects/{project}/access/admins | Get all admins
[**projects_project_access_admins_profiles_post**](DefaultApi.md#projects_project_access_admins_profiles_post) | **POST** /projects/{project}/access/admins/profiles | Add administrator
[**projects_project_access_admins_profiles_profile_delete**](DefaultApi.md#projects_project_access_admins_profiles_profile_delete) | **DELETE** /projects/{project}/access/admins/profiles/{profile} | Remove administrator
[**projects_project_access_admins_teams_post**](DefaultApi.md#projects_project_access_admins_teams_post) | **POST** /projects/{project}/access/admins/teams | Add Administrators team
[**projects_project_access_admins_teams_team_id_delete**](DefaultApi.md#projects_project_access_admins_teams_team_id_delete) | **DELETE** /projects/{project}/access/admins/teams/{teamId} | Remove Administrators team
[**projects_project_access_collaborators_get**](DefaultApi.md#projects_project_access_collaborators_get) | **GET** /projects/{project}/access/collaborators | Get all collaborators' profiles
[**projects_project_access_collaborators_profiles_delete**](DefaultApi.md#projects_project_access_collaborators_profiles_delete) | **DELETE** /projects/{project}/access/collaborators/profiles | Remove a collaborator
[**projects_project_access_collaborators_profiles_get**](DefaultApi.md#projects_project_access_collaborators_profiles_get) | **GET** /projects/{project}/access/collaborators/profiles | Get all individual collaborators
[**projects_project_access_collaborators_profiles_post**](DefaultApi.md#projects_project_access_collaborators_profiles_post) | **POST** /projects/{project}/access/collaborators/profiles | Add a collaborator
[**projects_project_access_collaborators_teams_delete**](DefaultApi.md#projects_project_access_collaborators_teams_delete) | **DELETE** /projects/{project}/access/collaborators/teams | Remove a collaborators' team
[**projects_project_access_collaborators_teams_get**](DefaultApi.md#projects_project_access_collaborators_teams_get) | **GET** /projects/{project}/access/collaborators/teams | Get all collaborators' teams
[**projects_project_access_collaborators_teams_post**](DefaultApi.md#projects_project_access_collaborators_teams_post) | **POST** /projects/{project}/access/collaborators/teams | Add a collaborators' team
[**projects_project_access_member_profiles_get**](DefaultApi.md#projects_project_access_member_profiles_get) | **GET** /projects/{project}/access/member-profiles | Get all member profiles
[**projects_project_access_members_profiles_post**](DefaultApi.md#projects_project_access_members_profiles_post) | **POST** /projects/{project}/access/members/profiles | Add member
[**projects_project_access_members_profiles_profile_delete**](DefaultApi.md#projects_project_access_members_profiles_profile_delete) | **DELETE** /projects/{project}/access/members/profiles/{profile} | Remove member
[**projects_project_access_members_teams_team_id_delete**](DefaultApi.md#projects_project_access_members_teams_team_id_delete) | **DELETE** /projects/{project}/access/members/teams/{teamId} | Remove team
[**projects_project_access_viewers_get**](DefaultApi.md#projects_project_access_viewers_get) | **GET** /projects/{project}/access/viewers | Organization profiles that can view the project
[**projects_project_automation_deployment_targets_favorites_get**](DefaultApi.md#projects_project_automation_deployment_targets_favorites_get) | **GET** /projects/{project}/automation/deployment-targets/favorites | List Favorites
[**projects_project_automation_deployment_targets_get**](DefaultApi.md#projects_project_automation_deployment_targets_get) | **GET** /projects/{project}/automation/deployment-targets | Get all deployment targets
[**projects_project_automation_deployment_targets_post**](DefaultApi.md#projects_project_automation_deployment_targets_post) | **POST** /projects/{project}/automation/deployment-targets | Create deployment target
[**projects_project_automation_deployment_targets_target_delete**](DefaultApi.md#projects_project_automation_deployment_targets_target_delete) | **DELETE** /projects/{project}/automation/deployment-targets/{target} | Delete deployment target
[**projects_project_automation_deployment_targets_target_get**](DefaultApi.md#projects_project_automation_deployment_targets_target_get) | **GET** /projects/{project}/automation/deployment-targets/{target} | Get deployment target
[**projects_project_automation_deployment_targets_target_patch**](DefaultApi.md#projects_project_automation_deployment_targets_target_patch) | **PATCH** /projects/{project}/automation/deployment-targets/{target} | Update deployment target
[**projects_project_automation_deployments_fail_post**](DefaultApi.md#projects_project_automation_deployments_fail_post) | **POST** /projects/{project}/automation/deployments/fail | Fail deployment
[**projects_project_automation_deployments_finish_post**](DefaultApi.md#projects_project_automation_deployments_finish_post) | **POST** /projects/{project}/automation/deployments/finish | Finish deployment
[**projects_project_automation_deployments_get**](DefaultApi.md#projects_project_automation_deployments_get) | **GET** /projects/{project}/automation/deployments | Get all deployments
[**projects_project_automation_deployments_patch**](DefaultApi.md#projects_project_automation_deployments_patch) | **PATCH** /projects/{project}/automation/deployments | Update deployment
[**projects_project_automation_deployments_schedule_post**](DefaultApi.md#projects_project_automation_deployments_schedule_post) | **POST** /projects/{project}/automation/deployments/schedule | Schedule deployment
[**projects_project_automation_deployments_start_post**](DefaultApi.md#projects_project_automation_deployments_start_post) | **POST** /projects/{project}/automation/deployments/start | Start deployment
[**projects_project_automation_deployments_target_identifier_deployment_identifier_delete**](DefaultApi.md#projects_project_automation_deployments_target_identifier_deployment_identifier_delete) | **DELETE** /projects/{project}/automation/deployments/{targetIdentifier}/{deploymentIdentifier} | Delete deployment
[**projects_project_automation_deployments_target_identifier_deployment_identifier_get**](DefaultApi.md#projects_project_automation_deployments_target_identifier_deployment_identifier_get) | **GET** /projects/{project}/automation/deployments/{targetIdentifier}/{deploymentIdentifier} | Get deployment
[**projects_project_automation_graph_executions_get**](DefaultApi.md#projects_project_automation_graph_executions_get) | **GET** /projects/{project}/automation/graph-executions | Get all graph executions
[**projects_project_automation_jobs_get**](DefaultApi.md#projects_project_automation_jobs_get) | **GET** /projects/{project}/automation/jobs | Get all jobs
[**projects_project_automation_jobs_job_id_start_post**](DefaultApi.md#projects_project_automation_jobs_job_id_start_post) | **POST** /projects/{project}/automation/jobs/{jobId}/start | Start job
[**projects_project_code_reviews_code_discussions_discussion_id_accept_suggested_edit_post**](DefaultApi.md#projects_project_code_reviews_code_discussions_discussion_id_accept_suggested_edit_post) | **POST** /projects/{project}/code-reviews/code-discussions/{discussionId}/accept-suggested-edit | Accept suggested edit
[**projects_project_code_reviews_code_discussions_discussion_id_reject_suggested_edit_post**](DefaultApi.md#projects_project_code_reviews_code_discussions_discussion_id_reject_suggested_edit_post) | **POST** /projects/{project}/code-reviews/code-discussions/{discussionId}/reject-suggested-edit | Reject suggested edit
[**projects_project_code_reviews_code_discussions_discussion_id_reopen_suggested_edit_post**](DefaultApi.md#projects_project_code_reviews_code_discussions_discussion_id_reopen_suggested_edit_post) | **POST** /projects/{project}/code-reviews/code-discussions/{discussionId}/reopen-suggested-edit | Reopen suggested edit
[**projects_project_code_reviews_code_discussions_discussion_id_suggested_edit_patch**](DefaultApi.md#projects_project_code_reviews_code_discussions_discussion_id_suggested_edit_patch) | **PATCH** /projects/{project}/code-reviews/code-discussions/{discussionId}/suggested-edit | Alter suggested edit
[**projects_project_code_reviews_code_discussions_post**](DefaultApi.md#projects_project_code_reviews_code_discussions_post) | **POST** /projects/{project}/code-reviews/code-discussions | Create code discussion
[**projects_project_code_reviews_commit_set_review_post**](DefaultApi.md#projects_project_code_reviews_commit_set_review_post) | **POST** /projects/{project}/code-reviews/commit-set-review | Create review based on commit set
[**projects_project_code_reviews_get**](DefaultApi.md#projects_project_code_reviews_get) | **GET** /projects/{project}/code-reviews | Get all code reviews
[**projects_project_code_reviews_merge_requests_post**](DefaultApi.md#projects_project_code_reviews_merge_requests_post) | **POST** /projects/{project}/code-reviews/merge-requests | Create merge request
[**projects_project_code_reviews_review_id_description_patch**](DefaultApi.md#projects_project_code_reviews_review_id_description_patch) | **PATCH** /projects/{project}/code-reviews/{reviewId}/description | Edit review description
[**projects_project_code_reviews_review_id_details_get**](DefaultApi.md#projects_project_code_reviews_review_id_details_get) | **GET** /projects/{project}/code-reviews/{reviewId}/details | Get review details
[**projects_project_code_reviews_review_id_files_get**](DefaultApi.md#projects_project_code_reviews_review_id_files_get) | **GET** /projects/{project}/code-reviews/{reviewId}/files | Get the modified files in code review
[**projects_project_code_reviews_review_id_get**](DefaultApi.md#projects_project_code_reviews_review_id_get) | **GET** /projects/{project}/code-reviews/{reviewId} | Get code review
[**projects_project_code_reviews_review_id_make_read_only_patch**](DefaultApi.md#projects_project_code_reviews_review_id_make_read_only_patch) | **PATCH** /projects/{project}/code-reviews/{reviewId}/make-read-only | Make review read-only
[**projects_project_code_reviews_review_id_merge_files_get**](DefaultApi.md#projects_project_code_reviews_review_id_merge_files_get) | **GET** /projects/{project}/code-reviews/{reviewId}/merge-files | Get the Merge Request files
[**projects_project_code_reviews_review_id_merge_put**](DefaultApi.md#projects_project_code_reviews_review_id_merge_put) | **PUT** /projects/{project}/code-reviews/{reviewId}/merge | Merge a merge request
[**projects_project_code_reviews_review_id_participants_user_delete**](DefaultApi.md#projects_project_code_reviews_review_id_participants_user_delete) | **DELETE** /projects/{project}/code-reviews/{reviewId}/participants/{user} | Remove review participant
[**projects_project_code_reviews_review_id_participants_user_post**](DefaultApi.md#projects_project_code_reviews_review_id_participants_user_post) | **POST** /projects/{project}/code-reviews/{reviewId}/participants/{user} | Add review participant
[**projects_project_code_reviews_review_id_rebase_put**](DefaultApi.md#projects_project_code_reviews_review_id_rebase_put) | **PUT** /projects/{project}/code-reviews/{reviewId}/rebase | Rebase a merge request
[**projects_project_code_reviews_review_id_revisions_delete**](DefaultApi.md#projects_project_code_reviews_review_id_revisions_delete) | **DELETE** /projects/{project}/code-reviews/{reviewId}/revisions | Remove revisions from review
[**projects_project_code_reviews_review_id_revisions_post**](DefaultApi.md#projects_project_code_reviews_review_id_revisions_post) | **POST** /projects/{project}/code-reviews/{reviewId}/revisions | Add revisions to review
[**projects_project_code_reviews_review_id_state_patch**](DefaultApi.md#projects_project_code_reviews_review_id_state_patch) | **PATCH** /projects/{project}/code-reviews/{reviewId}/state | Edit review state
[**projects_project_code_reviews_review_id_suggested_reviewers_get**](DefaultApi.md#projects_project_code_reviews_review_id_suggested_reviewers_get) | **GET** /projects/{project}/code-reviews/{reviewId}/suggested-reviewers | Get suggested reviewers
[**projects_project_code_reviews_review_id_title_patch**](DefaultApi.md#projects_project_code_reviews_review_id_title_patch) | **PATCH** /projects/{project}/code-reviews/{reviewId}/title | Edit review title
[**projects_project_code_reviews_review_id_unbound_discussions_discussion_id_toggle_patch**](DefaultApi.md#projects_project_code_reviews_review_id_unbound_discussions_discussion_id_toggle_patch) | **PATCH** /projects/{project}/code-reviews/{reviewId}/unbound-discussions/{discussionId}/toggle | Toggle unbound discussion resolution
[**projects_project_code_reviews_review_id_unbound_discussions_get**](DefaultApi.md#projects_project_code_reviews_review_id_unbound_discussions_get) | **GET** /projects/{project}/code-reviews/{reviewId}/unbound-discussions | Get all unbound discussions
[**projects_project_code_reviews_review_id_unbound_discussions_post**](DefaultApi.md#projects_project_code_reviews_review_id_unbound_discussions_post) | **POST** /projects/{project}/code-reviews/{reviewId}/unbound-discussions | Create unbound discussion
[**projects_project_code_reviews_safe_merge_delete**](DefaultApi.md#projects_project_code_reviews_safe_merge_delete) | **DELETE** /projects/{project}/code-reviews/safe-merge | Stop Safe Merge execution
[**projects_project_code_reviews_safe_merge_get**](DefaultApi.md#projects_project_code_reviews_safe_merge_get) | **GET** /projects/{project}/code-reviews/safe-merge | Get Safe Merge execution status
[**projects_project_code_reviews_safe_merge_post**](DefaultApi.md#projects_project_code_reviews_safe_merge_post) | **POST** /projects/{project}/code-reviews/safe-merge | Start Safe Merge execution status
[**projects_project_delete**](DefaultApi.md#projects_project_delete) | **DELETE** /projects/{project} | Delete project
[**projects_project_documents_document_id_access_get**](DefaultApi.md#projects_project_documents_document_id_access_get) | **GET** /projects/{project}/documents/{documentId}/access | Document own access permissions
[**projects_project_documents_document_id_access_patch**](DefaultApi.md#projects_project_documents_document_id_access_patch) | **PATCH** /projects/{project}/documents/{documentId}/access | Update document access permissions
[**projects_project_documents_document_id_copy_post**](DefaultApi.md#projects_project_documents_document_id_copy_post) | **POST** /projects/{project}/documents/{documentId}/copy | Copy document
[**projects_project_documents_document_id_delete**](DefaultApi.md#projects_project_documents_document_id_delete) | **DELETE** /projects/{project}/documents/{documentId} | Archive document
[**projects_project_documents_document_id_get**](DefaultApi.md#projects_project_documents_document_id_get) | **GET** /projects/{project}/documents/{documentId} | Get document
[**projects_project_documents_document_id_move_patch**](DefaultApi.md#projects_project_documents_document_id_move_patch) | **PATCH** /projects/{project}/documents/{documentId}/move | Move document
[**projects_project_documents_document_id_patch**](DefaultApi.md#projects_project_documents_document_id_patch) | **PATCH** /projects/{project}/documents/{documentId} | Update document
[**projects_project_documents_document_id_unarchive_patch**](DefaultApi.md#projects_project_documents_document_id_unarchive_patch) | **PATCH** /projects/{project}/documents/{documentId}/unarchive | Unarchive document
[**projects_project_documents_folders_folder_access_get**](DefaultApi.md#projects_project_documents_folders_folder_access_get) | **GET** /projects/{project}/documents/folders/{folder}/access | Folder own access permissions
[**projects_project_documents_folders_folder_access_patch**](DefaultApi.md#projects_project_documents_folders_folder_access_patch) | **PATCH** /projects/{project}/documents/folders/{folder}/access | Update folder access permissions
[**projects_project_documents_folders_folder_delete**](DefaultApi.md#projects_project_documents_folders_folder_delete) | **DELETE** /projects/{project}/documents/folders/{folder} | Archive folder
[**projects_project_documents_folders_folder_documents_get**](DefaultApi.md#projects_project_documents_folders_folder_documents_get) | **GET** /projects/{project}/documents/folders/{folder}/documents | List documents in folder
[**projects_project_documents_folders_folder_get**](DefaultApi.md#projects_project_documents_folders_folder_get) | **GET** /projects/{project}/documents/folders/{folder} | Get folder
[**projects_project_documents_folders_folder_introduction_delete**](DefaultApi.md#projects_project_documents_folders_folder_introduction_delete) | **DELETE** /projects/{project}/documents/folders/{folder}/introduction | Remove folder introduction
[**projects_project_documents_folders_folder_introduction_document_id_patch**](DefaultApi.md#projects_project_documents_folders_folder_introduction_document_id_patch) | **PATCH** /projects/{project}/documents/folders/{folder}/introduction/{documentId} | Add folder introduction
[**projects_project_documents_folders_folder_move_patch**](DefaultApi.md#projects_project_documents_folders_folder_move_patch) | **PATCH** /projects/{project}/documents/folders/{folder}/move | Move folder
[**projects_project_documents_folders_folder_patch**](DefaultApi.md#projects_project_documents_folders_folder_patch) | **PATCH** /projects/{project}/documents/folders/{folder} | Rename folder
[**projects_project_documents_folders_folder_search_get**](DefaultApi.md#projects_project_documents_folders_folder_search_get) | **GET** /projects/{project}/documents/folders/{folder}/search | Search documents and folders
[**projects_project_documents_folders_folder_subfolders_get**](DefaultApi.md#projects_project_documents_folders_folder_subfolders_get) | **GET** /projects/{project}/documents/folders/{folder}/subfolders | List subfolders
[**projects_project_documents_folders_post**](DefaultApi.md#projects_project_documents_folders_post) | **POST** /projects/{project}/documents/folders | Create folder
[**projects_project_documents_post**](DefaultApi.md#projects_project_documents_post) | **POST** /projects/{project}/documents | Create document
[**projects_project_feature_pins_patch**](DefaultApi.md#projects_project_feature_pins_patch) | **PATCH** /projects/{project}/feature-pins | Update feature pin
[**projects_project_get**](DefaultApi.md#projects_project_get) | **GET** /projects/{project} | Get project
[**projects_project_packages_repositories_get**](DefaultApi.md#projects_project_packages_repositories_get) | **GET** /projects/{project}/packages/repositories | Get repositories
[**projects_project_packages_repositories_post**](DefaultApi.md#projects_project_packages_repositories_post) | **POST** /projects/{project}/packages/repositories | Create new repository
[**projects_project_packages_repositories_repository_access_get**](DefaultApi.md#projects_project_packages_repositories_repository_access_get) | **GET** /projects/{project}/packages/repositories/{repository}/access | Get repository own access
[**projects_project_packages_repositories_repository_access_patch**](DefaultApi.md#projects_project_packages_repositories_repository_access_patch) | **PATCH** /projects/{project}/packages/repositories/{repository}/access | Update repository own access
[**projects_project_packages_repositories_repository_cleanup_dry_post**](DefaultApi.md#projects_project_packages_repositories_repository_cleanup_dry_post) | **POST** /projects/{project}/packages/repositories/{repository}/cleanup/dry | Dry run repository cleanup
[**projects_project_packages_repositories_repository_cleanup_post**](DefaultApi.md#projects_project_packages_repositories_repository_cleanup_post) | **POST** /projects/{project}/packages/repositories/{repository}/cleanup | Cleanup repository
[**projects_project_packages_repositories_repository_connections_connection_id_publish_get**](DefaultApi.md#projects_project_packages_repositories_repository_connections_connection_id_publish_get) | **GET** /projects/{project}/packages/repositories/{repository}/connections/{connectionId}/publish | Get list of publishing to remote repository
[**projects_project_packages_repositories_repository_connections_connection_id_publish_post**](DefaultApi.md#projects_project_packages_repositories_repository_connections_connection_id_publish_post) | **POST** /projects/{project}/packages/repositories/{repository}/connections/{connectionId}/publish | Publish packages to remote repository
[**projects_project_packages_repositories_repository_connections_get**](DefaultApi.md#projects_project_packages_repositories_repository_connections_get) | **GET** /projects/{project}/packages/repositories/{repository}/connections | Get all remote repositories
[**projects_project_packages_repositories_repository_delete**](DefaultApi.md#projects_project_packages_repositories_repository_delete) | **DELETE** /projects/{project}/packages/repositories/{repository} | Delete repository
[**projects_project_packages_repositories_repository_files_folderfolder_path_delete**](DefaultApi.md#projects_project_packages_repositories_repository_files_folderfolder_path_delete) | **DELETE** /projects/{project}/packages/repositories/{repository}/files/folder:{folderPath} | Delete folder
[**projects_project_packages_repositories_repository_files_get**](DefaultApi.md#projects_project_packages_repositories_repository_files_get) | **GET** /projects/{project}/packages/repositories/{repository}/files | Get list of files
[**projects_project_packages_repositories_repository_files_namefile_path_delete**](DefaultApi.md#projects_project_packages_repositories_repository_files_namefile_path_delete) | **DELETE** /projects/{project}/packages/repositories/{repository}/files/name:{filePath} | Delete file
[**projects_project_packages_repositories_repository_files_namefile_path_get**](DefaultApi.md#projects_project_packages_repositories_repository_files_namefile_path_get) | **GET** /projects/{project}/packages/repositories/{repository}/files/name:{filePath} | Get file details
[**projects_project_packages_repositories_repository_get**](DefaultApi.md#projects_project_packages_repositories_repository_get) | **GET** /projects/{project}/packages/repositories/{repository} | Get repository
[**projects_project_packages_repositories_repository_packages_get**](DefaultApi.md#projects_project_packages_repositories_repository_packages_get) | **GET** /projects/{project}/packages/repositories/{repository}/packages | Get all packages
[**projects_project_packages_repositories_repository_packages_namepackage_name_delete**](DefaultApi.md#projects_project_packages_repositories_repository_packages_namepackage_name_delete) | **DELETE** /projects/{project}/packages/repositories/{repository}/packages/name:{packageName} | Delete package
[**projects_project_packages_repositories_repository_packages_namepackage_name_metadata_get**](DefaultApi.md#projects_project_packages_repositories_repository_packages_namepackage_name_metadata_get) | **GET** /projects/{project}/packages/repositories/{repository}/packages/name:{packageName}/metadata | Get package metadata
[**projects_project_packages_repositories_repository_packages_namepackage_name_metadata_put**](DefaultApi.md#projects_project_packages_repositories_repository_packages_namepackage_name_metadata_put) | **PUT** /projects/{project}/packages/repositories/{repository}/packages/name:{packageName}/metadata | Report package  metadata
[**projects_project_packages_repositories_repository_packages_namepackage_name_metadata_versionpackage_version_put**](DefaultApi.md#projects_project_packages_repositories_repository_packages_namepackage_name_metadata_versionpackage_version_put) | **PUT** /projects/{project}/packages/repositories/{repository}/packages/name:{packageName}/metadata/version:{packageVersion} | Report package version metadata
[**projects_project_packages_repositories_repository_packages_namepackage_name_versions_get**](DefaultApi.md#projects_project_packages_repositories_repository_packages_namepackage_name_versions_get) | **GET** /projects/{project}/packages/repositories/{repository}/packages/name:{packageName}/versions | Get all package versions
[**projects_project_packages_repositories_repository_packages_namepackage_name_versions_versionpackage_version_delete**](DefaultApi.md#projects_project_packages_repositories_repository_packages_namepackage_name_versions_versionpackage_version_delete) | **DELETE** /projects/{project}/packages/repositories/{repository}/packages/name:{packageName}/versions/version:{packageVersion} | Delete package version
[**projects_project_packages_repositories_repository_packages_namepackage_name_versions_versionpackage_version_get**](DefaultApi.md#projects_project_packages_repositories_repository_packages_namepackage_name_versions_versionpackage_version_get) | **GET** /projects/{project}/packages/repositories/{repository}/packages/name:{packageName}/versions/version:{packageVersion} | Get package version details
[**projects_project_packages_repositories_repository_patch**](DefaultApi.md#projects_project_packages_repositories_repository_patch) | **PATCH** /projects/{project}/packages/repositories/{repository} | Update repository
[**projects_project_packages_repositories_repository_url_get**](DefaultApi.md#projects_project_packages_repositories_repository_url_get) | **GET** /projects/{project}/packages/repositories/{repository}/url | Get repository URL
[**projects_project_packages_repositories_typetype_repositoryrepository_name_packages_get**](DefaultApi.md#projects_project_packages_repositories_typetype_repositoryrepository_name_packages_get) | **GET** /projects/{project}/packages/repositories/type:{type}/repository:{repositoryName}/packages | Get all packages
[**projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_get**](DefaultApi.md#projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_get) | **GET** /projects/{project}/packages/repositories/type:{type}/repository:{repositoryName}/packages/name:{packageName}/versions | Get all package versions
[**projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_versionpackage_version_delete**](DefaultApi.md#projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_versionpackage_version_delete) | **DELETE** /projects/{project}/packages/repositories/type:{type}/repository:{repositoryName}/packages/name:{packageName}/versions/version:{packageVersion} | Delete package version
[**projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_versionpackage_version_get**](DefaultApi.md#projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_versionpackage_version_get) | **GET** /projects/{project}/packages/repositories/type:{type}/repository:{repositoryName}/packages/name:{packageName}/versions/version:{packageVersion} | Get package version details
[**projects_project_packages_search_get**](DefaultApi.md#projects_project_packages_search_get) | **GET** /projects/{project}/packages/search | Find packages in repository
[**projects_project_packages_types_get**](DefaultApi.md#projects_project_packages_types_get) | **GET** /projects/{project}/packages/types | Get all types
[**projects_project_patch**](DefaultApi.md#projects_project_patch) | **PATCH** /projects/{project} | Update project
[**projects_project_people_members_by_ids_get**](DefaultApi.md#projects_project_people_members_by_ids_get) | **GET** /projects/{project}/people/members/by-ids | Get participants by profiles
[**projects_project_people_members_get**](DefaultApi.md#projects_project_people_members_get) | **GET** /projects/{project}/people/members | Get all participants
[**projects_project_people_members_profile_delete**](DefaultApi.md#projects_project_people_members_profile_delete) | **DELETE** /projects/{project}/people/members/{profile} | Remove participant
[**projects_project_people_members_update_post**](DefaultApi.md#projects_project_people_members_update_post) | **POST** /projects/{project}/people/members/update | Update participant roles
[**projects_project_people_teams_by_ids_get**](DefaultApi.md#projects_project_people_teams_by_ids_get) | **GET** /projects/{project}/people/teams/by-ids | Get participants by teams
[**projects_project_people_teams_get**](DefaultApi.md#projects_project_people_teams_get) | **GET** /projects/{project}/people/teams | Get all participants
[**projects_project_people_teams_team_delete**](DefaultApi.md#projects_project_people_teams_team_delete) | **DELETE** /projects/{project}/people/teams/{team} | Remove participant
[**projects_project_people_teams_update_post**](DefaultApi.md#projects_project_people_teams_update_post) | **POST** /projects/{project}/people/teams/update | Update participant roles
[**projects_project_personal_feature_pins_patch**](DefaultApi.md#projects_project_personal_feature_pins_patch) | **PATCH** /projects/{project}/personal-feature-pins | Update personal feature pin
[**projects_project_planning_boards_board_archive_delete**](DefaultApi.md#projects_project_planning_boards_board_archive_delete) | **DELETE** /projects/{project}/planning/boards/{board}/archive | Archive board
[**projects_project_planning_boards_get**](DefaultApi.md#projects_project_planning_boards_get) | **GET** /projects/{project}/planning/boards | Get all boards
[**projects_project_planning_boards_post**](DefaultApi.md#projects_project_planning_boards_post) | **POST** /projects/{project}/planning/boards | Create board
[**projects_project_planning_boards_sprints_get**](DefaultApi.md#projects_project_planning_boards_sprints_get) | **GET** /projects/{project}/planning/boards/sprints | Get all sprints
[**projects_project_planning_boards_starred_get**](DefaultApi.md#projects_project_planning_boards_starred_get) | **GET** /projects/{project}/planning/boards/starred | Get all starred boards
[**projects_project_planning_checklists_checklist_id_delete**](DefaultApi.md#projects_project_planning_checklists_checklist_id_delete) | **DELETE** /projects/{project}/planning/checklists/{checklistId} | Delete checklist
[**projects_project_planning_checklists_checklist_id_full_checklist_tree_get**](DefaultApi.md#projects_project_planning_checklists_checklist_id_full_checklist_tree_get) | **GET** /projects/{project}/planning/checklists/{checklistId}/full-checklist-tree | Get full checklist tree
[**projects_project_planning_checklists_checklist_id_import_post**](DefaultApi.md#projects_project_planning_checklists_checklist_id_import_post) | **POST** /projects/{project}/planning/checklists/{checklistId}/import | Import checklist lines
[**projects_project_planning_checklists_checklist_id_patch**](DefaultApi.md#projects_project_planning_checklists_checklist_id_patch) | **PATCH** /projects/{project}/planning/checklists/{checklistId} | Update checklist
[**projects_project_planning_checklists_get**](DefaultApi.md#projects_project_planning_checklists_get) | **GET** /projects/{project}/planning/checklists | Get all checklists
[**projects_project_planning_checklists_import_post**](DefaultApi.md#projects_project_planning_checklists_import_post) | **POST** /projects/{project}/planning/checklists/import | Import checklist
[**projects_project_planning_checklists_post**](DefaultApi.md#projects_project_planning_checklists_post) | **POST** /projects/{project}/planning/checklists | Create checklist
[**projects_project_planning_checklists_starred_get**](DefaultApi.md#projects_project_planning_checklists_starred_get) | **GET** /projects/{project}/planning/checklists/starred | Get all starred checklists
[**projects_project_planning_issues_fields_order_get**](DefaultApi.md#projects_project_planning_issues_fields_order_get) | **GET** /projects/{project}/planning/issues/fields/order | Get issue field order
[**projects_project_planning_issues_fields_order_patch**](DefaultApi.md#projects_project_planning_issues_fields_order_patch) | **PATCH** /projects/{project}/planning/issues/fields/order | Set issue field order
[**projects_project_planning_issues_fields_visibility_get**](DefaultApi.md#projects_project_planning_issues_fields_visibility_get) | **GET** /projects/{project}/planning/issues/fields/visibility | Get issue field visibility
[**projects_project_planning_issues_fields_visibility_patch**](DefaultApi.md#projects_project_planning_issues_fields_visibility_patch) | **PATCH** /projects/{project}/planning/issues/fields/visibility | Update issue field visibility
[**projects_project_planning_issues_get**](DefaultApi.md#projects_project_planning_issues_get) | **GET** /projects/{project}/planning/issues | Get all issues
[**projects_project_planning_issues_import_post**](DefaultApi.md#projects_project_planning_issues_import_post) | **POST** /projects/{project}/planning/issues/import | Import issues
[**projects_project_planning_issues_issue_id_attachment_attachment_id_delete**](DefaultApi.md#projects_project_planning_issues_issue_id_attachment_attachment_id_delete) | **DELETE** /projects/{project}/planning/issues/{issueId}/attachment/{attachmentId} | Remove attachment
[**projects_project_planning_issues_issue_id_attachment_post**](DefaultApi.md#projects_project_planning_issues_issue_id_attachment_post) | **POST** /projects/{project}/planning/issues/{issueId}/attachment | Add attachment
[**projects_project_planning_issues_issue_id_attachments_delete**](DefaultApi.md#projects_project_planning_issues_issue_id_attachments_delete) | **DELETE** /projects/{project}/planning/issues/{issueId}/attachments | Remove attachments
[**projects_project_planning_issues_issue_id_attachments_post**](DefaultApi.md#projects_project_planning_issues_issue_id_attachments_post) | **POST** /projects/{project}/planning/issues/{issueId}/attachments | Add attachments
[**projects_project_planning_issues_issue_id_checklists_checklist_id_delete**](DefaultApi.md#projects_project_planning_issues_issue_id_checklists_checklist_id_delete) | **DELETE** /projects/{project}/planning/issues/{issueId}/checklists/{checklistId} | Remove issue checklist
[**projects_project_planning_issues_issue_id_checklists_checklist_id_post**](DefaultApi.md#projects_project_planning_issues_issue_id_checklists_checklist_id_post) | **POST** /projects/{project}/planning/issues/{issueId}/checklists/{checklistId} | Add issue checklist
[**projects_project_planning_issues_issue_id_code_reviews_delete**](DefaultApi.md#projects_project_planning_issues_issue_id_code_reviews_delete) | **DELETE** /projects/{project}/planning/issues/{issueId}/code-reviews | Remove code review links
[**projects_project_planning_issues_issue_id_code_reviews_post**](DefaultApi.md#projects_project_planning_issues_issue_id_code_reviews_post) | **POST** /projects/{project}/planning/issues/{issueId}/code-reviews | Add code review links
[**projects_project_planning_issues_issue_id_comments_import_post**](DefaultApi.md#projects_project_planning_issues_issue_id_comments_import_post) | **POST** /projects/{project}/planning/issues/{issueId}/comments/import | Import issue comment history
[**projects_project_planning_issues_issue_id_commits_delete**](DefaultApi.md#projects_project_planning_issues_issue_id_commits_delete) | **DELETE** /projects/{project}/planning/issues/{issueId}/commits | Remove commit links
[**projects_project_planning_issues_issue_id_commits_post**](DefaultApi.md#projects_project_planning_issues_issue_id_commits_post) | **POST** /projects/{project}/planning/issues/{issueId}/commits | Add commit links
[**projects_project_planning_issues_issue_id_delete**](DefaultApi.md#projects_project_planning_issues_issue_id_delete) | **DELETE** /projects/{project}/planning/issues/{issueId} | Delete issue
[**projects_project_planning_issues_issue_id_get**](DefaultApi.md#projects_project_planning_issues_issue_id_get) | **GET** /projects/{project}/planning/issues/{issueId} | Get issue
[**projects_project_planning_issues_issue_id_patch**](DefaultApi.md#projects_project_planning_issues_issue_id_patch) | **PATCH** /projects/{project}/planning/issues/{issueId} | Update issue
[**projects_project_planning_issues_issue_id_restore_post**](DefaultApi.md#projects_project_planning_issues_issue_id_restore_post) | **POST** /projects/{project}/planning/issues/{issueId}/restore | Restore issue
[**projects_project_planning_issues_issue_id_tags_tag_id_delete**](DefaultApi.md#projects_project_planning_issues_issue_id_tags_tag_id_delete) | **DELETE** /projects/{project}/planning/issues/{issueId}/tags/{tagId} | Remove issue tag
[**projects_project_planning_issues_issue_id_tags_tag_id_post**](DefaultApi.md#projects_project_planning_issues_issue_id_tags_tag_id_post) | **POST** /projects/{project}/planning/issues/{issueId}/tags/{tagId} | Add issue tag
[**projects_project_planning_issues_issue_id_toggle_resolved_post**](DefaultApi.md#projects_project_planning_issues_issue_id_toggle_resolved_post) | **POST** /projects/{project}/planning/issues/{issueId}/toggle-resolved | Toggle issue resolved status
[**projects_project_planning_issues_numbernumber_get**](DefaultApi.md#projects_project_planning_issues_numbernumber_get) | **GET** /projects/{project}/planning/issues/number:{number} | Get issue by number
[**projects_project_planning_issues_post**](DefaultApi.md#projects_project_planning_issues_post) | **POST** /projects/{project}/planning/issues | Create issue
[**projects_project_planning_issues_statuses_auto_update_on_merge_request_merge_get**](DefaultApi.md#projects_project_planning_issues_statuses_auto_update_on_merge_request_merge_get) | **GET** /projects/{project}/planning/issues/statuses/auto-update-on-merge-request-merge | Get auto update target issue status for merge request merge
[**projects_project_planning_issues_statuses_auto_update_on_merge_request_merge_patch**](DefaultApi.md#projects_project_planning_issues_statuses_auto_update_on_merge_request_merge_patch) | **PATCH** /projects/{project}/planning/issues/statuses/auto-update-on-merge-request-merge | Set auto update target issue status for merge request merge
[**projects_project_planning_issues_statuses_distribution_get**](DefaultApi.md#projects_project_planning_issues_statuses_distribution_get) | **GET** /projects/{project}/planning/issues/statuses/distribution | Get issue status distribution
[**projects_project_planning_issues_statuses_get**](DefaultApi.md#projects_project_planning_issues_statuses_get) | **GET** /projects/{project}/planning/issues/statuses | Get all issue statuses
[**projects_project_planning_issues_statuses_patch**](DefaultApi.md#projects_project_planning_issues_statuses_patch) | **PATCH** /projects/{project}/planning/issues/statuses | Update issue statuses list
[**projects_project_planning_issues_sync_batch_get**](DefaultApi.md#projects_project_planning_issues_sync_batch_get) | **GET** /projects/{project}/planning/issues/sync-batch | Get sync batch
[**projects_project_planning_tags_get**](DefaultApi.md#projects_project_planning_tags_get) | **GET** /projects/{project}/planning/tags | Get all hierarchical tags
[**projects_project_planning_tags_post**](DefaultApi.md#projects_project_planning_tags_post) | **POST** /projects/{project}/planning/tags | Create hierarchical tag
[**projects_project_repositories_repository_additional_info_get**](DefaultApi.md#projects_project_repositories_repository_additional_info_get) | **GET** /projects/{project}/repositories/{repository}/additional-info | Get additional repository info
[**projects_project_repositories_repository_changes_get**](DefaultApi.md#projects_project_repositories_repository_changes_get) | **GET** /projects/{project}/repositories/{repository}/changes | Get commit changes
[**projects_project_repositories_repository_cherry_pick_commit_post**](DefaultApi.md#projects_project_repositories_repository_cherry_pick_commit_post) | **POST** /projects/{project}/repositories/{repository}/cherry-pick-commit | Cherry pick commit
[**projects_project_repositories_repository_commit_branches_get**](DefaultApi.md#projects_project_repositories_repository_commit_branches_get) | **GET** /projects/{project}/repositories/{repository}/commit-branches | List the heads which contains given commit
[**projects_project_repositories_repository_commit_post**](DefaultApi.md#projects_project_repositories_repository_commit_post) | **POST** /projects/{project}/repositories/{repository}/commit | Commit changes to repository
[**projects_project_repositories_repository_commits_get**](DefaultApi.md#projects_project_repositories_repository_commits_get) | **GET** /projects/{project}/repositories/{repository}/commits | List commits matching query
[**projects_project_repositories_repository_default_branch_get**](DefaultApi.md#projects_project_repositories_repository_default_branch_get) | **GET** /projects/{project}/repositories/{repository}/default-branch | Get repository default branch
[**projects_project_repositories_repository_default_branch_post**](DefaultApi.md#projects_project_repositories_repository_default_branch_post) | **POST** /projects/{project}/repositories/{repository}/default-branch | Set repository default branch
[**projects_project_repositories_repository_delete**](DefaultApi.md#projects_project_repositories_repository_delete) | **DELETE** /projects/{project}/repositories/{repository} | Delete repository
[**projects_project_repositories_repository_delete_branch_post**](DefaultApi.md#projects_project_repositories_repository_delete_branch_post) | **POST** /projects/{project}/repositories/{repository}/delete-branch | Delete branch
[**projects_project_repositories_repository_description_post**](DefaultApi.md#projects_project_repositories_repository_description_post) | **POST** /projects/{project}/repositories/{repository}/description | Set repository description
[**projects_project_repositories_repository_files_get**](DefaultApi.md#projects_project_repositories_repository_files_get) | **GET** /projects/{project}/repositories/{repository}/files | List files in directory
[**projects_project_repositories_repository_gc_post**](DefaultApi.md#projects_project_repositories_repository_gc_post) | **POST** /projects/{project}/repositories/{repository}/gc | Invoke garbage collection on repository
[**projects_project_repositories_repository_get**](DefaultApi.md#projects_project_repositories_repository_get) | **GET** /projects/{project}/repositories/{repository} | Get repository info
[**projects_project_repositories_repository_head_post**](DefaultApi.md#projects_project_repositories_repository_head_post) | **POST** /projects/{project}/repositories/{repository}/head | Set head to given target commit
[**projects_project_repositories_repository_heads_get**](DefaultApi.md#projects_project_repositories_repository_heads_get) | **GET** /projects/{project}/repositories/{repository}/heads | Get repository heads
[**projects_project_repositories_repository_inline_merge_diff_get**](DefaultApi.md#projects_project_repositories_repository_inline_merge_diff_get) | **GET** /projects/{project}/repositories/{repository}/inline-merge-diff | Get inline merge diff
[**projects_project_repositories_repository_merge_branch_post**](DefaultApi.md#projects_project_repositories_repository_merge_branch_post) | **POST** /projects/{project}/repositories/{repository}/merge-branch | Merge branch
[**projects_project_repositories_repository_merge_preview_get**](DefaultApi.md#projects_project_repositories_repository_merge_preview_get) | **GET** /projects/{project}/repositories/{repository}/merge-preview | List files to be merged on merge branches
[**projects_project_repositories_repository_merge_preview_status_get**](DefaultApi.md#projects_project_repositories_repository_merge_preview_status_get) | **GET** /projects/{project}/repositories/{repository}/merge-preview-status | Preview merge branches result
[**projects_project_repositories_repository_migrate_post**](DefaultApi.md#projects_project_repositories_repository_migrate_post) | **POST** /projects/{project}/repositories/{repository}/migrate | Migrate repository
[**projects_project_repositories_repository_post**](DefaultApi.md#projects_project_repositories_repository_post) | **POST** /projects/{project}/repositories/{repository} | Create new repository
[**projects_project_repositories_repository_readonly_get**](DefaultApi.md#projects_project_repositories_repository_readonly_get) | **GET** /projects/{project}/repositories/{repository}/readonly | Get repository frozen state
[**projects_project_repositories_repository_readonly_post**](DefaultApi.md#projects_project_repositories_repository_readonly_post) | **POST** /projects/{project}/repositories/{repository}/readonly | Set repository frozen state
[**projects_project_repositories_repository_rebase_branch_post**](DefaultApi.md#projects_project_repositories_repository_rebase_branch_post) | **POST** /projects/{project}/repositories/{repository}/rebase-branch | Rebase branch
[**projects_project_repositories_repository_revisions_revision_external_checks_get**](DefaultApi.md#projects_project_repositories_repository_revisions_revision_external_checks_get) | **GET** /projects/{project}/repositories/{repository}/revisions/{revision}/external-checks | Get external checks for a commit
[**projects_project_repositories_repository_revisions_revision_external_checks_post**](DefaultApi.md#projects_project_repositories_repository_revisions_revision_external_checks_post) | **POST** /projects/{project}/repositories/{repository}/revisions/{revision}/external-checks | Report external check status
[**projects_project_repositories_repository_settings_get**](DefaultApi.md#projects_project_repositories_repository_settings_get) | **GET** /projects/{project}/repositories/{repository}/settings | Get repository settings
[**projects_project_repositories_repository_settings_post**](DefaultApi.md#projects_project_repositories_repository_settings_post) | **POST** /projects/{project}/repositories/{repository}/settings | Set repository settings
[**projects_project_repositories_repository_url_get**](DefaultApi.md#projects_project_repositories_repository_url_get) | **GET** /projects/{project}/repositories/{repository}/url | Get remote URL of a Git repository
[**projects_project_repositories_test_connection_post**](DefaultApi.md#projects_project_repositories_test_connection_post) | **POST** /projects/{project}/repositories/test-connection | Test Remote Connection
[**projects_project_responsibilities_responsibility_id_assignees_profile_id_delete**](DefaultApi.md#projects_project_responsibilities_responsibility_id_assignees_profile_id_delete) | **DELETE** /projects/{project}/responsibilities/{responsibilityId}/assignees/{profileId} | Remove responsible
[**projects_project_responsibilities_responsibility_id_assignees_profile_id_post**](DefaultApi.md#projects_project_responsibilities_responsibility_id_assignees_profile_id_post) | **POST** /projects/{project}/responsibilities/{responsibilityId}/assignees/{profileId} | Assign responsible
[**projects_project_responsibilities_scheme_get**](DefaultApi.md#projects_project_responsibilities_scheme_get) | **GET** /projects/{project}/responsibilities/scheme | Get project responsibility scheme
[**projects_project_responsibilities_subjects_post**](DefaultApi.md#projects_project_responsibilities_subjects_post) | **POST** /projects/{project}/responsibilities/subjects | Add responsibility subject
[**projects_project_responsibilities_subjects_subject_id_patch**](DefaultApi.md#projects_project_responsibilities_subjects_subject_id_patch) | **PATCH** /projects/{project}/responsibilities/subjects/{subjectId} | Edit responsibility subject
[**projects_repositories_find_get**](DefaultApi.md#projects_repositories_find_get) | **GET** /projects/repositories/find | Find Repositories
[**projects_responsibilities_post**](DefaultApi.md#projects_responsibilities_post) | **POST** /projects/responsibilities | Add responsibility
[**projects_responsibilities_responsibility_id_delete**](DefaultApi.md#projects_responsibilities_responsibility_id_delete) | **DELETE** /projects/responsibilities/{responsibilityId} | Delete responsibility
[**projects_responsibilities_responsibility_id_patch**](DefaultApi.md#projects_responsibilities_responsibility_id_patch) | **PATCH** /projects/responsibilities/{responsibilityId} | Update responsibility
[**projects_responsibilities_subjects_subject_id_delete**](DefaultApi.md#projects_responsibilities_subjects_subject_id_delete) | **DELETE** /projects/responsibilities/subjects/{subjectId} | Delete responsibility subject
[**projects_right_coderight_code_get**](DefaultApi.md#projects_right_coderight_code_get) | **GET** /projects/right-code:{rightCode} | Get all projects with right
[**projects_right_unique_coderight_get**](DefaultApi.md#projects_right_unique_coderight_get) | **GET** /projects/right-unique-code:{right} | Get all projects with right code
[**projects_secrets_default_bundle_get**](DefaultApi.md#projects_secrets_default_bundle_get) | **GET** /projects/secrets/default-bundle | Get all default bundle
[**projects_secrets_default_bundle_post**](DefaultApi.md#projects_secrets_default_bundle_post) | **POST** /projects/secrets/default-bundle | Create default bundle
[**projects_secrets_id_delete**](DefaultApi.md#projects_secrets_id_delete) | **DELETE** /projects/secrets/{id} | Delete secret
[**projects_secrets_id_patch**](DefaultApi.md#projects_secrets_id_patch) | **PATCH** /projects/secrets/{id} | Update secret
[**projects_secrets_in_default_bundle_get**](DefaultApi.md#projects_secrets_in_default_bundle_get) | **GET** /projects/secrets/in-default-bundle | Get all in default bundle
[**projects_secrets_in_default_bundle_post**](DefaultApi.md#projects_secrets_in_default_bundle_post) | **POST** /projects/secrets/in-default-bundle | Create in default bundle
[**projects_tags_get**](DefaultApi.md#projects_tags_get) | **GET** /projects/tags | Get all tags
[**projects_tags_track_access_post**](DefaultApi.md#projects_tags_track_access_post) | **POST** /projects/tags/track-access | Track tag access
[**projects_teamteam_get**](DefaultApi.md#projects_teamteam_get) | **GET** /projects/team:{team} | Get all projects by team
[**projects_vault_get**](DefaultApi.md#projects_vault_get) | **GET** /projects/vault | Get vault
[**projects_vault_id_delete**](DefaultApi.md#projects_vault_id_delete) | **DELETE** /projects/vault/{id} | Delete vault
[**projects_vault_id_patch**](DefaultApi.md#projects_vault_id_patch) | **PATCH** /projects/vault/{id} | Update vault
[**projects_vault_post**](DefaultApi.md#projects_vault_post) | **POST** /projects/vault | Create vault
[**public_holidays_calendars_get**](DefaultApi.md#public_holidays_calendars_get) | **GET** /public-holidays/calendars | Get all calendars
[**public_holidays_calendars_id_delete**](DefaultApi.md#public_holidays_calendars_id_delete) | **DELETE** /public-holidays/calendars/{id} | Delete calendar
[**public_holidays_calendars_id_patch**](DefaultApi.md#public_holidays_calendars_id_patch) | **PATCH** /public-holidays/calendars/{id} | Update calendar
[**public_holidays_calendars_import_post**](DefaultApi.md#public_holidays_calendars_import_post) | **POST** /public-holidays/calendars/import | Import calendar
[**public_holidays_calendars_post**](DefaultApi.md#public_holidays_calendars_post) | **POST** /public-holidays/calendars | Create calendar
[**public_holidays_holidays_get**](DefaultApi.md#public_holidays_holidays_get) | **GET** /public-holidays/holidays | Get all holidays
[**public_holidays_holidays_id_delete**](DefaultApi.md#public_holidays_holidays_id_delete) | **DELETE** /public-holidays/holidays/{id} | Delete holiday
[**public_holidays_holidays_id_patch**](DefaultApi.md#public_holidays_holidays_id_patch) | **PATCH** /public-holidays/holidays/{id} | Update holiday
[**public_holidays_holidays_post**](DefaultApi.md#public_holidays_holidays_post) | **POST** /public-holidays/holidays | Create holiday
[**public_holidays_holidays_profile_holidays_get**](DefaultApi.md#public_holidays_holidays_profile_holidays_get) | **GET** /public-holidays/holidays/profile-holidays | Get all profile holidays
[**public_holidays_holidays_related_holidays_get**](DefaultApi.md#public_holidays_holidays_related_holidays_get) | **GET** /public-holidays/holidays/related-holidays | Get all related holidays
[**reactions_item_emoji_delete**](DefaultApi.md#reactions_item_emoji_delete) | **DELETE** /reactions/{item}/{emoji} | Remove reaction
[**reactions_item_emoji_get**](DefaultApi.md#reactions_item_emoji_get) | **GET** /reactions/{item}/{emoji} | List reacted users and applications
[**reactions_item_emoji_post**](DefaultApi.md#reactions_item_emoji_post) | **POST** /reactions/{item}/{emoji} | Add reaction
[**reactions_item_get**](DefaultApi.md#reactions_item_get) | **GET** /reactions/{item} | List reactions
[**rich_text_parse_markdown_post**](DefaultApi.md#rich_text_parse_markdown_post) | **POST** /rich-text/parse-markdown | Parse Markdown
[**team_directory_calendar_events_absence_events_get**](DefaultApi.md#team_directory_calendar_events_absence_events_get) | **GET** /team-directory/calendar-events/absence-events | Get all absence events
[**team_directory_calendar_events_birthday_events_get**](DefaultApi.md#team_directory_calendar_events_birthday_events_get) | **GET** /team-directory/calendar-events/birthday-events | Get all birthday events
[**team_directory_calendar_events_birthday_events_starred_get**](DefaultApi.md#team_directory_calendar_events_birthday_events_starred_get) | **GET** /team-directory/calendar-events/birthday-events/starred | Get all starred birthday events
[**team_directory_calendar_events_get**](DefaultApi.md#team_directory_calendar_events_get) | **GET** /team-directory/calendar-events | Get all calendar events
[**team_directory_calendar_events_holidays_get**](DefaultApi.md#team_directory_calendar_events_holidays_get) | **GET** /team-directory/calendar-events/holidays | Get all holidays
[**team_directory_calendar_events_id_get**](DefaultApi.md#team_directory_calendar_events_id_get) | **GET** /team-directory/calendar-events/{id} | Get calendar event
[**team_directory_calendar_events_meeting_participations_id_patch**](DefaultApi.md#team_directory_calendar_events_meeting_participations_id_patch) | **PATCH** /team-directory/calendar-events/meeting-participations/{id} | Update meeting participation
[**team_directory_calendar_events_membership_events_get**](DefaultApi.md#team_directory_calendar_events_membership_events_get) | **GET** /team-directory/calendar-events/membership-events | Get all membership events
[**team_directory_calendar_events_non_working_days_events_get**](DefaultApi.md#team_directory_calendar_events_non_working_days_events_get) | **GET** /team-directory/calendar-events/non-working-days-events | Get all non working days events
[**team_directory_invitation_links_get**](DefaultApi.md#team_directory_invitation_links_get) | **GET** /team-directory/invitation-links | Get all invitation links
[**team_directory_invitation_links_invitation_link_id_delete**](DefaultApi.md#team_directory_invitation_links_invitation_link_id_delete) | **DELETE** /team-directory/invitation-links/{invitationLinkId} | Delete invitation link
[**team_directory_invitation_links_invitation_link_id_patch**](DefaultApi.md#team_directory_invitation_links_invitation_link_id_patch) | **PATCH** /team-directory/invitation-links/{invitationLinkId} | Update invitation link
[**team_directory_invitation_links_post**](DefaultApi.md#team_directory_invitation_links_post) | **POST** /team-directory/invitation-links | Create invitation link
[**team_directory_invitations_get**](DefaultApi.md#team_directory_invitations_get) | **GET** /team-directory/invitations | Get all invitations
[**team_directory_invitations_id_delete**](DefaultApi.md#team_directory_invitations_id_delete) | **DELETE** /team-directory/invitations/{id} | Delete invitation
[**team_directory_invitations_id_patch**](DefaultApi.md#team_directory_invitations_id_patch) | **PATCH** /team-directory/invitations/{id} | Update invitation
[**team_directory_invitations_post**](DefaultApi.md#team_directory_invitations_post) | **POST** /team-directory/invitations | Create invitation
[**team_directory_languages_get**](DefaultApi.md#team_directory_languages_get) | **GET** /team-directory/languages | Get all languages
[**team_directory_location_equipment_types_get**](DefaultApi.md#team_directory_location_equipment_types_get) | **GET** /team-directory/location-equipment-types | Get all location equipment types
[**team_directory_location_equipment_types_namename_delete**](DefaultApi.md#team_directory_location_equipment_types_namename_delete) | **DELETE** /team-directory/location-equipment-types/name:{name} | Delete location equipment type by name
[**team_directory_location_map_member_points_get**](DefaultApi.md#team_directory_location_map_member_points_get) | **GET** /team-directory/location-map-member-points | Get all location map member points
[**team_directory_location_map_member_points_location_point_id_delete**](DefaultApi.md#team_directory_location_map_member_points_location_point_id_delete) | **DELETE** /team-directory/location-map-member-points/{locationPointId} | Delete location map member point
[**team_directory_location_map_member_points_location_point_id_patch**](DefaultApi.md#team_directory_location_map_member_points_location_point_id_patch) | **PATCH** /team-directory/location-map-member-points/{locationPointId} | Update location map member point
[**team_directory_location_map_member_points_post**](DefaultApi.md#team_directory_location_map_member_points_post) | **POST** /team-directory/location-map-member-points | Create location map member point
[**team_directory_locations_get**](DefaultApi.md#team_directory_locations_get) | **GET** /team-directory/locations | Get all locations
[**team_directory_locations_id_delete**](DefaultApi.md#team_directory_locations_id_delete) | **DELETE** /team-directory/locations/{id} | Archive location
[**team_directory_locations_id_get**](DefaultApi.md#team_directory_locations_id_get) | **GET** /team-directory/locations/{id} | Get location
[**team_directory_locations_id_map_get**](DefaultApi.md#team_directory_locations_id_map_get) | **GET** /team-directory/locations/{id}/map | Get map
[**team_directory_locations_id_map_patch**](DefaultApi.md#team_directory_locations_id_map_patch) | **PATCH** /team-directory/locations/{id}/map | Update map
[**team_directory_locations_id_patch**](DefaultApi.md#team_directory_locations_id_patch) | **PATCH** /team-directory/locations/{id} | Update location
[**team_directory_locations_id_restore_post**](DefaultApi.md#team_directory_locations_id_restore_post) | **POST** /team-directory/locations/{id}/restore | Restore location
[**team_directory_locations_post**](DefaultApi.md#team_directory_locations_post) | **POST** /team-directory/locations | Create location
[**team_directory_locations_restore_post**](DefaultApi.md#team_directory_locations_restore_post) | **POST** /team-directory/locations/restore | Restore multiple locations
[**team_directory_locations_with_timezone_get**](DefaultApi.md#team_directory_locations_with_timezone_get) | **GET** /team-directory/locations-with-timezone | Get all locations with timezone
[**team_directory_member_locations_get**](DefaultApi.md#team_directory_member_locations_get) | **GET** /team-directory/member-locations | Get all member locations
[**team_directory_member_locations_member_location_id_delete**](DefaultApi.md#team_directory_member_locations_member_location_id_delete) | **DELETE** /team-directory/member-locations/{memberLocationId} | Delete member location
[**team_directory_member_locations_member_location_id_get**](DefaultApi.md#team_directory_member_locations_member_location_id_get) | **GET** /team-directory/member-locations/{memberLocationId} | Get member location
[**team_directory_member_locations_member_location_id_patch**](DefaultApi.md#team_directory_member_locations_member_location_id_patch) | **PATCH** /team-directory/member-locations/{memberLocationId} | Update member location
[**team_directory_member_locations_post**](DefaultApi.md#team_directory_member_locations_post) | **POST** /team-directory/member-locations | Create member location
[**team_directory_membership_events_get**](DefaultApi.md#team_directory_membership_events_get) | **GET** /team-directory/membership-events | Get all membership events
[**team_directory_memberships_get**](DefaultApi.md#team_directory_memberships_get) | **GET** /team-directory/memberships | Get all memberships
[**team_directory_memberships_manager_candidates_get**](DefaultApi.md#team_directory_memberships_manager_candidates_get) | **GET** /team-directory/memberships/manager-candidates | Get manager candidate
[**team_directory_memberships_membership_id_delete**](DefaultApi.md#team_directory_memberships_membership_id_delete) | **DELETE** /team-directory/memberships/{membershipId} | Delete membership
[**team_directory_memberships_membership_id_get**](DefaultApi.md#team_directory_memberships_membership_id_get) | **GET** /team-directory/memberships/{membershipId} | Get membership
[**team_directory_memberships_membership_id_patch**](DefaultApi.md#team_directory_memberships_membership_id_patch) | **PATCH** /team-directory/memberships/{membershipId} | Update membership
[**team_directory_memberships_membership_id_request_revoke_patch**](DefaultApi.md#team_directory_memberships_membership_id_request_revoke_patch) | **PATCH** /team-directory/memberships/{membershipId}/request-revoke | Request membership revocation
[**team_directory_memberships_membership_id_revoke_delete**](DefaultApi.md#team_directory_memberships_membership_id_revoke_delete) | **DELETE** /team-directory/memberships/{membershipId}/revoke | Revoke membership
[**team_directory_memberships_post**](DefaultApi.md#team_directory_memberships_post) | **POST** /team-directory/memberships | Create membership
[**team_directory_memberships_requests_get**](DefaultApi.md#team_directory_memberships_requests_get) | **GET** /team-directory/memberships/requests | Get all requests
[**team_directory_memberships_requests_membership_request_id_delete**](DefaultApi.md#team_directory_memberships_requests_membership_request_id_delete) | **DELETE** /team-directory/memberships/requests/{membershipRequestId} | Delete request
[**team_directory_memberships_requests_membership_request_id_patch**](DefaultApi.md#team_directory_memberships_requests_membership_request_id_patch) | **PATCH** /team-directory/memberships/requests/{membershipRequestId} | Update request
[**team_directory_memberships_sync_batch_get**](DefaultApi.md#team_directory_memberships_sync_batch_get) | **GET** /team-directory/memberships/sync-batch | Get sync batch
[**team_directory_profiles_authentication_sessions_owner_get**](DefaultApi.md#team_directory_profiles_authentication_sessions_owner_get) | **GET** /team-directory/profiles/authentication-sessions/{owner} | Get all authentication sessions
[**team_directory_profiles_authentication_sessions_owner_session_id_delete**](DefaultApi.md#team_directory_profiles_authentication_sessions_owner_session_id_delete) | **DELETE** /team-directory/profiles/authentication-sessions/{owner}/{sessionId} | Terminate own authentication session
[**team_directory_profiles_dashboards_dashboard_get**](DefaultApi.md#team_directory_profiles_dashboards_dashboard_get) | **GET** /team-directory/profiles/dashboards/{dashboard} | Get dashboard
[**team_directory_profiles_dashboards_dashboard_patch**](DefaultApi.md#team_directory_profiles_dashboards_dashboard_patch) | **PATCH** /team-directory/profiles/dashboards/{dashboard} | Update dashboard
[**team_directory_profiles_emailemail_get**](DefaultApi.md#team_directory_profiles_emailemail_get) | **GET** /team-directory/profiles/email:{email} | Get profile by email
[**team_directory_profiles_favorites_deployment_targets_get**](DefaultApi.md#team_directory_profiles_favorites_deployment_targets_get) | **GET** /team-directory/profiles/favorites/deployment-targets | Get favorite deployment targets
[**team_directory_profiles_favorites_documents_get**](DefaultApi.md#team_directory_profiles_favorites_documents_get) | **GET** /team-directory/profiles/favorites/documents | Get favorite documents
[**team_directory_profiles_favorites_id_delete**](DefaultApi.md#team_directory_profiles_favorites_id_delete) | **DELETE** /team-directory/profiles/favorites/{id} | Remove from favorites
[**team_directory_profiles_favorites_jobs_get**](DefaultApi.md#team_directory_profiles_favorites_jobs_get) | **GET** /team-directory/profiles/favorites/jobs | Get favorite jobs
[**team_directory_profiles_favorites_locations_get**](DefaultApi.md#team_directory_profiles_favorites_locations_get) | **GET** /team-directory/profiles/favorites/locations | Get favorite locations
[**team_directory_profiles_favorites_post**](DefaultApi.md#team_directory_profiles_favorites_post) | **POST** /team-directory/profiles/favorites | Add to favorites
[**team_directory_profiles_favorites_profiles_get**](DefaultApi.md#team_directory_profiles_favorites_profiles_get) | **GET** /team-directory/profiles/favorites/profiles | Get followed profiles
[**team_directory_profiles_favorites_projects_get**](DefaultApi.md#team_directory_profiles_favorites_projects_get) | **GET** /team-directory/profiles/favorites/projects | Get favorite projects
[**team_directory_profiles_favorites_repositories_get**](DefaultApi.md#team_directory_profiles_favorites_repositories_get) | **GET** /team-directory/profiles/favorites/repositories | Get favorite repositories
[**team_directory_profiles_favorites_teams_get**](DefaultApi.md#team_directory_profiles_favorites_teams_get) | **GET** /team-directory/profiles/favorites/teams | Get favorite teams
[**team_directory_profiles_get**](DefaultApi.md#team_directory_profiles_get) | **GET** /team-directory/profiles | Get all profiles
[**team_directory_profiles_oauth_consents_owner_applications_application_delete**](DefaultApi.md#team_directory_profiles_oauth_consents_owner_applications_application_delete) | **DELETE** /team-directory/profiles/oauth-consents/{owner}/applications/{application} | Delete application
[**team_directory_profiles_oauth_consents_owner_approved_scopes_id_delete**](DefaultApi.md#team_directory_profiles_oauth_consents_owner_approved_scopes_id_delete) | **DELETE** /team-directory/profiles/oauth-consents/{owner}/approved-scopes/{id} | Delete approved scope
[**team_directory_profiles_oauth_consents_owner_get**](DefaultApi.md#team_directory_profiles_oauth_consents_owner_get) | **GET** /team-directory/profiles/oauth-consents/{owner} | Get OAuth consents
[**team_directory_profiles_oauth_consents_owner_internal_applications_client_id_delete**](DefaultApi.md#team_directory_profiles_oauth_consents_owner_internal_applications_client_id_delete) | **DELETE** /team-directory/profiles/oauth-consents/{owner}/internal-applications/{clientId} | Delete internal application
[**team_directory_profiles_oauth_consents_owner_refresh_tokens_id_delete**](DefaultApi.md#team_directory_profiles_oauth_consents_owner_refresh_tokens_id_delete) | **DELETE** /team-directory/profiles/oauth-consents/{owner}/refresh-tokens/{id} | Delete refresh token
[**team_directory_profiles_post**](DefaultApi.md#team_directory_profiles_post) | **POST** /team-directory/profiles | Create profile
[**team_directory_profiles_profile2_fa_requirements_get**](DefaultApi.md#team_directory_profiles_profile2_fa_requirements_get) | **GET** /team-directory/profiles/{profile}/2-fa/requirements | Two-factor authentication requirements
[**team_directory_profiles_profile2_fa_status_get**](DefaultApi.md#team_directory_profiles_profile2_fa_status_get) | **GET** /team-directory/profiles/{profile}/2-fa/status | Two-factor authentication status
[**team_directory_profiles_profile2_fa_totp_confirm_post**](DefaultApi.md#team_directory_profiles_profile2_fa_totp_confirm_post) | **POST** /team-directory/profiles/{profile}/2-fa/totp/confirm | Confirm TOTP two-factor authentication settings
[**team_directory_profiles_profile2_fa_totp_delete**](DefaultApi.md#team_directory_profiles_profile2_fa_totp_delete) | **DELETE** /team-directory/profiles/{profile}/2-fa/totp | Delete current TOTP two-factor authentication settings
[**team_directory_profiles_profile2_fa_totp_patch**](DefaultApi.md#team_directory_profiles_profile2_fa_totp_patch) | **PATCH** /team-directory/profiles/{profile}/2-fa/totp | Update TOTP two-factor authentication settings
[**team_directory_profiles_profile2_fa_totp_post**](DefaultApi.md#team_directory_profiles_profile2_fa_totp_post) | **POST** /team-directory/profiles/{profile}/2-fa/totp | Set up TOTP two-factor authentication
[**team_directory_profiles_profile_application_passwords_get**](DefaultApi.md#team_directory_profiles_profile_application_passwords_get) | **GET** /team-directory/profiles/{profile}/application-passwords | Get all application passwords
[**team_directory_profiles_profile_application_passwords_password_id_delete**](DefaultApi.md#team_directory_profiles_profile_application_passwords_password_id_delete) | **DELETE** /team-directory/profiles/{profile}/application-passwords/{passwordId} | Delete application password
[**team_directory_profiles_profile_application_passwords_password_id_patch**](DefaultApi.md#team_directory_profiles_profile_application_passwords_password_id_patch) | **PATCH** /team-directory/profiles/{profile}/application-passwords/{passwordId} | Update application password
[**team_directory_profiles_profile_application_passwords_post**](DefaultApi.md#team_directory_profiles_profile_application_passwords_post) | **POST** /team-directory/profiles/{profile}/application-passwords | Create application password
[**team_directory_profiles_profile_checklists_checklist_id_delete**](DefaultApi.md#team_directory_profiles_profile_checklists_checklist_id_delete) | **DELETE** /team-directory/profiles/{profile}/checklists/{checklistId} | Delete checklist
[**team_directory_profiles_profile_checklists_checklist_id_full_checklist_tree_get**](DefaultApi.md#team_directory_profiles_profile_checklists_checklist_id_full_checklist_tree_get) | **GET** /team-directory/profiles/{profile}/checklists/{checklistId}/full-checklist-tree | Get full checklist tree
[**team_directory_profiles_profile_checklists_checklist_id_import_post**](DefaultApi.md#team_directory_profiles_profile_checklists_checklist_id_import_post) | **POST** /team-directory/profiles/{profile}/checklists/{checklistId}/import | Import checklist lines
[**team_directory_profiles_profile_checklists_checklist_id_patch**](DefaultApi.md#team_directory_profiles_profile_checklists_checklist_id_patch) | **PATCH** /team-directory/profiles/{profile}/checklists/{checklistId} | Update checklist
[**team_directory_profiles_profile_checklists_get**](DefaultApi.md#team_directory_profiles_profile_checklists_get) | **GET** /team-directory/profiles/{profile}/checklists | Get all checklists
[**team_directory_profiles_profile_checklists_import_post**](DefaultApi.md#team_directory_profiles_profile_checklists_import_post) | **POST** /team-directory/profiles/{profile}/checklists/import | Import checklist
[**team_directory_profiles_profile_checklists_post**](DefaultApi.md#team_directory_profiles_profile_checklists_post) | **POST** /team-directory/profiles/{profile}/checklists | Create checklist
[**team_directory_profiles_profile_checklists_starred_get**](DefaultApi.md#team_directory_profiles_profile_checklists_starred_get) | **GET** /team-directory/profiles/{profile}/checklists/starred | Get all starred checklists
[**team_directory_profiles_profile_convert_to_guest_patch**](DefaultApi.md#team_directory_profiles_profile_convert_to_guest_patch) | **PATCH** /team-directory/profiles/{profile}/convert-to-guest | Convert organization member into guest user
[**team_directory_profiles_profile_convert_to_member_patch**](DefaultApi.md#team_directory_profiles_profile_convert_to_member_patch) | **PATCH** /team-directory/profiles/{profile}/convert-to-member | Convert guest user into organization member
[**team_directory_profiles_profile_deactivate_delete**](DefaultApi.md#team_directory_profiles_profile_deactivate_delete) | **DELETE** /team-directory/profiles/{profile}/deactivate | Deactivate user profile
[**team_directory_profiles_profile_delete**](DefaultApi.md#team_directory_profiles_profile_delete) | **DELETE** /team-directory/profiles/{profile} | Delete profile
[**team_directory_profiles_profile_documents_document_id_access_get**](DefaultApi.md#team_directory_profiles_profile_documents_document_id_access_get) | **GET** /team-directory/profiles/{profile}/documents/{documentId}/access | Document own access permissions
[**team_directory_profiles_profile_documents_document_id_access_patch**](DefaultApi.md#team_directory_profiles_profile_documents_document_id_access_patch) | **PATCH** /team-directory/profiles/{profile}/documents/{documentId}/access | Update document access permissions
[**team_directory_profiles_profile_documents_document_id_copy_post**](DefaultApi.md#team_directory_profiles_profile_documents_document_id_copy_post) | **POST** /team-directory/profiles/{profile}/documents/{documentId}/copy | Copy document
[**team_directory_profiles_profile_documents_document_id_delete**](DefaultApi.md#team_directory_profiles_profile_documents_document_id_delete) | **DELETE** /team-directory/profiles/{profile}/documents/{documentId} | Archive document
[**team_directory_profiles_profile_documents_document_id_get**](DefaultApi.md#team_directory_profiles_profile_documents_document_id_get) | **GET** /team-directory/profiles/{profile}/documents/{documentId} | Get document
[**team_directory_profiles_profile_documents_document_id_move_patch**](DefaultApi.md#team_directory_profiles_profile_documents_document_id_move_patch) | **PATCH** /team-directory/profiles/{profile}/documents/{documentId}/move | Move document
[**team_directory_profiles_profile_documents_document_id_patch**](DefaultApi.md#team_directory_profiles_profile_documents_document_id_patch) | **PATCH** /team-directory/profiles/{profile}/documents/{documentId} | Update document
[**team_directory_profiles_profile_documents_document_id_unarchive_patch**](DefaultApi.md#team_directory_profiles_profile_documents_document_id_unarchive_patch) | **PATCH** /team-directory/profiles/{profile}/documents/{documentId}/unarchive | Unarchive document
[**team_directory_profiles_profile_documents_folders_folder_access_get**](DefaultApi.md#team_directory_profiles_profile_documents_folders_folder_access_get) | **GET** /team-directory/profiles/{profile}/documents/folders/{folder}/access | Folder own access permissions
[**team_directory_profiles_profile_documents_folders_folder_access_patch**](DefaultApi.md#team_directory_profiles_profile_documents_folders_folder_access_patch) | **PATCH** /team-directory/profiles/{profile}/documents/folders/{folder}/access | Update folder access permissions
[**team_directory_profiles_profile_documents_folders_folder_delete**](DefaultApi.md#team_directory_profiles_profile_documents_folders_folder_delete) | **DELETE** /team-directory/profiles/{profile}/documents/folders/{folder} | Archive folder
[**team_directory_profiles_profile_documents_folders_folder_documents_get**](DefaultApi.md#team_directory_profiles_profile_documents_folders_folder_documents_get) | **GET** /team-directory/profiles/{profile}/documents/folders/{folder}/documents | List documents in folder
[**team_directory_profiles_profile_documents_folders_folder_get**](DefaultApi.md#team_directory_profiles_profile_documents_folders_folder_get) | **GET** /team-directory/profiles/{profile}/documents/folders/{folder} | Get folder
[**team_directory_profiles_profile_documents_folders_folder_introduction_delete**](DefaultApi.md#team_directory_profiles_profile_documents_folders_folder_introduction_delete) | **DELETE** /team-directory/profiles/{profile}/documents/folders/{folder}/introduction | Remove folder introduction
[**team_directory_profiles_profile_documents_folders_folder_introduction_document_id_patch**](DefaultApi.md#team_directory_profiles_profile_documents_folders_folder_introduction_document_id_patch) | **PATCH** /team-directory/profiles/{profile}/documents/folders/{folder}/introduction/{documentId} | Add folder introduction
[**team_directory_profiles_profile_documents_folders_folder_move_patch**](DefaultApi.md#team_directory_profiles_profile_documents_folders_folder_move_patch) | **PATCH** /team-directory/profiles/{profile}/documents/folders/{folder}/move | Move folder
[**team_directory_profiles_profile_documents_folders_folder_patch**](DefaultApi.md#team_directory_profiles_profile_documents_folders_folder_patch) | **PATCH** /team-directory/profiles/{profile}/documents/folders/{folder} | Rename folder
[**team_directory_profiles_profile_documents_folders_folder_search_get**](DefaultApi.md#team_directory_profiles_profile_documents_folders_folder_search_get) | **GET** /team-directory/profiles/{profile}/documents/folders/{folder}/search | Search documents and folders
[**team_directory_profiles_profile_documents_folders_folder_subfolders_get**](DefaultApi.md#team_directory_profiles_profile_documents_folders_folder_subfolders_get) | **GET** /team-directory/profiles/{profile}/documents/folders/{folder}/subfolders | List subfolders
[**team_directory_profiles_profile_documents_folders_post**](DefaultApi.md#team_directory_profiles_profile_documents_folders_post) | **POST** /team-directory/profiles/{profile}/documents/folders | Create folder
[**team_directory_profiles_profile_documents_post**](DefaultApi.md#team_directory_profiles_profile_documents_post) | **POST** /team-directory/profiles/{profile}/documents | Create document
[**team_directory_profiles_profile_get**](DefaultApi.md#team_directory_profiles_profile_get) | **GET** /team-directory/profiles/{profile} | Get profile
[**team_directory_profiles_profile_gpg_keys_fingerprint_delete**](DefaultApi.md#team_directory_profiles_profile_gpg_keys_fingerprint_delete) | **DELETE** /team-directory/profiles/{profile}/gpg-keys/{fingerprint} | Delete public GPG key
[**team_directory_profiles_profile_gpg_keys_fingerprint_patch**](DefaultApi.md#team_directory_profiles_profile_gpg_keys_fingerprint_patch) | **PATCH** /team-directory/profiles/{profile}/gpg-keys/{fingerprint} | Revoke public GPG key
[**team_directory_profiles_profile_gpg_keys_get**](DefaultApi.md#team_directory_profiles_profile_gpg_keys_get) | **GET** /team-directory/profiles/{profile}/gpg-keys | List public GPG keys
[**team_directory_profiles_profile_gpg_keys_post**](DefaultApi.md#team_directory_profiles_profile_gpg_keys_post) | **POST** /team-directory/profiles/{profile}/gpg-keys | Add public GPG key
[**team_directory_profiles_profile_is_team_member_get**](DefaultApi.md#team_directory_profiles_profile_is_team_member_get) | **GET** /team-directory/profiles/{profile}/is-team-member | Check if profile is team member
[**team_directory_profiles_profile_leads_get**](DefaultApi.md#team_directory_profiles_profile_leads_get) | **GET** /team-directory/profiles/{profile}/leads | Get all leads
[**team_directory_profiles_profile_nav_bar_menu_items_get**](DefaultApi.md#team_directory_profiles_profile_nav_bar_menu_items_get) | **GET** /team-directory/profiles/{profile}/nav-bar-menu-items | Get all nav bar menu items
[**team_directory_profiles_profile_nav_bar_menu_items_patch**](DefaultApi.md#team_directory_profiles_profile_nav_bar_menu_items_patch) | **PATCH** /team-directory/profiles/{profile}/nav-bar-menu-items | Update nav bar menu item
[**team_directory_profiles_profile_nav_bar_projects_get**](DefaultApi.md#team_directory_profiles_profile_nav_bar_projects_get) | **GET** /team-directory/profiles/{profile}/nav-bar-projects | Get all nav bar projects
[**team_directory_profiles_profile_nav_bar_projects_post**](DefaultApi.md#team_directory_profiles_profile_nav_bar_projects_post) | **POST** /team-directory/profiles/{profile}/nav-bar-projects | Create nav bar project
[**team_directory_profiles_profile_nav_bar_projects_project_delete**](DefaultApi.md#team_directory_profiles_profile_nav_bar_projects_project_delete) | **DELETE** /team-directory/profiles/{profile}/nav-bar-projects/{project} | Delete nav bar project
[**team_directory_profiles_profile_notification_settings_get**](DefaultApi.md#team_directory_profiles_profile_notification_settings_get) | **GET** /team-directory/profiles/{profile}/notification-settings | Get Space global notification settings for a profile
[**team_directory_profiles_profile_notification_settings_patch**](DefaultApi.md#team_directory_profiles_profile_notification_settings_patch) | **PATCH** /team-directory/profiles/{profile}/notification-settings | Set Space global notification settings for a profile
[**team_directory_profiles_profile_patch**](DefaultApi.md#team_directory_profiles_profile_patch) | **PATCH** /team-directory/profiles/{profile} | Update profile
[**team_directory_profiles_profile_permanent_tokens_current_delete**](DefaultApi.md#team_directory_profiles_profile_permanent_tokens_current_delete) | **DELETE** /team-directory/profiles/{profile}/permanent-tokens/current | Delete current permanent token
[**team_directory_profiles_profile_permanent_tokens_get**](DefaultApi.md#team_directory_profiles_profile_permanent_tokens_get) | **GET** /team-directory/profiles/{profile}/permanent-tokens | Get all permanent tokens
[**team_directory_profiles_profile_permanent_tokens_post**](DefaultApi.md#team_directory_profiles_profile_permanent_tokens_post) | **POST** /team-directory/profiles/{profile}/permanent-tokens | Create permanent token
[**team_directory_profiles_profile_permanent_tokens_token_id_delete**](DefaultApi.md#team_directory_profiles_profile_permanent_tokens_token_id_delete) | **DELETE** /team-directory/profiles/{profile}/permanent-tokens/{tokenId} | Delete permanent token
[**team_directory_profiles_profile_permanent_tokens_token_id_patch**](DefaultApi.md#team_directory_profiles_profile_permanent_tokens_token_id_patch) | **PATCH** /team-directory/profiles/{profile}/permanent-tokens/{tokenId} | Update permanent token
[**team_directory_profiles_profile_reactivate_patch**](DefaultApi.md#team_directory_profiles_profile_reactivate_patch) | **PATCH** /team-directory/profiles/{profile}/reactivate | Reactivate user profile
[**team_directory_profiles_profile_restore_patch**](DefaultApi.md#team_directory_profiles_profile_restore_patch) | **PATCH** /team-directory/profiles/{profile}/restore | Restore suspended user profile
[**team_directory_profiles_profile_settings_get**](DefaultApi.md#team_directory_profiles_profile_settings_get) | **GET** /team-directory/profiles/{profile}/settings | Get Space personalization data for a profile
[**team_directory_profiles_profile_settings_patch**](DefaultApi.md#team_directory_profiles_profile_settings_patch) | **PATCH** /team-directory/profiles/{profile}/settings | Set Space personalization data for a profile
[**team_directory_profiles_profile_spoken_languages_get**](DefaultApi.md#team_directory_profiles_profile_spoken_languages_get) | **GET** /team-directory/profiles/{profile}/spoken-languages | Get all spoken languages
[**team_directory_profiles_profile_spoken_languages_language_delete**](DefaultApi.md#team_directory_profiles_profile_spoken_languages_language_delete) | **DELETE** /team-directory/profiles/{profile}/spoken-languages/{language} | Delete spoken language
[**team_directory_profiles_profile_spoken_languages_post**](DefaultApi.md#team_directory_profiles_profile_spoken_languages_post) | **POST** /team-directory/profiles/{profile}/spoken-languages | Create spoken language
[**team_directory_profiles_profile_ssh_keys_fingerprint_delete**](DefaultApi.md#team_directory_profiles_profile_ssh_keys_fingerprint_delete) | **DELETE** /team-directory/profiles/{profile}/ssh-keys/{fingerprint} | Remove association between SSH key and profile
[**team_directory_profiles_profile_ssh_keys_get**](DefaultApi.md#team_directory_profiles_profile_ssh_keys_get) | **GET** /team-directory/profiles/{profile}/ssh-keys | Get all SSH keys
[**team_directory_profiles_profile_ssh_keys_post**](DefaultApi.md#team_directory_profiles_profile_ssh_keys_post) | **POST** /team-directory/profiles/{profile}/ssh-keys | Associate SSH key with profile
[**team_directory_profiles_profile_suspend_patch**](DefaultApi.md#team_directory_profiles_profile_suspend_patch) | **PATCH** /team-directory/profiles/{profile}/suspend | Suspend user profile
[**team_directory_profiles_profile_timezone_get**](DefaultApi.md#team_directory_profiles_profile_timezone_get) | **GET** /team-directory/profiles/{profile}/timezone | Get timezone
[**team_directory_profiles_profile_working_days_get**](DefaultApi.md#team_directory_profiles_profile_working_days_get) | **GET** /team-directory/profiles/{profile}/working-days | Query working days for a profile
[**team_directory_profiles_profile_working_days_post**](DefaultApi.md#team_directory_profiles_profile_working_days_post) | **POST** /team-directory/profiles/{profile}/working-days | Add working days
[**team_directory_profiles_profile_working_days_working_days_id_delete**](DefaultApi.md#team_directory_profiles_profile_working_days_working_days_id_delete) | **DELETE** /team-directory/profiles/{profile}/working-days/{workingDaysId} | Delete working days
[**team_directory_profiles_profile_working_days_working_days_id_patch**](DefaultApi.md#team_directory_profiles_profile_working_days_working_days_id_patch) | **PATCH** /team-directory/profiles/{profile}/working-days/{workingDaysId} | Update working days
[**team_directory_profiles_widget_settings_widget_get**](DefaultApi.md#team_directory_profiles_widget_settings_widget_get) | **GET** /team-directory/profiles/widget-settings/{widget} | Get widget setting
[**team_directory_profiles_widget_settings_widget_patch**](DefaultApi.md#team_directory_profiles_widget_settings_widget_patch) | **PATCH** /team-directory/profiles/widget-settings/{widget} | Update widget setting
[**team_directory_profiles_working_days_get**](DefaultApi.md#team_directory_profiles_working_days_get) | **GET** /team-directory/profiles/working-days | Query all working days
[**team_directory_roles_get**](DefaultApi.md#team_directory_roles_get) | **GET** /team-directory/roles | Get all roles
[**team_directory_roles_id_delete**](DefaultApi.md#team_directory_roles_id_delete) | **DELETE** /team-directory/roles/{id} | Archive role
[**team_directory_roles_id_get**](DefaultApi.md#team_directory_roles_id_get) | **GET** /team-directory/roles/{id} | Get role
[**team_directory_roles_id_patch**](DefaultApi.md#team_directory_roles_id_patch) | **PATCH** /team-directory/roles/{id} | Update role
[**team_directory_roles_id_restore_post**](DefaultApi.md#team_directory_roles_id_restore_post) | **POST** /team-directory/roles/{id}/restore | Restore role
[**team_directory_roles_post**](DefaultApi.md#team_directory_roles_post) | **POST** /team-directory/roles | Create role
[**team_directory_stats_get**](DefaultApi.md#team_directory_stats_get) | **GET** /team-directory/stats | Get all stats
[**team_directory_teams_get**](DefaultApi.md#team_directory_teams_get) | **GET** /team-directory/teams | Get all teams
[**team_directory_teams_id_cancel_disbanding_post**](DefaultApi.md#team_directory_teams_id_cancel_disbanding_post) | **POST** /team-directory/teams/{id}/cancel-disbanding | Cancel team disbanding
[**team_directory_teams_id_delete**](DefaultApi.md#team_directory_teams_id_delete) | **DELETE** /team-directory/teams/{id} | Archive team
[**team_directory_teams_id_direct_members_get**](DefaultApi.md#team_directory_teams_id_direct_members_get) | **GET** /team-directory/teams/{id}/direct-members | Get all direct members
[**team_directory_teams_id_disband_delete**](DefaultApi.md#team_directory_teams_id_disband_delete) | **DELETE** /team-directory/teams/{id}/disband | Disband team
[**team_directory_teams_id_get**](DefaultApi.md#team_directory_teams_id_get) | **GET** /team-directory/teams/{id} | Get team
[**team_directory_teams_id_patch**](DefaultApi.md#team_directory_teams_id_patch) | **PATCH** /team-directory/teams/{id} | Update team
[**team_directory_teams_id_restore_post**](DefaultApi.md#team_directory_teams_id_restore_post) | **POST** /team-directory/teams/{id}/restore | Restore team
[**team_directory_teams_post**](DefaultApi.md#team_directory_teams_post) | **POST** /team-directory/teams | Create team
[**team_directory_teams_sync_batch_get**](DefaultApi.md#team_directory_teams_sync_batch_get) | **GET** /team-directory/teams/sync-batch | Get sync batch
[**time_tracking_items_get**](DefaultApi.md#time_tracking_items_get) | **GET** /time-tracking/items | Get all items
[**time_tracking_items_item_id_delete**](DefaultApi.md#time_tracking_items_item_id_delete) | **DELETE** /time-tracking/items/{itemId} | Delete item
[**time_tracking_items_item_id_patch**](DefaultApi.md#time_tracking_items_item_id_patch) | **PATCH** /time-tracking/items/{itemId} | Update item
[**time_tracking_items_post**](DefaultApi.md#time_tracking_items_post) | **POST** /time-tracking/items | Create item
[**todo_get**](DefaultApi.md#todo_get) | **GET** /todo | Get all to-do items
[**todo_id_delete**](DefaultApi.md#todo_id_delete) | **DELETE** /todo/{id} | Delete to-do item
[**todo_id_patch**](DefaultApi.md#todo_id_patch) | **PATCH** /todo/{id} | Update to-do item
[**todo_post**](DefaultApi.md#todo_post) | **POST** /todo | Create to-do item
[**trusted_certificates_get**](DefaultApi.md#trusted_certificates_get) | **GET** /trusted-certificates | Get all trusted certificates
[**trusted_certificates_id_delete**](DefaultApi.md#trusted_certificates_id_delete) | **DELETE** /trusted-certificates/{id} | Delete trusted certificate
[**trusted_certificates_id_patch**](DefaultApi.md#trusted_certificates_id_patch) | **PATCH** /trusted-certificates/{id} | Update trusted certificate
[**trusted_certificates_info_get**](DefaultApi.md#trusted_certificates_info_get) | **GET** /trusted-certificates/info | Get certificate info
[**trusted_certificates_post**](DefaultApi.md#trusted_certificates_post) | **POST** /trusted-certificates | Create trusted certificate
[**unfurls_block_unfurl_global_post**](DefaultApi.md#unfurls_block_unfurl_global_post) | **POST** /unfurls/block-unfurl-global | Block link unfurling for organization
[**unfurls_block_unfurl_post**](DefaultApi.md#unfurls_block_unfurl_post) | **POST** /unfurls/block-unfurl | Block link unfurling
[**unfurls_check_blocked_post**](DefaultApi.md#unfurls_check_blocked_post) | **POST** /unfurls/check-blocked | Check if unfurl is blocked
[**unfurls_list_blocked_get**](DefaultApi.md#unfurls_list_blocked_get) | **GET** /unfurls/list-blocked | List blocked unfurls
[**unfurls_unblock_unfurl_global_post**](DefaultApi.md#unfurls_unblock_unfurl_global_post) | **POST** /unfurls/unblock-unfurl-global | Unblock link unfurling for organization
[**unfurls_unblock_unfurl_post**](DefaultApi.md#unfurls_unblock_unfurl_post) | **POST** /unfurls/unblock-unfurl | Unblock link unfurling
[**uploads_chat_public_url_channel_message_attachment_id_get**](DefaultApi.md#uploads_chat_public_url_channel_message_attachment_id_get) | **GET** /uploads/chat/public-url/{channel}/{message}/{attachmentId} | Get public url
[**uploads_image_id_get**](DefaultApi.md#uploads_image_id_get) | **GET** /uploads/image/{id} | Get image attachment metadata
[**uploads_post**](DefaultApi.md#uploads_post) | **POST** /uploads | Create upload



## absences_absence_reasons_get

> Vec<crate::models::AbsenceReasonRecord> absences_absence_reasons_get(with_archived, dollar_fields)
Get all absence reasons

Get available absence reasons

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**with_archived** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AbsenceReasonRecord>**](AbsenceReasonRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_absence_reasons_id_delete

> absences_absence_reasons_id_delete(id, delete)
Delete absence reason

Archive/restore an existing absence reason. Setting delete to true will archive the absence reason, false will restore it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**delete** | Option<**bool**> |  |  |[default to true]

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_absence_reasons_id_get

> crate::models::AbsenceReasonRecord absences_absence_reasons_id_get(id, dollar_fields)
Get absence reason

Get an absence reason

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AbsenceReasonRecord**](AbsenceReasonRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_absence_reasons_id_patch

> crate::models::AbsenceReasonRecord absences_absence_reasons_id_patch(id, absences_absence_reasons_post_request, dollar_fields)
Update absence reason

Update an existing absence reason

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**absences_absence_reasons_post_request** | [**AbsencesAbsenceReasonsPostRequest**](AbsencesAbsenceReasonsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AbsenceReasonRecord**](AbsenceReasonRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_absence_reasons_post

> crate::models::AbsenceReasonRecord absences_absence_reasons_post(absences_absence_reasons_post_request, dollar_fields)
Create absence reason

Create a new absence reason

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**absences_absence_reasons_post_request** | [**AbsencesAbsenceReasonsPostRequest**](AbsencesAbsenceReasonsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AbsenceReasonRecord**](AbsenceReasonRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_get

> crate::models::AbsencesGet200Response absences_get(dollar_skip, dollar_top, member, members, location, team, since, till, view_mode, reason, dollar_fields)
Get all absences

Search absences. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**member** | Option<**String**> |  |  |
**members** | Option<[**Vec<String>**](String.md)> |  |  |
**location** | Option<**String**> |  |  |
**team** | Option<**String**> |  |  |
**since** | Option<**String**> |  |  |
**till** | Option<**String**> |  |  |
**view_mode** | Option<[**AbsenceListMode**](.md)> |  |  |
**reason** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AbsencesGet200Response**](_absences_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_id_approve_post

> absences_id_approve_post(id, absences_id_approve_post_request)
Approve absence

Approve/unapprove an existing absence. Setting approve to true will approve the absence, false will remove the approval.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**absences_id_approve_post_request** | [**AbsencesIdApprovePostRequest**](AbsencesIdApprovePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_id_delete

> absences_id_delete(id, delete)
Delete absence

Archive/restore an existing absence. Setting delete to true will archive the absence, false will restore it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**delete** | Option<**bool**> |  |  |[default to true]

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_id_delete_approval_delete

> absences_id_delete_approval_delete(id)
Delete absence approval

Delete approval for a given absence

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_id_get

> crate::models::AbsenceRecord absences_id_get(id, dollar_fields)
Get absence

Get an absence

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AbsenceRecord**](AbsenceRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_id_patch

> crate::models::AbsenceRecord absences_id_patch(id, absences_id_patch_request, dollar_fields)
Update absence

Update an existing absence. Optional parameters will be ignored when not specified and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**absences_id_patch_request** | [**AbsencesIdPatchRequest**](AbsencesIdPatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AbsenceRecord**](AbsenceRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_membermember_get

> Vec<crate::models::AbsenceRecord> absences_membermember_get(member, dollar_fields)
Get all absences by member

Get absences for a given profile ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AbsenceRecord>**](AbsenceRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## absences_post

> crate::models::AbsenceRecord absences_post(absences_post_request, dollar_fields)
Create absence

Create an absence for a given profile (member)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**absences_post_request** | [**AbsencesPostRequest**](AbsencesPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AbsenceRecord**](AbsenceRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## administration_support_post

> crate::models::SupportProfileDto administration_support_post(dollar_fields)
Create support

Create a profile for support

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::SupportProfileDto**](SupportProfileDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## administration_user_agreement_enabled_get

> bool administration_user_agreement_enabled_get()
Is user agreement enabled?

### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## administration_user_agreement_enabled_post

> administration_user_agreement_enabled_post(administration_user_agreement_enabled_post_request)
Enable / disable user agreement

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**administration_user_agreement_enabled_post_request** | [**AdministrationUserAgreementEnabledPostRequest**](AdministrationUserAgreementEnabledPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## administration_user_agreement_get

> crate::models::UaUserAgreement administration_user_agreement_get(dollar_fields)
Get user agreement

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::UaUserAgreement**](UA_UserAgreement.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## administration_user_agreement_patch

> crate::models::UaUserAgreement administration_user_agreement_patch(administration_user_agreement_patch_request, dollar_fields)
Upload new user agreement

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**administration_user_agreement_patch_request** | [**AdministrationUserAgreementPatchRequest**](AdministrationUserAgreementPatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::UaUserAgreement**](UA_UserAgreement.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## administration_user_agreement_status_get

> crate::models::AdministrationUserAgreementStatusGet200Response administration_user_agreement_status_get(dollar_skip, dollar_top, query, accepted, active_profiles_only, dollar_fields)
Get all user agreement statuses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**query** | Option<**String**> |  |  |[default to ]
**accepted** | Option<**bool**> |  |  |
**active_profiles_only** | Option<**bool**> |  |  |[default to true]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AdministrationUserAgreementStatusGet200Response**](_administration_user_agreement_status_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## administration_user_agreement_status_profile_get

> crate::models::UaUserAgreementStatus administration_user_agreement_status_profile_get(profile, dollar_fields)
Get user agreement status for a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::UaUserAgreementStatus**](UA_UserAgreementStatus.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_authorizations_authorized_contexts_get

> Vec<crate::models::PermissionContextApi> applications_application_authorizations_authorized_contexts_get(application, dollar_fields)
Get all authorized contexts

List authorized contexts of an application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PermissionContextApi>**](PermissionContextApi.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_authorizations_authorized_rights_delete

> applications_application_authorizations_authorized_rights_delete(application, context_identifier)
Delete authorized right

Remove application authorization in specified context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**context_identifier** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_authorizations_authorized_rights_get

> Vec<crate::models::RightDto> applications_application_authorizations_authorized_rights_get(application, context_identifier, dollar_fields)
Get all authorized rights

List authorized rights of an application in specified context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**context_identifier** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::RightDto>**](RightDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_authorizations_authorized_rights_patch

> applications_application_authorizations_authorized_rights_patch(application, applications_application_authorizations_authorized_rights_patch_request)
Update authorized right

Generic method for editing authorized right status in given context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_authorizations_authorized_rights_patch_request** | [**ApplicationsApplicationAuthorizationsAuthorizedRightsPatchRequest**](ApplicationsApplicationAuthorizationsAuthorizedRightsPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_authorizations_authorized_rights_request_rights_patch

> applications_application_authorizations_authorized_rights_request_rights_patch(application, applications_application_authorizations_authorized_rights_request_rights_patch_request)
Request Rights

Request rights for an application in specified context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_authorizations_authorized_rights_request_rights_patch_request** | [**ApplicationsApplicationAuthorizationsAuthorizedRightsRequestRightsPatchRequest**](ApplicationsApplicationAuthorizationsAuthorizedRightsRequestRightsPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_authorizations_required_rights_get

> Vec<crate::models::RightDto> applications_application_authorizations_required_rights_get(application, dollar_fields)
Get all required rights

List required rights for an application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::RightDto>**](RightDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_authorizations_required_rights_patch

> applications_application_authorizations_required_rights_patch(application, applications_application_authorizations_required_rights_patch_request)
Update required right

Update list of required rights for an application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_authorizations_required_rights_patch_request** | [**ApplicationsApplicationAuthorizationsRequiredRightsPatchRequest**](ApplicationsApplicationAuthorizationsRequiredRightsPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_bearer_token_get

> String applications_application_bearer_token_get(application)
Bearer Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_client_secret_get

> String applications_application_client_secret_get(application)
Get client secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_client_secret_regenerate_post

> applications_application_client_secret_regenerate_post(application)
Regenerate app secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_delete

> applications_application_delete(application)
Delete application

Removes specified application. If the application is connected (installed from Marketplace or through an install link), Space sends `ApplicationUninstalledPayload` to the application's server. The application is only actually deleted when the application server responds or when the `ApplicationUninstalledPayload` request times out multiple times.  This API method does not wait until the `ApplicationUninstalledPayload` request is finished and instead returns immediately. Consequently, the application may still be active right after this API method call.  If sending `ApplicationUninstalledPayload` has failed at least one time, a user may choose to force-remove the application. In this case the access for the application is terminated and it can no longer make requests. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_force_remove_post

> applications_application_force_remove_post(application)
Force-remove application

Removes the application that has previously failed to respond with code 200 to `ApplicationUninstalledPayload` request, without sending additional `ApplicationUninstalledPayload` requests. The application is archived and its access terminated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_get

> crate::models::EsApp applications_application_get(application, dollar_fields)
Get application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::EsApp**](ES_App.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_gpg_keys_fingerprint_delete

> applications_application_gpg_keys_fingerprint_delete(application, fingerprint)
Delete GPG key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**fingerprint** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_gpg_keys_fingerprint_patch

> applications_application_gpg_keys_fingerprint_patch(application, fingerprint, applications_application_gpg_keys_fingerprint_patch_request)
Revoke GPG key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**fingerprint** | **String** |  | [required] |
**applications_application_gpg_keys_fingerprint_patch_request** | Option<[**ApplicationsApplicationGpgKeysFingerprintPatchRequest**](ApplicationsApplicationGpgKeysFingerprintPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_gpg_keys_get

> Vec<crate::models::GpgKeyData> applications_application_gpg_keys_get(application, dollar_fields)
Get GPG keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::GpgKeyData>**](GpgKeyData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_gpg_keys_post

> crate::models::GpgKeyData applications_application_gpg_keys_post(application, applications_application_gpg_keys_post_request, dollar_fields)
Add GPG key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_gpg_keys_post_request** | [**ApplicationsApplicationGpgKeysPostRequest**](ApplicationsApplicationGpgKeysPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GpgKeyData**](GpgKeyData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_last_client_credentials_access_get

> crate::models::AccessRecord applications_application_last_client_credentials_access_get(application, dollar_fields)
Get last client credentials access info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AccessRecord**](AccessRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_patch

> crate::models::EsApp applications_application_patch(application, dollar_fields, applications_application_patch_request)
Update application

Update existing application. Multi-org applications (created with the parameter `connectToSpace = true` or installed from JetBrains Marketplace) can only be updated by the application itself. Learn more about multi-org applications in the [documentation](https://www.jetbrains.com/help/space/distribute-your-application.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**applications_application_patch_request** | Option<[**ApplicationsApplicationPatchRequest**](ApplicationsApplicationPatchRequest.md)> |  |  |

### Return type

[**crate::models::EsApp**](ES_App.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_permanent_tokens_current_delete

> applications_application_permanent_tokens_current_delete(application)
Delete current permanent token

Delete personal token of the given application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_permanent_tokens_get

> crate::models::ApplicationsApplicationPermanentTokensGet200Response applications_application_permanent_tokens_get(application, dollar_skip, dollar_top, dollar_fields)
Get all permanent tokens

Get permanent tokens used to access the current organization by the given application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ApplicationsApplicationPermanentTokensGet200Response**](_applications__application__permanent_tokens_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_permanent_tokens_post

> crate::models::ApplicationsApplicationPermanentTokensPost200Response applications_application_permanent_tokens_post(application, applications_application_permanent_tokens_post_request, dollar_fields)
Create permanent token

Create a permanent token for the given application that can be used to access the current organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_permanent_tokens_post_request** | [**ApplicationsApplicationPermanentTokensPostRequest**](ApplicationsApplicationPermanentTokensPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ApplicationsApplicationPermanentTokensPost200Response**](_applications__application__permanent_tokens_post_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_permanent_tokens_token_id_delete

> applications_application_permanent_tokens_token_id_delete(application, token_id)
Delete permanent token

Delete a personal token used to access the current organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**token_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_permanent_tokens_token_id_patch

> applications_application_permanent_tokens_token_id_patch(application, token_id, applications_application_permanent_tokens_token_id_patch_request)
Update permanent token

Update an existing personal token used to access the current organization. The permanent token's name and/or scope can be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**token_id** | **String** |  | [required] |
**applications_application_permanent_tokens_token_id_patch_request** | Option<[**ApplicationsApplicationPermanentTokensTokenIdPatchRequest**](ApplicationsApplicationPermanentTokensTokenIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_public_keys_get

> String applications_application_public_keys_get(application)
Public Keys

Returns list of public keys in JWKS format. If message signature is successfully verified with any of the returned public keys, the message can be considered authentic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_restore_post

> applications_application_restore_post(application)
Restore application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_signing_key_get

> String applications_application_signing_key_get(application)
Get signing key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_signing_key_regenerate_post

> applications_application_signing_key_regenerate_post(application)
Regenerate signing key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_ssh_keys_fingerprint_delete

> applications_application_ssh_keys_fingerprint_delete(application, fingerprint)
Delete SSH key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**fingerprint** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_ssh_keys_get

> Vec<crate::models::SshKeyData> applications_application_ssh_keys_get(application, dollar_fields)
Get SSH keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::SshKeyData>**](SshKeyData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_ssh_keys_post

> applications_application_ssh_keys_post(application, applications_application_ssh_keys_post_request)
Add SSH key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_ssh_keys_post_request** | [**ApplicationsApplicationSshKeysPostRequest**](ApplicationsApplicationSshKeysPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_ui_extensions_disable_for_everybody_patch

> applications_application_ui_extensions_disable_for_everybody_patch(application, applications_application_ui_extensions_disable_for_everybody_patch_request)
Disable application UI

Disable application UI for everybody in specified context. Requires Superadmin right for global context, AdminProject for project context, AdminChannel for channel context. Users will still be able to enable application UI individually.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_ui_extensions_disable_for_everybody_patch_request** | [**ApplicationsApplicationUiExtensionsDisableForEverybodyPatchRequest**](ApplicationsApplicationUiExtensionsDisableForEverybodyPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_ui_extensions_disable_for_me_patch

> applications_application_ui_extensions_disable_for_me_patch(application, applications_application_ui_extensions_disable_for_everybody_patch_request)
Disable application UI for me

Disable application UI in specified context for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_ui_extensions_disable_for_everybody_patch_request** | [**ApplicationsApplicationUiExtensionsDisableForEverybodyPatchRequest**](ApplicationsApplicationUiExtensionsDisableForEverybodyPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_ui_extensions_enable_for_everybody_patch

> applications_application_ui_extensions_enable_for_everybody_patch(application, applications_application_ui_extensions_disable_for_everybody_patch_request)
Enable application UI

Enable application UI for everybody in specified context. Requires Superadmin right for global context, AdminProject for project context, AdminChannel for channel context. Users will still be able to disable application UI individually.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_ui_extensions_disable_for_everybody_patch_request** | [**ApplicationsApplicationUiExtensionsDisableForEverybodyPatchRequest**](ApplicationsApplicationUiExtensionsDisableForEverybodyPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_ui_extensions_enable_for_me_patch

> applications_application_ui_extensions_enable_for_me_patch(application, applications_application_ui_extensions_disable_for_everybody_patch_request)
Enable application UI for me

Enable application UI in specified context for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_ui_extensions_disable_for_everybody_patch_request** | [**ApplicationsApplicationUiExtensionsDisableForEverybodyPatchRequest**](ApplicationsApplicationUiExtensionsDisableForEverybodyPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_ui_extensions_get

> Vec<crate::models::AppUiExtensionApi> applications_application_ui_extensions_get(application, context_identifier, dollar_fields)
Get UI extensions

Get UI extensions supported by the application in specified context. Omit contextIdentifier to get UI extensions in all contexts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**context_identifier** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AppUiExtensionApi>**](AppUiExtensionApi.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_unfurl_domains_authorize_post

> applications_application_unfurl_domains_authorize_post(application, applications_application_unfurl_domains_authorize_post_request)
Authorize unfurled domains

Authorize domains for unfurling by the application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_unfurl_domains_authorize_post_request** | [**ApplicationsApplicationUnfurlDomainsAuthorizePostRequest**](ApplicationsApplicationUnfurlDomainsAuthorizePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_unfurl_domains_get

> Vec<crate::models::ApplicationUnfurlDomain> applications_application_unfurl_domains_get(application, dollar_fields)
Get all unfurl domains

List domains for unfurling by the application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ApplicationUnfurlDomain>**](ApplicationUnfurlDomain.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_unfurl_patterns_authorize_post

> applications_application_unfurl_patterns_authorize_post(application, applications_application_unfurl_patterns_authorize_post_request)
Authorize unfurled patterns

Authorize patterns for unfurling by the application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_unfurl_patterns_authorize_post_request** | [**ApplicationsApplicationUnfurlPatternsAuthorizePostRequest**](ApplicationsApplicationUnfurlPatternsAuthorizePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_unfurl_patterns_get

> Vec<crate::models::ApplicationUnfurlPattern> applications_application_unfurl_patterns_get(application, dollar_fields)
Get all unfurl patterns

List patterns for unfurling by the application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ApplicationUnfurlPattern>**](ApplicationUnfurlPattern.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_verification_token_get

> String applications_application_verification_token_get(application)
Get verification token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_verification_token_regenerate_post

> applications_application_verification_token_regenerate_post(application)
Regenerate verification token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_get

> crate::models::ApplicationsApplicationWebhooksGet200Response applications_application_webhooks_get(application, with_archived, query, dollar_skip, dollar_top, dollar_fields)
Get all webhooks

Get application webhooks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**with_archived** | Option<**bool**> |  |  |[default to false]
**query** | Option<**String**> |  |  |[default to ]
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ApplicationsApplicationWebhooksGet200Response**](_applications__application__webhooks_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_post

> crate::models::WebhookRecord applications_application_webhooks_post(application, applications_application_webhooks_post_request, dollar_fields)
Create webhook

Create application webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**applications_application_webhooks_post_request** | [**ApplicationsApplicationWebhooksPostRequest**](ApplicationsApplicationWebhooksPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::WebhookRecord**](WebhookRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_bearer_token_get

> String applications_application_webhooks_webhook_id_bearer_token_get(application, webhook_id)
Bearer Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_custom_headers_get

> Vec<crate::models::CustomHttpHeaderDto> applications_application_webhooks_webhook_id_custom_headers_get(application, webhook_id, dollar_fields)
Get custom header

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::CustomHttpHeaderDto>**](CustomHttpHeaderDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_custom_headers_post

> applications_application_webhooks_webhook_id_custom_headers_post(application, webhook_id, applications_application_webhooks_webhook_id_custom_headers_post_request)
Post custom header

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |
**applications_application_webhooks_webhook_id_custom_headers_post_request** | [**ApplicationsApplicationWebhooksWebhookIdCustomHeadersPostRequest**](ApplicationsApplicationWebhooksWebhookIdCustomHeadersPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_delete

> applications_application_webhooks_webhook_id_delete(application, webhook_id)
Delete webhook

Archive application webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_patch

> applications_application_webhooks_webhook_id_patch(application, webhook_id, applications_application_webhooks_webhook_id_patch_request)
Update webhook

Update application webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |
**applications_application_webhooks_webhook_id_patch_request** | Option<[**ApplicationsApplicationWebhooksWebhookIdPatchRequest**](ApplicationsApplicationWebhooksWebhookIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_post

> applications_application_webhooks_webhook_id_post(application, webhook_id)
Post webhook

Restore archived application webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_signing_key_get

> String applications_application_webhooks_webhook_id_signing_key_get(application, webhook_id)
Get signing key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_signing_key_regenerate_post

> applications_application_webhooks_webhook_id_signing_key_regenerate_post(application, webhook_id)
Regenerate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_subscriptions_get

> Vec<crate::models::SubscriptionDto> applications_application_webhooks_webhook_id_subscriptions_get(application, webhook_id, dollar_fields)
Get all subscriptions

Get webhook subscriptions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::SubscriptionDto>**](SubscriptionDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_subscriptions_post

> crate::models::SubscriptionDto applications_application_webhooks_webhook_id_subscriptions_post(application, webhook_id, applications_application_webhooks_webhook_id_subscriptions_post_request, dollar_fields)
Create subscription

Add webhook subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |
**applications_application_webhooks_webhook_id_subscriptions_post_request** | [**ApplicationsApplicationWebhooksWebhookIdSubscriptionsPostRequest**](ApplicationsApplicationWebhooksWebhookIdSubscriptionsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::SubscriptionDto**](SubscriptionDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_subscriptions_subscription_id_delete

> applications_application_webhooks_webhook_id_subscriptions_subscription_id_delete(application, webhook_id, subscription_id)
Delete subscription

Delete webhook subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_subscriptions_subscription_id_patch

> crate::models::SubscriptionDto applications_application_webhooks_webhook_id_subscriptions_subscription_id_patch(application, webhook_id, subscription_id, dollar_fields, applications_application_webhooks_webhook_id_subscriptions_subscription_id_patch_request)
Update subscription

Update webhook subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**applications_application_webhooks_webhook_id_subscriptions_subscription_id_patch_request** | Option<[**ApplicationsApplicationWebhooksWebhookIdSubscriptionsSubscriptionIdPatchRequest**](ApplicationsApplicationWebhooksWebhookIdSubscriptionsSubscriptionIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::SubscriptionDto**](SubscriptionDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_application_webhooks_webhook_id_subscriptions_subscription_id_request_missing_rights_post

> applications_application_webhooks_webhook_id_subscriptions_subscription_id_request_missing_rights_post(application, webhook_id, subscription_id)
Request Missing Rights

Ensures that all permissions required for this subscription are requested in the corresponding permission role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**webhook_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_authorizations_authorized_applications_get

> Vec<crate::models::EsApp> applications_authorizations_authorized_applications_get(context_identifier, dollar_fields)
Get applications authorized in context

List applications authorized in specified context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_identifier** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::EsApp>**](ES_App.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_error_message_post

> applications_error_message_post(applications_error_message_post_request)
Set error message

Provide error message to display on application page in Space UI. Provide `null` message to remove it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applications_error_message_post_request** | Option<[**ApplicationsErrorMessagePostRequest**](ApplicationsErrorMessagePostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_paged_get

> crate::models::ApplicationsPagedGet200Response applications_paged_get(dollar_skip, dollar_top, name, owner, with_archived, with_managed, ordering, dollar_fields)
Get all applications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**name** | Option<**String**> |  |  |
**owner** | Option<[**Vec<String>**](String.md)> |  |  |
**with_archived** | Option<**bool**> |  |  |[default to false]
**with_managed** | Option<**bool**> |  |  |[default to true]
**ordering** | Option<[**AppsOrdering**](.md)> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ApplicationsPagedGet200Response**](_applications_paged_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_parameters_get

> Vec<crate::models::AppParameter> applications_parameters_get(dollar_fields)
Get all parameters

Return all application parameters. Only accessible with an app token, not a user token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AppParameter>**](AppParameter.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_parameters_key_delete

> applications_parameters_key_delete(key)
Remove parameter

Remove application parameter by key. Only accessible with an app token, not a user token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_parameters_key_get

> String applications_parameters_key_get(key)
Get parameter

Get application parameter by key. Only accessible with an app token, not a user token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_parameters_key_patch

> applications_parameters_key_patch(key, applications_parameters_key_patch_request)
Set parameter

Set application parameter by key. Only accessible with an app token, not a user token. There is a limit of 100 app parameters per app. The key cannot be longer than 64 characters. The value cannot be longer than 1000 characters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**applications_parameters_key_patch_request** | [**ApplicationsParametersKeyPatchRequest**](ApplicationsParametersKeyPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_parameters_profile_get

> Vec<crate::models::AppParameter> applications_parameters_profile_get(dollar_fields)
Get all profile parameters

Return all profile parameters, profile and application are derived from the access token. Only accessible with a user token, issued to an application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AppParameter>**](AppParameter.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_parameters_profile_key_delete

> applications_parameters_profile_key_delete(key)
Remove profile parameter

Remove profile parameter by key, profile and application are derived from the access token. Only accessible with a user token, issued to an application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_parameters_profile_key_get

> String applications_parameters_profile_key_get(key)
Get profile parameter

Get profile parameter by key, profile and application are derived from the access token. Only accessible with a user token, issued to an application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_parameters_profile_key_patch

> applications_parameters_profile_key_patch(key, applications_parameters_key_patch_request)
Set profile parameter

Set profile parameter by key, profile and application are derived from the access token. Only accessible with a user token, issued to an application. There is a limit of 100 app parameters per app per profile. The key cannot be longer than 64 characters. The value cannot be longer than 1000 characters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**applications_parameters_key_patch_request** | [**ApplicationsParametersKeyPatchRequest**](ApplicationsParametersKeyPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_post

> crate::models::EsApp applications_post(applications_post_request, dollar_fields)
Create application

Creates a new application. Marketplace application cannot be installed using this endpoint.  To create a multi-org application (and connect application server to the current Space instance), pass `connectToSpace = true`. Learn more about multi-org applications in the [documentation](https://www.jetbrains.com/help/space/distribute-your-application.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applications_post_request** | [**ApplicationsPostRequest**](ApplicationsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::EsApp**](ES_App.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_report_application_as_healthy_post

> applications_report_application_as_healthy_post()
Report application as healthy

Application may periodically call this api method to notify Space that it is functioning properly. This is mandatory for applications that connect external issue trackers.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_ui_extensions_patch

> applications_ui_extensions_patch(applications_ui_extensions_patch_request)
Set UI extensions

Set UI extensions supported by the calling application in specified context. Only the application itself can set its extensions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applications_ui_extensions_patch_request** | [**ApplicationsUiExtensionsPatchRequest**](ApplicationsUiExtensionsPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_unfurls_domains_patch

> applications_unfurls_domains_patch(applications_unfurls_domains_patch_request)
Update unfurled domains

Update list of domains for unfurling by the application. Method is to be called by the application providing unfurls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applications_unfurls_domains_patch_request** | [**ApplicationsUnfurlsDomainsPatchRequest**](ApplicationsUnfurlsDomainsPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_unfurls_patterns_patch

> applications_unfurls_patterns_patch(applications_unfurls_patterns_patch_request)
Update unfurled patterns

Update list of external ID prefixes for unfurling by the application. Method is to be called by the application providing unfurls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applications_unfurls_patterns_patch_request** | [**ApplicationsUnfurlsPatternsPatchRequest**](ApplicationsUnfurlsPatternsPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_unfurls_queue_content_post

> Vec<crate::models::PostUnfurlContentResult> applications_unfurls_queue_content_post(applications_unfurls_queue_content_post_request, dollar_fields)
Post unfurls content

Provide Space with unfurls content. Method is to be called by the application providing unfurls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applications_unfurls_queue_content_post_request** | [**ApplicationsUnfurlsQueueContentPostRequest**](ApplicationsUnfurlsQueueContentPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PostUnfurlContentResult>**](PostUnfurlContentResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_unfurls_queue_get

> Vec<crate::models::ApplicationUnfurlQueueItem> applications_unfurls_queue_get(batch_size, from_etag, dollar_fields)
Get unfurl queue items

Get links for unfurling by the application. Method is to be called by the application providing unfurls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_size** | **i32** |  | [required] |
**from_etag** | Option<**i64**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ApplicationUnfurlQueueItem>**](ApplicationUnfurlQueueItem.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_unfurls_queue_request_external_auth_post

> applications_unfurls_queue_request_external_auth_post(applications_unfurls_queue_request_external_auth_post_request)
Request external system authentication

Request user to authenticate in external system to provide unfurls from it. Method is to be called by the application providing unfurls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applications_unfurls_queue_request_external_auth_post_request** | [**ApplicationsUnfurlsQueueRequestExternalAuthPostRequest**](ApplicationsUnfurlsQueueRequestExternalAuthPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_unfurls_queue_reset_external_auth_requests_post

> applications_unfurls_queue_reset_external_auth_requests_post(applications_unfurls_queue_reset_external_auth_requests_post_request)
Clear external system authentication requests

Clear all external system authentication requests for the specified user. Method is to be called by the application providing unfurls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applications_unfurls_queue_reset_external_auth_requests_post_request** | [**ApplicationsUnfurlsQueueResetExternalAuthRequestsPostRequest**](ApplicationsUnfurlsQueueResetExternalAuthRequestsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_config_delete

> crate::models::AuthConfig auth_modules_config_delete(dollar_fields)
Delete config

Reset authentication configuration to default

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AuthConfig**](AuthConfig.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_config_get

> crate::models::AuthConfig auth_modules_config_get(dollar_fields)
Get config

Get authentication configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AuthConfig**](AuthConfig.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_config_put

> auth_modules_config_put(auth_modules_config_put_request)
Put config

Set authentication configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_modules_config_put_request** | Option<[**AuthModulesConfigPutRequest**](AuthModulesConfigPutRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_discover_oidc_get

> crate::models::OidcDiscovery auth_modules_discover_oidc_get(discovery_endpoint, dollar_fields)
Discover OIDC

Automatically discovers the endpoints for the OpenID Connect provider via discovery document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discovery_endpoint** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::OidcDiscovery**](OIDCDiscovery.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_get

> Vec<crate::models::EsAuthModule> auth_modules_get(with_disabled, dollar_fields)
Get all auth modules

Get all authentication modules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**with_disabled** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::EsAuthModule>**](ES_AuthModule.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_id_delete

> auth_modules_id_delete(id)
Delete auth module

Delete an existing authentication module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_id_logins_identifier_change_post

> auth_modules_id_logins_identifier_change_post(id, identifier, auth_modules_id_logins_identifier_change_post_request)
Change password

Change password for a given authentication module (id) and profile (identifier)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**identifier** | **String** |  | [required] |
**auth_modules_id_logins_identifier_change_post_request** | [**AuthModulesIdLoginsIdentifierChangePostRequest**](AuthModulesIdLoginsIdentifierChangePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_id_logins_identifier_delete

> auth_modules_id_logins_identifier_delete(identifier, id)
Delete login

Detach a profile login from an authentication module. The id parameter refers to the authentication module, the identifier parameter refers to the login.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_id_logins_identifier_reset_post

> auth_modules_id_logins_identifier_reset_post(id, identifier)
Reset password

Request a password reset for a given authentication module (id) and profile (identifier)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**identifier** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_id_patch

> auth_modules_id_patch(id, auth_modules_id_patch_request)
Update auth module

Update an existing authentication module. Optional parameters will be ignored when not specified and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**auth_modules_id_patch_request** | Option<[**AuthModulesIdPatchRequest**](AuthModulesIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_id_saml_metadata_post

> crate::models::SamlMetadataResponse auth_modules_id_saml_metadata_post(id, auth_modules_id_saml_metadata_post_request, dollar_fields)
SAML metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**auth_modules_id_saml_metadata_post_request** | [**AuthModulesIdSamlMetadataPostRequest**](AuthModulesIdSamlMetadataPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::SamlMetadataResponse**](SamlMetadataResponse.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_keykey_get

> crate::models::EsAuthModule auth_modules_keykey_get(key, dollar_fields)
Get auth module by key

Get an existing authentication module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::EsAuthModule**](ES_AuthModule.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_post

> crate::models::EsAuthModule auth_modules_post(auth_modules_post_request, dollar_fields)
Create auth module

Create a new authentication module. Settings are specific to the type of authentication module being created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_modules_post_request** | [**AuthModulesPostRequest**](AuthModulesPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::EsAuthModule**](ES_AuthModule.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_reorder_post

> auth_modules_reorder_post(auth_modules_reorder_post_request)
Reorder authentication modules

Define the order of authentication modules. This affects the order of the federated authentication module buttons on the sign-in page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_modules_reorder_post_request** | [**AuthModulesReorderPostRequest**](AuthModulesReorderPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_test_built_in_post

> crate::models::TdMemberProfile auth_modules_test_built_in_post(auth_modules_test_built_in_post_request, dollar_fields)
Test built-in settings

For a username/password combination, test built-in authentication with updated settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_modules_test_built_in_post_request** | [**AuthModulesTestBuiltInPostRequest**](AuthModulesTestBuiltInPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMemberProfile**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_test_ldap_post

> crate::models::EsDefaultProfileLoginDetails auth_modules_test_ldap_post(auth_modules_test_ldap_post_request, dollar_fields)
Test LDAP settings

For a username/password combination, test LDAP authentication with updated settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_modules_test_ldap_post_request** | [**AuthModulesTestLdapPostRequest**](AuthModulesTestLdapPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::EsDefaultProfileLoginDetails**](ES_DefaultProfileLoginDetails.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_throttled_logins_delete

> auth_modules_throttled_logins_delete(logins)
Reset throttling status

Resets the counter that tracks failed login attempts for the account with the specified logins. The member who use these accounts are no longer blocked from attempting to log in to Space.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logins** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_throttled_logins_get

> crate::models::AuthModulesThrottledLoginsGet200Response auth_modules_throttled_logins_get(dollar_skip, dollar_top, login, dollar_fields)
Get throttled logins

Returns logins that are currently subjected to rate limits when logging in to Space

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**login** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AuthModulesThrottledLoginsGet200Response**](_auth_modules_throttled_logins_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_throttled_logins_org_status_delete

> auth_modules_throttled_logins_org_status_delete()
Reset organization throttling

Resets date and time until which the organization are throttled

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_throttled_logins_org_status_get

> crate::models::OrgThrottlingStatus auth_modules_throttled_logins_org_status_get(dollar_fields)
Get organization throttling status

Returns date and time until which the organization are throttled

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::OrgThrottlingStatus**](OrgThrottlingStatus.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_modules_usages_get

> Vec<crate::models::AuthModuleUsage> auth_modules_usages_get(dollar_fields)
Get all usages

Retrieve a list of authentication module usage count

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AuthModuleUsage>**](AuthModuleUsage.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_admin_features_get

> crate::models::TierFeatureLimits billing_admin_features_get(dollar_fields)
Get features

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TierFeatureLimits**](TierFeatureLimits.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_admin_overdrafts_get

> crate::models::Overdrafts billing_admin_overdrafts_get(dollar_fields)
Get overdrafts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Overdrafts**](Overdrafts.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_admin_overdrafts_put

> billing_admin_overdrafts_put(billing_admin_overdrafts_put_request)
Set overdrafts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_admin_overdrafts_put_request** | [**BillingAdminOverdraftsPutRequest**](BillingAdminOverdraftsPutRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_admin_reports_billing_period_get

> crate::models::BillingReport billing_admin_reports_billing_period_get(billing_period, dollar_fields)
Get billing report

Returns a billing report for the given billing period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_period** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BillingReport**](BillingReport.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_admin_reports_today_get

> crate::models::TodayBillingReport billing_admin_reports_today_get(date, dollar_fields)
Get billing report for today

Returns a billing report for today

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TodayBillingReport**](TodayBillingReport.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_admin_trial_put

> billing_admin_trial_put(billing_admin_trial_put_request)
Activate trial. Not available for On-Premises installations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_admin_trial_put_request** | [**BillingAdminTrialPutRequest**](BillingAdminTrialPutRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blog_aliasalias_get

> crate::models::ArticleRecord blog_aliasalias_get(alias, dollar_fields)
Get blog post by alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleRecord**](ArticleRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blog_external_idid_get

> crate::models::ArticleRecord blog_external_idid_get(id, dollar_fields)
Get blog post by external ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleRecord**](ArticleRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blog_get

> crate::models::BlogGet200Response blog_get(dollar_skip, dollar_top, term, date_from, date_to, author_id, team_id, location_id, for_profile, dollar_fields)
Get all blog posts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**term** | Option<**String**> |  |  |
**date_from** | Option<**String**> |  |  |
**date_to** | Option<**String**> |  |  |
**author_id** | Option<**String**> |  |  |
**team_id** | Option<**String**> |  |  |
**location_id** | Option<**String**> |  |  |
**for_profile** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BlogGet200Response**](_blog_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blog_id_delete

> blog_id_delete(id)
Unpublish blog post

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blog_id_get

> crate::models::ArticleRecord blog_id_get(id, dollar_fields)
Get blog post

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleRecord**](ArticleRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blog_id_patch

> crate::models::ArticleRecord blog_id_patch(id, dollar_fields, blog_id_patch_request)
Update blog post

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**blog_id_patch_request** | Option<[**BlogIdPatchRequest**](BlogIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::ArticleRecord**](ArticleRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blog_import_post

> Vec<crate::models::ArticleImportResult> blog_import_post(blog_import_post_request, dollar_fields)
Import blog posts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blog_import_post_request** | [**BlogImportPostRequest**](BlogImportPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ArticleImportResult>**](ArticleImportResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blog_post

> crate::models::ArticleRecord blog_post(blog_post_request, dollar_fields)
Publish blog post

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blog_post_request** | [**BlogPostRequest**](BlogPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleRecord**](ArticleRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blog_stats_get

> crate::models::BgStats blog_stats_get(date_from, date_to, author_id, team_id, location_id, dollar_fields)
Get stats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | Option<**String**> |  |  |
**date_to** | Option<**String**> |  |  |
**author_id** | Option<**String**> |  |  |
**team_id** | Option<**String**> |  |  |
**location_id** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BgStats**](BG_Stats.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_absence_events_get

> Vec<crate::models::AbsenceEvent> calendars_absence_events_get(date_from, date_to, team, location, role, dollar_fields)
Get all absence events

Get/search absences. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**team** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AbsenceEvent>**](AbsenceEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_birthday_events_get

> Vec<crate::models::BirthdayEvent> calendars_birthday_events_get(date_from, date_to, team, location, role, dollar_fields)
Get all birthday events

Get/search birthdays. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**team** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::BirthdayEvent>**](BirthdayEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_birthday_events_starred_get

> Vec<crate::models::BirthdayEvent> calendars_birthday_events_starred_get(date_from, date_to, dollar_fields)
Get all starred birthday events

Get/search birthdays in a specific time period for starred profiles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::BirthdayEvent>**](BirthdayEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_event_participations_id_patch

> crate::models::MeetingRecord calendars_event_participations_id_patch(id, calendars_event_participations_id_patch_request, dollar_fields)
Update event participation

Update RSVP / calendar event participation status for a calendar event attached to an article

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**calendars_event_participations_id_patch_request** | [**CalendarsEventParticipationsIdPatchRequest**](CalendarsEventParticipationsIdPatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::MeetingRecord**](MeetingRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_events_get

> Vec<crate::models::MeetingRecord> calendars_events_get(date_from, date_to, dollar_fields)
Get all events

Get calendar events attached to an article in a specific time period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::MeetingRecord>**](MeetingRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_events_id_get

> crate::models::MeetingRecord calendars_events_id_get(id, dollar_fields)
Get event

Get a calendar event attached to an article

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::MeetingRecord**](MeetingRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_holidays_get

> Vec<crate::models::HolidaysEvent> calendars_holidays_get(start_date, end_date, team, location, role, working_days, dollar_fields)
Get all holidays

Get/search holidays. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** |  | [required] |
**end_date** | **String** |  | [required] |
**team** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**working_days** | Option<**bool**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::HolidaysEvent>**](HolidaysEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_get

> crate::models::CalendarsMeetingsGet200Response calendars_meetings_get(dollar_skip, dollar_top, summary_query, locations_query, starting_after, ending_after, ending_before, starting_before, profiles, teams, organizer, include_private, include_archived, include_meeting_instances, dollar_fields)
Get all meetings

Search meetings by name, location, time period and other parameters. Parameters are applied as 'AND' filters while values in lists of locations, profiles and teams have 'OR' semantics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**summary_query** | Option<**String**> |  |  |[default to ]
**locations_query** | Option<[**Vec<String>**](String.md)> |  |  |[default to []]
**starting_after** | Option<**String**> |  |  |
**ending_after** | Option<**String**> |  |  |
**ending_before** | Option<**String**> |  |  |
**starting_before** | Option<**String**> |  |  |
**profiles** | Option<[**Vec<String>**](String.md)> |  |  |[default to []]
**teams** | Option<[**Vec<String>**](String.md)> |  |  |[default to []]
**organizer** | Option<**String**> |  |  |
**include_private** | Option<**bool**> |  |  |[default to false]
**include_archived** | Option<**bool**> |  |  |[default to false]
**include_meeting_instances** | Option<**bool**> |  |  |[default to true]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CalendarsMeetingsGet200Response**](_calendars_meetings_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_id_conference_rooms_delete

> calendars_meetings_id_conference_rooms_delete(id, room_id, date_time)
Remove conference room

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**room_id** | **String** |  | [required] |
**date_time** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_id_conference_rooms_post

> calendars_meetings_id_conference_rooms_post(id, calendars_meetings_id_conference_rooms_post_request)
Add conference room

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**calendars_meetings_id_conference_rooms_post_request** | [**CalendarsMeetingsIdConferenceRoomsPostRequest**](CalendarsMeetingsIdConferenceRoomsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_id_delete

> crate::models::DtoMeeting calendars_meetings_id_delete(id, target_date, modification_kind, dollar_fields)
Delete meeting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**target_date** | Option<**String**> |  |  |
**modification_kind** | Option<[**RecurrentModification**](.md)> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DtoMeeting**](DTO_Meeting.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_id_get

> crate::models::DtoMeeting calendars_meetings_id_get(id, dollar_fields)
Get meeting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DtoMeeting**](DTO_Meeting.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_id_participation_status_patch

> crate::models::DtoMeeting calendars_meetings_id_participation_status_patch(id, calendars_meetings_id_participation_status_patch_request, dollar_fields)
Update profile participation status

Update profile participation status for a meeting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**calendars_meetings_id_participation_status_patch_request** | [**CalendarsMeetingsIdParticipationStatusPatchRequest**](CalendarsMeetingsIdParticipationStatusPatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DtoMeeting**](DTO_Meeting.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_id_patch

> crate::models::DtoMeeting calendars_meetings_id_patch(id, dollar_fields, calendars_meetings_id_patch_request)
Update meeting

Patch a meeting. Only not-null parameters and not empty diffs will be applied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**calendars_meetings_id_patch_request** | Option<[**CalendarsMeetingsIdPatchRequest**](CalendarsMeetingsIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::DtoMeeting**](DTO_Meeting.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_next_occurrence_get

> crate::models::MeetingOccurrenceTime calendars_meetings_next_occurrence_get(meeting_id, since, dollar_fields)
Get next meeting occurrence

Search for the next meeting occurrence that starts after the provided time point or after the current time if it is not specified

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meeting_id** | **String** |  | [required] |
**since** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::MeetingOccurrenceTime**](MeetingOccurrenceTime.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_occurrences_by_meeting_get

> Vec<crate::models::MeetingWithOccurrenceTime> calendars_meetings_occurrences_by_meeting_get(meeting_ids, since, until, limit, dollar_fields)
Get meeting occurrences for period for multiple meetings

Search for occurrences of a meeting that start in the provided time interval. Interval limits are inclusive, response is limited by the first 1_000 results by default (per meeting).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meeting_ids** | [**Vec<String>**](String.md) |  | [required] |
**since** | **String** |  | [required] |
**until** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::MeetingWithOccurrenceTime>**](MeetingWithOccurrenceTime.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_occurrences_get

> Vec<crate::models::MeetingOccurrenceTime> calendars_meetings_occurrences_get(meeting_id, since, until, limit, dollar_fields)
Get meeting occurrences for period

Search for occurrences of a meeting that start in the provided time interval. Interval limits are inclusive, response is limited by the first 1_000 results by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meeting_id** | **String** |  | [required] |
**since** | **String** |  | [required] |
**until** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::MeetingOccurrenceTime>**](MeetingOccurrenceTime.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_participation_statuses_external_get

> Vec<crate::models::EventParticipationStatus> calendars_meetings_participation_statuses_external_get(id, emails)
Get RSVP statuses for external users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**emails** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**Vec<crate::models::EventParticipationStatus>**](EventParticipationStatus.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_participation_statuses_get

> Vec<crate::models::EventParticipationStatus> calendars_meetings_participation_statuses_get(id, profile_ids)
Get meeting participation statuses for profiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**profile_ids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**Vec<crate::models::EventParticipationStatus>**](EventParticipationStatus.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_post

> crate::models::DtoMeeting calendars_meetings_post(calendars_meetings_post_request, dollar_fields)
Create meeting

Create a meeting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calendars_meetings_post_request** | [**CalendarsMeetingsPostRequest**](CalendarsMeetingsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DtoMeeting**](DTO_Meeting.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_profile_participation_get

> Vec<crate::models::CalendarsMeetingsProfileParticipationGet200ResponseInner> calendars_meetings_profile_participation_get(profile_id, events, dollar_fields)
Get profile participation statuses for meetings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** |  | [required] |
**events** | [**Vec<String>**](String.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::CalendarsMeetingsProfileParticipationGet200ResponseInner>**](_calendars_meetings_profile_participation_get_200_response_inner.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_meetings_profile_participation_records_get

> Vec<crate::models::DtoMeetingRsvp> calendars_meetings_profile_participation_records_get(profile_id, events, dollar_fields)
Get profile participation status records for meetings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** |  | [required] |
**events** | [**Vec<String>**](String.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::DtoMeetingRsvp>**](DTO_MeetingRSVP.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_membership_events_get

> Vec<crate::models::MembershipEvent> calendars_membership_events_get(date_from, date_to, team, location, role, dollar_fields)
Get all membership events

Get/search membership events. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**team** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::MembershipEvent>**](MembershipEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendars_non_working_days_events_get

> Vec<crate::models::NonWorkingDaysEvent> calendars_non_working_days_events_get(date_from, date_to, member, team, location, role, dollar_fields)
Get all non working days events

Get/search non-working day events. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**member** | Option<**String**> |  |  |
**team** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::NonWorkingDaysEvent>**](NonWorkingDaysEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calls_post

> crate::models::Room calls_post(calls_post_request, dollar_fields)
Create call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calls_post_request** | [**CallsPostRequest**](CallsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Room**](Room.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_all_channels_get

> crate::models::ChatsChannelsAllChannelsGet200Response chats_channels_all_channels_get(query, dollar_skip, dollar_top, quick_filter, sort_column, sort_order, public_only, with_archived, subscriber, dollar_fields)
List all channels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**quick_filter** | Option<[**AllChannelsFilter**](.md)> |  |  |
**sort_column** | Option<[**AllChannelsSortColumn**](.md)> |  |  |
**sort_order** | Option<[**ColumnSortOrder**](.md)> |  |  |
**public_only** | Option<**bool**> |  |  |[default to false]
**with_archived** | Option<**bool**> |  |  |[default to true]
**subscriber** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsAllChannelsGet200Response**](_chats_channels_all_channels_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_administrator_get

> crate::models::CPrincipal chats_channels_channel_administrator_get(channel, dollar_fields)
Get channel administrator

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CPrincipal**](CPrincipal.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_administrator_patch

> chats_channels_channel_administrator_patch(channel, chats_channels_channel_administrator_patch_request)
Assign channel administrator

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**chats_channels_channel_administrator_patch_request** | [**ChatsChannelsChannelAdministratorPatchRequest**](ChatsChannelsChannelAdministratorPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_archive_delete

> chats_channels_channel_archive_delete(channel)
Archive channel

Archive a channel and reject new messages being added. It is still possible to view messages from an archived channel. It is possible to restore the channel later.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_attachments_files_get

> crate::models::ChatsChannelsChannelAttachmentsGet200Response chats_channels_channel_attachments_files_get(channel, dollar_skip, dollar_top, dollar_fields)
List file attachments in channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelAttachmentsGet200Response**](_chats_channels__channel__attachments_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_attachments_get

> crate::models::ChatsChannelsChannelAttachmentsGet200Response chats_channels_channel_attachments_get(channel, dollar_skip, dollar_top, dollar_fields)
List attachments in channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelAttachmentsGet200Response**](_chats_channels__channel__attachments_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_attachments_images_get

> crate::models::ChatsChannelsChannelAttachmentsGet200Response chats_channels_channel_attachments_images_get(channel, dollar_skip, dollar_top, dollar_fields)
List images in channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelAttachmentsGet200Response**](_chats_channels__channel__attachments_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_attachments_links_get

> crate::models::ChatsChannelsChannelAttachmentsGet200Response chats_channels_channel_attachments_links_get(channel, dollar_skip, dollar_top, dollar_fields)
List links in channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelAttachmentsGet200Response**](_chats_channels__channel__attachments_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_attachments_videos_get

> crate::models::ChatsChannelsChannelAttachmentsGet200Response chats_channels_channel_attachments_videos_get(channel, dollar_skip, dollar_top, dollar_fields)
List videos in channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelAttachmentsGet200Response**](_chats_channels__channel__attachments_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_delete

> chats_channels_channel_delete(channel)
Delete channel

Delete a channel. No one will be able to view this channel or its threads. This action cannot be undone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_description_patch

> chats_channels_channel_description_patch(channel, chats_channels_channel_description_patch_request)
Change channel description

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**chats_channels_channel_description_patch_request** | [**ChatsChannelsChannelDescriptionPatchRequest**](ChatsChannelsChannelDescriptionPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_get

> crate::models::M2ChannelRecord chats_channels_channel_get(channel, dollar_fields)
Get channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::M2ChannelRecord**](M2ChannelRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_icon_patch

> chats_channels_channel_icon_patch(channel, chats_channels_channel_icon_patch_request)
Change channel icon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**chats_channels_channel_icon_patch_request** | Option<[**ChatsChannelsChannelIconPatchRequest**](ChatsChannelsChannelIconPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_name_patch

> chats_channels_channel_name_patch(channel, chats_channels_is_name_free_post_request)
Rename channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**chats_channels_is_name_free_post_request** | [**ChatsChannelsIsNameFreePostRequest**](ChatsChannelsIsNameFreePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_restore_archived_post

> chats_channels_channel_restore_archived_post(channel)
Restore archived channel

Restore an archived channel and allow new messages to be added again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_subscribers_teams_delete

> chats_channels_channel_subscribers_teams_delete(channel, teams)
Remove teams from channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**teams** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_subscribers_teams_get

> crate::models::ChatsChannelsChannelSubscribersTeamsGet200Response chats_channels_channel_subscribers_teams_get(channel, query, dollar_skip, dollar_top, dollar_fields)
List teams subscribed to channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**query** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelSubscribersTeamsGet200Response**](_chats_channels__channel__subscribers_teams_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_subscribers_teams_post

> chats_channels_channel_subscribers_teams_post(channel, chats_channels_channel_subscribers_teams_post_request)
Add teams to channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**chats_channels_channel_subscribers_teams_post_request** | [**ChatsChannelsChannelSubscribersTeamsPostRequest**](ChatsChannelsChannelSubscribersTeamsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_subscribers_users_delete

> chats_channels_channel_subscribers_users_delete(channel, profiles)
Remove users from channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**profiles** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_subscribers_users_get

> crate::models::ChatsChannelsChannelSubscribersUsersGet200Response chats_channels_channel_subscribers_users_get(channel, query, dollar_skip, dollar_top, dollar_fields)
List users subscribed to channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**query** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelSubscribersUsersGet200Response**](_chats_channels__channel__subscribers_users_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_channel_subscribers_users_post

> chats_channels_channel_subscribers_users_post(channel, chats_channels_channel_subscribers_users_post_request)
Add users to channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**chats_channels_channel_subscribers_users_post_request** | [**ChatsChannelsChannelSubscribersUsersPostRequest**](ChatsChannelsChannelSubscribersUsersPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_conversations_channel_convert_post

> crate::models::M2ChannelRecord chats_channels_conversations_channel_convert_post(channel, chats_channels_conversations_channel_convert_post_request, dollar_fields)
Convert conversation to private channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**chats_channels_conversations_channel_convert_post_request** | [**ChatsChannelsConversationsChannelConvertPostRequest**](ChatsChannelsConversationsChannelConvertPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::M2ChannelRecord**](M2ChannelRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_conversations_channel_subject_patch

> chats_channels_conversations_channel_subject_patch(channel, chats_channels_conversations_channel_subject_patch_request)
Change conversation subject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**chats_channels_conversations_channel_subject_patch_request** | [**ChatsChannelsConversationsChannelSubjectPatchRequest**](ChatsChannelsConversationsChannelSubjectPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_conversations_post

> crate::models::M2ChannelRecord chats_channels_conversations_post(chats_channels_conversations_post_request, dollar_fields)
Create conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chats_channels_conversations_post_request** | [**ChatsChannelsConversationsPostRequest**](ChatsChannelsConversationsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::M2ChannelRecord**](M2ChannelRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_dm_post

> crate::models::M2ChannelRecord chats_channels_dm_post(chats_channels_dm_post_request, dollar_fields)
Get or create direct messages channel

Create or get a direct messages channel with a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chats_channels_dm_post_request** | [**ChatsChannelsDmPostRequest**](ChatsChannelsDmPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::M2ChannelRecord**](M2ChannelRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_is_name_free_post

> bool chats_channels_is_name_free_post(chats_channels_is_name_free_post_request)
Is name free?

Check whether a channel name is available. Returns true when the channel name can be used to create a new channel, false otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chats_channels_is_name_free_post_request** | [**ChatsChannelsIsNameFreePostRequest**](ChatsChannelsIsNameFreePostRequest.md) |  | [required] |

### Return type

**bool**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_channels_post

> crate::models::M2ChannelRecord chats_channels_post(chats_channels_post_request, dollar_fields)
Add new channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chats_channels_post_request** | [**ChatsChannelsPostRequest**](ChatsChannelsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::M2ChannelRecord**](M2ChannelRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_delete_message_post

> chats_messages_delete_message_post(chats_messages_delete_message_post_request)
Delete message

Delete a message from a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chats_messages_delete_message_post_request** | [**ChatsMessagesDeleteMessagePostRequest**](ChatsMessagesDeleteMessagePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_edit_message_post

> chats_messages_edit_message_post(chats_messages_edit_message_post_request)
Edit message

Edit an existing message. Message content can be a string, or a block with one or several sections of information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chats_messages_edit_message_post_request** | [**ChatsMessagesEditMessagePostRequest**](ChatsMessagesEditMessagePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_get

> crate::models::GetMessagesResponse chats_messages_get(channel, sorting, batch_size, start_from_date, dollar_fields)
Get channel messages

Retrieve a batch of messages from a channel. Messages are divided into batches by providing `sorting`, `startFromDate` and `batchSize` parameters. If the retrieved number of messages is less than `batchSize`, there are currently no more messages to return. Return data also contains next value for `startFromDate` as well as the `orgLimitReached` flag indicating whether part of messages could not be retrieved because of organization plan limitation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**sorting** | [**MessagesSorting**](.md) |  | [required] |
**batch_size** | **i32** |  | [required] |
**start_from_date** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GetMessagesResponse**](GetMessagesResponse.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_import_post

> chats_messages_import_post(chats_messages_import_post_request)
Import messages

This API method is intended to be used only by applications. The `createdAtUtc` and `editedAtUtc` parameters are Unix epoch timestamps in *milliseconds*.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chats_messages_import_post_request** | [**ChatsMessagesImportPostRequest**](ChatsMessagesImportPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_message_get

> crate::models::ChannelItemRecord chats_messages_message_get(message, channel, dollar_fields)
Get message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message** | **String** |  | [required] |
**channel** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChannelItemRecord**](ChannelItemRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_pin_patch

> chats_messages_pin_patch(chats_messages_pin_patch_request)
Pin message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chats_messages_pin_patch_request** | [**ChatsMessagesPinPatchRequest**](ChatsMessagesPinPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_pinned_messages_get

> crate::models::ChatsMessagesPinnedMessagesGet200Response chats_messages_pinned_messages_get(channel, dollar_skip, dollar_top, dollar_fields)
List pinned messages in channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsMessagesPinnedMessagesGet200Response**](_chats_messages_pinned_messages_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_send_message_post

> crate::models::ChannelItemRecord chats_messages_send_message_post(chats_messages_send_message_post_request, dollar_fields)
Send message

Send a message to a channel, thread, member, issue, code review, etc. Message content can be a string, or a block with one or several sections of information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chats_messages_send_message_post_request** | [**ChatsMessagesSendMessagePostRequest**](ChatsMessagesSendMessagePostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChannelItemRecord**](ChannelItemRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_send_post

> crate::models::ChannelItemRecord chats_messages_send_post(chats_messages_send_post_request, dollar_fields)
Send text message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chats_messages_send_post_request** | [**ChatsMessagesSendPostRequest**](ChatsMessagesSendPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChannelItemRecord**](ChannelItemRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_sync_batch_current_etag_get

> String chats_messages_sync_batch_current_etag_get(channel)
Get current sync etag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_sync_batch_get

> crate::models::ChatsMessagesSyncBatchGet200Response chats_messages_sync_batch_get(batch_info, channel, dollar_fields)
Get sync batch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_info** | **String** |  | [required] |
**channel** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsMessagesSyncBatchGet200Response**](_chats_messages_sync_batch_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chats_messages_unpin_patch

> chats_messages_unpin_patch(chats_messages_pin_patch_request)
Unpin message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chats_messages_pin_patch_request** | [**ChatsMessagesPinPatchRequest**](ChatsMessagesPinPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checklists_checklist_items_plan_item_delete

> checklists_checklist_items_plan_item_delete(checklist, plan_item)
Delete plan item

Delete plan item and its children from a checklist

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**checklist** | **String** |  | [required] |
**plan_item** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checklists_checklist_items_plan_item_get

> crate::models::PlanItem checklists_checklist_items_plan_item_get(checklist, plan_item, dollar_fields)
Get plan item

Get plan item by its identifier in a checklist

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**checklist** | **String** |  | [required] |
**plan_item** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PlanItem**](PlanItem.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checklists_checklist_items_plan_item_move_post

> crate::models::PlanItem checklists_checklist_items_plan_item_move_post(checklist, plan_item, checklists_checklist_items_plan_item_move_post_request, dollar_fields)
Move plan item

Move plan item in a checklist

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**checklist** | **String** |  | [required] |
**plan_item** | **String** |  | [required] |
**checklists_checklist_items_plan_item_move_post_request** | [**ChecklistsChecklistItemsPlanItemMovePostRequest**](ChecklistsChecklistItemsPlanItemMovePostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PlanItem**](PlanItem.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checklists_checklist_items_plan_item_patch

> crate::models::PlanItem checklists_checklist_items_plan_item_patch(checklist, plan_item, dollar_fields, checklists_checklist_items_plan_item_patch_request)
Update plan item

Update plan item in a checklist

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**checklist** | **String** |  | [required] |
**plan_item** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**checklists_checklist_items_plan_item_patch_request** | Option<[**ChecklistsChecklistItemsPlanItemPatchRequest**](ChecklistsChecklistItemsPlanItemPatchRequest.md)> |  |  |

### Return type

[**crate::models::PlanItem**](PlanItem.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checklists_checklist_items_post

> crate::models::PlanItem checklists_checklist_items_post(checklist, checklists_checklist_items_post_request, dollar_fields)
Create plan item

Create plan item as the last element of the top level in a checklist if parent plan item is null, or as the last child if parent plan item is provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**checklist** | **String** |  | [required] |
**checklists_checklist_items_post_request** | [**ChecklistsChecklistItemsPostRequest**](ChecklistsChecklistItemsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PlanItem**](PlanItem.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_extended_types_get

> Vec<crate::models::ExtendedType> custom_fields_extended_types_get(scope, dollar_fields)
Get all extended types

Get all types that support custom fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | Option<[**ExtendedTypeScopeType**](.md)> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ExtendedType>**](ExtendedType.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_all_values_get

> crate::models::CustomFieldsTypeKeyAllValuesGet200Response custom_fields_type_key_all_values_get(type_key, dollar_skip, dollar_top, extended_entity_ids, scope, dollar_fields)
Get all values

Get all custom field values for a type. Optionally, extendedEntityIds can be used to get data for one or more entity IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**extended_entity_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**scope** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CustomFieldsTypeKeyAllValuesGet200Response**](_custom_fields__typeKey__all_values_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_entity_id_values_get

> crate::models::CustomFieldsRecord custom_fields_type_key_entity_id_values_get(type_key, entity_id, scope, dollar_fields)
Get value

Get custom field value for a type and entity ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**entity_id** | **String** |  | [required] |
**scope** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CustomFieldsRecord**](CustomFieldsRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_entity_id_values_patch

> custom_fields_type_key_entity_id_values_patch(entity_id, type_key, custom_fields_type_key_entity_id_values_patch_request)
Update value

Update custom field value(s) for a type and entity ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** |  | [required] |
**type_key** | **String** |  | [required] |
**custom_fields_type_key_entity_id_values_patch_request** | [**CustomFieldsTypeKeyEntityIdValuesPatchRequest**](CustomFieldsTypeKeyEntityIdValuesPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_enum_values_custom_field_id_get

> crate::models::CustomFieldsTypeKeyEnumValuesCustomFieldIdGet200Response custom_fields_type_key_enum_values_custom_field_id_get(type_key, custom_field_id, dollar_skip, dollar_top, query, ordering, count_records, added_by_profile_id, scope, dollar_fields)
Get all enum values

Get a page of options for custom field of `Select from options` type with `Open-ended` flag set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**custom_field_id** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**query** | Option<**String**> |  |  |
**ordering** | Option<[**EnumValueOrdering**](.md)> |  |  |
**count_records** | Option<**bool**> |  |  |
**added_by_profile_id** | Option<**String**> |  |  |
**scope** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CustomFieldsTypeKeyEnumValuesCustomFieldIdGet200Response**](_custom_fields__typeKey__enum_values__customFieldId__get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_enum_values_custom_field_id_post

> Vec<crate::models::EnumValueData> custom_fields_type_key_enum_values_custom_field_id_post(type_key, custom_field_id, custom_fields_type_key_enum_values_custom_field_id_post_request, dollar_fields)
Create enum value

Add new option to custom field of `Select from options` type. Options can only be added via this API call if custom field has the `Open-ended` flag set. Returns saved records.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**custom_field_id** | **String** |  | [required] |
**custom_fields_type_key_enum_values_custom_field_id_post_request** | [**CustomFieldsTypeKeyEnumValuesCustomFieldIdPostRequest**](CustomFieldsTypeKeyEnumValuesCustomFieldIdPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::EnumValueData>**](EnumValueData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_fields_get

> Vec<crate::models::CustomField> custom_fields_type_key_fields_get(type_key, with_archived, scope, dollar_fields)
Get all fields

Get custom fields for a type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**with_archived** | Option<**bool**> |  |  |[default to false]
**scope** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::CustomField>**](CustomField.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_fields_id_archive_post

> custom_fields_type_key_fields_id_archive_post(type_key, id, custom_fields_type_key_fields_id_archive_post_request)
Archive field

Archive a custom field for a type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**id** | **String** |  | [required] |
**custom_fields_type_key_fields_id_archive_post_request** | [**CustomFieldsTypeKeyFieldsIdArchivePostRequest**](CustomFieldsTypeKeyFieldsIdArchivePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_fields_id_delete

> custom_fields_type_key_fields_id_delete(type_key, id, scope)
Delete field

Remove custom field for a type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**id** | **String** |  | [required] |
**scope** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_fields_id_filter_values_get

> crate::models::CustomFieldsTypeKeyFieldsIdFilterValuesGet200Response custom_fields_type_key_fields_id_filter_values_get(type_key, id, dollar_skip, dollar_top, scope, calculate_total, dollar_fields)
Get all filter values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**id** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**scope** | Option<**String**> |  |  |
**calculate_total** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CustomFieldsTypeKeyFieldsIdFilterValuesGet200Response**](_custom_fields__typeKey__fields__id__filter_values_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_fields_id_patch

> custom_fields_type_key_fields_id_patch(type_key, id, custom_fields_type_key_fields_id_patch_request)
Update field

Update custom field for a type. Optional parameters will be ignored when not specified and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**id** | **String** |  | [required] |
**custom_fields_type_key_fields_id_patch_request** | [**CustomFieldsTypeKeyFieldsIdPatchRequest**](CustomFieldsTypeKeyFieldsIdPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_fields_id_restore_post

> custom_fields_type_key_fields_id_restore_post(type_key, id, custom_fields_type_key_fields_id_archive_post_request)
Restore field

Restore custom field for a type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**id** | **String** |  | [required] |
**custom_fields_type_key_fields_id_archive_post_request** | [**CustomFieldsTypeKeyFieldsIdArchivePostRequest**](CustomFieldsTypeKeyFieldsIdArchivePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_fields_post

> crate::models::CustomField custom_fields_type_key_fields_post(type_key, custom_fields_type_key_fields_post_request, dollar_fields)
Create field

Create custom field for a type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**custom_fields_type_key_fields_post_request** | [**CustomFieldsTypeKeyFieldsPostRequest**](CustomFieldsTypeKeyFieldsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CustomField**](CustomField.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_type_key_fields_reorder_post

> custom_fields_type_key_fields_reorder_post(type_key, custom_fields_type_key_fields_reorder_post_request)
Reorder fields

Re-order custom fields. Pass IDs of the custom fields in the order you wish the custom fields to be.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_key** | **String** |  | [required] |
**custom_fields_type_key_fields_reorder_post_request** | [**CustomFieldsTypeKeyFieldsReorderPostRequest**](CustomFieldsTypeKeyFieldsReorderPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_custom_field_archive_post

> custom_fields_v2_entity_type_fields_custom_field_archive_post(entity_type, custom_field)
Archive custom field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_custom_field_delete

> custom_fields_v2_entity_type_fields_custom_field_delete(entity_type, custom_field)
Delete custom field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_custom_field_enum_values_bulk_update_post

> custom_fields_v2_entity_type_fields_custom_field_enum_values_bulk_update_post(entity_type, custom_field, custom_fields_v2_entity_type_fields_custom_field_enum_values_bulk_update_post_request)
Bulk update enum values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |
**custom_fields_v2_entity_type_fields_custom_field_enum_values_bulk_update_post_request** | [**CustomFieldsV2EntityTypeFieldsCustomFieldEnumValuesBulkUpdatePostRequest**](CustomFieldsV2EntityTypeFieldsCustomFieldEnumValuesBulkUpdatePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_custom_field_enum_values_enum_value_to_remove_delete

> custom_fields_v2_entity_type_fields_custom_field_enum_values_enum_value_to_remove_delete(entity_type, custom_field, enum_value_to_remove)
Delete enum value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |
**enum_value_to_remove** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_custom_field_enum_values_get

> crate::models::CustomFieldsV2EntityTypeFieldsCustomFieldEnumValuesGet200Response custom_fields_v2_entity_type_fields_custom_field_enum_values_get(entity_type, custom_field, query, ordering, added_by_profile_id, dollar_skip, dollar_top, dollar_fields)
Get enum values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |
**query** | Option<**String**> |  |  |
**ordering** | Option<[**EnumValueOrdering**](.md)> |  |  |
**added_by_profile_id** | Option<**String**> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CustomFieldsV2EntityTypeFieldsCustomFieldEnumValuesGet200Response**](_custom_fields_v2__entityType__fields__customField__enum_values_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_custom_field_enum_values_patch

> custom_fields_v2_entity_type_fields_custom_field_enum_values_patch(entity_type, custom_field, custom_fields_v2_entity_type_fields_custom_field_enum_values_patch_request)
Update enum value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |
**custom_fields_v2_entity_type_fields_custom_field_enum_values_patch_request** | [**CustomFieldsV2EntityTypeFieldsCustomFieldEnumValuesPatchRequest**](CustomFieldsV2EntityTypeFieldsCustomFieldEnumValuesPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_custom_field_enum_values_post

> crate::models::CfEnumValue custom_fields_v2_entity_type_fields_custom_field_enum_values_post(entity_type, custom_field, custom_fields_v2_entity_type_fields_custom_field_enum_values_post_request, dollar_fields)
Create enum value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |
**custom_fields_v2_entity_type_fields_custom_field_enum_values_post_request** | [**CustomFieldsV2EntityTypeFieldsCustomFieldEnumValuesPostRequest**](CustomFieldsV2EntityTypeFieldsCustomFieldEnumValuesPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CfEnumValue**](CFEnumValue.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_custom_field_get

> crate::models::CustomFieldData custom_fields_v2_entity_type_fields_custom_field_get(entity_type, custom_field, dollar_fields)
Get single custom field

Get configured custom field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CustomFieldData**](CustomFieldData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_custom_field_patch

> custom_fields_v2_entity_type_fields_custom_field_patch(entity_type, custom_field, custom_fields_v2_entity_type_fields_custom_field_patch_request)
Update custom field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |
**custom_fields_v2_entity_type_fields_custom_field_patch_request** | Option<[**CustomFieldsV2EntityTypeFieldsCustomFieldPatchRequest**](CustomFieldsV2EntityTypeFieldsCustomFieldPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_custom_field_restore_post

> custom_fields_v2_entity_type_fields_custom_field_restore_post(entity_type, custom_field)
Restore custom field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_get

> Vec<crate::models::CustomFieldData> custom_fields_v2_entity_type_fields_get(entity_type, with_archived, dollar_fields)
Get custom fields

Get all configured custom fields for an entity type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**with_archived** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::CustomFieldData>**](CustomFieldData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_post

> crate::models::CustomFieldData custom_fields_v2_entity_type_fields_post(entity_type, custom_fields_v2_entity_type_fields_post_request, dollar_fields)
Create custom field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_fields_v2_entity_type_fields_post_request** | [**CustomFieldsV2EntityTypeFieldsPostRequest**](CustomFieldsV2EntityTypeFieldsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CustomFieldData**](CustomFieldData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_entity_type_fields_reorder_post

> custom_fields_v2_entity_type_fields_reorder_post(entity_type, custom_fields_v2_entity_type_fields_reorder_post_request)
Reorder custom fields

Re-order custom fields. Pass identifiers of the custom fields in the order you wish the custom fields to be.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** |  | [required] |
**custom_fields_v2_entity_type_fields_reorder_post_request** | [**CustomFieldsV2EntityTypeFieldsReorderPostRequest**](CustomFieldsV2EntityTypeFieldsReorderPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_values_entity_custom_field_get

> crate::models::CustomFieldValueData custom_fields_v2_values_entity_custom_field_get(entity, custom_field, dollar_fields)
Get single value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CustomFieldValueData**](CustomFieldValueData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_values_entity_custom_field_post

> custom_fields_v2_values_entity_custom_field_post(entity, custom_field, custom_fields_v2_values_entity_custom_field_post_request)
Set single value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity** | **String** |  | [required] |
**custom_field** | **String** |  | [required] |
**custom_fields_v2_values_entity_custom_field_post_request** | [**CustomFieldsV2ValuesEntityCustomFieldPostRequest**](CustomFieldsV2ValuesEntityCustomFieldPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_values_entity_get

> Vec<crate::models::CustomFieldValueData> custom_fields_v2_values_entity_get(entity, dollar_fields)
Get all values for entity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::CustomFieldValueData>**](CustomFieldValueData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_v2_values_entity_post

> custom_fields_v2_values_entity_post(entity, custom_fields_v2_values_entity_post_request)
Set values for entity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity** | **String** |  | [required] |
**custom_fields_v2_values_entity_post_request** | [**CustomFieldsV2ValuesEntityPostRequest**](CustomFieldsV2ValuesEntityPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emojis_add_post

> emojis_add_post(emojis_add_post_request)
Add emoji

Add custom emoji

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emojis_add_post_request** | [**EmojisAddPostRequest**](EmojisAddPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emojis_delete_post

> emojis_delete_post(emojis_delete_post_request)
Delete emoji

Delete an emoji by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emojis_delete_post_request** | [**EmojisDeletePostRequest**](EmojisDeletePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emojis_exists_get

> bool emojis_exists_get(emoji)
Check if emoji exists

Check whether a given emoji name exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emoji** | **String** |  | [required] |

### Return type

**bool**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emojis_frequently_used_get

> Vec<String> emojis_frequently_used_get()
Get frequently used emojis

List frequently used emojis

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emojis_record_usage_post

> emojis_record_usage_post(emojis_record_usage_post_request)
Record emojis usage

Record emojis usage and update frequently used list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emojis_record_usage_post_request** | [**EmojisRecordUsagePostRequest**](EmojisRecordUsagePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emojis_search_get

> crate::models::EmojisSearchGet200Response emojis_search_get(query, dollar_skip, dollar_top, version, dollar_fields)
Search emoji

Search for emoji

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**version** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::EmojisSearchGet200Response**](_emojis_search_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emojis_sync_batch_get

> crate::models::EmojisSyncBatchGet200Response emojis_sync_batch_get(batch_info, dollar_fields)
Get sync batch

Get custom emojis for synchronization with third-party system. Custom emojis with etag greater than specified value are returned. Read more in the [documentation](https://www.jetbrains.com/help/space/sync-api.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_info** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::EmojisSyncBatchGet200Response**](_emojis_sync_batch_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_default_issue_status_for_mr_merge_post

> external_issues_default_issue_status_for_mr_merge_post(external_issues_default_issue_status_for_mr_merge_post_request)
Set default target issue status for merge request merge

Set default status to move external issues to when linked merge request is merged in Space

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_issues_default_issue_status_for_mr_merge_post_request** | [**ExternalIssuesDefaultIssueStatusForMrMergePostRequest**](ExternalIssuesDefaultIssueStatusForMrMergePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_events_queue_get

> crate::models::ExternalIssueEventQueueItemsBatch external_issues_events_queue_get(batch_size, from_etag, issue_code_linking_events_shape, include_commit_changes, dollar_fields)
Get external issue event queue items

Fetch events about external issues from Space

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_size** | **i32** |  | [required] |
**from_etag** | Option<**i64**> |  |  |
**issue_code_linking_events_shape** | Option<[**ExternalIssueCodeLinkingEventsShape**](.md)> |  |  |
**include_commit_changes** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ExternalIssueEventQueueItemsBatch**](ExternalIssueEventQueueItemsBatch.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_external_tracker_projects_delete

> external_issues_external_tracker_projects_delete(issue_prefix)
Disconnect external issue tracker project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_prefix** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_external_tracker_projects_get

> Vec<crate::models::ExternalIssueTrackerProjectApi> external_issues_external_tracker_projects_get(application, dollar_fields)
Get all connected external issue tracker projects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ExternalIssueTrackerProjectApi>**](ExternalIssueTrackerProjectApi.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_external_tracker_projects_post

> Vec<crate::models::CreateExternalIssueProjectResult> external_issues_external_tracker_projects_post(external_issues_external_tracker_projects_post_request, dollar_fields)
Connect external issue tracker projects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_issues_external_tracker_projects_post_request** | [**ExternalIssuesExternalTrackerProjectsPostRequest**](ExternalIssuesExternalTrackerProjectsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::CreateExternalIssueProjectResult>**](CreateExternalIssueProjectResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_issue_content_post

> external_issues_issue_content_post(external_issues_issue_content_post_request)
Post external issue data

Provide information about an issue from external issue tracker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_issues_issue_content_post_request** | [**ExternalIssuesIssueContentPostRequest**](ExternalIssuesIssueContentPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_issue_statuses_post

> external_issues_issue_statuses_post(external_issues_issue_statuses_post_request)
Provide all possible statuses for external issues

Provide Space with all possible statuses for external issues for a given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_issues_issue_statuses_post_request** | [**ExternalIssuesIssueStatusesPostRequest**](ExternalIssuesIssueStatusesPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_issues_issue_prefix_issue_id_code_reviews_delete

> external_issues_issues_issue_prefix_issue_id_code_reviews_delete(issue_prefix, issue_id, project, code_review_ids)
Unlink code reviews from external issue

Remove code review links from an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_prefix** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**project** | **String** |  | [required] |
**code_review_ids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_issues_issue_prefix_issue_id_code_reviews_post

> external_issues_issues_issue_prefix_issue_id_code_reviews_post(issue_prefix, issue_id, external_issues_issues_issue_prefix_issue_id_code_reviews_post_request)
Link code reviews to external issue

Add code review links to an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_prefix** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**external_issues_issues_issue_prefix_issue_id_code_reviews_post_request** | [**ExternalIssuesIssuesIssuePrefixIssueIdCodeReviewsPostRequest**](ExternalIssuesIssuesIssuePrefixIssueIdCodeReviewsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_issues_issue_prefix_issue_id_commits_delete

> external_issues_issues_issue_prefix_issue_id_commits_delete(issue_prefix, issue_id, project, repository, commit_ids)
Unlink commits from external issue

Remove commit links from an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_prefix** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**commit_ids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_issues_issue_prefix_issue_id_commits_post

> external_issues_issues_issue_prefix_issue_id_commits_post(issue_prefix, issue_id, external_issues_issues_issue_prefix_issue_id_commits_post_request)
Link commits to external issue

Add commit links to an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_prefix** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**external_issues_issues_issue_prefix_issue_id_commits_post_request** | [**ExternalIssuesIssuesIssuePrefixIssueIdCommitsPostRequest**](ExternalIssuesIssuesIssuePrefixIssueIdCommitsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_issues_mark_issues_as_deleted_post

> external_issues_mark_issues_as_deleted_post(external_issues_mark_issues_as_deleted_post_request)
Mark external issues as deleted

Notify Space about issues that were deleted in external issue tracker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_issues_mark_issues_as_deleted_post_request** | [**ExternalIssuesMarkIssuesAsDeletedPostRequest**](ExternalIssuesMarkIssuesAsDeletedPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_link_patterns_delete

> external_link_patterns_delete(pattern)
Delete external link pattern

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pattern** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_link_patterns_get

> Vec<crate::models::ExternalLinkPattern> external_link_patterns_get(dollar_fields)
Get all external link patterns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ExternalLinkPattern>**](ExternalLinkPattern.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_link_patterns_post

> external_link_patterns_post(external_link_patterns_post_request)
Create external link pattern

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_link_patterns_post_request** | [**ExternalLinkPatternsPostRequest**](ExternalLinkPatternsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## http_api_model_get

> crate::models::HaModel http_api_model_get(dollar_fields)
Get HTTP API model

Get the HTTP API model that describes the available HTTP APIs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::HaModel**](HA_Model.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issues_get

> crate::models::Issue issues_get(issue_id, dollar_fields)
Get issue

Retrieve an issue by its identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Issue**](Issue.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issues_get_by_ids_post

> Vec<crate::models::Issue> issues_get_by_ids_post(issues_get_by_ids_post_request, dollar_fields)
Get issues by identifiers

Retrieve list of issues by identifiers. Issues can belong to different projects. Up to 100 issues can be retrieved within a single request. See also [Get all issues](/extensions/httpApiPlayground?resource=projects_xxx_planning_issues&endpoint=rest_query) (`/projects/{project}/planning/issues`)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issues_get_by_ids_post_request** | [**IssuesGetByIdsPostRequest**](IssuesGetByIdsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Issue>**](Issue.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_channel_subscriptions_get

> Vec<crate::models::SubscriptionDto> notifications_channel_subscriptions_get(channel, dollar_fields)
Get all channel subscriptions

List subscriptions for a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::SubscriptionDto>**](SubscriptionDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_channel_subscriptions_id_delete

> notifications_channel_subscriptions_id_delete(id)
Delete channel subscription

Delete channel subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_channel_subscriptions_id_patch

> crate::models::SubscriptionDto notifications_channel_subscriptions_id_patch(id, dollar_fields, notifications_channel_subscriptions_id_patch_request)
Update channel subscription

Update subscription for a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**notifications_channel_subscriptions_id_patch_request** | Option<[**NotificationsChannelSubscriptionsIdPatchRequest**](NotificationsChannelSubscriptionsIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::SubscriptionDto**](SubscriptionDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_channel_subscriptions_id_request_missing_rights_post

> notifications_channel_subscriptions_id_request_missing_rights_post(id)
Request Missing Rights

Ensures that all permissions required for this subscription are requested in the corresponding permission role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_channel_subscriptions_post

> crate::models::SubscriptionDto notifications_channel_subscriptions_post(notifications_channel_subscriptions_post_request, dollar_fields)
Create channel subscription

Add subscription for a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notifications_channel_subscriptions_post_request** | [**NotificationsChannelSubscriptionsPostRequest**](NotificationsChannelSubscriptionsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::SubscriptionDto**](SubscriptionDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_get

> Vec<crate::models::EventSubjectInfoDto> notifications_get(dollar_fields)
Get all notifications

List all subscription subjects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::EventSubjectInfoDto>**](EventSubjectInfoDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_personal_custom_subscriptions_get

> Vec<crate::models::SubscriptionDto> notifications_personal_custom_subscriptions_get(profile, dollar_fields)
Get all personal custom subscriptions

List personal custom subscriptions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::SubscriptionDto>**](SubscriptionDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_personal_custom_subscriptions_id_delete

> notifications_personal_custom_subscriptions_id_delete(id)
Delete personal custom subscription

Delete personal custom subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_personal_custom_subscriptions_id_patch

> crate::models::SubscriptionDto notifications_personal_custom_subscriptions_id_patch(id, dollar_fields, notifications_personal_custom_subscriptions_id_patch_request)
Update personal custom subscription

Create personal custom subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**notifications_personal_custom_subscriptions_id_patch_request** | Option<[**NotificationsPersonalCustomSubscriptionsIdPatchRequest**](NotificationsPersonalCustomSubscriptionsIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::SubscriptionDto**](SubscriptionDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_personal_custom_subscriptions_post

> crate::models::SubscriptionDto notifications_personal_custom_subscriptions_post(notifications_personal_custom_subscriptions_post_request, dollar_fields)
Create personal custom subscription

Create personal custom subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notifications_personal_custom_subscriptions_post_request** | [**NotificationsPersonalCustomSubscriptionsPostRequest**](NotificationsPersonalCustomSubscriptionsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::SubscriptionDto**](SubscriptionDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_personal_subscriptions_all_personal_subscription_targets_get

> Vec<crate::models::PersonalSubscriptionTarget> notifications_personal_subscriptions_all_personal_subscription_targets_get(dollar_fields)
All personal subscription targets

List all personal subscription targets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PersonalSubscriptionTarget>**](PersonalSubscriptionTarget.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_personal_subscriptions_personal_subscription_settings_get

> crate::models::PersonalSubscriptionSettings notifications_personal_subscriptions_personal_subscription_settings_get(profile, feed, dollar_fields)
Get personal subscription settings

Get personal subscription settings for a member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**feed** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PersonalSubscriptionSettings**](PersonalSubscriptionSettings.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_personal_subscriptions_update_personal_subscription_subject_post

> notifications_personal_subscriptions_update_personal_subscription_subject_post(notifications_personal_subscriptions_update_personal_subscription_subject_post_request)
Update personal subscription subject

Update personal subscription settings for a member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notifications_personal_subscriptions_update_personal_subscription_subject_post_request** | [**NotificationsPersonalSubscriptionsUpdatePersonalSubscriptionSubjectPostRequest**](NotificationsPersonalSubscriptionsUpdatePersonalSubscriptionSubjectPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_personal_subscriptions_update_personal_subscription_target_post

> notifications_personal_subscriptions_update_personal_subscription_target_post(notifications_personal_subscriptions_update_personal_subscription_target_post_request)
Update personal subscription target

Update personal subscription settings for a member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notifications_personal_subscriptions_update_personal_subscription_target_post_request** | [**NotificationsPersonalSubscriptionsUpdatePersonalSubscriptionTargetPostRequest**](NotificationsPersonalSubscriptionsUpdatePersonalSubscriptionTargetPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_private_feeds_get

> Vec<crate::models::PrivateFeed> notifications_private_feeds_get(profile, dollar_fields)
Get all private feeds

List personal feeds for a member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PrivateFeed>**](PrivateFeed.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_private_feeds_id_delete

> notifications_private_feeds_id_delete(id)
Delete private feed

Delete personal feed for member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_private_feeds_id_patch

> crate::models::PrivateFeed notifications_private_feeds_id_patch(id, dollar_fields, notifications_private_feeds_id_patch_request)
Update private feed

Update personal feed for a member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**notifications_private_feeds_id_patch_request** | Option<[**NotificationsPrivateFeedsIdPatchRequest**](NotificationsPrivateFeedsIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::PrivateFeed**](PrivateFeed.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_private_feeds_post

> crate::models::PrivateFeed notifications_private_feeds_post(notifications_private_feeds_post_request, dollar_fields)
Create private feed

Create personal feed for member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notifications_private_feeds_post_request** | [**NotificationsPrivateFeedsPostRequest**](NotificationsPrivateFeedsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PrivateFeed**](PrivateFeed.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_domains_check_get

> crate::models::CDomainStatus organization_domains_check_get(domain)
Check domain availability

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |

### Return type

[**crate::models::CDomainStatus**](CDomainStatus.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_domains_get

> Vec<crate::models::OrgDomainDto> organization_domains_get(dollar_fields)
Get all domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::OrgDomainDto>**](OrgDomainDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_domains_patch

> organization_domains_patch(organization_domains_patch_request)
Update organization domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_domains_patch_request** | [**OrganizationDomainsPatchRequest**](OrganizationDomainsPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_get

> crate::models::OrganizationRecord organization_get(dollar_fields)
Get organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::OrganizationRecord**](OrganizationRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_jet_sales_license_activation_url_get

> String organization_jet_sales_license_activation_url_get()
Get license activation url

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_jet_sales_url_get

> String organization_jet_sales_url_get()
Get JetSales URL

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_patch

> organization_patch(organization_patch_request)
Update organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_patch_request** | [**OrganizationPatchRequest**](OrganizationPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_create_post

> crate::models::RoleDto permission_roles_create_post(permission_roles_create_post_request, dollar_fields)
Create role

Create new custom permission role in specified permission context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission_roles_create_post_request** | [**PermissionRolesCreatePostRequest**](PermissionRolesCreatePostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::RoleDto**](RoleDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_get_post

> Vec<crate::models::RoleDto> permission_roles_get_post(applications_application_ui_extensions_disable_for_everybody_patch_request, dollar_fields)
Get roles

List all permission roles in permission context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applications_application_ui_extensions_disable_for_everybody_patch_request** | [**ApplicationsApplicationUiExtensionsDisableForEverybodyPatchRequest**](ApplicationsApplicationUiExtensionsDisableForEverybodyPatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::RoleDto>**](RoleDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id2_fa_requirement_get

> crate::models::TwoFactorAuthenticationRequirement permission_roles_role_id2_fa_requirement_get(role_id, dollar_fields)
Get 2FA requirement

Get 2FA requirement for permission role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TwoFactorAuthenticationRequirement**](TwoFactorAuthenticationRequirement.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id2_fa_requirement_patch

> permission_roles_role_id2_fa_requirement_patch(role_id, permission_roles_role_id2_fa_requirement_patch_request)
Set 2FA requirement

Set 2FA requirement for permission role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**permission_roles_role_id2_fa_requirement_patch_request** | Option<[**PermissionRolesRoleId2FaRequirementPatchRequest**](PermissionRolesRoleId2FaRequirementPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_delete

> permission_roles_role_id_delete(role_id)
Delete role

Delete custom permission role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_patch

> permission_roles_role_id_patch(role_id, chats_channels_is_name_free_post_request)
Update role

Update custom permission role name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**chats_channels_is_name_free_post_request** | [**ChatsChannelsIsNameFreePostRequest**](ChatsChannelsIsNameFreePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_permissions_delete

> permission_roles_role_id_permissions_delete(role_id, right_codes)
Revoke role permissions

Revoke permissions from the specified role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**right_codes** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_permissions_get

> Vec<crate::models::RightDto> permission_roles_role_id_permissions_get(role_id, dollar_fields)
Get role permissions

Get role permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::RightDto>**](RightDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_permissions_post

> permission_roles_role_id_permissions_post(role_id, permission_roles_role_id_permissions_post_request)
Grant role permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**permission_roles_role_id_permissions_post_request** | [**PermissionRolesRoleIdPermissionsPostRequest**](PermissionRolesRoleIdPermissionsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_profiles_get

> Vec<crate::models::TdMemberProfile> permission_roles_role_id_profiles_get(role_id, dollar_fields)
Get role members

Get list of profiles with the specified role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_profiles_profile_delete

> permission_roles_role_id_profiles_profile_delete(role_id, profile)
Remove role member

Remove permission role from the profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**profile** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_profiles_profile_post

> permission_roles_role_id_profiles_profile_post(role_id, profile)
Add role member

Assign permission role to the profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**profile** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_reset_role_permissions_to_default_post

> permission_roles_role_id_reset_role_permissions_to_default_post(role_id)
Reset role permissions to default

Reset permissions for the role to the standard ones. Only applicable to roles with PermissionRoleType = PREDEFINED, not applicable to custom roles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_teams_get

> Vec<crate::models::TdTeam> permission_roles_role_id_teams_get(role_id, dollar_fields)
Get role teams

Get list of teams with the specified role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdTeam>**](TD_Team.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_teams_team_delete

> permission_roles_role_id_teams_team_delete(role_id, team)
Remove role team

Remove permission role from the team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**team** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_roles_role_id_teams_team_post

> permission_roles_role_id_teams_team_post(role_id, team)
Add role team

Assign permission role to the team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**team** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permissions_check_permission_post

> bool permissions_check_permission_post(permissions_check_permission_post_request)
Check permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permissions_check_permission_post_request** | [**PermissionsCheckPermissionPostRequest**](PermissionsCheckPermissionPostRequest.md) |  | [required] |

### Return type

**bool**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permissions_get

> crate::models::DtoRightsWithHierarchy permissions_get(dollar_fields)
Get all permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DtoRightsWithHierarchy**](DTO_RightsWithHierarchy.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_deployment_targets_audit_log_get

> crate::models::ProjectsAutomationDeploymentTargetsAuditLogGet200Response projects_automation_deployment_targets_audit_log_get(target_identifier, dollar_skip, dollar_top, dollar_fields)
Get audit messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_identifier** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsAutomationDeploymentTargetsAuditLogGet200Response**](_projects_automation_deployment_targets_audit_log_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_deployment_targets_full_number_id_get

> crate::models::DeployTargetRecord projects_automation_deployment_targets_full_number_id_get(full_number_id, dollar_fields)
Get deployment target by full number id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_number_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DeployTargetRecord**](DeployTargetRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_deployment_targets_get

> crate::models::ProjectsAutomationDeploymentTargetsSearchPost200Response projects_automation_deployment_targets_get(project, search, custom_filters, sort_by, sort_order, dollar_skip, dollar_top, dollar_fields)
Get all deployment targets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |
**custom_filters** | Option<[**Vec<String>**](String.md)> |  |  |
**sort_by** | Option<**String**> |  |  |
**sort_order** | Option<[**ColumnSortOrder**](.md)> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsAutomationDeploymentTargetsSearchPost200Response**](_projects_automation_deployment_targets_search_post_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_deployment_targets_search_post

> crate::models::ProjectsAutomationDeploymentTargetsSearchPost200Response projects_automation_deployment_targets_search_post(projects_automation_deployment_targets_search_post_request, dollar_fields)
search

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_automation_deployment_targets_search_post_request** | [**ProjectsAutomationDeploymentTargetsSearchPostRequest**](ProjectsAutomationDeploymentTargetsSearchPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsAutomationDeploymentTargetsSearchPost200Response**](_projects_automation_deployment_targets_search_post_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_dsl_evaluations_config_get

> crate::models::DslEvaluationConfig projects_automation_dsl_evaluations_config_get(dollar_fields)
Get DSL evaluation configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DslEvaluationConfig**](DslEvaluationConfig.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_graph_executions_id_get

> crate::models::JobExecutionDto projects_automation_graph_executions_id_get(id, dollar_fields)
Get graph execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::JobExecutionDto**](JobExecutionDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_graph_executions_id_stop_post

> projects_automation_graph_executions_id_stop_post(id)
Stop execution

Stop execution by ExecutionId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_job_executions_current_get

> crate::models::JobExecutionDto projects_automation_job_executions_current_get(dollar_fields)
Get current

Returns the job execution associated to the currently authenticated principal. This endpoint can only be used with the credentials provided to an Automation job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::JobExecutionDto**](JobExecutionDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_jobs_job_id_get

> crate::models::JobDto projects_automation_jobs_job_id_get(job_id, project, dollar_fields)
Get job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::JobDto**](JobDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_step_executions_step_exec_id_parameters_key_get

> String projects_automation_step_executions_step_exec_id_parameters_key_get(step_exec_id, key)
Get parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**step_exec_id** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_step_executions_step_exec_id_parameters_key_patch

> projects_automation_step_executions_step_exec_id_parameters_key_patch(step_exec_id, key, applications_parameters_key_patch_request)
Update parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**step_exec_id** | **String** |  | [required] |
**key** | **String** |  | [required] |
**applications_parameters_key_patch_request** | [**ApplicationsParametersKeyPatchRequest**](ApplicationsParametersKeyPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_step_executions_used_parameters_param_parameter_id_get

> Vec<crate::models::ParameterLastUsageDto> projects_automation_step_executions_used_parameters_param_parameter_id_get(parameter_id, dollar_fields)
Get param

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parameter_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ParameterLastUsageDto>**](ParameterLastUsageDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_step_executions_used_parameters_secret_secret_id_get

> Vec<crate::models::ParameterLastUsageDto> projects_automation_step_executions_used_parameters_secret_secret_id_get(secret_id, dollar_fields)
Get secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ParameterLastUsageDto>**](ParameterLastUsageDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_automation_subscriptions_legacy_channels_delete

> projects_automation_subscriptions_legacy_channels_delete(project, job_id, unsubscribed_only)
Delete legacy channels

Delete the legacy subscription channels matching the given filters (applied as AND). If no filter is provided, all subscription channels corresponding to unsubscribed jobs for the logged in user are deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | Option<**String**> |  |  |
**job_id** | Option<**String**> |  |  |
**unsubscribed_only** | Option<**bool**> |  |  |[default to true]

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_collaboratorprofile_get

> crate::models::ProjectsGet200Response projects_collaboratorprofile_get(profile, dollar_skip, dollar_top, dollar_fields)
Get all projects by collaborator

Get all projects in which given user is a collaborator

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsGet200Response**](_projects_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_get

> crate::models::ProjectsGet200Response projects_get(dollar_skip, dollar_top, term, tag, starred, dollar_fields)
Get all projects

Get/search all projects. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**term** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**starred** | Option<**bool**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsGet200Response**](_projects_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_membermember_get

> crate::models::ProjectsGet200Response projects_membermember_get(member, dollar_skip, dollar_top, dollar_fields)
Get all projects by member

Get all projects for a member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsGet200Response**](_projects_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_params_default_bundle_get

> crate::models::ProjectsParamsDefaultBundleGet200Response projects_params_default_bundle_get(project, dollar_skip, dollar_top, dollar_fields)
Get all default bundle

List project parameters in a parameter bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsParamsDefaultBundleGet200Response**](_projects_params_default_bundle_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_params_default_bundle_post

> String projects_params_default_bundle_post(projects_params_default_bundle_post_request)
Create default bundle

Create a new project parameter in the default parameter bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_params_default_bundle_post_request** | [**ProjectsParamsDefaultBundlePostRequest**](ProjectsParamsDefaultBundlePostRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_params_id_delete

> projects_params_id_delete(id)
Delete param

Delete a project parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_params_id_patch

> projects_params_id_patch(id, projects_params_id_patch_request)
Update param

Update a project parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**projects_params_id_patch_request** | [**ProjectsParamsIdPatchRequest**](ProjectsParamsIdPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_params_in_default_bundle_get

> crate::models::ProjectsParamsDefaultBundleGet200Response projects_params_in_default_bundle_get(project_id, dollar_skip, dollar_top, dollar_fields)
Get all in default bundle

List project parameters in a parameter bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsParamsDefaultBundleGet200Response**](_projects_params_default_bundle_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_params_in_default_bundle_post

> String projects_params_in_default_bundle_post(projects_params_in_default_bundle_post_request)
Create in default bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_params_in_default_bundle_post_request** | [**ProjectsParamsInDefaultBundlePostRequest**](ProjectsParamsInDefaultBundlePostRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_board_delete

> projects_planning_boards_board_delete(board)
Delete board

Delete an existing board. This operation can be performed by board owners or other members who are granted permission to manage boards in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_board_get

> crate::models::BoardRecord projects_planning_boards_board_get(board, dollar_fields)
Get board

Get a board by identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BoardRecord**](BoardRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_board_issues_get

> crate::models::ProjectsPlanningBoardsSprintsSprintIssuesGet200Response projects_planning_boards_board_issues_get(board, dollar_skip, dollar_top, dollar_fields)
Get all issues on board

Fetch issues from the board across all its non-archived sprints

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsPlanningBoardsSprintsSprintIssuesGet200Response**](_projects_planning_boards_sprints__sprint__issues_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_board_issues_issue_delete

> projects_planning_boards_board_issues_issue_delete(issue, board)
Remove issue from board

Remove an existing issue in a project from a board or all of its sprints

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue** | **String** |  | [required] |
**board** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_board_issues_issue_post

> projects_planning_boards_board_issues_issue_post(issue, board)
Add issue to board

Add an existing issue in a project to a board or its current sprint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue** | **String** |  | [required] |
**board** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_board_patch

> projects_planning_boards_board_patch(board, projects_planning_boards_board_patch_request)
Update board

Update an existing board. This operation can be performed by board owners or other members who are granted permission to manage boards in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board** | **String** |  | [required] |
**projects_planning_boards_board_patch_request** | Option<[**ProjectsPlanningBoardsBoardPatchRequest**](ProjectsPlanningBoardsBoardPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_sprints_post

> crate::models::SprintRecord projects_planning_boards_sprints_post(projects_planning_boards_sprints_post_request, dollar_fields)
Create sprint

Create a new sprint in a board. This operation can be performed by board owners or other members who are granted permission to manage boards in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_planning_boards_sprints_post_request** | [**ProjectsPlanningBoardsSprintsPostRequest**](ProjectsPlanningBoardsSprintsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::SprintRecord**](SprintRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_sprints_sprint_archive_delete

> projects_planning_boards_sprints_sprint_archive_delete(sprint)
Archive sprint

Archive closed or planned sprint. This operation can be performed by board owners or other members who are granted permission to manage boards in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sprint** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_sprints_sprint_issues_get

> crate::models::ProjectsPlanningBoardsSprintsSprintIssuesGet200Response projects_planning_boards_sprints_sprint_issues_get(sprint, dollar_skip, dollar_top, unresolved_only, dollar_fields)
Get all issues in sprint

Fetch issues from an existing non-archived sprint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sprint** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**unresolved_only** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsPlanningBoardsSprintsSprintIssuesGet200Response**](_projects_planning_boards_sprints__sprint__issues_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_sprints_sprint_issues_issue_delete

> projects_planning_boards_sprints_sprint_issues_issue_delete(issue, sprint)
Remove issue from sprint

Remove an existing issue in a project from a sprint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue** | **String** |  | [required] |
**sprint** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_sprints_sprint_issues_issue_post

> projects_planning_boards_sprints_sprint_issues_issue_post(issue, sprint)
Add issue to sprint

Add an existing issue in a project to a sprint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue** | **String** |  | [required] |
**sprint** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_sprints_sprint_launch_post

> projects_planning_boards_sprints_sprint_launch_post(sprint, projects_planning_boards_sprints_sprint_launch_post_request)
Launch planned sprint

Launch a planned sprint. This operation can be performed by board owners or other members who are granted permission to manage boards in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sprint** | **String** |  | [required] |
**projects_planning_boards_sprints_sprint_launch_post_request** | [**ProjectsPlanningBoardsSprintsSprintLaunchPostRequest**](ProjectsPlanningBoardsSprintsSprintLaunchPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_planning_boards_sprints_sprint_patch

> projects_planning_boards_sprints_sprint_patch(sprint, projects_planning_boards_sprints_sprint_patch_request)
Update sprint

Update an existing sprint in a board. This operation can be performed by board owners or other members who are granted permission to manage boards in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sprint** | **String** |  | [required] |
**projects_planning_boards_sprints_sprint_patch_request** | Option<[**ProjectsPlanningBoardsSprintsSprintPatchRequest**](ProjectsPlanningBoardsSprintsSprintPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_post

> crate::models::PrProject projects_post(projects_post_request, dollar_fields)
Create project

Create a new project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_post_request** | [**ProjectsPostRequest**](ProjectsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PrProject**](PR_Project.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_private_projects_get

> Vec<crate::models::PrPrivateProject> projects_private_projects_get(dollar_fields)
Get all private projects

List private projects in the current organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PrPrivateProject>**](PR_PrivateProject.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_private_projects_project_request_access_post

> projects_private_projects_project_request_access_post(project)
Request access to project

Request access to a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_admins_get

> Vec<crate::models::TdMemberProfile> projects_project_access_admins_get(project, dollar_fields)
Get all admins

Returns the list of all project administrators

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_admins_profiles_post

> projects_project_access_admins_profiles_post(project, chats_channels_channel_administrator_patch_request)
Add administrator

Add a member as administrator to a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**chats_channels_channel_administrator_patch_request** | [**ChatsChannelsChannelAdministratorPatchRequest**](ChatsChannelsChannelAdministratorPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_admins_profiles_profile_delete

> projects_project_access_admins_profiles_profile_delete(project, profile)
Remove administrator

Remove a member as administrator from a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**profile** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_admins_teams_post

> projects_project_access_admins_teams_post(project, projects_project_access_admins_teams_post_request)
Add Administrators team

Add a team as administrators to a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_access_admins_teams_post_request** | [**ProjectsProjectAccessAdminsTeamsPostRequest**](ProjectsProjectAccessAdminsTeamsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_admins_teams_team_id_delete

> projects_project_access_admins_teams_team_id_delete(project, team_id)
Remove Administrators team

Remove a team as administrators from a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**team_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_collaborators_get

> Vec<crate::models::TdMemberProfile> projects_project_access_collaborators_get(project, dollar_fields)
Get all collaborators' profiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_collaborators_profiles_delete

> projects_project_access_collaborators_profiles_delete(project, profile)
Remove a collaborator

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**profile** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_collaborators_profiles_get

> Vec<crate::models::TdMemberProfile> projects_project_access_collaborators_profiles_get(project, dollar_fields)
Get all individual collaborators

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_collaborators_profiles_post

> projects_project_access_collaborators_profiles_post(project, chats_channels_channel_administrator_patch_request)
Add a collaborator

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**chats_channels_channel_administrator_patch_request** | [**ChatsChannelsChannelAdministratorPatchRequest**](ChatsChannelsChannelAdministratorPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_collaborators_teams_delete

> projects_project_access_collaborators_teams_delete(project, team_id)
Remove a collaborators' team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**team_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_collaborators_teams_get

> Vec<crate::models::TdTeam> projects_project_access_collaborators_teams_get(project, dollar_fields)
Get all collaborators' teams

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdTeam>**](TD_Team.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_collaborators_teams_post

> projects_project_access_collaborators_teams_post(project, projects_project_access_admins_teams_post_request)
Add a collaborators' team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_access_admins_teams_post_request** | [**ProjectsProjectAccessAdminsTeamsPostRequest**](ProjectsProjectAccessAdminsTeamsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_member_profiles_get

> crate::models::ChatsChannelsChannelSubscribersUsersGet200Response projects_project_access_member_profiles_get(project, dollar_skip, dollar_top, query, including_admins, dollar_fields)
Get all member profiles

Get project members for a given project key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**query** | Option<**String**> |  |  |[default to ]
**including_admins** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelSubscribersUsersGet200Response**](_chats_channels__channel__subscribers_users_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_members_profiles_post

> projects_project_access_members_profiles_post(project, chats_channels_channel_administrator_patch_request)
Add member

Add a member to a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**chats_channels_channel_administrator_patch_request** | [**ChatsChannelsChannelAdministratorPatchRequest**](ChatsChannelsChannelAdministratorPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_members_profiles_profile_delete

> projects_project_access_members_profiles_profile_delete(project, profile)
Remove member

Remove a member from a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**profile** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_members_teams_team_id_delete

> projects_project_access_members_teams_team_id_delete(project, team_id)
Remove team

Remove a team from a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**team_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_access_viewers_get

> crate::models::ChatsChannelsChannelSubscribersUsersGet200Response projects_project_access_viewers_get(project, term, dollar_skip, dollar_top, me_on_top, dollar_fields)
Organization profiles that can view the project

Get organization members who can view a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**term** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**me_on_top** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelSubscribersUsersGet200Response**](_chats_channels__channel__subscribers_users_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployment_targets_favorites_get

> Vec<crate::models::DeployTargetRecord> projects_project_automation_deployment_targets_favorites_get(project, sort_by, sort_order, dollar_fields)
List Favorites

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**sort_by** | Option<**String**> |  |  |
**sort_order** | Option<[**ColumnSortOrder**](.md)> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::DeployTargetRecord>**](DeployTargetRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployment_targets_get

> crate::models::ProjectsAutomationDeploymentTargetsSearchPost200Response projects_project_automation_deployment_targets_get(project, search, custom_filters, show_archived, sort_by, sort_order, dollar_skip, dollar_top, dollar_fields)
Get all deployment targets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**search** | Option<**String**> |  |  |
**custom_filters** | Option<[**Vec<String>**](String.md)> |  |  |
**show_archived** | Option<**bool**> |  |  |
**sort_by** | Option<**String**> |  |  |
**sort_order** | Option<[**ColumnSortOrder**](.md)> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsAutomationDeploymentTargetsSearchPost200Response**](_projects_automation_deployment_targets_search_post_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployment_targets_post

> crate::models::DeployTargetRecord projects_project_automation_deployment_targets_post(project, projects_project_automation_deployment_targets_post_request, dollar_fields)
Create deployment target

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_automation_deployment_targets_post_request** | [**ProjectsProjectAutomationDeploymentTargetsPostRequest**](ProjectsProjectAutomationDeploymentTargetsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DeployTargetRecord**](DeployTargetRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployment_targets_target_delete

> projects_project_automation_deployment_targets_target_delete(project, target)
Delete deployment target

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**target** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployment_targets_target_get

> crate::models::DeployTargetRecord projects_project_automation_deployment_targets_target_get(project, target, dollar_fields)
Get deployment target

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**target** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DeployTargetRecord**](DeployTargetRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployment_targets_target_patch

> projects_project_automation_deployment_targets_target_patch(project, target, projects_project_automation_deployment_targets_target_patch_request)
Update deployment target

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**target** | **String** |  | [required] |
**projects_project_automation_deployment_targets_target_patch_request** | Option<[**ProjectsProjectAutomationDeploymentTargetsTargetPatchRequest**](ProjectsProjectAutomationDeploymentTargetsTargetPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployments_fail_post

> crate::models::DeploymentRecord projects_project_automation_deployments_fail_post(project, projects_project_automation_deployments_fail_post_request, dollar_fields)
Fail deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_automation_deployments_fail_post_request** | [**ProjectsProjectAutomationDeploymentsFailPostRequest**](ProjectsProjectAutomationDeploymentsFailPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DeploymentRecord**](DeploymentRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployments_finish_post

> crate::models::DeploymentRecord projects_project_automation_deployments_finish_post(project, projects_project_automation_deployments_finish_post_request, dollar_fields)
Finish deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_automation_deployments_finish_post_request** | [**ProjectsProjectAutomationDeploymentsFinishPostRequest**](ProjectsProjectAutomationDeploymentsFinishPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DeploymentRecord**](DeploymentRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployments_get

> crate::models::ProjectsProjectAutomationDeploymentsGet200Response projects_project_automation_deployments_get(project, target_identifier, job_execution_id, dollar_skip, dollar_top, dollar_fields)
Get all deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**target_identifier** | Option<**String**> |  |  |
**job_execution_id** | Option<**String**> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectAutomationDeploymentsGet200Response**](_projects__project__automation_deployments_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployments_patch

> crate::models::DeploymentRecord projects_project_automation_deployments_patch(project, projects_project_automation_deployments_patch_request, dollar_fields)
Update deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_automation_deployments_patch_request** | [**ProjectsProjectAutomationDeploymentsPatchRequest**](ProjectsProjectAutomationDeploymentsPatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DeploymentRecord**](DeploymentRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployments_schedule_post

> crate::models::DeploymentRecord projects_project_automation_deployments_schedule_post(project, projects_project_automation_deployments_schedule_post_request, dollar_fields)
Schedule deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_automation_deployments_schedule_post_request** | [**ProjectsProjectAutomationDeploymentsSchedulePostRequest**](ProjectsProjectAutomationDeploymentsSchedulePostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DeploymentRecord**](DeploymentRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployments_start_post

> crate::models::DeploymentRecord projects_project_automation_deployments_start_post(project, projects_project_automation_deployments_start_post_request, dollar_fields)
Start deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_automation_deployments_start_post_request** | [**ProjectsProjectAutomationDeploymentsStartPostRequest**](ProjectsProjectAutomationDeploymentsStartPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DeploymentRecord**](DeploymentRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployments_target_identifier_deployment_identifier_delete

> projects_project_automation_deployments_target_identifier_deployment_identifier_delete(project, target_identifier, deployment_identifier)
Delete deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**target_identifier** | **String** |  | [required] |
**deployment_identifier** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_deployments_target_identifier_deployment_identifier_get

> crate::models::DeploymentRecord projects_project_automation_deployments_target_identifier_deployment_identifier_get(project, target_identifier, deployment_identifier, dollar_fields)
Get deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**target_identifier** | **String** |  | [required] |
**deployment_identifier** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DeploymentRecord**](DeploymentRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_graph_executions_get

> crate::models::ProjectsProjectAutomationGraphExecutionsGet200Response projects_project_automation_graph_executions_get(project, job_id, branch_filter, status_filter, job_trigger_filter, dollar_skip, dollar_top, dollar_fields)
Get all graph executions

Search executions. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**job_id** | **String** |  | [required] |
**branch_filter** | Option<**String**> |  |  |
**status_filter** | Option<[**ExecutionStatus**](.md)> |  |  |
**job_trigger_filter** | Option<[**JobTriggerType**](.md)> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectAutomationGraphExecutionsGet200Response**](_projects__project__automation_graph_executions_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_jobs_get

> crate::models::ProjectsProjectAutomationJobsGet200Response projects_project_automation_jobs_get(project, repo_filter, branch_filter, trigger, dollar_skip, dollar_top, dollar_fields)
Get all jobs

List jobs. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repo_filter** | **String** |  | [required] |
**branch_filter** | **String** |  | [required] |
**trigger** | Option<[**JobTriggerType**](.md)> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectAutomationJobsGet200Response**](_projects__project__automation_jobs_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_automation_jobs_job_id_start_post

> crate::models::LaunchResult projects_project_automation_jobs_job_id_start_post(project, job_id, projects_project_automation_jobs_job_id_start_post_request, dollar_fields)
Start job

Start job. Returns ExecutionId, see projects/automation/graph-executions/{id}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**job_id** | **String** |  | [required] |
**projects_project_automation_jobs_job_id_start_post_request** | [**ProjectsProjectAutomationJobsJobIdStartPostRequest**](ProjectsProjectAutomationJobsJobIdStartPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::LaunchResult**](LaunchResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_code_discussions_discussion_id_accept_suggested_edit_post

> projects_project_code_reviews_code_discussions_discussion_id_accept_suggested_edit_post(project, discussion_id, projects_project_code_reviews_code_discussions_discussion_id_accept_suggested_edit_post_request)
Accept suggested edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**discussion_id** | **String** |  | [required] |
**projects_project_code_reviews_code_discussions_discussion_id_accept_suggested_edit_post_request** | [**ProjectsProjectCodeReviewsCodeDiscussionsDiscussionIdAcceptSuggestedEditPostRequest**](ProjectsProjectCodeReviewsCodeDiscussionsDiscussionIdAcceptSuggestedEditPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_code_discussions_discussion_id_reject_suggested_edit_post

> projects_project_code_reviews_code_discussions_discussion_id_reject_suggested_edit_post(project, discussion_id)
Reject suggested edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**discussion_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_code_discussions_discussion_id_reopen_suggested_edit_post

> projects_project_code_reviews_code_discussions_discussion_id_reopen_suggested_edit_post(project, discussion_id)
Reopen suggested edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**discussion_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_code_discussions_discussion_id_suggested_edit_patch

> projects_project_code_reviews_code_discussions_discussion_id_suggested_edit_patch(project, discussion_id, projects_project_code_reviews_code_discussions_discussion_id_suggested_edit_patch_request)
Alter suggested edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**discussion_id** | **String** |  | [required] |
**projects_project_code_reviews_code_discussions_discussion_id_suggested_edit_patch_request** | [**ProjectsProjectCodeReviewsCodeDiscussionsDiscussionIdSuggestedEditPatchRequest**](ProjectsProjectCodeReviewsCodeDiscussionsDiscussionIdSuggestedEditPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_code_discussions_post

> crate::models::CodeDiscussionRecord projects_project_code_reviews_code_discussions_post(project, projects_project_code_reviews_code_discussions_post_request, dollar_fields)
Create code discussion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_code_reviews_code_discussions_post_request** | [**ProjectsProjectCodeReviewsCodeDiscussionsPostRequest**](ProjectsProjectCodeReviewsCodeDiscussionsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CodeDiscussionRecord**](CodeDiscussionRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_commit_set_review_post

> crate::models::CommitSetReviewRecord projects_project_code_reviews_commit_set_review_post(project, projects_project_code_reviews_commit_set_review_post_request, dollar_fields)
Create review based on commit set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_code_reviews_commit_set_review_post_request** | [**ProjectsProjectCodeReviewsCommitSetReviewPostRequest**](ProjectsProjectCodeReviewsCommitSetReviewPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CommitSetReviewRecord**](CommitSetReviewRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_get

> crate::models::ProjectsProjectCodeReviewsGet200Response projects_project_code_reviews_get(project, dollar_skip, dollar_top, state, text, author, from, to, sort, reviewer, r#type, repository, dollar_fields)
Get all code reviews

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**state** | Option<[**CodeReviewStateFilter**](.md)> |  |  |
**text** | Option<**String**> |  |  |
**author** | Option<**String**> |  |  |
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |
**sort** | Option<[**ReviewSorting**](.md)> |  |  |
**reviewer** | Option<**String**> |  |  |
**r#type** | Option<[**ReviewType**](.md)> |  |  |
**repository** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectCodeReviewsGet200Response**](_projects__project__code_reviews_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_merge_requests_post

> crate::models::MergeRequestRecord projects_project_code_reviews_merge_requests_post(project, projects_project_code_reviews_merge_requests_post_request, dollar_fields)
Create merge request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_code_reviews_merge_requests_post_request** | [**ProjectsProjectCodeReviewsMergeRequestsPostRequest**](ProjectsProjectCodeReviewsMergeRequestsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::MergeRequestRecord**](MergeRequestRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_description_patch

> projects_project_code_reviews_review_id_description_patch(project, review_id, chats_channels_channel_description_patch_request)
Edit review description

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**chats_channels_channel_description_patch_request** | [**ChatsChannelsChannelDescriptionPatchRequest**](ChatsChannelsChannelDescriptionPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_details_get

> crate::models::CodeReviewDetailedInfo projects_project_code_reviews_review_id_details_get(project, review_id, dollar_fields)
Get review details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CodeReviewDetailedInfo**](CodeReviewDetailedInfo.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_files_get

> crate::models::ProjectsProjectCodeReviewsReviewIdFilesGet200Response projects_project_code_reviews_review_id_files_get(project, review_id, dollar_skip, dollar_top, dollar_fields)
Get the modified files in code review

List files changed in commits under code review

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectCodeReviewsReviewIdFilesGet200Response**](_projects__project__code_reviews__reviewId__files_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_get

> crate::models::CodeReviewRecord projects_project_code_reviews_review_id_get(project, review_id, dollar_fields)
Get code review

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CodeReviewRecord**](CodeReviewRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_make_read_only_patch

> projects_project_code_reviews_review_id_make_read_only_patch(project, review_id)
Make review read-only

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_merge_files_get

> crate::models::ProjectsProjectCodeReviewsReviewIdMergeFilesGet200Response projects_project_code_reviews_review_id_merge_files_get(project, review_id, dollar_skip, dollar_top, dollar_fields)
Get the Merge Request files

List files in merge request which will be merged into target branch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectCodeReviewsReviewIdMergeFilesGet200Response**](_projects__project__code_reviews__reviewId__merge_files_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_merge_put

> crate::models::GitMergeResultHttp projects_project_code_reviews_review_id_merge_put(project, review_id, projects_project_code_reviews_review_id_merge_put_request, dollar_fields)
Merge a merge request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**projects_project_code_reviews_review_id_merge_put_request** | [**ProjectsProjectCodeReviewsReviewIdMergePutRequest**](ProjectsProjectCodeReviewsReviewIdMergePutRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GitMergeResultHttp**](GitMergeResultHttp.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_participants_user_delete

> projects_project_code_reviews_review_id_participants_user_delete(project, review_id, user, role)
Remove review participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**user** | **String** |  | [required] |
**role** | [**CodeReviewParticipantRole**](.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_participants_user_post

> projects_project_code_reviews_review_id_participants_user_post(project, review_id, user, projects_project_code_reviews_review_id_participants_user_post_request)
Add review participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**user** | **String** |  | [required] |
**projects_project_code_reviews_review_id_participants_user_post_request** | [**ProjectsProjectCodeReviewsReviewIdParticipantsUserPostRequest**](ProjectsProjectCodeReviewsReviewIdParticipantsUserPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_rebase_put

> crate::models::GitRebaseResultHttp projects_project_code_reviews_review_id_rebase_put(project, review_id, projects_project_code_reviews_review_id_rebase_put_request, dollar_fields)
Rebase a merge request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**projects_project_code_reviews_review_id_rebase_put_request** | [**ProjectsProjectCodeReviewsReviewIdRebasePutRequest**](ProjectsProjectCodeReviewsReviewIdRebasePutRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GitRebaseResultHttp**](GitRebaseResultHttp.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_revisions_delete

> projects_project_code_reviews_review_id_revisions_delete(project, review_id, revisions)
Remove revisions from review

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**revisions** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_revisions_post

> projects_project_code_reviews_review_id_revisions_post(project, review_id, projects_project_code_reviews_review_id_revisions_post_request)
Add revisions to review

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**projects_project_code_reviews_review_id_revisions_post_request** | [**ProjectsProjectCodeReviewsReviewIdRevisionsPostRequest**](ProjectsProjectCodeReviewsReviewIdRevisionsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_state_patch

> projects_project_code_reviews_review_id_state_patch(project, review_id, projects_project_code_reviews_review_id_state_patch_request)
Edit review state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**projects_project_code_reviews_review_id_state_patch_request** | [**ProjectsProjectCodeReviewsReviewIdStatePatchRequest**](ProjectsProjectCodeReviewsReviewIdStatePatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_suggested_reviewers_get

> Vec<crate::models::TdMemberProfile> projects_project_code_reviews_review_id_suggested_reviewers_get(project, review_id, dollar_fields)
Get suggested reviewers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_title_patch

> projects_project_code_reviews_review_id_title_patch(project, review_id, projects_project_code_reviews_review_id_title_patch_request)
Edit review title

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**projects_project_code_reviews_review_id_title_patch_request** | [**ProjectsProjectCodeReviewsReviewIdTitlePatchRequest**](ProjectsProjectCodeReviewsReviewIdTitlePatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_unbound_discussions_discussion_id_toggle_patch

> projects_project_code_reviews_review_id_unbound_discussions_discussion_id_toggle_patch(project, review_id, discussion_id, projects_project_code_reviews_review_id_unbound_discussions_discussion_id_toggle_patch_request)
Toggle unbound discussion resolution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**discussion_id** | **String** |  | [required] |
**projects_project_code_reviews_review_id_unbound_discussions_discussion_id_toggle_patch_request** | Option<[**ProjectsProjectCodeReviewsReviewIdUnboundDiscussionsDiscussionIdTogglePatchRequest**](ProjectsProjectCodeReviewsReviewIdUnboundDiscussionsDiscussionIdTogglePatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_unbound_discussions_get

> crate::models::ProjectsProjectCodeReviewsReviewIdUnboundDiscussionsGet200Response projects_project_code_reviews_review_id_unbound_discussions_get(project, review_id, dollar_skip, dollar_top, dollar_fields)
Get all unbound discussions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectCodeReviewsReviewIdUnboundDiscussionsGet200Response**](_projects__project__code_reviews__reviewId__unbound_discussions_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_review_id_unbound_discussions_post

> crate::models::CodeReviewUnboundDiscussionRecord projects_project_code_reviews_review_id_unbound_discussions_post(project, review_id, projects_project_code_reviews_review_id_unbound_discussions_post_request, dollar_fields)
Create unbound discussion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**review_id** | **String** |  | [required] |
**projects_project_code_reviews_review_id_unbound_discussions_post_request** | [**ProjectsProjectCodeReviewsReviewIdUnboundDiscussionsPostRequest**](ProjectsProjectCodeReviewsReviewIdUnboundDiscussionsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CodeReviewUnboundDiscussionRecord**](CodeReviewUnboundDiscussionRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_safe_merge_delete

> Vec<crate::models::SafeMergeMessage> projects_project_code_reviews_safe_merge_delete(project, merge_request_id, dollar_fields)
Stop Safe Merge execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**merge_request_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::SafeMergeMessage>**](SafeMergeMessage.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_safe_merge_get

> crate::models::SafeMerge projects_project_code_reviews_safe_merge_get(project, merge_request_id, dollar_fields)
Get Safe Merge execution status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**merge_request_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::SafeMerge**](SafeMerge.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_code_reviews_safe_merge_post

> Vec<crate::models::SafeMergeMessage> projects_project_code_reviews_safe_merge_post(project, projects_project_code_reviews_safe_merge_post_request, dollar_fields)
Start Safe Merge execution status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_code_reviews_safe_merge_post_request** | [**ProjectsProjectCodeReviewsSafeMergePostRequest**](ProjectsProjectCodeReviewsSafeMergePostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::SafeMergeMessage>**](SafeMergeMessage.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_delete

> projects_project_delete(project)
Delete project

Delete a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_document_id_access_get

> crate::models::DocumentAccess projects_project_documents_document_id_access_get(project, document_id, dollar_fields)
Document own access permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DocumentAccess**](DocumentAccess.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_document_id_access_patch

> projects_project_documents_document_id_access_patch(project, document_id, projects_project_documents_document_id_access_patch_request)
Update document access permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**projects_project_documents_document_id_access_patch_request** | [**ProjectsProjectDocumentsDocumentIdAccessPatchRequest**](ProjectsProjectDocumentsDocumentIdAccessPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_document_id_copy_post

> crate::models::Document projects_project_documents_document_id_copy_post(project, document_id, projects_project_documents_document_id_copy_post_request, dollar_fields)
Copy document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**projects_project_documents_document_id_copy_post_request** | [**ProjectsProjectDocumentsDocumentIdCopyPostRequest**](ProjectsProjectDocumentsDocumentIdCopyPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_document_id_delete

> projects_project_documents_document_id_delete(project, document_id)
Archive document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**document_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_document_id_get

> crate::models::Document projects_project_documents_document_id_get(project, document_id, dollar_fields)
Get document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_document_id_move_patch

> crate::models::Document projects_project_documents_document_id_move_patch(project, document_id, projects_project_documents_document_id_move_patch_request, dollar_fields)
Move document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**projects_project_documents_document_id_move_patch_request** | [**ProjectsProjectDocumentsDocumentIdMovePatchRequest**](ProjectsProjectDocumentsDocumentIdMovePatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_document_id_patch

> crate::models::Document projects_project_documents_document_id_patch(project, document_id, dollar_fields, projects_project_documents_document_id_patch_request)
Update document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**projects_project_documents_document_id_patch_request** | Option<[**ProjectsProjectDocumentsDocumentIdPatchRequest**](ProjectsProjectDocumentsDocumentIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_document_id_unarchive_patch

> crate::models::Document projects_project_documents_document_id_unarchive_patch(project, document_id, dollar_fields)
Unarchive document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_folder_access_get

> crate::models::FolderAccess projects_project_documents_folders_folder_access_get(project, folder, dollar_fields)
Folder own access permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::FolderAccess**](FolderAccess.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_folder_access_patch

> projects_project_documents_folders_folder_access_patch(project, folder, projects_project_documents_folders_folder_access_patch_request)
Update folder access permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**projects_project_documents_folders_folder_access_patch_request** | [**ProjectsProjectDocumentsFoldersFolderAccessPatchRequest**](ProjectsProjectDocumentsFoldersFolderAccessPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_folder_delete

> projects_project_documents_folders_folder_delete(project, folder)
Archive folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**folder** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_folder_documents_get

> crate::models::ProjectsProjectDocumentsFoldersFolderDocumentsGet200Response projects_project_documents_folders_folder_documents_get(project, folder, with_archived, sort_by, order, dollar_skip, dollar_top, dollar_fields)
List documents in folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**with_archived** | Option<**bool**> |  |  |[default to false]
**sort_by** | Option<**String**> |  |  |
**order** | Option<[**ColumnSortOrder**](.md)> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectDocumentsFoldersFolderDocumentsGet200Response**](_projects__project__documents_folders__folder__documents_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_folder_get

> crate::models::DocumentFolder projects_project_documents_folders_folder_get(project, folder, dollar_fields)
Get folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DocumentFolder**](DocumentFolder.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_folder_introduction_delete

> projects_project_documents_folders_folder_introduction_delete(project, folder)
Remove folder introduction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**folder** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_folder_introduction_document_id_patch

> projects_project_documents_folders_folder_introduction_document_id_patch(project, folder, document_id)
Add folder introduction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**document_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_folder_move_patch

> crate::models::DocumentFolder projects_project_documents_folders_folder_move_patch(project, folder, projects_project_documents_folders_folder_move_patch_request, dollar_fields)
Move folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**projects_project_documents_folders_folder_move_patch_request** | [**ProjectsProjectDocumentsFoldersFolderMovePatchRequest**](ProjectsProjectDocumentsFoldersFolderMovePatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DocumentFolder**](DocumentFolder.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_folder_patch

> projects_project_documents_folders_folder_patch(project, folder, chats_channels_is_name_free_post_request)
Rename folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**chats_channels_is_name_free_post_request** | [**ChatsChannelsIsNameFreePostRequest**](ChatsChannelsIsNameFreePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_folder_search_get

> crate::models::ProjectsProjectDocumentsFoldersFolderSearchGet200Response projects_project_documents_folders_folder_search_get(project, folder, query, include_body, folders_only, dollar_skip, dollar_top, dollar_fields)
Search documents and folders

Executes search for project documents and folders in specified folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**query** | **String** |  | [required] |
**include_body** | Option<**bool**> |  |  |
**folders_only** | Option<**bool**> |  |  |[default to false]
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectDocumentsFoldersFolderSearchGet200Response**](_projects__project__documents_folders__folder__search_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_folder_subfolders_get

> crate::models::ProjectsProjectDocumentsFoldersFolderSubfoldersGet200Response projects_project_documents_folders_folder_subfolders_get(project, folder, with_archived, sort_by, order, dollar_skip, dollar_top, dollar_fields)
List subfolders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**with_archived** | Option<**bool**> |  |  |[default to false]
**sort_by** | Option<**String**> |  |  |
**order** | Option<[**ColumnSortOrder**](.md)> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectDocumentsFoldersFolderSubfoldersGet200Response**](_projects__project__documents_folders__folder__subfolders_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_folders_post

> crate::models::DocumentFolder projects_project_documents_folders_post(project, projects_project_documents_folders_post_request, dollar_fields)
Create folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_documents_folders_post_request** | [**ProjectsProjectDocumentsFoldersPostRequest**](ProjectsProjectDocumentsFoldersPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DocumentFolder**](DocumentFolder.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_documents_post

> crate::models::Document projects_project_documents_post(project, projects_project_documents_post_request, dollar_fields)
Create document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_documents_post_request** | [**ProjectsProjectDocumentsPostRequest**](ProjectsProjectDocumentsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_feature_pins_patch

> projects_project_feature_pins_patch(project, projects_project_feature_pins_patch_request)
Update feature pin

Update list of project items pinned for the project by default

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_feature_pins_patch_request** | [**ProjectsProjectFeaturePinsPatchRequest**](ProjectsProjectFeaturePinsPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_get

> crate::models::PrProject projects_project_get(project, dollar_fields)
Get project

Get project by ID or project key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PrProject**](PR_Project.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_get

> Vec<crate::models::ProjectPackageRepository> projects_project_packages_repositories_get(project, r#type, query, dollar_fields)
Get repositories

Gets a list of package repositories for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**r#type** | Option<**String**> |  |  |
**query** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ProjectPackageRepository>**](ProjectPackageRepository.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_post

> crate::models::ProjectPackageRepository projects_project_packages_repositories_post(project, projects_project_packages_repositories_post_request, dollar_fields)
Create new repository

Creates a new package repository for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_packages_repositories_post_request** | [**ProjectsProjectPackagesRepositoriesPostRequest**](ProjectsProjectPackagesRepositoriesPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectPackageRepository**](ProjectPackageRepository.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_access_get

> crate::models::PackagesAccess projects_project_packages_repositories_repository_access_get(project, repository, dollar_fields)
Get repository own access

Updates package repository settings for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PackagesAccess**](PackagesAccess.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_access_patch

> projects_project_packages_repositories_repository_access_patch(project, repository, projects_project_packages_repositories_repository_access_patch_request)
Update repository own access

Updates package repository settings for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_packages_repositories_repository_access_patch_request** | [**ProjectsProjectPackagesRepositoriesRepositoryAccessPatchRequest**](ProjectsProjectPackagesRepositoriesRepositoryAccessPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_cleanup_dry_post

> crate::models::DryCleanupResults projects_project_packages_repositories_repository_cleanup_dry_post(project, repository, projects_project_packages_repositories_repository_cleanup_dry_post_request, dollar_fields)
Dry run repository cleanup

Dry run of cleanup for specified package repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_packages_repositories_repository_cleanup_dry_post_request** | [**ProjectsProjectPackagesRepositoriesRepositoryCleanupDryPostRequest**](ProjectsProjectPackagesRepositoriesRepositoryCleanupDryPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DryCleanupResults**](DryCleanupResults.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_cleanup_post

> crate::models::PackagesExecutionResult projects_project_packages_repositories_repository_cleanup_post(project, repository, dollar_fields, projects_project_packages_repositories_repository_cleanup_post_request)
Cleanup repository

Cleanup specified package repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**projects_project_packages_repositories_repository_cleanup_post_request** | Option<[**ProjectsProjectPackagesRepositoriesRepositoryCleanupPostRequest**](ProjectsProjectPackagesRepositoriesRepositoryCleanupPostRequest.md)> |  |  |

### Return type

[**crate::models::PackagesExecutionResult**](PackagesExecutionResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_connections_connection_id_publish_get

> crate::models::ProjectsProjectPackagesRepositoriesRepositoryConnectionsConnectionIdPublishGet200Response projects_project_packages_repositories_repository_connections_connection_id_publish_get(project, repository, connection_id, dollar_skip, dollar_top, dollar_fields)
Get list of publishing to remote repository

Get list of publishing to remote repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**connection_id** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPackagesRepositoriesRepositoryConnectionsConnectionIdPublishGet200Response**](_projects__project__packages_repositories__repository__connections__connectionId__publish_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_connections_connection_id_publish_post

> String projects_project_packages_repositories_repository_connections_connection_id_publish_post(project, repository, connection_id, projects_project_packages_repositories_repository_connections_connection_id_publish_post_request)
Publish packages to remote repository

Publishes packages to remote repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**connection_id** | **String** |  | [required] |
**projects_project_packages_repositories_repository_connections_connection_id_publish_post_request** | [**ProjectsProjectPackagesRepositoriesRepositoryConnectionsConnectionIdPublishPostRequest**](ProjectsProjectPackagesRepositoriesRepositoryConnectionsConnectionIdPublishPostRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_connections_get

> Vec<crate::models::PackageRepositoryConnection> projects_project_packages_repositories_repository_connections_get(project, repository, dollar_fields)
Get all remote repositories

Gets a list of remote package repositories for given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PackageRepositoryConnection>**](PackageRepositoryConnection.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_delete

> projects_project_packages_repositories_repository_delete(project, repository)
Delete repository

Removes package repository for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_files_folderfolder_path_delete

> projects_project_packages_repositories_repository_files_folderfolder_path_delete(project, repository, folder_path)
Delete folder

Removes a folder in repository for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**folder_path** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_files_get

> Vec<crate::models::FileData> projects_project_packages_repositories_repository_files_get(project, repository, parent_path, dollar_fields)
Get list of files

Gets a list of repository files for a given project ID in parent folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**parent_path** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::FileData>**](FileData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_files_namefile_path_delete

> projects_project_packages_repositories_repository_files_namefile_path_delete(project, repository, file_path)
Delete file

Removes a file in repository for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**file_path** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_files_namefile_path_get

> crate::models::FileDetails projects_project_packages_repositories_repository_files_namefile_path_get(project, repository, file_path, dollar_fields)
Get file details

Gets a details for repository file for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**file_path** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::FileDetails**](FileDetails.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_get

> crate::models::ProjectPackageRepository projects_project_packages_repositories_repository_get(project, repository, dollar_fields)
Get repository

Gets a package repository for a given project ID by type and name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectPackageRepository**](ProjectPackageRepository.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_packages_get

> crate::models::ProjectsProjectPackagesRepositoriesRepositoryPackagesGet200Response projects_project_packages_repositories_repository_packages_get(project, repository, query, connection_id, dollar_skip, dollar_top, dollar_fields)
Get all packages

Gets a list of repository packages for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**query** | **String** |  | [required] |
**connection_id** | Option<**String**> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPackagesRepositoriesRepositoryPackagesGet200Response**](_projects__project__packages_repositories__repository__packages_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_packages_namepackage_name_delete

> projects_project_packages_repositories_repository_packages_namepackage_name_delete(project, repository, package_name)
Delete package

Removes all package versions in repository for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**package_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_packages_namepackage_name_metadata_get

> crate::models::PackageMetadata projects_project_packages_repositories_repository_packages_namepackage_name_metadata_get(project, repository, package_name, dollar_fields)
Get package metadata

Get package metadata in repository for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**package_name** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PackageMetadata**](PackageMetadata.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_packages_namepackage_name_metadata_put

> projects_project_packages_repositories_repository_packages_namepackage_name_metadata_put(project, repository, package_name, projects_project_packages_repositories_repository_packages_name_package_name_metadata_put_request)
Report package  metadata

Update a package metadata in repository for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**package_name** | **String** |  | [required] |
**projects_project_packages_repositories_repository_packages_name_package_name_metadata_put_request** | Option<[**ProjectsProjectPackagesRepositoriesRepositoryPackagesNamePackageNameMetadataPutRequest**](ProjectsProjectPackagesRepositoriesRepositoryPackagesNamePackageNameMetadataPutRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_packages_namepackage_name_metadata_versionpackage_version_put

> projects_project_packages_repositories_repository_packages_namepackage_name_metadata_versionpackage_version_put(project, repository, package_name, package_version, projects_project_packages_repositories_repository_packages_name_package_name_metadata_version_package_version_put_request)
Report package version metadata

Report a package version metadata in repository for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**package_name** | **String** |  | [required] |
**package_version** | **String** |  | [required] |
**projects_project_packages_repositories_repository_packages_name_package_name_metadata_version_package_version_put_request** | [**ProjectsProjectPackagesRepositoriesRepositoryPackagesNamePackageNameMetadataVersionPackageVersionPutRequest**](ProjectsProjectPackagesRepositoriesRepositoryPackagesNamePackageNameMetadataVersionPackageVersionPutRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_packages_namepackage_name_versions_get

> crate::models::ProjectsProjectPackagesRepositoriesRepositoryPackagesNamePackageNameVersionsGet200Response projects_project_packages_repositories_repository_packages_namepackage_name_versions_get(project, repository, package_name, query, sort_column, sort_order, connection_id, pinned, dollar_skip, dollar_top, dollar_fields)
Get all package versions

Gets a list of repository package versions for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**package_name** | **String** |  | [required] |
**query** | **String** |  | [required] |
**sort_column** | [**PackagesSortColumn**](.md) |  | [required] |
**sort_order** | [**ColumnSortOrder**](.md) |  | [required] |
**connection_id** | Option<**String**> |  |  |
**pinned** | Option<**bool**> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPackagesRepositoriesRepositoryPackagesNamePackageNameVersionsGet200Response**](_projects__project__packages_repositories__repository__packages_name__packageName__versions_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_packages_namepackage_name_versions_versionpackage_version_delete

> projects_project_packages_repositories_repository_packages_namepackage_name_versions_versionpackage_version_delete(project, repository, package_name, package_version)
Delete package version

Removes a package version in repository for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**package_name** | **String** |  | [required] |
**package_version** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_packages_namepackage_name_versions_versionpackage_version_get

> crate::models::PackageVersionDetails projects_project_packages_repositories_repository_packages_namepackage_name_versions_versionpackage_version_get(project, repository, package_name, package_version, dollar_fields)
Get package version details

Gets a details for repository package version for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**package_name** | **String** |  | [required] |
**package_version** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PackageVersionDetails**](PackageVersionDetails.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_patch

> projects_project_packages_repositories_repository_patch(project, repository, projects_project_packages_repositories_repository_patch_request)
Update repository

Updates package repository settings for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_packages_repositories_repository_patch_request** | Option<[**ProjectsProjectPackagesRepositoriesRepositoryPatchRequest**](ProjectsProjectPackagesRepositoriesRepositoryPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_repository_url_get

> String projects_project_packages_repositories_repository_url_get(project, repository)
Get repository URL

Gets a package repository URL for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_typetype_repositoryrepository_name_packages_get

> crate::models::ProjectsProjectPackagesRepositoriesRepositoryPackagesGet200Response projects_project_packages_repositories_typetype_repositoryrepository_name_packages_get(project, r#type, repository_name, query, dollar_skip, dollar_top, dollar_fields)
Get all packages

Gets a list of repository packages for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**repository_name** | **String** |  | [required] |
**query** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPackagesRepositoriesRepositoryPackagesGet200Response**](_projects__project__packages_repositories__repository__packages_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_get

> crate::models::ProjectsProjectPackagesRepositoriesRepositoryPackagesNamePackageNameVersionsGet200Response projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_get(project, r#type, repository_name, package_name, query, sort_column, sort_order, dollar_skip, dollar_top, dollar_fields)
Get all package versions

Gets a list of repository package versions for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**repository_name** | **String** |  | [required] |
**package_name** | **String** |  | [required] |
**query** | **String** |  | [required] |
**sort_column** | [**PackagesSortColumn**](.md) |  | [required] |
**sort_order** | [**ColumnSortOrder**](.md) |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPackagesRepositoriesRepositoryPackagesNamePackageNameVersionsGet200Response**](_projects__project__packages_repositories__repository__packages_name__packageName__versions_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_versionpackage_version_delete

> projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_versionpackage_version_delete(project, r#type, repository_name, package_name, package_version)
Delete package version

Removes a package version in repository for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**repository_name** | **String** |  | [required] |
**package_name** | **String** |  | [required] |
**package_version** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_versionpackage_version_get

> crate::models::PackageVersionDetails projects_project_packages_repositories_typetype_repositoryrepository_name_packages_namepackage_name_versions_versionpackage_version_get(project, r#type, repository_name, package_name, package_version, dollar_fields)
Get package version details

Gets a details for repository package version for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**repository_name** | **String** |  | [required] |
**package_name** | **String** |  | [required] |
**package_version** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PackageVersionDetails**](PackageVersionDetails.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_search_get

> crate::models::ProjectsProjectPackagesRepositoriesRepositoryPackagesNamePackageNameVersionsGet200Response projects_project_packages_search_get(project, r#type, query, connection_id, dollar_skip, dollar_top, dollar_fields)
Find packages in repository

Executes a package search for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**query** | **String** |  | [required] |
**connection_id** | Option<**String**> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPackagesRepositoriesRepositoryPackagesNamePackageNameVersionsGet200Response**](_projects__project__packages_repositories__repository__packages_name__packageName__versions_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_packages_types_get

> Vec<crate::models::PackageType> projects_project_packages_types_get(project, dollar_fields)
Get all types

Returns a list of available repository types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PackageType>**](PackageType.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_patch

> crate::models::PrProject projects_project_patch(project, dollar_fields, projects_project_patch_request)
Update project

Update an existing project. Optional parameters will be ignored when not specified and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**projects_project_patch_request** | Option<[**ProjectsProjectPatchRequest**](ProjectsProjectPatchRequest.md)> |  |  |

### Return type

[**crate::models::PrProject**](PR_Project.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_people_members_by_ids_get

> Vec<crate::models::ParticipantOnProject> projects_project_people_members_by_ids_get(project, profiles, dollar_fields)
Get participants by profiles

Returns project participants by provided profiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**profiles** | [**Vec<String>**](String.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ParticipantOnProject>**](ParticipantOnProject.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_people_members_get

> crate::models::ProjectsProjectPeopleMembersGet200Response projects_project_people_members_get(project, dollar_skip, dollar_top, role, query, dollar_fields)
Get all participants

Returns all project participants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**role** | Option<**String**> |  |  |
**query** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPeopleMembersGet200Response**](_projects__project__people_members_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_people_members_profile_delete

> projects_project_people_members_profile_delete(project, profile)
Remove participant

Removes participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**profile** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_people_members_update_post

> projects_project_people_members_update_post(project, projects_project_people_members_update_post_request)
Update participant roles

Adds or removes project participant roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_people_members_update_post_request** | [**ProjectsProjectPeopleMembersUpdatePostRequest**](ProjectsProjectPeopleMembersUpdatePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_people_teams_by_ids_get

> Vec<crate::models::ParticipantTeamOnProject> projects_project_people_teams_by_ids_get(project, teams, dollar_fields)
Get participants by teams

Returns project participant teams by provided teams

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**teams** | [**Vec<String>**](String.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ParticipantTeamOnProject>**](ParticipantTeamOnProject.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_people_teams_get

> crate::models::ProjectsProjectPeopleTeamsGet200Response projects_project_people_teams_get(project, dollar_skip, dollar_top, role, query, dollar_fields)
Get all participants

Returns all project participant teams

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**role** | Option<**String**> |  |  |
**query** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPeopleTeamsGet200Response**](_projects__project__people_teams_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_people_teams_team_delete

> projects_project_people_teams_team_delete(project, team)
Remove participant

Removes participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**team** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_people_teams_update_post

> projects_project_people_teams_update_post(project, projects_project_people_teams_update_post_request)
Update participant roles

Adds or removes project team participant roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_people_teams_update_post_request** | [**ProjectsProjectPeopleTeamsUpdatePostRequest**](ProjectsProjectPeopleTeamsUpdatePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_personal_feature_pins_patch

> projects_project_personal_feature_pins_patch(project, projects_project_personal_feature_pins_patch_request)
Update personal feature pin

Update list of project items pinned for the project personally for you

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_personal_feature_pins_patch_request** | [**ProjectsProjectPersonalFeaturePinsPatchRequest**](ProjectsProjectPersonalFeaturePinsPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_boards_board_archive_delete

> projects_project_planning_boards_board_archive_delete(project, board)
Archive board

Archive an existing board. This operation can be performed by board owners or other members who are granted permission to manage boards in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**board** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_boards_get

> crate::models::ProjectsProjectPlanningBoardsGet200Response projects_project_planning_boards_get(project, dollar_skip, dollar_top, query, dollar_fields)
Get all boards

Search existing boards in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**query** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPlanningBoardsGet200Response**](_projects__project__planning_boards_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_boards_post

> crate::models::BoardRecord projects_project_planning_boards_post(project, projects_project_planning_boards_post_request, dollar_fields)
Create board

Create a new issue board in a project. The user will become the owner of the board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_planning_boards_post_request** | [**ProjectsProjectPlanningBoardsPostRequest**](ProjectsProjectPlanningBoardsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BoardRecord**](BoardRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_boards_sprints_get

> crate::models::ProjectsProjectPlanningBoardsSprintsGet200Response projects_project_planning_boards_sprints_get(project, dollar_skip, dollar_top, board, query, dollar_fields)
Get all sprints

Search existing sprints in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**board** | Option<**String**> |  |  |
**query** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPlanningBoardsSprintsGet200Response**](_projects__project__planning_boards_sprints_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_boards_starred_get

> Vec<crate::models::BoardRecord> projects_project_planning_boards_starred_get(project, dollar_fields)
Get all starred boards

Get all starred boards in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::BoardRecord>**](BoardRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_checklists_checklist_id_delete

> projects_project_planning_checklists_checklist_id_delete(project, checklist_id)
Delete checklist

Delete an existing checklist in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**checklist_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_checklists_checklist_id_full_checklist_tree_get

> Vec<crate::models::PlanItemChildren> projects_project_planning_checklists_checklist_id_full_checklist_tree_get(project, checklist_id, dollar_fields)
Get full checklist tree

Get the content of a checklist in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**checklist_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PlanItemChildren>**](PlanItemChildren.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_checklists_checklist_id_import_post

> projects_project_planning_checklists_checklist_id_import_post(project, checklist_id, projects_project_planning_checklists_checklist_id_import_post_request)
Import checklist lines

Tab indented lines are converted into checkable items following the same rules as in Import Checklist. The result is placed inside of the specified project checklist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**checklist_id** | **String** |  | [required] |
**projects_project_planning_checklists_checklist_id_import_post_request** | [**ProjectsProjectPlanningChecklistsChecklistIdImportPostRequest**](ProjectsProjectPlanningChecklistsChecklistIdImportPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_checklists_checklist_id_patch

> projects_project_planning_checklists_checklist_id_patch(project, checklist_id, projects_project_planning_checklists_checklist_id_patch_request)
Update checklist

Update an existing checklist in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**checklist_id** | **String** |  | [required] |
**projects_project_planning_checklists_checklist_id_patch_request** | Option<[**ProjectsProjectPlanningChecklistsChecklistIdPatchRequest**](ProjectsProjectPlanningChecklistsChecklistIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_checklists_get

> crate::models::ProjectsProjectPlanningChecklistsGet200Response projects_project_planning_checklists_get(project, dollar_skip, dollar_top, query, sorting, descending, dollar_fields)
Get all checklists

Search existing checklists in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**query** | Option<**String**> |  |  |
**sorting** | Option<[**ChecklistSorting**](.md)> |  |  |
**descending** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPlanningChecklistsGet200Response**](_projects__project__planning_checklists_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_checklists_import_post

> crate::models::Checklist projects_project_planning_checklists_import_post(project, projects_project_planning_checklists_import_post_request, dollar_fields)
Import checklist

Create a new checklist in a project using tab indented lines as checkable items. The items with the same indent level will be placed one under the other. An issue URL will be converted into the corresponding issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_planning_checklists_import_post_request** | [**ProjectsProjectPlanningChecklistsImportPostRequest**](ProjectsProjectPlanningChecklistsImportPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Checklist**](Checklist.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_checklists_post

> crate::models::Checklist projects_project_planning_checklists_post(project, chats_channels_is_name_free_post_request, dollar_fields)
Create checklist

Create a new checklist in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**chats_channels_is_name_free_post_request** | [**ChatsChannelsIsNameFreePostRequest**](ChatsChannelsIsNameFreePostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Checklist**](Checklist.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_checklists_starred_get

> Vec<crate::models::Checklist> projects_project_planning_checklists_starred_get(project, dollar_fields)
Get all starred checklists

Get all starred checklists in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Checklist>**](Checklist.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_fields_order_get

> crate::models::TrackerIssueFieldOrder projects_project_planning_issues_fields_order_get(project, only_visible, dollar_fields)
Get issue field order

Query order for built-in issue fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**only_visible** | Option<**bool**> |  |  |[default to true]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TrackerIssueFieldOrder**](TrackerIssueFieldOrder.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_fields_order_patch

> projects_project_planning_issues_fields_order_patch(project, projects_project_planning_issues_fields_order_patch_request)
Set issue field order

Query order for built-in issue fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_planning_issues_fields_order_patch_request** | [**ProjectsProjectPlanningIssuesFieldsOrderPatchRequest**](ProjectsProjectPlanningIssuesFieldsOrderPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_fields_visibility_get

> crate::models::TrackerIssueFieldVisibility projects_project_planning_issues_fields_visibility_get(project, dollar_fields)
Get issue field visibility

Query visibility for built-in issue fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TrackerIssueFieldVisibility**](TrackerIssueFieldVisibility.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_fields_visibility_patch

> projects_project_planning_issues_fields_visibility_patch(project, projects_project_planning_issues_fields_visibility_patch_request)
Update issue field visibility

Set visibility for a built-in issue field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_planning_issues_fields_visibility_patch_request** | [**ProjectsProjectPlanningIssuesFieldsVisibilityPatchRequest**](ProjectsProjectPlanningIssuesFieldsVisibilityPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_get

> crate::models::ProjectsPlanningBoardsSprintsSprintIssuesGet200Response projects_project_planning_issues_get(project, sorting, descending, dollar_skip, dollar_top, assignee_id, created_by_profile_id, created_by, statuses, tag_id, query, tags, sprints, boards, custom_fields, import_transaction, creation_time_from, creation_time_to, due_date_from, due_date_to, topics, grouping, deployment, dollar_fields)
Get all issues

Search existing issues in a project. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**sorting** | [**IssuesSorting**](.md) |  | [required] |
**descending** | **bool** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**assignee_id** | Option<[**Vec<String>**](String.md)> |  |  |[default to []]
**created_by_profile_id** | Option<**String**> |  |  |
**created_by** | Option<[**Vec<String>**](String.md)> |  |  |[default to []]
**statuses** | Option<[**Vec<String>**](String.md)> |  |  |[default to []]
**tag_id** | Option<**String**> |  |  |
**query** | Option<**String**> |  |  |
**tags** | Option<[**Vec<String>**](String.md)> |  |  |[default to []]
**sprints** | Option<[**Vec<String>**](String.md)> |  |  |[default to []]
**boards** | Option<[**Vec<String>**](String.md)> |  |  |[default to []]
**custom_fields** | Option<[**Vec<String>**](String.md)> |  |  |
**import_transaction** | Option<**String**> |  |  |
**creation_time_from** | Option<**String**> |  |  |
**creation_time_to** | Option<**String**> |  |  |
**due_date_from** | Option<**String**> |  |  |
**due_date_to** | Option<**String**> |  |  |
**topics** | Option<[**Vec<String>**](String.md)> |  |  |
**grouping** | Option<[**IssueListGrouping**](.md)> |  |  |
**deployment** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsPlanningBoardsSprintsSprintIssuesGet200Response**](_projects_planning_boards_sprints__sprint__issues_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_import_post

> crate::models::IssueImportResult projects_project_planning_issues_import_post(project, projects_project_planning_issues_import_post_request, dollar_fields)
Import issues

Import issues in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_planning_issues_import_post_request** | [**ProjectsProjectPlanningIssuesImportPostRequest**](ProjectsProjectPlanningIssuesImportPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::IssueImportResult**](IssueImportResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_attachment_attachment_id_delete

> projects_project_planning_issues_issue_id_attachment_attachment_id_delete(project, issue_id, attachment_id)
Remove attachment

Remove attachment from an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**attachment_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_attachment_post

> projects_project_planning_issues_issue_id_attachment_post(project, issue_id, projects_project_planning_issues_issue_id_attachment_post_request)
Add attachment

Add attachment to an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**projects_project_planning_issues_issue_id_attachment_post_request** | [**ProjectsProjectPlanningIssuesIssueIdAttachmentPostRequest**](ProjectsProjectPlanningIssuesIssueIdAttachmentPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_attachments_delete

> projects_project_planning_issues_issue_id_attachments_delete(project, issue_id, identities)
Remove attachments

Remove attachments from an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**identities** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_attachments_post

> projects_project_planning_issues_issue_id_attachments_post(project, issue_id, projects_project_planning_issues_issue_id_attachments_post_request)
Add attachments

Add attachments to an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**projects_project_planning_issues_issue_id_attachments_post_request** | [**ProjectsProjectPlanningIssuesIssueIdAttachmentsPostRequest**](ProjectsProjectPlanningIssuesIssueIdAttachmentsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_checklists_checklist_id_delete

> projects_project_planning_issues_issue_id_checklists_checklist_id_delete(project, issue_id, checklist_id)
Remove issue checklist

Remove the checklist from an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**checklist_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_checklists_checklist_id_post

> projects_project_planning_issues_issue_id_checklists_checklist_id_post(project, issue_id, checklist_id)
Add issue checklist

Add the checklist to an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**checklist_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_code_reviews_delete

> projects_project_planning_issues_issue_id_code_reviews_delete(project, issue_id, code_review_ids)
Remove code review links

Remove code review links from an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**code_review_ids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_code_reviews_post

> projects_project_planning_issues_issue_id_code_reviews_post(project, issue_id, projects_project_planning_issues_issue_id_code_reviews_post_request)
Add code review links

Add code review links to an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**projects_project_planning_issues_issue_id_code_reviews_post_request** | [**ProjectsProjectPlanningIssuesIssueIdCodeReviewsPostRequest**](ProjectsProjectPlanningIssuesIssueIdCodeReviewsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_comments_import_post

> Vec<String> projects_project_planning_issues_issue_id_comments_import_post(project, issue_id, projects_project_planning_issues_issue_id_comments_import_post_request)
Import issue comment history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**projects_project_planning_issues_issue_id_comments_import_post_request** | [**ProjectsProjectPlanningIssuesIssueIdCommentsImportPostRequest**](ProjectsProjectPlanningIssuesIssueIdCommentsImportPostRequest.md) |  | [required] |

### Return type

**Vec<String>**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_commits_delete

> projects_project_planning_issues_issue_id_commits_delete(project, issue_id, repository, commit_ids)
Remove commit links

Remove commit links from an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**commit_ids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_commits_post

> projects_project_planning_issues_issue_id_commits_post(project, issue_id, projects_project_planning_issues_issue_id_commits_post_request)
Add commit links

Add commit links to an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**projects_project_planning_issues_issue_id_commits_post_request** | [**ProjectsProjectPlanningIssuesIssueIdCommitsPostRequest**](ProjectsProjectPlanningIssuesIssueIdCommitsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_delete

> projects_project_planning_issues_issue_id_delete(project, issue_id)
Delete issue

Delete an issue from a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_get

> crate::models::Issue projects_project_planning_issues_issue_id_get(project, issue_id, with_deleted, dollar_fields)
Get issue

Retrieve issue by identifier. To retrieve multiple issues at once, use [Get issues by identifiers](/extensions/httpApiPlayground?resource=issues&parent-resource=issues&endpoint=http_post_get-by-ids) (`/issues/get-by-ids`)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**with_deleted** | Option<**bool**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Issue**](Issue.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_patch

> projects_project_planning_issues_issue_id_patch(project, issue_id, projects_project_planning_issues_issue_id_patch_request)
Update issue

Update an existing issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**projects_project_planning_issues_issue_id_patch_request** | Option<[**ProjectsProjectPlanningIssuesIssueIdPatchRequest**](ProjectsProjectPlanningIssuesIssueIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_restore_post

> projects_project_planning_issues_issue_id_restore_post(project, issue_id)
Restore issue

Restore an issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_tags_tag_id_delete

> projects_project_planning_issues_issue_id_tags_tag_id_delete(project, issue_id, tag_id)
Remove issue tag

Remove an existing tag from an issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_tags_tag_id_post

> projects_project_planning_issues_issue_id_tags_tag_id_post(project, issue_id, tag_id)
Add issue tag

Add an existing tag to an issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_issue_id_toggle_resolved_post

> projects_project_planning_issues_issue_id_toggle_resolved_post(project, issue_id, projects_project_planning_issues_issue_id_toggle_resolved_post_request)
Toggle issue resolved status

Toggle status of an existing issue between resolved and unresolved

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**issue_id** | **String** |  | [required] |
**projects_project_planning_issues_issue_id_toggle_resolved_post_request** | [**ProjectsProjectPlanningIssuesIssueIdToggleResolvedPostRequest**](ProjectsProjectPlanningIssuesIssueIdToggleResolvedPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_numbernumber_get

> crate::models::Issue projects_project_planning_issues_numbernumber_get(project, number, resolve_alias, with_deleted, dollar_fields)
Get issue by number

Find an existing issue by a given number in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**number** | **i32** |  | [required] |
**resolve_alias** | Option<**bool**> |  |  |[default to false]
**with_deleted** | Option<**bool**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Issue**](Issue.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_post

> crate::models::Issue projects_project_planning_issues_post(project, projects_project_planning_issues_post_request, dollar_fields)
Create issue

Create a new issue in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_planning_issues_post_request** | [**ProjectsProjectPlanningIssuesPostRequest**](ProjectsProjectPlanningIssuesPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Issue**](Issue.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_statuses_auto_update_on_merge_request_merge_get

> crate::models::IssueStatus projects_project_planning_issues_statuses_auto_update_on_merge_request_merge_get(project, dollar_fields)
Get auto update target issue status for merge request merge

Get target issue status for auto updating issues on linked merge request merge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::IssueStatus**](IssueStatus.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_statuses_auto_update_on_merge_request_merge_patch

> projects_project_planning_issues_statuses_auto_update_on_merge_request_merge_patch(project, projects_project_planning_issues_statuses_auto_update_on_merge_request_merge_patch_request)
Set auto update target issue status for merge request merge

Set target issue status for auto updating issues on linked merge request merge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_planning_issues_statuses_auto_update_on_merge_request_merge_patch_request** | Option<[**ProjectsProjectPlanningIssuesStatusesAutoUpdateOnMergeRequestMergePatchRequest**](ProjectsProjectPlanningIssuesStatusesAutoUpdateOnMergeRequestMergePatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_statuses_distribution_get

> Vec<crate::models::IssueStatusWithUsages> projects_project_planning_issues_statuses_distribution_get(project, dollar_fields)
Get issue status distribution

Get all existing issue statuses with their usage, number of existing issues, in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::IssueStatusWithUsages>**](IssueStatusWithUsages.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_statuses_get

> Vec<crate::models::IssueStatus> projects_project_planning_issues_statuses_get(project, dollar_fields)
Get all issue statuses

Get all existing issue statuses in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::IssueStatus>**](IssueStatus.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_statuses_patch

> projects_project_planning_issues_statuses_patch(project, projects_project_planning_issues_statuses_patch_request)
Update issue statuses list

Configure issue statuses in a project. The list must contain at least one resolved and one unresolved status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_planning_issues_statuses_patch_request** | [**ProjectsProjectPlanningIssuesStatusesPatchRequest**](ProjectsProjectPlanningIssuesStatusesPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_issues_sync_batch_get

> crate::models::ProjectsProjectPlanningIssuesSyncBatchGet200Response projects_project_planning_issues_sync_batch_get(project, batch_info, dollar_fields)
Get sync batch

Get issues in specified project for synchronization with third-party system. Issues with etag greater than specified value are returned. Read more in the [documentation](https://www.jetbrains.com/help/space/sync-api.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**batch_info** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPlanningIssuesSyncBatchGet200Response**](_projects__project__planning_issues_sync_batch_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_tags_get

> crate::models::ProjectsProjectPlanningTagsGet200Response projects_project_planning_tags_get(project, dollar_skip, dollar_top, query, dollar_fields)
Get all hierarchical tags

Search existing tags in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**query** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectPlanningTagsGet200Response**](_projects__project__planning_tags_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_planning_tags_post

> crate::models::PlanningTag projects_project_planning_tags_post(project, projects_project_planning_tags_post_request, dollar_fields)
Create hierarchical tag

Create a new hierarchical tag in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_planning_tags_post_request** | [**ProjectsProjectPlanningTagsPostRequest**](ProjectsProjectPlanningTagsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PlanningTag**](PlanningTag.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_additional_info_get

> crate::models::AdditionalRepositoryInfo projects_project_repositories_repository_additional_info_get(project, repository, dollar_fields)
Get additional repository info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AdditionalRepositoryInfo**](AdditionalRepositoryInfo.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_changes_get

> Vec<crate::models::GitFileChange> projects_project_repositories_repository_changes_get(project, repository, commit, limit, dollar_fields)
Get commit changes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**commit** | **String** |  | [required] |
**limit** | **i32** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::GitFileChange>**](GitFileChange.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_cherry_pick_commit_post

> crate::models::GitCherryPickResult projects_project_repositories_repository_cherry_pick_commit_post(project, repository, projects_project_repositories_repository_cherry_pick_commit_post_request, dollar_fields)
Cherry pick commit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_repositories_repository_cherry_pick_commit_post_request** | [**ProjectsProjectRepositoriesRepositoryCherryPickCommitPostRequest**](ProjectsProjectRepositoriesRepositoryCherryPickCommitPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GitCherryPickResult**](GitCherryPickResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_commit_branches_get

> Vec<crate::models::BranchInfo> projects_project_repositories_repository_commit_branches_get(project, repository, commit, prefix, limit, dollar_fields)
List the heads which contains given commit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**commit** | **String** |  | [required] |
**prefix** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::BranchInfo>**](BranchInfo.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_commit_post

> crate::models::GitCommitResult projects_project_repositories_repository_commit_post(project, repository, projects_project_repositories_repository_commit_post_request, dollar_fields)
Commit changes to repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_repositories_repository_commit_post_request** | [**ProjectsProjectRepositoriesRepositoryCommitPostRequest**](ProjectsProjectRepositoriesRepositoryCommitPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GitCommitResult**](GitCommitResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_commits_get

> crate::models::ProjectsProjectRepositoriesRepositoryCommitsGet200Response projects_project_repositories_repository_commits_get(project, repository, dollar_skip, dollar_top, query, dollar_fields)
List commits matching query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**query** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectRepositoriesRepositoryCommitsGet200Response**](_projects__project__repositories__repository__commits_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_default_branch_get

> String projects_project_repositories_repository_default_branch_get(project, repository)
Get repository default branch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_default_branch_post

> projects_project_repositories_repository_default_branch_post(project, repository, projects_project_repositories_repository_delete_branch_post_request)
Set repository default branch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_repositories_repository_delete_branch_post_request** | [**ProjectsProjectRepositoriesRepositoryDeleteBranchPostRequest**](ProjectsProjectRepositoriesRepositoryDeleteBranchPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_delete

> projects_project_repositories_repository_delete(project, repository)
Delete repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_delete_branch_post

> projects_project_repositories_repository_delete_branch_post(project, repository, projects_project_repositories_repository_delete_branch_post_request)
Delete branch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_repositories_repository_delete_branch_post_request** | [**ProjectsProjectRepositoriesRepositoryDeleteBranchPostRequest**](ProjectsProjectRepositoriesRepositoryDeleteBranchPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_description_post

> projects_project_repositories_repository_description_post(project, repository, chats_channels_channel_description_patch_request)
Set repository description

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**chats_channels_channel_description_patch_request** | [**ChatsChannelsChannelDescriptionPatchRequest**](ChatsChannelsChannelDescriptionPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_files_get

> Vec<crate::models::GitFile> projects_project_repositories_repository_files_get(project, repository, commit, path, dollar_fields)
List files in directory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**commit** | **String** |  | [required] |
**path** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::GitFile>**](GitFile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_gc_post

> projects_project_repositories_repository_gc_post(project, repository)
Invoke garbage collection on repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_get

> crate::models::PrRepositoryInfo projects_project_repositories_repository_get(project, repository, dollar_fields)
Get repository info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PrRepositoryInfo**](PR_RepositoryInfo.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_head_post

> projects_project_repositories_repository_head_post(project, repository, projects_project_repositories_repository_head_post_request)
Set head to given target commit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_repositories_repository_head_post_request** | [**ProjectsProjectRepositoriesRepositoryHeadPostRequest**](ProjectsProjectRepositoriesRepositoryHeadPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_heads_get

> crate::models::ProjectsProjectRepositoriesRepositoryHeadsGet200Response projects_project_repositories_repository_heads_get(project, repository, pattern, is_regex, dollar_skip, dollar_top, dollar_fields)
Get repository heads

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**pattern** | Option<[**Vec<String>**](String.md)> |  |  |[default to []]
**is_regex** | Option<**bool**> |  |  |[default to false]
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectRepositoriesRepositoryHeadsGet200Response**](_projects__project__repositories__repository__heads_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_inline_merge_diff_get

> crate::models::InlineDiff projects_project_repositories_repository_inline_merge_diff_get(project, repository, entry_type, base_blob_id, source_blob_id, target_blob_id, ignore_whitespaces, squash_simple_changes, dollar_fields)
Get inline merge diff

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**entry_type** | [**GitEntryType**](.md) |  | [required] |
**base_blob_id** | Option<**String**> |  |  |
**source_blob_id** | Option<**String**> |  |  |
**target_blob_id** | Option<**String**> |  |  |
**ignore_whitespaces** | Option<**bool**> |  |  |[default to false]
**squash_simple_changes** | Option<**bool**> |  |  |[default to true]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::InlineDiff**](InlineDiff.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_merge_branch_post

> crate::models::GitMergeBranchResult projects_project_repositories_repository_merge_branch_post(project, repository, projects_project_repositories_repository_merge_branch_post_request, dollar_fields)
Merge branch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_repositories_repository_merge_branch_post_request** | [**ProjectsProjectRepositoriesRepositoryMergeBranchPostRequest**](ProjectsProjectRepositoriesRepositoryMergeBranchPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GitMergeBranchResult**](GitMergeBranchResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_merge_preview_get

> crate::models::ProjectsProjectCodeReviewsReviewIdMergeFilesGet200Response projects_project_repositories_repository_merge_preview_get(project, repository, source_branch, target_branch, dollar_skip, dollar_top, dollar_fields)
List files to be merged on merge branches

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**source_branch** | **String** |  | [required] |
**target_branch** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectCodeReviewsReviewIdMergeFilesGet200Response**](_projects__project__code_reviews__reviewId__merge_files_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_merge_preview_status_get

> crate::models::GitMergeBranchResult projects_project_repositories_repository_merge_preview_status_get(project, repository, source_branch, target_branch, dollar_fields)
Preview merge branches result

Dry run merge source branch into target without modifying the repository. Please note that conflicting status is based on per-file analysis, so it may not be accurate on too diverged branches.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**source_branch** | **String** |  | [required] |
**target_branch** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GitMergeBranchResult**](GitMergeBranchResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_migrate_post

> crate::models::PrRepositoryInfo projects_project_repositories_repository_migrate_post(project, repository, projects_project_repositories_repository_migrate_post_request, dollar_fields)
Migrate repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_repositories_repository_migrate_post_request** | [**ProjectsProjectRepositoriesRepositoryMigratePostRequest**](ProjectsProjectRepositoriesRepositoryMigratePostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PrRepositoryInfo**](PR_RepositoryInfo.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_post

> crate::models::PrRepositoryInfo projects_project_repositories_repository_post(project, repository, dollar_fields, projects_project_repositories_repository_post_request)
Create new repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**projects_project_repositories_repository_post_request** | Option<[**ProjectsProjectRepositoriesRepositoryPostRequest**](ProjectsProjectRepositoriesRepositoryPostRequest.md)> |  |  |

### Return type

[**crate::models::PrRepositoryInfo**](PR_RepositoryInfo.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_readonly_get

> bool projects_project_repositories_repository_readonly_get(project, repository)
Get repository frozen state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |

### Return type

**bool**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_readonly_post

> projects_project_repositories_repository_readonly_post(project, repository, projects_project_repositories_repository_readonly_post_request)
Set repository frozen state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_repositories_repository_readonly_post_request** | [**ProjectsProjectRepositoriesRepositoryReadonlyPostRequest**](ProjectsProjectRepositoriesRepositoryReadonlyPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_rebase_branch_post

> crate::models::GitRebaseResult projects_project_repositories_repository_rebase_branch_post(project, repository, projects_project_repositories_repository_rebase_branch_post_request, dollar_fields)
Rebase branch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_repositories_repository_rebase_branch_post_request** | [**ProjectsProjectRepositoriesRepositoryRebaseBranchPostRequest**](ProjectsProjectRepositoriesRepositoryRebaseBranchPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GitRebaseResult**](GitRebaseResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_revisions_revision_external_checks_get

> Vec<crate::models::ExternalCheckDto> projects_project_repositories_repository_revisions_revision_external_checks_get(project, repository, revision, dollar_fields)
Get external checks for a commit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**revision** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ExternalCheckDto>**](ExternalCheckDTO.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_revisions_revision_external_checks_post

> projects_project_repositories_repository_revisions_revision_external_checks_post(project, repository, revision, projects_project_repositories_repository_revisions_revision_external_checks_post_request)
Report external check status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**revision** | **String** |  | [required] |
**projects_project_repositories_repository_revisions_revision_external_checks_post_request** | [**ProjectsProjectRepositoriesRepositoryRevisionsRevisionExternalChecksPostRequest**](ProjectsProjectRepositoriesRepositoryRevisionsRevisionExternalChecksPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_settings_get

> crate::models::GitRepositorySettings projects_project_repositories_repository_settings_get(project, repository, dollar_fields)
Get repository settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GitRepositorySettings**](GitRepositorySettings.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_settings_post

> projects_project_repositories_repository_settings_post(project, repository, projects_project_repositories_repository_settings_post_request)
Set repository settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**projects_project_repositories_repository_settings_post_request** | [**ProjectsProjectRepositoriesRepositorySettingsPostRequest**](ProjectsProjectRepositoriesRepositorySettingsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_repository_url_get

> crate::models::RepositoryUrls projects_project_repositories_repository_url_get(project, repository, dollar_fields)
Get remote URL of a Git repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**repository** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::RepositoryUrls**](RepositoryUrls.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_repositories_test_connection_post

> crate::models::TestConnectionResult projects_project_repositories_test_connection_post(project, projects_project_repositories_test_connection_post_request, dollar_fields)
Test Remote Connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_repositories_test_connection_post_request** | [**ProjectsProjectRepositoriesTestConnectionPostRequest**](ProjectsProjectRepositoriesTestConnectionPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TestConnectionResult**](TestConnectionResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_responsibilities_responsibility_id_assignees_profile_id_delete

> projects_project_responsibilities_responsibility_id_assignees_profile_id_delete(project, responsibility_id, profile_id, role)
Remove responsible

Remove a responsible person for a given project ID and responsibility ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**responsibility_id** | **String** |  | [required] |
**profile_id** | **String** |  | [required] |
**role** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_responsibilities_responsibility_id_assignees_profile_id_post

> projects_project_responsibilities_responsibility_id_assignees_profile_id_post(project, responsibility_id, profile_id, projects_project_responsibilities_responsibility_id_assignees_profile_id_post_request)
Assign responsible

Assign a responsible person for a given project ID and responsibility ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**responsibility_id** | **String** |  | [required] |
**profile_id** | **String** |  | [required] |
**projects_project_responsibilities_responsibility_id_assignees_profile_id_post_request** | Option<[**ProjectsProjectResponsibilitiesResponsibilityIdAssigneesProfileIdPostRequest**](ProjectsProjectResponsibilitiesResponsibilityIdAssigneesProfileIdPostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_responsibilities_scheme_get

> Vec<crate::models::SubjectResponsibilitiesTable> projects_project_responsibilities_scheme_get(project, dollar_fields)
Get project responsibility scheme

Get the responsibilities schema for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::SubjectResponsibilitiesTable>**](SubjectResponsibilitiesTable.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_responsibilities_subjects_post

> String projects_project_responsibilities_subjects_post(project, projects_project_responsibilities_subjects_post_request)
Add responsibility subject

Add a responsibility subject for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**projects_project_responsibilities_subjects_post_request** | Option<[**ProjectsProjectResponsibilitiesSubjectsPostRequest**](ProjectsProjectResponsibilitiesSubjectsPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_project_responsibilities_subjects_subject_id_patch

> projects_project_responsibilities_subjects_subject_id_patch(project, subject_id, projects_project_responsibilities_subjects_post_request)
Edit responsibility subject

Update an existing responsibility subject for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**subject_id** | **String** |  | [required] |
**projects_project_responsibilities_subjects_post_request** | Option<[**ProjectsProjectResponsibilitiesSubjectsPostRequest**](ProjectsProjectResponsibilitiesSubjectsPostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_repositories_find_get

> crate::models::ProjectsRepositoriesFindGet200Response projects_repositories_find_get(term, dollar_skip, dollar_top, dollar_fields)
Find Repositories

Find repositories by name substring.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsRepositoriesFindGet200Response**](_projects_repositories_find_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_responsibilities_post

> String projects_responsibilities_post(projects_responsibilities_post_request)
Add responsibility

Add a responsibility for a given subject ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_responsibilities_post_request** | [**ProjectsResponsibilitiesPostRequest**](ProjectsResponsibilitiesPostRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_responsibilities_responsibility_id_delete

> projects_responsibilities_responsibility_id_delete(responsibility_id)
Delete responsibility

Delete an existing responsibility

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**responsibility_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_responsibilities_responsibility_id_patch

> projects_responsibilities_responsibility_id_patch(responsibility_id, projects_responsibilities_responsibility_id_patch_request)
Update responsibility

Edit an existing responsibility

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**responsibility_id** | **String** |  | [required] |
**projects_responsibilities_responsibility_id_patch_request** | [**ProjectsResponsibilitiesResponsibilityIdPatchRequest**](ProjectsResponsibilitiesResponsibilityIdPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_responsibilities_subjects_subject_id_delete

> projects_responsibilities_subjects_subject_id_delete(subject_id, project)
Delete responsibility subject

Delete an existing responsibility subject for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |
**project** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_right_coderight_code_get

> crate::models::ProjectsGet200Response projects_right_coderight_code_get(right_code, dollar_skip, dollar_top, term, path, starred, dollar_fields)
Get all projects with right

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**right_code** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**term** | Option<**String**> |  |  |
**path** | Option<**String**> |  |  |
**starred** | Option<**bool**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsGet200Response**](_projects_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_right_unique_coderight_get

> crate::models::ProjectsGet200Response projects_right_unique_coderight_get(right, dollar_skip, dollar_top, term, path, starred, private, dollar_fields)
Get all projects with right code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**right** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**term** | Option<**String**> |  |  |
**path** | Option<**String**> |  |  |
**starred** | Option<**bool**> |  |  |
**private** | Option<**bool**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsGet200Response**](_projects_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_secrets_default_bundle_get

> crate::models::ProjectsSecretsDefaultBundleGet200Response projects_secrets_default_bundle_get(project, dollar_skip, dollar_top, dollar_fields)
Get all default bundle

List project secrets in a parameter bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsSecretsDefaultBundleGet200Response**](_projects_secrets_default_bundle_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_secrets_default_bundle_post

> String projects_secrets_default_bundle_post(projects_secrets_default_bundle_post_request)
Create default bundle

Create a new project secret. The secret value should be provided either as a base64-encoded value in [valueBase64], or as a reference to another secret in [secretReference].

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_secrets_default_bundle_post_request** | [**ProjectsSecretsDefaultBundlePostRequest**](ProjectsSecretsDefaultBundlePostRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_secrets_id_delete

> projects_secrets_id_delete(id)
Delete secret

Delete an existing project secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_secrets_id_patch

> projects_secrets_id_patch(id, projects_secrets_id_patch_request)
Update secret

Update an existing project secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**projects_secrets_id_patch_request** | Option<[**ProjectsSecretsIdPatchRequest**](ProjectsSecretsIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_secrets_in_default_bundle_get

> crate::models::ProjectsSecretsDefaultBundleGet200Response projects_secrets_in_default_bundle_get(project_id, dollar_skip, dollar_top, dollar_fields)
Get all in default bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsSecretsDefaultBundleGet200Response**](_projects_secrets_default_bundle_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_secrets_in_default_bundle_post

> String projects_secrets_in_default_bundle_post(projects_secrets_in_default_bundle_post_request)
Create in default bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_secrets_in_default_bundle_post_request** | [**ProjectsSecretsInDefaultBundlePostRequest**](ProjectsSecretsInDefaultBundlePostRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_tags_get

> Vec<crate::models::PrTag> projects_tags_get(dollar_fields)
Get all tags

List all tags, mapped to the number of projects they are used in

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PrTag>**](PR_Tag.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_tags_track_access_post

> projects_tags_track_access_post(projects_tags_track_access_post_request)
Track tag access

Track a tag has been accessed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_tags_track_access_post_request** | [**ProjectsTagsTrackAccessPostRequest**](ProjectsTagsTrackAccessPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_teamteam_get

> crate::models::ProjectsGet200Response projects_teamteam_get(team, dollar_skip, dollar_top, dollar_fields)
Get all projects by team

Get all projects for a team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsGet200Response**](_projects_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_vault_get

> Vec<crate::models::VaultConnectionRecord> projects_vault_get(project, dollar_fields)
Get vault

Get an existing Vault connections for project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::VaultConnectionRecord>**](VaultConnectionRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_vault_id_delete

> projects_vault_id_delete(id)
Delete vault

Delete an existing Vault connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_vault_id_patch

> projects_vault_id_patch(id, projects_vault_id_patch_request)
Update vault

Update an existing Vault connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**projects_vault_id_patch_request** | [**ProjectsVaultIdPatchRequest**](ProjectsVaultIdPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_vault_post

> String projects_vault_post(projects_vault_post_request)
Create vault

Create a new Vault connection for the project. Vault's AppRole Secret Id must be provided as base64 encoded string

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_vault_post_request** | [**ProjectsVaultPostRequest**](ProjectsVaultPostRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holidays_calendars_get

> crate::models::PublicHolidaysCalendarsGet200Response public_holidays_calendars_get(dollar_skip, dollar_top, dollar_fields)
Get all calendars

Get all public holiday calendars

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PublicHolidaysCalendarsGet200Response**](_public_holidays_calendars_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holidays_calendars_id_delete

> public_holidays_calendars_id_delete(id)
Delete calendar

Delete a public holiday calendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holidays_calendars_id_patch

> crate::models::PublicHolidayCalendarRecord public_holidays_calendars_id_patch(id, public_holidays_calendars_post_request, dollar_fields)
Update calendar

Update an existing public holiday calendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**public_holidays_calendars_post_request** | [**PublicHolidaysCalendarsPostRequest**](PublicHolidaysCalendarsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PublicHolidayCalendarRecord**](PublicHolidayCalendarRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holidays_calendars_import_post

> String public_holidays_calendars_import_post(public_holidays_calendars_import_post_request)
Import calendar

Import holidays in a public holiday calendar, using an attachment (.ics format) as the source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_holidays_calendars_import_post_request** | [**PublicHolidaysCalendarsImportPostRequest**](PublicHolidaysCalendarsImportPostRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holidays_calendars_post

> crate::models::PublicHolidayCalendarRecord public_holidays_calendars_post(public_holidays_calendars_post_request, dollar_fields)
Create calendar

Create a public holiday calendar for a location

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_holidays_calendars_post_request** | [**PublicHolidaysCalendarsPostRequest**](PublicHolidaysCalendarsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PublicHolidayCalendarRecord**](PublicHolidayCalendarRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holidays_holidays_get

> crate::models::PublicHolidaysHolidaysGet200Response public_holidays_holidays_get(dollar_skip, dollar_top, calendar, location, start_date, end_date, dollar_fields)
Get all holidays

Get/search all holidays in a public holiday calendar. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**calendar** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**start_date** | Option<**String**> |  |  |
**end_date** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PublicHolidaysHolidaysGet200Response**](_public_holidays_holidays_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holidays_holidays_id_delete

> public_holidays_holidays_id_delete(id)
Delete holiday

Delete a holiday from a public holiday calendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holidays_holidays_id_patch

> crate::models::PublicHoliday public_holidays_holidays_id_patch(id, dollar_fields, public_holidays_holidays_id_patch_request)
Update holiday

Update a holiday in a public holiday calendar. Optional parameters will be ignored when not specified and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**public_holidays_holidays_id_patch_request** | Option<[**PublicHolidaysHolidaysIdPatchRequest**](PublicHolidaysHolidaysIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::PublicHoliday**](PublicHoliday.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holidays_holidays_post

> crate::models::PublicHoliday public_holidays_holidays_post(public_holidays_holidays_post_request, dollar_fields)
Create holiday

Add a holiday to a public holiday calendar and specify if it is a working day or not

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_holidays_holidays_post_request** | [**PublicHolidaysHolidaysPostRequest**](PublicHolidaysHolidaysPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PublicHoliday**](PublicHoliday.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holidays_holidays_profile_holidays_get

> Vec<crate::models::PublicHoliday> public_holidays_holidays_profile_holidays_get(start_date, end_date, profile, working_days, dollar_fields)
Get all profile holidays

Get holidays observed in the location(s) of the current profile during the selected period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** |  | [required] |
**end_date** | **String** |  | [required] |
**profile** | **String** |  | [required] |
**working_days** | Option<**bool**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PublicHoliday>**](PublicHoliday.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holidays_holidays_related_holidays_get

> crate::models::PublicHolidaysHolidaysGet200Response public_holidays_holidays_related_holidays_get(dollar_skip, dollar_top, start_date, end_date, dollar_fields)
Get all related holidays

Search related holidays in all public holiday calendars, during the selected period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**start_date** | Option<**String**> |  |  |
**end_date** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PublicHolidaysHolidaysGet200Response**](_public_holidays_holidays_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_item_emoji_delete

> reactions_item_emoji_delete(item, emoji)
Remove reaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item** | **String** |  | [required] |
**emoji** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_item_emoji_get

> Vec<crate::models::CPrincipal> reactions_item_emoji_get(item, emoji, dollar_fields)
List reacted users and applications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item** | **String** |  | [required] |
**emoji** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::CPrincipal>**](CPrincipal.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_item_emoji_post

> reactions_item_emoji_post(item, emoji)
Add reaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item** | **String** |  | [required] |
**emoji** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_item_get

> Vec<crate::models::EmojiReaction> reactions_item_get(item, dollar_fields)
List reactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::EmojiReaction>**](EmojiReaction.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rich_text_parse_markdown_post

> crate::models::RtDocument rich_text_parse_markdown_post(rich_text_parse_markdown_post_request, dollar_fields)
Parse Markdown

Parses [Space markdown syntax](https://www.jetbrains.com/help/space/markdown-syntax.html) into a tree presentation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_text_parse_markdown_post_request** | [**RichTextParseMarkdownPostRequest**](RichTextParseMarkdownPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::RtDocument**](RtDocument.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_calendar_events_absence_events_get

> Vec<crate::models::AbsenceEvent> team_directory_calendar_events_absence_events_get(date_from, date_to, team, location, role, dollar_fields)
Get all absence events

Get/search absences. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**team** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AbsenceEvent>**](AbsenceEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_calendar_events_birthday_events_get

> Vec<crate::models::BirthdayEvent> team_directory_calendar_events_birthday_events_get(date_from, date_to, team, location, role, dollar_fields)
Get all birthday events

Get/search birthdays. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**team** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::BirthdayEvent>**](BirthdayEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_calendar_events_birthday_events_starred_get

> Vec<crate::models::BirthdayEvent> team_directory_calendar_events_birthday_events_starred_get(date_from, date_to, dollar_fields)
Get all starred birthday events

Get/search birthdays in a specific time period for starred profiles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::BirthdayEvent>**](BirthdayEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_calendar_events_get

> Vec<crate::models::MeetingRecord> team_directory_calendar_events_get(date_from, date_to, dollar_fields)
Get all calendar events

Get calendar events attached to an article in a specific time period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::MeetingRecord>**](MeetingRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_calendar_events_holidays_get

> Vec<crate::models::HolidaysEvent> team_directory_calendar_events_holidays_get(start_date, end_date, team, location, role, working_days, dollar_fields)
Get all holidays

Get/search holidays. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** |  | [required] |
**end_date** | **String** |  | [required] |
**team** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**working_days** | Option<**bool**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::HolidaysEvent>**](HolidaysEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_calendar_events_id_get

> crate::models::MeetingRecord team_directory_calendar_events_id_get(id, dollar_fields)
Get calendar event

Get a calendar event attached to an article

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::MeetingRecord**](MeetingRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_calendar_events_meeting_participations_id_patch

> crate::models::MeetingRecord team_directory_calendar_events_meeting_participations_id_patch(id, calendars_event_participations_id_patch_request, dollar_fields)
Update meeting participation

Update RSVP / calendar event participation status for a calendar event attached to an article

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**calendars_event_participations_id_patch_request** | [**CalendarsEventParticipationsIdPatchRequest**](CalendarsEventParticipationsIdPatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::MeetingRecord**](MeetingRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_calendar_events_membership_events_get

> Vec<crate::models::MembershipEvent> team_directory_calendar_events_membership_events_get(date_from, date_to, team, location, role, dollar_fields)
Get all membership events

Get/search membership events. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**team** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::MembershipEvent>**](MembershipEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_calendar_events_non_working_days_events_get

> Vec<crate::models::NonWorkingDaysEvent> team_directory_calendar_events_non_working_days_events_get(date_from, date_to, member, team, location, role, dollar_fields)
Get all non working days events

Get/search non-working day events. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** |  | [required] |
**date_to** | **String** |  | [required] |
**member** | Option<**String**> |  |  |
**team** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::NonWorkingDaysEvent>**](NonWorkingDaysEvent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_invitation_links_get

> crate::models::TeamDirectoryInvitationLinksGet200Response team_directory_invitation_links_get(dollar_skip, dollar_top, with_deleted, projects, teams, dollar_fields)
Get all invitation links

Get organization-wide invitation links

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**with_deleted** | Option<**bool**> |  |  |[default to false]
**projects** | Option<[**Vec<String>**](String.md)> |  |  |
**teams** | Option<[**Vec<String>**](String.md)> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryInvitationLinksGet200Response**](_team_directory_invitation_links_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_invitation_links_invitation_link_id_delete

> team_directory_invitation_links_invitation_link_id_delete(invitation_link_id)
Delete invitation link

Delete currently active organization-wide invitation link

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_link_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_invitation_links_invitation_link_id_patch

> team_directory_invitation_links_invitation_link_id_patch(invitation_link_id, team_directory_invitation_links_invitation_link_id_patch_request)
Update invitation link

Update an organization-wide invitation link

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_link_id** | **String** |  | [required] |
**team_directory_invitation_links_invitation_link_id_patch_request** | Option<[**TeamDirectoryInvitationLinksInvitationLinkIdPatchRequest**](TeamDirectoryInvitationLinksInvitationLinkIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_invitation_links_post

> crate::models::TeamDirectoryInvitationLinksPost200Response team_directory_invitation_links_post(team_directory_invitation_links_post_request, dollar_fields)
Create invitation link

Create an organization-wide invitation link

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_directory_invitation_links_post_request** | [**TeamDirectoryInvitationLinksPostRequest**](TeamDirectoryInvitationLinksPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryInvitationLinksPost200Response**](_team_directory_invitation_links_post_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_invitations_get

> crate::models::TeamDirectoryInvitationsGet200Response team_directory_invitations_get(dollar_skip, dollar_top, with_deleted, projects, teams, dollar_fields)
Get all invitations

Get a list of invitations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**with_deleted** | Option<**bool**> |  |  |[default to false]
**projects** | Option<[**Vec<String>**](String.md)> |  |  |
**teams** | Option<[**Vec<String>**](String.md)> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryInvitationsGet200Response**](_team_directory_invitations_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_invitations_id_delete

> team_directory_invitations_id_delete(id)
Delete invitation

Delete an invitation. Deleted invitations can no longer be used to join the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_invitations_id_patch

> team_directory_invitations_id_patch(id, team_directory_invitations_id_patch_request)
Update invitation

Update an invitation. Optional parameters will be ignored when not specified and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**team_directory_invitations_id_patch_request** | Option<[**TeamDirectoryInvitationsIdPatchRequest**](TeamDirectoryInvitationsIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_invitations_post

> crate::models::Invitation team_directory_invitations_post(team_directory_invitations_post_request, dollar_fields)
Create invitation

Create an invitation to join the current organization. Optionally, the team and/or role to join when accepting the invitation can be specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_directory_invitations_post_request** | [**TeamDirectoryInvitationsPostRequest**](TeamDirectoryInvitationsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Invitation**](Invitation.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_languages_get

> Vec<crate::models::TdLanguage> team_directory_languages_get(dollar_fields)
Get all languages

Get all languages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdLanguage>**](TD_Language.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_location_equipment_types_get

> Vec<crate::models::TdLocationEquipmentType> team_directory_location_equipment_types_get(with_archived, dollar_fields)
Get all location equipment types

Get all equipment types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**with_archived** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdLocationEquipmentType>**](TD_LocationEquipmentType.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_location_equipment_types_namename_delete

> team_directory_location_equipment_types_namename_delete(name, delete)
Delete location equipment type by name

Archive/restore location equipment type. Setting delete to true will archive the equipment type, false will restore it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**delete** | **bool** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_location_map_member_points_get

> crate::models::TeamDirectoryLocationMapMemberPointsGet200Response team_directory_location_map_member_points_get(location_id, dollar_skip, dollar_top, include_unmarked, dollar_fields)
Get all location map member points

Get members on a map for a location ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**include_unmarked** | Option<**bool**> |  |  |[default to true]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryLocationMapMemberPointsGet200Response**](_team_directory_location_map_member_points_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_location_map_member_points_location_point_id_delete

> team_directory_location_map_member_points_location_point_id_delete(location_point_id, delete)
Delete location map member point

Delete member location from a map

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_point_id** | **String** |  | [required] |
**delete** | **bool** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_location_map_member_points_location_point_id_patch

> crate::models::TdLocationMapPoint team_directory_location_map_member_points_location_point_id_patch(location_point_id, dollar_fields, team_directory_location_map_member_points_location_point_id_patch_request)
Update location map member point

Update member location on a map

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_point_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**team_directory_location_map_member_points_location_point_id_patch_request** | Option<[**TeamDirectoryLocationMapMemberPointsLocationPointIdPatchRequest**](TeamDirectoryLocationMapMemberPointsLocationPointIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::TdLocationMapPoint**](TD_LocationMapPoint.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_location_map_member_points_post

> crate::models::TdLocationMapPoint team_directory_location_map_member_points_post(team_directory_location_map_member_points_post_request, dollar_fields)
Create location map member point

Mark member location on a map

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_directory_location_map_member_points_post_request** | [**TeamDirectoryLocationMapMemberPointsPostRequest**](TeamDirectoryLocationMapMemberPointsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdLocationMapPoint**](TD_LocationMapPoint.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_locations_get

> Vec<crate::models::TdLocation> team_directory_locations_get(query, r#type, with_archived, dollar_fields)
Get all locations

Get/search all locations. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |[default to ]
**r#type** | Option<**String**> |  |  |
**with_archived** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdLocation>**](TD_Location.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_locations_id_delete

> Vec<crate::models::TdLocation> team_directory_locations_id_delete(id, dollar_fields)
Archive location

Archive a location

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdLocation>**](TD_Location.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_locations_id_get

> crate::models::TdLocation team_directory_locations_id_get(id, dollar_fields)
Get location

Get a location by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdLocation**](TD_Location.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_locations_id_map_get

> crate::models::TdLocationMap team_directory_locations_id_map_get(id, dollar_fields)
Get map

Get map for a location ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdLocationMap**](TD_LocationMap.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_locations_id_map_patch

> crate::models::TdLocationMap team_directory_locations_id_map_patch(id, team_directory_locations_id_map_patch_request, dollar_fields)
Update map

Update the map for a location

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**team_directory_locations_id_map_patch_request** | [**TeamDirectoryLocationsIdMapPatchRequest**](TeamDirectoryLocationsIdMapPatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdLocationMap**](TD_LocationMap.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_locations_id_patch

> crate::models::TdLocation team_directory_locations_id_patch(id, dollar_fields, team_directory_locations_id_patch_request)
Update location

Update a location. Optional parameters will be ignored when null and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**team_directory_locations_id_patch_request** | Option<[**TeamDirectoryLocationsIdPatchRequest**](TeamDirectoryLocationsIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::TdLocation**](TD_Location.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_locations_id_restore_post

> crate::models::TdLocation team_directory_locations_id_restore_post(id, dollar_fields)
Restore location

Restore an archived location

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdLocation**](TD_Location.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_locations_post

> crate::models::TdLocation team_directory_locations_post(team_directory_locations_post_request, dollar_fields)
Create location

Create a location

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_directory_locations_post_request** | [**TeamDirectoryLocationsPostRequest**](TeamDirectoryLocationsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdLocation**](TD_Location.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_locations_restore_post

> Vec<crate::models::TdLocation> team_directory_locations_restore_post(team_directory_locations_restore_post_request, dollar_fields)
Restore multiple locations

Restore one or more archived locations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_directory_locations_restore_post_request** | [**TeamDirectoryLocationsRestorePostRequest**](TeamDirectoryLocationsRestorePostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdLocation>**](TD_Location.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_locations_with_timezone_get

> Vec<crate::models::TdLocationWithTimeZone> team_directory_locations_with_timezone_get(dollar_fields)
Get all locations with timezone

Get all locations with their time zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdLocationWithTimeZone>**](TD_LocationWithTimeZone.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_member_locations_get

> crate::models::TeamDirectoryMemberLocationsGet200Response team_directory_member_locations_get(dollar_skip, dollar_top, profiles, locations, since, till, with_archived, dollar_fields)
Get all member locations

Get/search member locations. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**profiles** | Option<[**Vec<String>**](String.md)> |  |  |
**locations** | Option<[**Vec<String>**](String.md)> |  |  |
**since** | Option<**String**> |  |  |
**till** | Option<**String**> |  |  |
**with_archived** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryMemberLocationsGet200Response**](_team_directory_member_locations_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_member_locations_member_location_id_delete

> team_directory_member_locations_member_location_id_delete(member_location_id, delete)
Delete member location

Archive/unarchive a member location. Setting delete to true will archive the member location, false will restore it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_location_id** | **String** |  | [required] |
**delete** | Option<**bool**> |  |  |[default to true]

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_member_locations_member_location_id_get

> crate::models::TdMemberLocation team_directory_member_locations_member_location_id_get(member_location_id, dollar_fields)
Get member location

Get a member location by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_location_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMemberLocation**](TD_MemberLocation.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_member_locations_member_location_id_patch

> crate::models::TdMemberLocation team_directory_member_locations_member_location_id_patch(member_location_id, dollar_fields, team_directory_member_locations_member_location_id_patch_request)
Update member location

Update member location. Optional parameters will be ignored when null and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_location_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**team_directory_member_locations_member_location_id_patch_request** | Option<[**TeamDirectoryMemberLocationsMemberLocationIdPatchRequest**](TeamDirectoryMemberLocationsMemberLocationIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::TdMemberLocation**](TD_MemberLocation.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_member_locations_post

> crate::models::TdMemberLocation team_directory_member_locations_post(team_directory_member_locations_post_request, dollar_fields)
Create member location

Add a member location, optionally from/until a given date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_directory_member_locations_post_request** | [**TeamDirectoryMemberLocationsPostRequest**](TeamDirectoryMemberLocationsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMemberLocation**](TD_MemberLocation.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_membership_events_get

> crate::models::TeamDirectoryMembershipEventsGet200Response team_directory_membership_events_get(dollar_skip, dollar_top, team_id, location_id, role_id, dollar_fields)
Get all membership events

Get/search membership events. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**team_id** | Option<**String**> |  |  |
**location_id** | Option<**String**> |  |  |
**role_id** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryMembershipEventsGet200Response**](_team_directory_membership_events_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_get

> crate::models::TeamDirectoryMembershipsGet200Response team_directory_memberships_get(dollar_skip, dollar_top, identifiers, profiles, teams, direct_teams, roles, direct_roles, since, till, requires_approval, with_archived, dollar_fields)
Get all memberships

Get/search team memberships. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**identifiers** | Option<[**Vec<String>**](String.md)> |  |  |
**profiles** | Option<[**Vec<String>**](String.md)> |  |  |
**teams** | Option<[**Vec<String>**](String.md)> |  |  |
**direct_teams** | Option<**bool**> |  |  |[default to false]
**roles** | Option<[**Vec<String>**](String.md)> |  |  |
**direct_roles** | Option<**bool**> |  |  |[default to false]
**since** | Option<**String**> |  |  |
**till** | Option<**String**> |  |  |
**requires_approval** | Option<**bool**> |  |  |
**with_archived** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryMembershipsGet200Response**](_team_directory_memberships_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_manager_candidates_get

> crate::models::ChatsChannelsChannelSubscribersUsersGet200Response team_directory_memberships_manager_candidates_get(term, dollar_skip, dollar_top, team_id, excluded_member_id, dollar_fields)
Get manager candidate

Query profiles that can be a manager

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**team_id** | Option<**String**> |  |  |
**excluded_member_id** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelSubscribersUsersGet200Response**](_chats_channels__channel__subscribers_users_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_membership_id_delete

> team_directory_memberships_membership_id_delete(membership_id, delete)
Delete membership

Archive/unarchive a team membership. Setting delete to true will archive the membership, false will restore it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **String** |  | [required] |
**delete** | Option<**bool**> |  |  |[default to true]

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_membership_id_get

> crate::models::TdMembership team_directory_memberships_membership_id_get(membership_id, dollar_fields)
Get membership

Get a single membership by its identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMembership**](TD_Membership.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_membership_id_patch

> crate::models::TdMembership team_directory_memberships_membership_id_patch(membership_id, dollar_fields, team_directory_memberships_membership_id_patch_request)
Update membership

Update a team membership. Optional parameters will be ignored when null and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**team_directory_memberships_membership_id_patch_request** | Option<[**TeamDirectoryMembershipsMembershipIdPatchRequest**](TeamDirectoryMembershipsMembershipIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::TdMembership**](TD_Membership.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_membership_id_request_revoke_patch

> team_directory_memberships_membership_id_request_revoke_patch(membership_id, team_directory_memberships_membership_id_request_revoke_patch_request)
Request membership revocation

Request a team membership to end at a given date/time. Will need approval.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **String** |  | [required] |
**team_directory_memberships_membership_id_request_revoke_patch_request** | [**TeamDirectoryMembershipsMembershipIdRequestRevokePatchRequest**](TeamDirectoryMembershipsMembershipIdRequestRevokePatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_membership_id_revoke_delete

> team_directory_memberships_membership_id_revoke_delete(membership_id, till)
Revoke membership

Revoke a team membership to end at a given date/time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **String** |  | [required] |
**till** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_post

> crate::models::TdMembership team_directory_memberships_post(team_directory_memberships_post_request, dollar_fields)
Create membership

Create a team membership

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_directory_memberships_post_request** | [**TeamDirectoryMembershipsPostRequest**](TeamDirectoryMembershipsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMembership**](TD_Membership.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_requests_get

> crate::models::TeamDirectoryMembershipsRequestsGet200Response team_directory_memberships_requests_get(dollar_skip, dollar_top, team_id, direct, dollar_fields)
Get all requests

Get/search all membership requests. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**team_id** | Option<**String**> |  |  |
**direct** | Option<**bool**> |  |  |[default to true]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryMembershipsRequestsGet200Response**](_team_directory_memberships_requests_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_requests_membership_request_id_delete

> crate::models::TdMembership team_directory_memberships_requests_membership_request_id_delete(membership_request_id, dollar_fields)
Delete request

Delete a team membership request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_request_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMembership**](TD_Membership.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_requests_membership_request_id_patch

> team_directory_memberships_requests_membership_request_id_patch(membership_request_id, team_directory_memberships_requests_membership_request_id_patch_request)
Update request

Approve/reject a team membership request. Setting approved to true will approve the membership request, false will reject it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_request_id** | **String** |  | [required] |
**team_directory_memberships_requests_membership_request_id_patch_request** | [**TeamDirectoryMembershipsRequestsMembershipRequestIdPatchRequest**](TeamDirectoryMembershipsRequestsMembershipRequestIdPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_memberships_sync_batch_get

> crate::models::TeamDirectoryMembershipsSyncBatchGet200Response team_directory_memberships_sync_batch_get(batch_info, dollar_fields)
Get sync batch

Get memberships for synchronization with third-party system. Memberships with etag greater than specified value are returned. Read more in the [documentation](https://www.jetbrains.com/help/space/sync-api.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_info** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryMembershipsSyncBatchGet200Response**](_team_directory_memberships_sync_batch_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_authentication_sessions_owner_get

> Vec<crate::models::EsAuthenticationSession> team_directory_profiles_authentication_sessions_owner_get(owner, dollar_fields)
Get all authentication sessions

Get the current authentication sessions for a given profile ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::EsAuthenticationSession>**](ES_AuthenticationSession.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_authentication_sessions_owner_session_id_delete

> team_directory_profiles_authentication_sessions_owner_session_id_delete(owner, session_id)
Terminate own authentication session

Terminate an existing authentication session. Doing so will close the session and log out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** |  | [required] |
**session_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_dashboards_dashboard_get

> crate::models::DashboardPreferencesRecord team_directory_profiles_dashboards_dashboard_get(dashboard, dollar_fields)
Get dashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DashboardPreferencesRecord**](DashboardPreferencesRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_dashboards_dashboard_patch

> team_directory_profiles_dashboards_dashboard_patch(dashboard, team_directory_profiles_dashboards_dashboard_patch_request)
Update dashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard** | **String** |  | [required] |
**team_directory_profiles_dashboards_dashboard_patch_request** | [**TeamDirectoryProfilesDashboardsDashboardPatchRequest**](TeamDirectoryProfilesDashboardsDashboardPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_emailemail_get

> crate::models::TdMemberProfile team_directory_profiles_emailemail_get(email, verified, dollar_fields)
Get profile by email

Get profile information by email address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**verified** | Option<**bool**> |  |  |[default to true]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMemberProfile**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_favorites_deployment_targets_get

> Vec<String> team_directory_profiles_favorites_deployment_targets_get()
Get favorite deployment targets

IDs of favorite deployment targets

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_favorites_documents_get

> Vec<String> team_directory_profiles_favorites_documents_get()
Get favorite documents

IDs of favorite documents

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_favorites_id_delete

> team_directory_profiles_favorites_id_delete(id, kind)
Remove from favorites

Remove an entity with the given `id` and of the given `kind` from favorites. For profiles this operation is called “unfollow” in the user interface.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**kind** | [**StarredItemKind**](.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_favorites_jobs_get

> Vec<String> team_directory_profiles_favorites_jobs_get()
Get favorite jobs

IDs of favorite jobs

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_favorites_locations_get

> Vec<crate::models::TdLocation> team_directory_profiles_favorites_locations_get(dollar_fields)
Get favorite locations

Favorite locations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdLocation>**](TD_Location.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_favorites_post

> team_directory_profiles_favorites_post(team_directory_profiles_favorites_post_request)
Add to favorites

Add an entity with the given `id` and of the given `kind` to favorites. For profiles this operation is called “follow” in the user interface.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_directory_profiles_favorites_post_request** | [**TeamDirectoryProfilesFavoritesPostRequest**](TeamDirectoryProfilesFavoritesPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_favorites_profiles_get

> Vec<crate::models::TdMemberProfile> team_directory_profiles_favorites_profiles_get(dollar_fields)
Get followed profiles

Followed profiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_favorites_projects_get

> Vec<crate::models::PrProject> team_directory_profiles_favorites_projects_get(dollar_fields)
Get favorite projects

Favorite projects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PrProject>**](PR_Project.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_favorites_repositories_get

> Vec<String> team_directory_profiles_favorites_repositories_get()
Get favorite repositories

IDs of favorite code repositories

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_favorites_teams_get

> Vec<crate::models::TdTeam> team_directory_profiles_favorites_teams_get(dollar_fields)
Get favorite teams

Favorite teams

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdTeam>**](TD_Team.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_get

> crate::models::ChatsChannelsChannelSubscribersUsersGet200Response team_directory_profiles_get(dollar_skip, dollar_top, query, report_past_members, report_future_members, report_current_members, team_id, location_id, role_id, me_on_top, order, org_relation, profiles, dollar_fields)
Get all profiles

Get/search all profiles. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**query** | Option<**String**> |  |  |[default to ]
**report_past_members** | Option<**bool**> |  |  |[default to false]
**report_future_members** | Option<**bool**> |  |  |[default to false]
**report_current_members** | Option<**bool**> |  |  |[default to true]
**team_id** | Option<**String**> |  |  |
**location_id** | Option<**String**> |  |  |
**role_id** | Option<**String**> |  |  |
**me_on_top** | Option<**bool**> |  |  |[default to false]
**order** | Option<[**ProfileOrder**](.md)> |  |  |
**org_relation** | Option<[**ProfileOrgRelation**](.md)> |  |  |
**profiles** | Option<[**Vec<String>**](String.md)> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelSubscribersUsersGet200Response**](_chats_channels__channel__subscribers_users_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_oauth_consents_owner_applications_application_delete

> team_directory_profiles_oauth_consents_owner_applications_application_delete(owner, application)
Delete application

Remove a previously approved application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** |  | [required] |
**application** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_oauth_consents_owner_approved_scopes_id_delete

> team_directory_profiles_oauth_consents_owner_approved_scopes_id_delete(owner, id)
Delete approved scope

Remove a previously approved scope

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_oauth_consents_owner_get

> Vec<crate::models::EsOAuthConsent> team_directory_profiles_oauth_consents_owner_get(owner, dollar_fields)
Get OAuth consents

Get all OAuth consents for a given profile ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::EsOAuthConsent>**](ES_OAuthConsent.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_oauth_consents_owner_internal_applications_client_id_delete

> team_directory_profiles_oauth_consents_owner_internal_applications_client_id_delete(owner, client_id)
Delete internal application

Remove a previously approved internal application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** |  | [required] |
**client_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_oauth_consents_owner_refresh_tokens_id_delete

> team_directory_profiles_oauth_consents_owner_refresh_tokens_id_delete(owner, id)
Delete refresh token

Remove a refresh token. This will require the client to re-authenticate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_post

> crate::models::TdMemberProfile team_directory_profiles_post(team_directory_profiles_post_request, dollar_fields)
Create profile

Create a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_directory_profiles_post_request** | [**TeamDirectoryProfilesPostRequest**](TeamDirectoryProfilesPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMemberProfile**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile2_fa_requirements_get

> Vec<crate::models::Profile2FaRequirement> team_directory_profiles_profile2_fa_requirements_get(profile, dollar_fields)
Two-factor authentication requirements

Get two-factor authentication requirements for a given profile ID. The response indicates whether two-factor authentication is required by participation in some permission roles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Profile2FaRequirement>**](Profile2FARequirement.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile2_fa_status_get

> crate::models::TwoFactorAuthenticationStatus team_directory_profiles_profile2_fa_status_get(profile)
Two-factor authentication status

Get two-factor authentication status for a given profile ID. The response indicates whether two-factor authentication is active, not active, or not set up yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |

### Return type

[**crate::models::TwoFactorAuthenticationStatus**](TwoFactorAuthenticationStatus.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile2_fa_totp_confirm_post

> team_directory_profiles_profile2_fa_totp_confirm_post(profile, team_directory_profiles_profile2_fa_totp_confirm_post_request)
Confirm TOTP two-factor authentication settings

Confirm two-factor authentication for a given profile ID using a TOTP (Time-based One-time Password) code from an app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile2_fa_totp_confirm_post_request** | [**TeamDirectoryProfilesProfile2FaTotpConfirmPostRequest**](TeamDirectoryProfilesProfile2FaTotpConfirmPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile2_fa_totp_delete

> team_directory_profiles_profile2_fa_totp_delete(profile, code)
Delete current TOTP two-factor authentication settings

Remove two-factor authentication settings for a given profile ID. Previously generated TOTP (Time-based One-time Password) are rendered invalid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**code** | Option<**i32**> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile2_fa_totp_patch

> team_directory_profiles_profile2_fa_totp_patch(profile, team_directory_profiles_profile2_fa_totp_patch_request)
Update TOTP two-factor authentication settings

Enable/disable two-factor authentication settings for a given profile ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile2_fa_totp_patch_request** | [**TeamDirectoryProfilesProfile2FaTotpPatchRequest**](TeamDirectoryProfilesProfile2FaTotpPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile2_fa_totp_post

> crate::models::TwoFactorAuthenticationSecret team_directory_profiles_profile2_fa_totp_post(profile, dollar_fields)
Set up TOTP two-factor authentication

Set up two-factor authentication using TOTP (Time-based One-time Password) for a given profile ID. The response will return a QR code (base64 encoded) that can be scanned with an app to setup two-factor authentication. The code that the app generates has to be confirmed in Space to enable TOTP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TwoFactorAuthenticationSecret**](TwoFactorAuthenticationSecret.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_application_passwords_get

> crate::models::TeamDirectoryProfilesProfileApplicationPasswordsGet200Response team_directory_profiles_profile_application_passwords_get(profile, dollar_skip, dollar_top, dollar_fields)
Get all application passwords

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryProfilesProfileApplicationPasswordsGet200Response**](_team_directory_profiles__profile__application_passwords_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_application_passwords_password_id_delete

> team_directory_profiles_profile_application_passwords_password_id_delete(profile, password_id)
Delete application password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**password_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_application_passwords_password_id_patch

> team_directory_profiles_profile_application_passwords_password_id_patch(profile, password_id, team_directory_profiles_profile_application_passwords_password_id_patch_request)
Update application password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**password_id** | **String** |  | [required] |
**team_directory_profiles_profile_application_passwords_password_id_patch_request** | Option<[**TeamDirectoryProfilesProfileApplicationPasswordsPasswordIdPatchRequest**](TeamDirectoryProfilesProfileApplicationPasswordsPasswordIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_application_passwords_post

> crate::models::TeamDirectoryProfilesProfileApplicationPasswordsPost200Response team_directory_profiles_profile_application_passwords_post(profile, team_directory_profiles_profile_application_passwords_post_request, dollar_fields)
Create application password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile_application_passwords_post_request** | [**TeamDirectoryProfilesProfileApplicationPasswordsPostRequest**](TeamDirectoryProfilesProfileApplicationPasswordsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryProfilesProfileApplicationPasswordsPost200Response**](_team_directory_profiles__profile__application_passwords_post_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_checklists_checklist_id_delete

> team_directory_profiles_profile_checklists_checklist_id_delete(profile, checklist_id)
Delete checklist

Delete an existing checklist associated with the profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**checklist_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_checklists_checklist_id_full_checklist_tree_get

> Vec<crate::models::PlanItemChildren> team_directory_profiles_profile_checklists_checklist_id_full_checklist_tree_get(profile, checklist_id, dollar_fields)
Get full checklist tree

Get the content of a checklist associated with the profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**checklist_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PlanItemChildren>**](PlanItemChildren.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_checklists_checklist_id_import_post

> team_directory_profiles_profile_checklists_checklist_id_import_post(profile, checklist_id, projects_project_planning_checklists_checklist_id_import_post_request)
Import checklist lines

Tab indented lines are converted into checkable items following the same rules as in Import Checklist. The result is placed inside of the specified personal checklist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**checklist_id** | **String** |  | [required] |
**projects_project_planning_checklists_checklist_id_import_post_request** | [**ProjectsProjectPlanningChecklistsChecklistIdImportPostRequest**](ProjectsProjectPlanningChecklistsChecklistIdImportPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_checklists_checklist_id_patch

> team_directory_profiles_profile_checklists_checklist_id_patch(profile, checklist_id, team_directory_profiles_profile_checklists_checklist_id_patch_request)
Update checklist

Update an existing checklist associated with the profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**checklist_id** | **String** |  | [required] |
**team_directory_profiles_profile_checklists_checklist_id_patch_request** | Option<[**TeamDirectoryProfilesProfileChecklistsChecklistIdPatchRequest**](TeamDirectoryProfilesProfileChecklistsChecklistIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_checklists_get

> Vec<crate::models::Checklist> team_directory_profiles_profile_checklists_get(profile, dollar_fields)
Get all checklists

Get all existing checklists associated with the profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Checklist>**](Checklist.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_checklists_import_post

> crate::models::Checklist team_directory_profiles_profile_checklists_import_post(profile, projects_project_planning_checklists_import_post_request, dollar_fields)
Import checklist

Create a new checklist associated with the profile using tab indented lines as checkable items. The items with the same indent level will be placed one under the other. An issue URL will be converted into the corresponding issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**projects_project_planning_checklists_import_post_request** | [**ProjectsProjectPlanningChecklistsImportPostRequest**](ProjectsProjectPlanningChecklistsImportPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Checklist**](Checklist.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_checklists_post

> crate::models::Checklist team_directory_profiles_profile_checklists_post(profile, chats_channels_is_name_free_post_request, dollar_fields)
Create checklist

Create a new checklist associated with the profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**chats_channels_is_name_free_post_request** | [**ChatsChannelsIsNameFreePostRequest**](ChatsChannelsIsNameFreePostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Checklist**](Checklist.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_checklists_starred_get

> Vec<crate::models::Checklist> team_directory_profiles_profile_checklists_starred_get(profile, dollar_fields)
Get all starred checklists

Get all starred checklists associated with the profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Checklist>**](Checklist.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_convert_to_guest_patch

> crate::models::DryRunResult team_directory_profiles_profile_convert_to_guest_patch(profile, team_directory_profiles_profile_convert_to_guest_patch_request, dollar_fields)
Convert organization member into guest user

Convert to guest profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile_convert_to_guest_patch_request** | [**TeamDirectoryProfilesProfileConvertToGuestPatchRequest**](TeamDirectoryProfilesProfileConvertToGuestPatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DryRunResult**](DryRunResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_convert_to_member_patch

> crate::models::DryRunResult team_directory_profiles_profile_convert_to_member_patch(profile, team_directory_profiles_profile_convert_to_member_patch_request, dollar_fields)
Convert guest user into organization member

Convert to organization member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile_convert_to_member_patch_request** | [**TeamDirectoryProfilesProfileConvertToMemberPatchRequest**](TeamDirectoryProfilesProfileConvertToMemberPatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DryRunResult**](DryRunResult.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_deactivate_delete

> crate::models::TdMemberProfile team_directory_profiles_profile_deactivate_delete(profile, at, dollar_fields)
Deactivate user profile

Deactivate a user profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**at** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMemberProfile**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_delete

> crate::models::TdMemberProfile team_directory_profiles_profile_delete(profile, dollar_fields)
Delete profile

Delete a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMemberProfile**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_document_id_access_get

> crate::models::DocumentAccess team_directory_profiles_profile_documents_document_id_access_get(profile, document_id, dollar_fields)
Document own access permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DocumentAccess**](DocumentAccess.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_document_id_access_patch

> team_directory_profiles_profile_documents_document_id_access_patch(profile, document_id, projects_project_documents_document_id_access_patch_request)
Update document access permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**projects_project_documents_document_id_access_patch_request** | [**ProjectsProjectDocumentsDocumentIdAccessPatchRequest**](ProjectsProjectDocumentsDocumentIdAccessPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_document_id_copy_post

> crate::models::Document team_directory_profiles_profile_documents_document_id_copy_post(profile, document_id, projects_project_documents_document_id_copy_post_request, dollar_fields)
Copy document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**projects_project_documents_document_id_copy_post_request** | [**ProjectsProjectDocumentsDocumentIdCopyPostRequest**](ProjectsProjectDocumentsDocumentIdCopyPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_document_id_delete

> team_directory_profiles_profile_documents_document_id_delete(profile, document_id)
Archive document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**document_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_document_id_get

> crate::models::Document team_directory_profiles_profile_documents_document_id_get(profile, document_id, dollar_fields)
Get document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_document_id_move_patch

> crate::models::Document team_directory_profiles_profile_documents_document_id_move_patch(profile, document_id, projects_project_documents_document_id_move_patch_request, dollar_fields)
Move document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**projects_project_documents_document_id_move_patch_request** | [**ProjectsProjectDocumentsDocumentIdMovePatchRequest**](ProjectsProjectDocumentsDocumentIdMovePatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_document_id_patch

> crate::models::Document team_directory_profiles_profile_documents_document_id_patch(profile, document_id, dollar_fields, projects_project_documents_document_id_patch_request)
Update document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**projects_project_documents_document_id_patch_request** | Option<[**ProjectsProjectDocumentsDocumentIdPatchRequest**](ProjectsProjectDocumentsDocumentIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_document_id_unarchive_patch

> crate::models::Document team_directory_profiles_profile_documents_document_id_unarchive_patch(profile, document_id, dollar_fields)
Unarchive document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**document_id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_folder_access_get

> crate::models::FolderAccess team_directory_profiles_profile_documents_folders_folder_access_get(profile, folder, dollar_fields)
Folder own access permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::FolderAccess**](FolderAccess.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_folder_access_patch

> team_directory_profiles_profile_documents_folders_folder_access_patch(profile, folder, projects_project_documents_folders_folder_access_patch_request)
Update folder access permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**projects_project_documents_folders_folder_access_patch_request** | [**ProjectsProjectDocumentsFoldersFolderAccessPatchRequest**](ProjectsProjectDocumentsFoldersFolderAccessPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_folder_delete

> team_directory_profiles_profile_documents_folders_folder_delete(profile, folder)
Archive folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**folder** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_folder_documents_get

> crate::models::ProjectsProjectDocumentsFoldersFolderDocumentsGet200Response team_directory_profiles_profile_documents_folders_folder_documents_get(profile, folder, with_archived, sort_by, order, dollar_skip, dollar_top, dollar_fields)
List documents in folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**with_archived** | Option<**bool**> |  |  |[default to false]
**sort_by** | Option<**String**> |  |  |
**order** | Option<[**ColumnSortOrder**](.md)> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectDocumentsFoldersFolderDocumentsGet200Response**](_projects__project__documents_folders__folder__documents_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_folder_get

> crate::models::DocumentFolder team_directory_profiles_profile_documents_folders_folder_get(profile, folder, dollar_fields)
Get folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DocumentFolder**](DocumentFolder.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_folder_introduction_delete

> team_directory_profiles_profile_documents_folders_folder_introduction_delete(profile, folder)
Remove folder introduction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**folder** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_folder_introduction_document_id_patch

> team_directory_profiles_profile_documents_folders_folder_introduction_document_id_patch(profile, folder, document_id)
Add folder introduction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**document_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_folder_move_patch

> crate::models::DocumentFolder team_directory_profiles_profile_documents_folders_folder_move_patch(profile, folder, projects_project_documents_folders_folder_move_patch_request, dollar_fields)
Move folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**projects_project_documents_folders_folder_move_patch_request** | [**ProjectsProjectDocumentsFoldersFolderMovePatchRequest**](ProjectsProjectDocumentsFoldersFolderMovePatchRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DocumentFolder**](DocumentFolder.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_folder_patch

> team_directory_profiles_profile_documents_folders_folder_patch(profile, folder, chats_channels_is_name_free_post_request)
Rename folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**chats_channels_is_name_free_post_request** | [**ChatsChannelsIsNameFreePostRequest**](ChatsChannelsIsNameFreePostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_folder_search_get

> crate::models::ProjectsProjectDocumentsFoldersFolderSearchGet200Response team_directory_profiles_profile_documents_folders_folder_search_get(profile, folder, query, include_body, folders_only, dollar_skip, dollar_top, dollar_fields)
Search documents and folders

Executes search for personal documents and folders in specified folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**query** | **String** |  | [required] |
**include_body** | Option<**bool**> |  |  |
**folders_only** | Option<**bool**> |  |  |[default to false]
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectDocumentsFoldersFolderSearchGet200Response**](_projects__project__documents_folders__folder__search_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_folder_subfolders_get

> crate::models::ProjectsProjectDocumentsFoldersFolderSubfoldersGet200Response team_directory_profiles_profile_documents_folders_folder_subfolders_get(profile, folder, with_archived, sort_by, order, dollar_skip, dollar_top, dollar_fields)
List subfolders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**folder** | **String** |  | [required] |
**with_archived** | Option<**bool**> |  |  |[default to false]
**sort_by** | Option<**String**> |  |  |
**order** | Option<[**ColumnSortOrder**](.md)> |  |  |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectDocumentsFoldersFolderSubfoldersGet200Response**](_projects__project__documents_folders__folder__subfolders_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_folders_post

> crate::models::DocumentFolder team_directory_profiles_profile_documents_folders_post(profile, projects_project_documents_folders_post_request, dollar_fields)
Create folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**projects_project_documents_folders_post_request** | [**ProjectsProjectDocumentsFoldersPostRequest**](ProjectsProjectDocumentsFoldersPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::DocumentFolder**](DocumentFolder.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_documents_post

> crate::models::Document team_directory_profiles_profile_documents_post(profile, projects_project_documents_post_request, dollar_fields)
Create document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**projects_project_documents_post_request** | [**ProjectsProjectDocumentsPostRequest**](ProjectsProjectDocumentsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_get

> crate::models::TdMemberProfile team_directory_profiles_profile_get(profile, dollar_fields)
Get profile

Get profile information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMemberProfile**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_gpg_keys_fingerprint_delete

> team_directory_profiles_profile_gpg_keys_fingerprint_delete(profile, fingerprint)
Delete public GPG key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**fingerprint** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_gpg_keys_fingerprint_patch

> team_directory_profiles_profile_gpg_keys_fingerprint_patch(profile, fingerprint, applications_application_gpg_keys_fingerprint_patch_request)
Revoke public GPG key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**fingerprint** | **String** |  | [required] |
**applications_application_gpg_keys_fingerprint_patch_request** | Option<[**ApplicationsApplicationGpgKeysFingerprintPatchRequest**](ApplicationsApplicationGpgKeysFingerprintPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_gpg_keys_get

> Vec<crate::models::GpgKeyData> team_directory_profiles_profile_gpg_keys_get(profile, dollar_fields)
List public GPG keys

List GPG public keys associated with a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::GpgKeyData>**](GpgKeyData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_gpg_keys_post

> crate::models::GpgKeyData team_directory_profiles_profile_gpg_keys_post(profile, team_directory_profiles_profile_gpg_keys_post_request, dollar_fields)
Add public GPG key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile_gpg_keys_post_request** | [**TeamDirectoryProfilesProfileGpgKeysPostRequest**](TeamDirectoryProfilesProfileGpgKeysPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GpgKeyData**](GpgKeyData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_is_team_member_get

> bool team_directory_profiles_profile_is_team_member_get(profile, team_ids)
Check if profile is team member

Check if a user profile is a member of one or more teams

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_ids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

**bool**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_leads_get

> Vec<crate::models::TdMemberProfile> team_directory_profiles_profile_leads_get(profile, dollar_fields)
Get all leads

Get team leads for a given profile ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_nav_bar_menu_items_get

> Vec<crate::models::NavBarMenuItem> team_directory_profiles_profile_nav_bar_menu_items_get(profile, dollar_fields)
Get all nav bar menu items

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::NavBarMenuItem>**](NavBarMenuItem.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_nav_bar_menu_items_patch

> team_directory_profiles_profile_nav_bar_menu_items_patch(profile, team_directory_profiles_profile_nav_bar_menu_items_patch_request)
Update nav bar menu item

Toggle visibility for a given navigation bar item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile_nav_bar_menu_items_patch_request** | [**TeamDirectoryProfilesProfileNavBarMenuItemsPatchRequest**](TeamDirectoryProfilesProfileNavBarMenuItemsPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_nav_bar_projects_get

> Vec<crate::models::PrProject> team_directory_profiles_profile_nav_bar_projects_get(profile, dollar_fields)
Get all nav bar projects

Add a project to the navigation bar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PrProject>**](PR_Project.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_nav_bar_projects_post

> team_directory_profiles_profile_nav_bar_projects_post(profile, team_directory_profiles_profile_nav_bar_projects_post_request)
Create nav bar project

Add a project to the navigation bar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile_nav_bar_projects_post_request** | [**TeamDirectoryProfilesProfileNavBarProjectsPostRequest**](TeamDirectoryProfilesProfileNavBarProjectsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_nav_bar_projects_project_delete

> team_directory_profiles_profile_nav_bar_projects_project_delete(profile, project)
Delete nav bar project

Remove a project from the navigation bar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**project** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_notification_settings_get

> crate::models::GlobalNotificationSettings team_directory_profiles_profile_notification_settings_get(profile, dollar_fields)
Get Space global notification settings for a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::GlobalNotificationSettings**](GlobalNotificationSettings.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_notification_settings_patch

> team_directory_profiles_profile_notification_settings_patch(profile, team_directory_profiles_profile_notification_settings_patch_request)
Set Space global notification settings for a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile_notification_settings_patch_request** | Option<[**TeamDirectoryProfilesProfileNotificationSettingsPatchRequest**](TeamDirectoryProfilesProfileNotificationSettingsPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_patch

> crate::models::TdMemberProfile team_directory_profiles_profile_patch(profile, dollar_fields, team_directory_profiles_profile_patch_request)
Update profile

Update a profile. Optional parameters will be ignored when null and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**team_directory_profiles_profile_patch_request** | Option<[**TeamDirectoryProfilesProfilePatchRequest**](TeamDirectoryProfilesProfilePatchRequest.md)> |  |  |

### Return type

[**crate::models::TdMemberProfile**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_permanent_tokens_current_delete

> team_directory_profiles_profile_permanent_tokens_current_delete(profile)
Delete current permanent token

Delete personal token of the given profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_permanent_tokens_get

> crate::models::TeamDirectoryProfilesProfilePermanentTokensGet200Response team_directory_profiles_profile_permanent_tokens_get(profile, dollar_skip, dollar_top, dollar_fields)
Get all permanent tokens

Get personal tokens used to access the current organization for the given profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryProfilesProfilePermanentTokensGet200Response**](_team_directory_profiles__profile__permanent_tokens_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_permanent_tokens_post

> crate::models::TeamDirectoryProfilesProfilePermanentTokensPost200Response team_directory_profiles_profile_permanent_tokens_post(profile, applications_application_permanent_tokens_post_request, dollar_fields)
Create permanent token

Create a personal token for the given profile that can be used to access the current organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**applications_application_permanent_tokens_post_request** | [**ApplicationsApplicationPermanentTokensPostRequest**](ApplicationsApplicationPermanentTokensPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryProfilesProfilePermanentTokensPost200Response**](_team_directory_profiles__profile__permanent_tokens_post_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_permanent_tokens_token_id_delete

> team_directory_profiles_profile_permanent_tokens_token_id_delete(profile, token_id)
Delete permanent token

Delete a specific personal token used to access the current organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**token_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_permanent_tokens_token_id_patch

> team_directory_profiles_profile_permanent_tokens_token_id_patch(profile, token_id, applications_application_permanent_tokens_token_id_patch_request)
Update permanent token

Update an existing personal token used to access the current organization. The name and/or scope of the personal token can be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**token_id** | **String** |  | [required] |
**applications_application_permanent_tokens_token_id_patch_request** | Option<[**ApplicationsApplicationPermanentTokensTokenIdPatchRequest**](ApplicationsApplicationPermanentTokensTokenIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_reactivate_patch

> crate::models::TdMemberProfile team_directory_profiles_profile_reactivate_patch(profile, dollar_fields, team_directory_profiles_profile_reactivate_patch_request)
Reactivate user profile

Reactivate a user profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**team_directory_profiles_profile_reactivate_patch_request** | Option<[**TeamDirectoryProfilesProfileReactivatePatchRequest**](TeamDirectoryProfilesProfileReactivatePatchRequest.md)> |  |  |

### Return type

[**crate::models::TdMemberProfile**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_restore_patch

> crate::models::TdMemberProfile team_directory_profiles_profile_restore_patch(profile, dollar_fields)
Restore suspended user profile

Restore a suspended user profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdMemberProfile**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_settings_get

> crate::models::SettingsValue team_directory_profiles_profile_settings_get(profile, dollar_fields)
Get Space personalization data for a profile

This endpoint will return profile information and Space personalisation data such as projects in the navigation bar, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::SettingsValue**](SettingsValue.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_settings_patch

> team_directory_profiles_profile_settings_patch(profile, team_directory_profiles_profile_settings_patch_request)
Set Space personalization data for a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile_settings_patch_request** | Option<[**TeamDirectoryProfilesProfileSettingsPatchRequest**](TeamDirectoryProfilesProfileSettingsPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_spoken_languages_get

> Vec<crate::models::TdProfileLanguage> team_directory_profiles_profile_spoken_languages_get(profile, dollar_fields)
Get all spoken languages

Get spoken language of a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdProfileLanguage>**](TD_ProfileLanguage.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_spoken_languages_language_delete

> team_directory_profiles_profile_spoken_languages_language_delete(profile, language)
Delete spoken language

Delete spoken language for a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**language** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_spoken_languages_post

> crate::models::TdProfileLanguage team_directory_profiles_profile_spoken_languages_post(profile, team_directory_profiles_profile_spoken_languages_post_request, dollar_fields)
Create spoken language

Update spoken language for a profile. Optionally, firstName and lastName can be specified to add a localized name to the profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile_spoken_languages_post_request** | [**TeamDirectoryProfilesProfileSpokenLanguagesPostRequest**](TeamDirectoryProfilesProfileSpokenLanguagesPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdProfileLanguage**](TD_ProfileLanguage.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_ssh_keys_fingerprint_delete

> team_directory_profiles_profile_ssh_keys_fingerprint_delete(profile, fingerprint)
Remove association between SSH key and profile

Remove association between the profile and the SSH public key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**fingerprint** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_ssh_keys_get

> Vec<crate::models::SshKeyData> team_directory_profiles_profile_ssh_keys_get(profile, dollar_fields)
Get all SSH keys

List SSH public keys associated with the profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::SshKeyData>**](SshKeyData.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_ssh_keys_post

> team_directory_profiles_profile_ssh_keys_post(profile, team_directory_profiles_profile_gpg_keys_post_request)
Associate SSH key with profile

Associate an SSH public key with the profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile_gpg_keys_post_request** | [**TeamDirectoryProfilesProfileGpgKeysPostRequest**](TeamDirectoryProfilesProfileGpgKeysPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_suspend_patch

> crate::models::TdMemberProfile team_directory_profiles_profile_suspend_patch(profile, dollar_fields, team_directory_profiles_profile_suspend_patch_request)
Suspend user profile

Suspend a user profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**team_directory_profiles_profile_suspend_patch_request** | Option<[**TeamDirectoryProfilesProfileSuspendPatchRequest**](TeamDirectoryProfilesProfileSuspendPatchRequest.md)> |  |  |

### Return type

[**crate::models::TdMemberProfile**](TD_MemberProfile.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_timezone_get

> crate::models::ATimeZone team_directory_profiles_profile_timezone_get(profile, dollar_fields)
Get timezone

Get profile timezone. Returns profile's working hours timezone, location timezone or device timezone, whichever is present first in this list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ATimeZone**](ATimeZone.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_working_days_get

> crate::models::TeamDirectoryProfilesProfileWorkingDaysGet200Response team_directory_profiles_profile_working_days_get(profile, dollar_skip, dollar_top, dollar_fields)
Query working days for a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryProfilesProfileWorkingDaysGet200Response**](_team_directory_profiles__profile__working_days_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_working_days_post

> crate::models::TdWorkingDays team_directory_profiles_profile_working_days_post(profile, team_directory_profiles_profile_working_days_post_request, dollar_fields)
Add working days

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**team_directory_profiles_profile_working_days_post_request** | [**TeamDirectoryProfilesProfileWorkingDaysPostRequest**](TeamDirectoryProfilesProfileWorkingDaysPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdWorkingDays**](TD_WorkingDays.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_working_days_working_days_id_delete

> team_directory_profiles_profile_working_days_working_days_id_delete(profile, working_days_id)
Delete working days

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**working_days_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_profile_working_days_working_days_id_patch

> crate::models::TdWorkingDays team_directory_profiles_profile_working_days_working_days_id_patch(profile, working_days_id, team_directory_profiles_profile_working_days_post_request, dollar_fields)
Update working days

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |
**working_days_id** | **String** |  | [required] |
**team_directory_profiles_profile_working_days_post_request** | [**TeamDirectoryProfilesProfileWorkingDaysPostRequest**](TeamDirectoryProfilesProfileWorkingDaysPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdWorkingDays**](TD_WorkingDays.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_widget_settings_widget_get

> crate::models::WidgetSettingsRecord team_directory_profiles_widget_settings_widget_get(widget, dollar_fields)
Get widget setting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**widget** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::WidgetSettingsRecord**](WidgetSettingsRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_widget_settings_widget_patch

> team_directory_profiles_widget_settings_widget_patch(widget, team_directory_profiles_widget_settings_widget_patch_request)
Update widget setting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**widget** | **String** |  | [required] |
**team_directory_profiles_widget_settings_widget_patch_request** | [**TeamDirectoryProfilesWidgetSettingsWidgetPatchRequest**](TeamDirectoryProfilesWidgetSettingsWidgetPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_profiles_working_days_get

> crate::models::TeamDirectoryProfilesWorkingDaysGet200Response team_directory_profiles_working_days_get(dollar_skip, dollar_top, profiles, since, till, dollar_fields)
Query all working days

Returns pairs of profiles and their working days. If several working days settings are defined for the same profile then several pairs are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**profiles** | Option<[**Vec<String>**](String.md)> |  |  |[default to []]
**since** | Option<**String**> |  |  |
**till** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryProfilesWorkingDaysGet200Response**](_team_directory_profiles_working_days_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_roles_get

> Vec<crate::models::TdRole> team_directory_roles_get(query, with_archived, dollar_fields)
Get all roles

Get/search all roles. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |[default to ]
**with_archived** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdRole>**](TD_Role.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_roles_id_delete

> team_directory_roles_id_delete(id)
Archive role

Archive a role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_roles_id_get

> crate::models::TdRole team_directory_roles_id_get(id, dollar_fields)
Get role

Get a role by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdRole**](TD_Role.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_roles_id_patch

> crate::models::TdRole team_directory_roles_id_patch(id, dollar_fields, team_directory_roles_id_patch_request)
Update role

Update a role. Optional parameters will be ignored when null and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**team_directory_roles_id_patch_request** | Option<[**TeamDirectoryRolesIdPatchRequest**](TeamDirectoryRolesIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::TdRole**](TD_Role.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_roles_id_restore_post

> crate::models::TdRole team_directory_roles_id_restore_post(id, dollar_fields)
Restore role

Restore an archived role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdRole**](TD_Role.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_roles_post

> crate::models::TdRole team_directory_roles_post(team_directory_roles_post_request, dollar_fields)
Create role

Create a role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_directory_roles_post_request** | [**TeamDirectoryRolesPostRequest**](TeamDirectoryRolesPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdRole**](TD_Role.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_stats_get

> crate::models::TdStats team_directory_stats_get(team_id, location_id, role_id, dollar_fields)
Get all stats

Get statistics of total members, as well as members per location, role, and team. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | Option<**String**> |  |  |
**location_id** | Option<**String**> |  |  |
**role_id** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdStats**](TD_Stats.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_teams_get

> crate::models::ChatsChannelsChannelSubscribersTeamsGet200Response team_directory_teams_get(dollar_skip, dollar_top, query, with_archived, dollar_fields)
Get all teams

Get or search all teams. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**query** | Option<**String**> |  |  |[default to ]
**with_archived** | Option<**bool**> |  |  |[default to false]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelSubscribersTeamsGet200Response**](_chats_channels__channel__subscribers_teams_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_teams_id_cancel_disbanding_post

> team_directory_teams_id_cancel_disbanding_post(id)
Cancel team disbanding

Cancel disbanding a team and restore its members

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_teams_id_delete

> Vec<crate::models::TdTeam> team_directory_teams_id_delete(id, dollar_fields)
Archive team

Archive a team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdTeam>**](TD_Team.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_teams_id_direct_members_get

> crate::models::ChatsChannelsChannelSubscribersUsersGet200Response team_directory_teams_id_direct_members_get(id, dollar_skip, dollar_top, query, dollar_fields)
Get all direct members

Get or search direct members of a given team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**query** | Option<**String**> |  |  |[default to ]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ChatsChannelsChannelSubscribersUsersGet200Response**](_chats_channels__channel__subscribers_users_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_teams_id_disband_delete

> Vec<crate::models::TdTeam> team_directory_teams_id_disband_delete(id, dollar_fields)
Disband team

Disband a team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TdTeam>**](TD_Team.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_teams_id_get

> crate::models::TdTeam team_directory_teams_id_get(id, dollar_fields)
Get team

Get a team by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdTeam**](TD_Team.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_teams_id_patch

> crate::models::TdTeam team_directory_teams_id_patch(id, dollar_fields, team_directory_teams_id_patch_request)
Update team

Update a team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |
**team_directory_teams_id_patch_request** | Option<[**TeamDirectoryTeamsIdPatchRequest**](TeamDirectoryTeamsIdPatchRequest.md)> |  |  |

### Return type

[**crate::models::TdTeam**](TD_Team.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_teams_id_restore_post

> crate::models::TdTeam team_directory_teams_id_restore_post(id, dollar_fields)
Restore team

Restore an archived team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdTeam**](TD_Team.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_teams_post

> crate::models::TdTeam team_directory_teams_post(team_directory_teams_post_request, dollar_fields)
Create team

Create a new team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_directory_teams_post_request** | [**TeamDirectoryTeamsPostRequest**](TeamDirectoryTeamsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TdTeam**](TD_Team.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_directory_teams_sync_batch_get

> crate::models::TeamDirectoryTeamsSyncBatchGet200Response team_directory_teams_sync_batch_get(batch_info, dollar_fields)
Get sync batch

Get teams for synchronization with third-party system. Teams with etag greater than specified value are returned. Read more in the [documentation](https://www.jetbrains.com/help/space/sync-api.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_info** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamDirectoryTeamsSyncBatchGet200Response**](_team_directory_teams_sync_batch_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## time_tracking_items_get

> crate::models::TimeTrackingItemsGet200Response time_tracking_items_get(subject, dollar_skip, dollar_top, dollar_fields)
Get all items

Get items for subject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** |  | [required] |
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TimeTrackingItemsGet200Response**](_time_tracking_items_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## time_tracking_items_item_id_delete

> time_tracking_items_item_id_delete(item_id)
Delete item

Delete single work item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## time_tracking_items_item_id_patch

> time_tracking_items_item_id_patch(item_id, time_tracking_items_item_id_patch_request)
Update item

Update a single work item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** |  | [required] |
**time_tracking_items_item_id_patch_request** | Option<[**TimeTrackingItemsItemIdPatchRequest**](TimeTrackingItemsItemIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## time_tracking_items_post

> crate::models::TimeTrackingItem time_tracking_items_post(time_tracking_items_post_request, dollar_fields)
Create item

Create work item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_tracking_items_post_request** | [**TimeTrackingItemsPostRequest**](TimeTrackingItemsPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TimeTrackingItem**](TimeTrackingItem.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## todo_get

> crate::models::TodoGet200Response todo_get(dollar_skip, dollar_top, open, from, till, dollar_fields)
Get all to-do items

Get all To-Do items that match given parameters. Parameters are applied as 'AND' filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**open** | Option<**bool**> |  |  |
**from** | Option<**String**> |  |  |
**till** | Option<**String**> |  |  |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TodoGet200Response**](_todo_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## todo_id_delete

> todo_id_delete(id)
Delete to-do item

Delete an existing To-Do item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## todo_id_patch

> todo_id_patch(id, todo_id_patch_request)
Update to-do item

Update an existing To-Do item. Optional parameters will be ignored when not specified and updated otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**todo_id_patch_request** | Option<[**TodoIdPatchRequest**](TodoIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## todo_post

> crate::models::TodoItemRecord todo_post(todo_post_request, dollar_fields)
Create to-do item

Create a new To-Do item, with an optional due date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**todo_post_request** | [**TodoPostRequest**](TodoPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TodoItemRecord**](TodoItemRecord.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trusted_certificates_get

> Vec<crate::models::TrustedCertificate> trusted_certificates_get(dollar_fields)
Get all trusted certificates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TrustedCertificate>**](TrustedCertificate.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trusted_certificates_id_delete

> trusted_certificates_id_delete(id)
Delete trusted certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trusted_certificates_id_patch

> trusted_certificates_id_patch(id, trusted_certificates_id_patch_request)
Update trusted certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**trusted_certificates_id_patch_request** | Option<[**TrustedCertificatesIdPatchRequest**](TrustedCertificatesIdPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trusted_certificates_info_get

> crate::models::CertificateInfo trusted_certificates_info_get(data, dollar_fields)
Get certificate info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CertificateInfo**](CertificateInfo.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trusted_certificates_post

> crate::models::TrustedCertificate trusted_certificates_post(trusted_certificates_post_request, dollar_fields)
Create trusted certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trusted_certificates_post_request** | [**TrustedCertificatesPostRequest**](TrustedCertificatesPostRequest.md) |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TrustedCertificate**](TrustedCertificate.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfurls_block_unfurl_global_post

> unfurls_block_unfurl_global_post(unfurls_block_unfurl_post_request)
Block link unfurling for organization

Block link unfurling for organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unfurls_block_unfurl_post_request** | [**UnfurlsBlockUnfurlPostRequest**](UnfurlsBlockUnfurlPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfurls_block_unfurl_post

> unfurls_block_unfurl_post(unfurls_block_unfurl_post_request)
Block link unfurling

Block link unfurling

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unfurls_block_unfurl_post_request** | [**UnfurlsBlockUnfurlPostRequest**](UnfurlsBlockUnfurlPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfurls_check_blocked_post

> bool unfurls_check_blocked_post(unfurls_check_blocked_post_request)
Check if unfurl is blocked

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unfurls_check_blocked_post_request** | [**UnfurlsCheckBlockedPostRequest**](UnfurlsCheckBlockedPostRequest.md) |  | [required] |

### Return type

**bool**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfurls_list_blocked_get

> crate::models::UnfurlsListBlockedGet200Response unfurls_list_blocked_get(dollar_skip, dollar_top, dollar_fields)
List blocked unfurls

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dollar_skip** | Option<**String**> |  |  |
**dollar_top** | Option<**i32**> |  |  |[default to 100]
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::UnfurlsListBlockedGet200Response**](_unfurls_list_blocked_get_200_response.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfurls_unblock_unfurl_global_post

> unfurls_unblock_unfurl_global_post(unfurls_block_unfurl_post_request)
Unblock link unfurling for organization

Unblock link unfurling for organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unfurls_block_unfurl_post_request** | [**UnfurlsBlockUnfurlPostRequest**](UnfurlsBlockUnfurlPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfurls_unblock_unfurl_post

> unfurls_unblock_unfurl_post(unfurls_block_unfurl_post_request)
Unblock link unfurling

Unblock link unfurling

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unfurls_block_unfurl_post_request** | [**UnfurlsBlockUnfurlPostRequest**](UnfurlsBlockUnfurlPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uploads_chat_public_url_channel_message_attachment_id_get

> String uploads_chat_public_url_channel_message_attachment_id_get(channel, message, attachment_id)
Get public url

Returns a URL that can be used to access attachment file without authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**message** | **String** |  | [required] |
**attachment_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uploads_image_id_get

> crate::models::ImageAttachmentMeta uploads_image_id_get(id, dollar_fields)
Get image attachment metadata

Get meta information for a previously uploaded image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dollar_fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ImageAttachmentMeta**](ImageAttachmentMeta.md)

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uploads_post

> String uploads_post(uploads_post_request)
Create upload

Request a URL that can be used to upload an attachment. An attachment can be uploaded to the URL that is returned, by making a PUT request that has a proper content-type header and the attachment data as the request body. The PUT request returns a string that is an id of the uploaded attachment. The attachment id can be passed to other API methods where this attachment needs to be used. Attachments are available for download at `/d/{attachmentId}`. The 'storagePrefix' parameter can be one of file, maps, emoji or attachments. The 'mediaType' parameter can be omitted for all uploads. For image uploads that need to be resized automatically for specific use, such as chat stickers or emoji, use one of `chat-image-attachment`, `chat-sticker`, `emoji`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uploads_post_request** | [**UploadsPostRequest**](UploadsPostRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[httpBearer](../README.md#httpBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

