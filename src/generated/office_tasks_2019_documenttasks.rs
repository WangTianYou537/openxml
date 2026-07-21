//! Auto-generated from `schemas_microsoft_com_office_tasks_2019_documenttasks.json`.
//! Target namespace: `http://schemas.microsoft.com/office/tasks/2019/documenttasks` (prefix `t`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/tasks/2019/documenttasks";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "t";

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

static CHILDREN_TASKS: &[ChildInfo] = &[
    ChildInfo { name: "t:CT_Task/t:Task", property_name: None },
    ChildInfo { name: "oel:CT_ExtensionList/t:extLst", property_name: None },
];
static ATTRS_TASK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_TASK: &[ChildInfo] = &[
    ChildInfo { name: "t:CT_TaskAnchor/t:Anchor", property_name: Some("TaskAnchor") },
    ChildInfo { name: "t:CT_TaskHistory/t:History", property_name: Some("TaskHistory") },
    ChildInfo { name: "oel:CT_ExtensionList/t:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "oel:CT_Extension/oel:ext", property_name: None },
];
static CHILDREN_TASK_ANCHOR: &[ChildInfo] = &[
    ChildInfo { name: "t:CT_CommentAnchor/t:Comment", property_name: Some("CommentAnchor") },
    ChildInfo { name: "oel:CT_ExtensionList/t:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_TASK_HISTORY: &[ChildInfo] = &[
    ChildInfo { name: "t:CT_TaskHistoryEvent/t:Event", property_name: None },
];
static ATTRS_TASK_HISTORY_EVENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":time", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_TASK_HISTORY_EVENT: &[ChildInfo] = &[
    ChildInfo { name: "t:CT_TaskUser/t:Attribution", property_name: Some("AttributionTaskUser") },
    ChildInfo { name: "t:CT_TaskAnchor/t:Anchor", property_name: Some("TaskAnchor") },
    ChildInfo { name: "t:CT_TaskUser/t:Assign", property_name: None },
    ChildInfo { name: "t:CT_TaskUser/t:Unassign", property_name: None },
    ChildInfo { name: "t:CT_TaskCreateEventInfo/t:Create", property_name: None },
    ChildInfo { name: "t:CT_TaskTitleEventInfo/t:SetTitle", property_name: None },
    ChildInfo { name: "t:CT_TaskScheduleEventInfo/t:Schedule", property_name: None },
    ChildInfo { name: "t:CT_TaskProgressEventInfo/t:Progress", property_name: None },
    ChildInfo { name: "t:CT_TaskPriorityEventInfo/t:Priority", property_name: None },
    ChildInfo { name: "t:CT_TaskDeleteEventInfo/t:Delete", property_name: None },
    ChildInfo { name: "t:CT_TaskUndeleteEventInfo/t:Undelete", property_name: None },
    ChildInfo { name: "t:CT_TaskUnassignAll/t:UnassignAll", property_name: None },
    ChildInfo { name: "t:CT_TaskUndo/t:Undo", property_name: None },
    ChildInfo { name: "oel:CT_ExtensionList/t:extLst", property_name: None },
];
static ATTRS_ATTRIBUTION_TASK_USER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":userId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":userName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":userProvider", property_name: None, type_name: "StringValue" },
];
static ATTRS_ASSIGN_TASK_USER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":userId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":userName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":userProvider", property_name: None, type_name: "StringValue" },
];
static ATTRS_UNASSIGN_TASK_USER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":userId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":userName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":userProvider", property_name: None, type_name: "StringValue" },
];
static ATTRS_TASK_TITLE_EVENT_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":title", property_name: None, type_name: "StringValue" },
];
static ATTRS_TASK_SCHEDULE_EVENT_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":startDate", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: ":dueDate", property_name: None, type_name: "DateTimeValue" },
];
static ATTRS_TASK_PROGRESS_EVENT_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":percentComplete", property_name: None, type_name: "Int32Value" },
];
static ATTRS_TASK_PRIORITY_EVENT_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":value", property_name: None, type_name: "Int32Value" },
];
static ATTRS_TASK_UNDO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static ATTRS_COMMENT_ANCHOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Tasks", local_name: "Tasks", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TASKS },
    ElementInfo { class_name: "Task", local_name: "Task", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TASK, children: CHILDREN_TASK },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "TaskAnchor", local_name: "Anchor", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TASK_ANCHOR },
    ElementInfo { class_name: "TaskHistory", local_name: "History", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TASK_HISTORY },
    ElementInfo { class_name: "TaskHistoryEvent", local_name: "Event", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TASK_HISTORY_EVENT, children: CHILDREN_TASK_HISTORY_EVENT },
    ElementInfo { class_name: "AttributionTaskUser", local_name: "Attribution", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ATTRIBUTION_TASK_USER, children: &[] },
    ElementInfo { class_name: "AssignTaskUser", local_name: "Assign", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ASSIGN_TASK_USER, children: &[] },
    ElementInfo { class_name: "UnassignTaskUser", local_name: "Unassign", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UNASSIGN_TASK_USER, children: &[] },
    ElementInfo { class_name: "TaskCreateEventInfo", local_name: "Create", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TaskTitleEventInfo", local_name: "SetTitle", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TASK_TITLE_EVENT_INFO, children: &[] },
    ElementInfo { class_name: "TaskScheduleEventInfo", local_name: "Schedule", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TASK_SCHEDULE_EVENT_INFO, children: &[] },
    ElementInfo { class_name: "TaskProgressEventInfo", local_name: "Progress", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TASK_PROGRESS_EVENT_INFO, children: &[] },
    ElementInfo { class_name: "TaskPriorityEventInfo", local_name: "Priority", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TASK_PRIORITY_EVENT_INFO, children: &[] },
    ElementInfo { class_name: "TaskDeleteEventInfo", local_name: "Delete", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TaskUndeleteEventInfo", local_name: "Undelete", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TaskUnassignAll", local_name: "UnassignAll", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TaskUndo", local_name: "Undo", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TASK_UNDO, children: &[] },
    ElementInfo { class_name: "CommentAnchor", local_name: "Comment", prefix: "t", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMMENT_ANCHOR, children: &[] },
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

/// Create a `<t:Tasks>` element (`Tasks`).
pub fn tasks(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Tasks").with_children(children)
}

/// Create a `<t:Task>` element (`Task`).
pub fn task(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Task").with_children(children)
}

/// Create a `<t:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<t:Anchor>` element (`TaskAnchor`).
pub fn task_anchor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Anchor").with_children(children)
}

/// Create a `<t:History>` element (`TaskHistory`).
pub fn task_history(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "History").with_children(children)
}

/// Create a `<t:Event>` element (`TaskHistoryEvent`).
pub fn task_history_event(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Event").with_children(children)
}

/// Create a `<t:Attribution>` element (`AttributionTaskUser`).
pub fn attribution_task_user() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Attribution")
}

/// Create a `<t:Assign>` element (`AssignTaskUser`).
pub fn assign_task_user() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Assign")
}

/// Create a `<t:Unassign>` element (`UnassignTaskUser`).
pub fn unassign_task_user() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Unassign")
}

/// Create a `<t:Create>` element (`TaskCreateEventInfo`).
pub fn task_create_event_info() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Create")
}

/// Create a `<t:SetTitle>` element (`TaskTitleEventInfo`).
pub fn task_title_event_info() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "SetTitle")
}

/// Create a `<t:Schedule>` element (`TaskScheduleEventInfo`).
pub fn task_schedule_event_info() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Schedule")
}

/// Create a `<t:Progress>` element (`TaskProgressEventInfo`).
pub fn task_progress_event_info() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Progress")
}

/// Create a `<t:Priority>` element (`TaskPriorityEventInfo`).
pub fn task_priority_event_info() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Priority")
}

/// Create a `<t:Delete>` element (`TaskDeleteEventInfo`).
pub fn task_delete_event_info() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Delete")
}

/// Create a `<t:Undelete>` element (`TaskUndeleteEventInfo`).
pub fn task_undelete_event_info() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Undelete")
}

/// Create a `<t:UnassignAll>` element (`TaskUnassignAll`).
pub fn task_unassign_all() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "UnassignAll")
}

/// Create a `<t:Undo>` element (`TaskUndo`).
pub fn task_undo() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Undo")
}

/// Create a `<t:Comment>` element (`CommentAnchor`).
pub fn comment_anchor() -> OpenXmlElement {
    OpenXmlElement::new("t", NAMESPACE_URI, "Comment")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 20;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 19;
