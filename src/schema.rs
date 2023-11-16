diesel::table! {
    BSSyncronyMetadata (uuid) {
        uuid -> Nullable<Text>,
        value -> Nullable<Binary>,
    }
}

diesel::table! {
    Meta (key) {
        key -> Nullable<Text>,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    TMArea (uuid) {
        uuid -> Nullable<Text>,
        title -> Nullable<Text>,
        visible -> Nullable<Integer>,
        index -> Nullable<Integer>,
        cachedTags -> Nullable<Binary>,
        experimental -> Nullable<Binary>,
    }
}

diesel::table! {
    TMChecklistItem (uuid) {
        uuid -> Nullable<Text>,
        userModificationDate -> Nullable<Float>,
        creationDate -> Nullable<Float>,
        title -> Nullable<Text>,
        status -> Nullable<Integer>,
        stopDate -> Nullable<Float>,
        index -> Nullable<Integer>,
        task -> Nullable<Text>,
        leavesTombstone -> Nullable<Integer>,
        experimental -> Nullable<Binary>,
    }
}

diesel::table! {
    TMCommand (uuid) {
        uuid -> Nullable<Text>,
        creationDate -> Nullable<Float>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        info -> Nullable<Binary>,
    }
}

diesel::table! {
    TMContact (uuid) {
        uuid -> Nullable<Text>,
        displayName -> Nullable<Text>,
        firstName -> Nullable<Text>,
        lastName -> Nullable<Text>,
        emails -> Nullable<Text>,
        appleAddressBookId -> Nullable<Text>,
        index -> Nullable<Integer>,
    }
}

diesel::table! {
    TMMetaItem (uuid) {
        uuid -> Nullable<Text>,
        value -> Nullable<Binary>,
    }
}

diesel::table! {
    TMSettings (uuid) {
        uuid -> Nullable<Text>,
        logInterval -> Nullable<Integer>,
        manualLogDate -> Nullable<Float>,
        groupTodayByParent -> Nullable<Integer>,
        uriSchemeAuthenticationToken -> Nullable<Text>,
        experimental -> Nullable<Binary>,
    }
}

diesel::table! {
    TMTag (uuid) {
        uuid -> Nullable<Text>,
        title -> Nullable<Text>,
        shortcut -> Nullable<Text>,
        usedDate -> Nullable<Float>,
        parent -> Nullable<Text>,
        index -> Nullable<Integer>,
        experimental -> Nullable<Binary>,
    }
}

diesel::table! {
    TMTask (uuid) {
        uuid -> Nullable<Text>,
        leavesTombstone -> Nullable<Integer>,
        creationDate -> Nullable<Float>,
        userModificationDate -> Nullable<Float>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        status -> Nullable<Integer>,
        stopDate -> Nullable<Float>,
        trashed -> Nullable<Integer>,
        title -> Nullable<Text>,
        notes -> Nullable<Text>,
        notesSync -> Nullable<Integer>,
        cachedTags -> Nullable<Binary>,
        start -> Nullable<Integer>,
        startDate -> Nullable<Integer>,
        startBucket -> Nullable<Integer>,
        reminderTime -> Nullable<Integer>,
        lastReminderInteractionDate -> Nullable<Float>,
        deadline -> Nullable<Integer>,
        deadlineSuppressionDate -> Nullable<Integer>,
        t2_deadlineOffset -> Nullable<Integer>,
        index -> Nullable<Integer>,
        todayIndex -> Nullable<Integer>,
        todayIndexReferenceDate -> Nullable<Integer>,
        area -> Nullable<Text>,
        project -> Nullable<Text>,
        heading -> Nullable<Text>,
        contact -> Nullable<Text>,
        untrashedLeafActionsCount -> Nullable<Integer>,
        openUntrashedLeafActionsCount -> Nullable<Integer>,
        checklistItemsCount -> Nullable<Integer>,
        openChecklistItemsCount -> Nullable<Integer>,
        rt1_repeatingTemplate -> Nullable<Text>,
        rt1_recurrenceRule -> Nullable<Binary>,
        rt1_instanceCreationStartDate -> Nullable<Integer>,
        rt1_instanceCreationPaused -> Nullable<Integer>,
        rt1_instanceCreationCount -> Nullable<Integer>,
        rt1_afterCompletionReferenceDate -> Nullable<Integer>,
        rt1_nextInstanceStartDate -> Nullable<Integer>,
        experimental -> Nullable<Binary>,
        repeater -> Nullable<Binary>,
        repeaterMigrationDate -> Nullable<Float>,
    }
}

diesel::table! {
    TMTombstone (uuid) {
        uuid -> Nullable<Text>,
        deletionDate -> Nullable<Float>,
        deletedObjectUUID -> Nullable<Text>,
    }
}

diesel::table! {
    ThingsTouch_ExtensionCommandStore_Commands (id) {
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        body -> Nullable<Binary>,
    }
}

diesel::table! {
    ThingsTouch_ExtensionCommandStore_Meta (key) {
        key -> Nullable<Text>,
        value -> Nullable<Binary>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    BSSyncronyMetadata,
    Meta,
    TMArea,
    TMChecklistItem,
    TMCommand,
    TMContact,
    TMMetaItem,
    TMSettings,
    TMTag,
    TMTask,
    TMTombstone,
    ThingsTouch_ExtensionCommandStore_Commands,
    ThingsTouch_ExtensionCommandStore_Meta,
);
