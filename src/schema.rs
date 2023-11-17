diesel::table! {
    #[sql_name="BSSyncronyMetadata"]
    bs_syncrony_metadata (uuid) {
        uuid -> Nullable<Text>,
        value -> Nullable<Binary>,
    }
}

diesel::table! {
    #[sql_name="Meta"]
    meta (key) {
        key -> Nullable<Text>,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    #[sql_name="TMArea"]
    tm_area (uuid) {
        uuid -> Nullable<Text>,
        title -> Nullable<Text>,
        visible -> Nullable<Integer>,
        index -> Nullable<Integer>,
        #[sql_name = "cachedTags"]
        cached_tags -> Nullable<Binary>,
        experimental -> Nullable<Binary>,
    }
}

diesel::table! {
    #[sql_name="TMChecklistItem"]
    tm_checklist_item (uuid) {
        uuid -> Nullable<Text>,
        #[sql_name = "userModificationDate"]
        user_modification_date -> Nullable<Float>,
        #[sql_name = "creationDate"]
        creation_date -> Nullable<Float>,
        title -> Nullable<Text>,
        status -> Nullable<Integer>,
        #[sql_name = "stopDate"]
        stop_date -> Nullable<Float>,
        index -> Nullable<Integer>,
        task -> Nullable<Text>,
        #[sql_name = "leavesTombstone"]
        leaves_tombstone -> Nullable<Integer>,
        experimental -> Nullable<Binary>,
    }
}

diesel::table! {
    #[sql_name="TMCommand"]
    tm_command (uuid) {
        uuid -> Nullable<Text>,
        #[sql_name = "creationDate"]
        creation_date -> Nullable<Float>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        info -> Nullable<Binary>,
    }
}

diesel::table! {
    #[sql_name="TMContact"]
    tm_contact (uuid) {
        uuid -> Nullable<Text>,
        #[sql_name = "displayName"]
        display_name -> Nullable<Text>,
        #[sql_name = "firstName"]
        first_name -> Nullable<Text>,
        #[sql_name = "lastName"]
        last_name -> Nullable<Text>,
        emails -> Nullable<Text>,
        #[sql_name = "appleAddressBookId"]
        apple_address_book_id -> Nullable<Text>,
        index -> Nullable<Integer>,
    }
}

diesel::table! {
    #[sql_name="TMMetaItem"]
    tm_meta_item (uuid) {
        uuid -> Nullable<Text>,
        value -> Nullable<Binary>,
    }
}

diesel::table! {
    #[sql_name="TMSettings"]
    tm_settings (uuid) {
        uuid -> Nullable<Text>,
        #[sql_name = "log_interval"]
        logInterval -> Nullable<Integer>,
        #[sql_name = "manualLogDate"]
        manual_log_date -> Nullable<Float>,
        #[sql_name = "groupTodayByParent"]
        group_today_by_parent -> Nullable<Integer>,
        #[sql_name = "uriSchemeAuthenticationToken"]
        uri_scheme_authentication_token -> Nullable<Text>,
        experimental -> Nullable<Binary>,
    }
}

diesel::table! {
    #[sql_name="TMTag"]
    tm_tag (uuid) {
        uuid -> Nullable<Text>,
        title -> Nullable<Text>,
        shortcut -> Nullable<Text>,
        #[sql_name = "usedDate"]
        used_date -> Nullable<Float>,
        parent -> Nullable<Text>,
        index -> Nullable<Integer>,
        experimental -> Nullable<Binary>,
    }
}

diesel::table! {
    #[sql_name="TMTask"]
    tm_task (uuid) {
        uuid -> Nullable<Text>,
        #[sql_name = "leavesTombstone"]
        leaves_tombstone -> Nullable<Integer>,
        #[sql_name = "creationDate"]
        creation_date -> Nullable<Float>,
        #[sql_name = "userModificationDate"]
        user_modification_date -> Nullable<Float>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        status -> Nullable<Integer>,
        #[sql_name = "stopDate"]
        stop_date -> Nullable<Float>,
        trashed -> Nullable<Integer>,
        title -> Nullable<Text>,
        notes -> Nullable<Text>,
        #[sql_name = "notesSync"]
        notes_sync -> Nullable<Integer>,
        #[sql_name = "cachedTags"]
        cached_tags -> Nullable<Binary>,
        start -> Nullable<Integer>,
        #[sql_name = "startDate"]
        start_date -> Nullable<Integer>,
        #[sql_name = "startBucket"]
        start_bucket -> Nullable<Integer>,
        #[sql_name = "reminderTime"]
        reminder_time -> Nullable<Integer>,
        #[sql_name = "lastReminderInteractionDate"]
        last_reminder_interaction_date -> Nullable<Float>,
        deadline -> Nullable<Integer>,
        #[sql_name = "deadlineSuppressionDate"]
        deadline_suppression_date -> Nullable<Integer>,
        #[sql_name = "t2_deadlineOffset"]
        t2_deadline_offset -> Nullable<Integer>,
        index -> Nullable<Integer>,
        #[sql_name = "todayIndex"]
        today_index -> Nullable<Integer>,
        #[sql_name = "todayIndexReferenceDate"]
        today_index_reference_date -> Nullable<Integer>,
        area -> Nullable<Text>,
        project -> Nullable<Text>,
        heading -> Nullable<Text>,
        contact -> Nullable<Text>,
        #[sql_name = "untrashedLeafActionsCount"]
        untrashed_leaf_actions_count -> Nullable<Integer>,
        #[sql_name = "openUntrashedLeafActionsCount"]
        open_untrashed_leafActions_count -> Nullable<Integer>,
        #[sql_name = "checklistItemsCount"]
        checklist_items_count -> Nullable<Integer>,
        #[sql_name = "openChecklistItemsCount"]
        open_checklist_items_count -> Nullable<Integer>,
        #[sql_name = "rt1_repeatingTemplate"]
        rt1_repeating_template -> Nullable<Text>,
        #[sql_name = "rt1_recurrenceRule"]
        rt1_recurrence_rule -> Nullable<Binary>,
        #[sql_name = "rt1_instanceCreationStartDate"]
        rt1_instance_creation_start_date -> Nullable<Integer>,
        #[sql_name = "rt1_instanceCreationPaused"]
        rt1_instance_creation_paused -> Nullable<Integer>,
        #[sql_name = "rt1_instanceCreationCount"]
        rt1_instance_creation_count -> Nullable<Integer>,
        #[sql_name = "rt1_afterCompletionReferenceDate"]
        rt1_after_completion_reference_date -> Nullable<Integer>,
        #[sql_name = "rt1_nextInstanceStartDate"]
        rt1_next_instance_start_date -> Nullable<Integer>,
        experimental -> Nullable<Binary>,
        repeater -> Nullable<Binary>,
        #[sql_name = "repeaterMigrationDate"]
        repeater_migration_date -> Nullable<Float>,
    }
}

diesel::table! {
    #[sql_name="TMTombstone"]
    tm_tombstone (uuid) {
        uuid -> Nullable<Text>,
        #[sql_name = "deletionDate"]
        deletion_date -> Nullable<Float>,
        #[sql_name = "deletedObjectUUID"]
        deleted_object_uuid -> Nullable<Text>,
    }
}

diesel::table! {
    #[sql_name="ThingsTouch_ExtensionCommandStore_Commands"]
    things_touch_extension_command_store_commands (id) {
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        body -> Nullable<Binary>,
    }
}

diesel::table! {
    #[sql_name="ThingsTouch_ExtensionCommandStore_Meta"]
    things_touch_extension_command_store_meta (key) {
        key -> Nullable<Text>,
        value -> Nullable<Binary>,
    }
}

diesel::table! {
    #[sql_name="TMAreaTag"]
    tm_area_tag (tags) {
        areas -> Text,
        tags -> Text,
    }
}

diesel::table! {
    #[sql_name="TMTaskTag"]
    tm_task_tag (tags) {
        tasks -> Text,
        tags -> Text,
    }
}


diesel::allow_tables_to_appear_in_same_query!(
    bs_syncrony_metadata,
    meta,
    tm_area,
    tm_checklist_item,
    tm_command,
    tm_contact,
    tm_meta_item,
    tm_settings,
    tm_tag,
    tm_task,
    tm_tombstone,
    things_touch_extension_command_store_commands,
    things_touch_extension_command_store_meta,
    tm_area_tag,
    tm_task_tag,
);
