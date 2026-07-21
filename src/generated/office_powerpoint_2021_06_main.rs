//! Auto-generated from `schemas_microsoft_com_office_powerpoint_2021_06_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/powerpoint/2021/06/main` (prefix `p216`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/powerpoint/2021/06/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "p216";

/// Metadata for a schema element.
#[derive(Debug, Clone, Copy)]
pub struct ElementInfo {
    pub class_name: &'static str,
    pub local_name: &'static str,
    pub prefix: &'static str,
    pub namespace_uri: &'static str,
    pub is_leaf: bool,
    pub is_leaf_text: bool,
    pub attributes: &'static [AttributeInfo],
    pub children: &'static [ChildInfo],
}

/// Schema attribute metadata.
#[derive(Debug, Clone, Copy)]
pub struct AttributeInfo {
    pub qname: &'static str,
    pub property_name: Option<&'static str>,
    pub type_name: &'static str,
}

/// Schema child-element metadata.
#[derive(Debug, Clone, Copy)]
pub struct ChildInfo {
    pub name: &'static str,
    pub property_name: Option<&'static str>,
}

static ATTRS_TASK_HISTORY_DETAILS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_TASK_HISTORY_DETAILS: &[ChildInfo] = &[
    ChildInfo { name: "p216:CT_TaskHistory/p216:history", property_name: Some("TaskHistory") },
    ChildInfo { name: "p:CT_ExtensionList/p216:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_COMMENT_ANCHOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Extension/p:ext", property_name: None },
];
static ATTRS_ATRBTN_TASK_ASSIGN_UNASSIGN_USER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":authorId", property_name: None, type_name: "StringValue" },
];
static ATTRS_ASGN_TASK_ASSIGN_UNASSIGN_USER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":authorId", property_name: None, type_name: "StringValue" },
];
static ATTRS_UN_ASGN_TASK_ASSIGN_UNASSIGN_USER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":authorId", property_name: None, type_name: "StringValue" },
];
static CHILDREN_TASK_ANCHOR: &[ChildInfo] = &[
    ChildInfo { name: "p216:CT_CommentAnchor/p216:comment", property_name: Some("CommentAnchor") },
    ChildInfo { name: "p:CT_ExtensionList/p216:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TASK_TITLE_EVENT_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "StringValue" },
];
static ATTRS_TASK_SCHEDULE_EVENT_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":stDt", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: ":endDt", property_name: None, type_name: "DateTimeValue" },
];
static ATTRS_TASK_PROGRESS_EVENT_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "Int32Value" },
];
static ATTRS_TASK_PRIORITY_RECORD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "Int32Value" },
];
static ATTRS_TASK_UNDO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static ATTRS_TASK_HISTORY_EVENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":time", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_TASK_HISTORY_EVENT: &[ChildInfo] = &[
    ChildInfo { name: "p216:CT_TaskAssignUnassignUser/p216:atrbtn", property_name: Some("AtrbtnTaskAssignUnassignUser") },
    ChildInfo { name: "p216:CT_TaskAnchor/p216:anchr", property_name: Some("TaskAnchor") },
    ChildInfo { name: "p216:CT_TaskAssignUnassignUser/p216:asgn", property_name: None },
    ChildInfo { name: "p216:CT_TaskAssignUnassignUser/p216:unAsgn", property_name: None },
    ChildInfo { name: "p:CT_Empty/p216:add", property_name: None },
    ChildInfo { name: "p216:CT_TaskTitleEventInfo/p216:title", property_name: None },
    ChildInfo { name: "p216:CT_TaskScheduleEventInfo/p216:date", property_name: None },
    ChildInfo { name: "p216:CT_TaskProgressEventInfo/p216:pcntCmplt", property_name: None },
    ChildInfo { name: "p216:CT_TaskPriorityRecord/p216:pri", property_name: None },
    ChildInfo { name: "p:CT_Empty/p216:unasgnAll", property_name: None },
    ChildInfo { name: "p216:CT_TaskUndo/p216:undo", property_name: None },
    ChildInfo { name: "p216:CT_TaskUnknownRecord/p216:unknown", property_name: None },
    ChildInfo { name: "p:CT_ExtensionList/p216:extLst", property_name: None },
];
static CHILDREN_TASK_HISTORY: &[ChildInfo] = &[
    ChildInfo { name: "p216:CT_TaskHistoryEvent/p216:event", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "TaskHistoryDetails", local_name: "taskHistoryDetails", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TASK_HISTORY_DETAILS, children: CHILDREN_TASK_HISTORY_DETAILS },
    ElementInfo { class_name: "CommentAnchor", local_name: "comment", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMMENT_ANCHOR, children: &[] },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "AtrbtnTaskAssignUnassignUser", local_name: "atrbtn", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ATRBTN_TASK_ASSIGN_UNASSIGN_USER, children: &[] },
    ElementInfo { class_name: "AsgnTaskAssignUnassignUser", local_name: "asgn", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ASGN_TASK_ASSIGN_UNASSIGN_USER, children: &[] },
    ElementInfo { class_name: "UnAsgnTaskAssignUnassignUser", local_name: "unAsgn", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UN_ASGN_TASK_ASSIGN_UNASSIGN_USER, children: &[] },
    ElementInfo { class_name: "TaskAnchor", local_name: "anchr", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TASK_ANCHOR },
    ElementInfo { class_name: "AddEmpty", local_name: "add", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "UnasgnAllEmpty", local_name: "unasgnAll", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TaskTitleEventInfo", local_name: "title", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TASK_TITLE_EVENT_INFO, children: &[] },
    ElementInfo { class_name: "TaskScheduleEventInfo", local_name: "date", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TASK_SCHEDULE_EVENT_INFO, children: &[] },
    ElementInfo { class_name: "TaskProgressEventInfo", local_name: "pcntCmplt", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TASK_PROGRESS_EVENT_INFO, children: &[] },
    ElementInfo { class_name: "TaskPriorityRecord", local_name: "pri", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TASK_PRIORITY_RECORD, children: &[] },
    ElementInfo { class_name: "TaskUndo", local_name: "undo", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TASK_UNDO, children: &[] },
    ElementInfo { class_name: "TaskUnknownRecord", local_name: "unknown", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TaskHistoryEvent", local_name: "event", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TASK_HISTORY_EVENT, children: CHILDREN_TASK_HISTORY_EVENT },
    ElementInfo { class_name: "TaskHistory", local_name: "history", prefix: "p216", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TASK_HISTORY },
];

/// Look up element metadata by class name.
pub fn info_by_class(class_name: &str) -> Option<&'static ElementInfo> {
    ELEMENTS.iter().find(|e| e.class_name == class_name)
}

/// Look up element metadata by local name (first match).
pub fn info_by_local_name(local_name: &str) -> Option<&'static ElementInfo> {
    ELEMENTS.iter().find(|e| e.local_name == local_name)
}

/// Create an empty element by its schema class name (e.g. `"Paragraph"`).
pub fn create(class_name: &str) -> Option<OpenXmlElement> {
    let info = info_by_class(class_name)?;
    Some(OpenXmlElement::new(info.prefix, info.namespace_uri, info.local_name))
}

// ---------------------------------------------------------------------------
// Typed constructors
// ---------------------------------------------------------------------------

/// Create a `<p216:taskHistoryDetails>` element (`TaskHistoryDetails`).
pub fn task_history_details(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "taskHistoryDetails").with_children(children)
}

/// Create a `<p216:comment>` element (`CommentAnchor`).
pub fn comment_anchor() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "comment")
}

/// Create a `<p216:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p216:atrbtn>` element (`AtrbtnTaskAssignUnassignUser`).
pub fn atrbtn_task_assign_unassign_user() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "atrbtn")
}

/// Create a `<p216:asgn>` element (`AsgnTaskAssignUnassignUser`).
pub fn asgn_task_assign_unassign_user() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "asgn")
}

/// Create a `<p216:unAsgn>` element (`UnAsgnTaskAssignUnassignUser`).
pub fn un_asgn_task_assign_unassign_user() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "unAsgn")
}

/// Create a `<p216:anchr>` element (`TaskAnchor`).
pub fn task_anchor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "anchr").with_children(children)
}

/// Create a `<p216:add>` element (`AddEmpty`).
pub fn add_empty() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "add")
}

/// Create a `<p216:unasgnAll>` element (`UnasgnAllEmpty`).
pub fn unasgn_all_empty() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "unasgnAll")
}

/// Create a `<p216:title>` element (`TaskTitleEventInfo`).
pub fn task_title_event_info() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "title")
}

/// Create a `<p216:date>` element (`TaskScheduleEventInfo`).
pub fn task_schedule_event_info() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "date")
}

/// Create a `<p216:pcntCmplt>` element (`TaskProgressEventInfo`).
pub fn task_progress_event_info() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "pcntCmplt")
}

/// Create a `<p216:pri>` element (`TaskPriorityRecord`).
pub fn task_priority_record() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "pri")
}

/// Create a `<p216:undo>` element (`TaskUndo`).
pub fn task_undo() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "undo")
}

/// Create a `<p216:unknown>` element (`TaskUnknownRecord`).
pub fn task_unknown_record() -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "unknown")
}

/// Create a `<p216:event>` element (`TaskHistoryEvent`).
pub fn task_history_event(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "event").with_children(children)
}

/// Create a `<p216:history>` element (`TaskHistory`).
pub fn task_history(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p216", NAMESPACE_URI, "history").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 19;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 17;
