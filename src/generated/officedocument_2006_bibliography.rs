//! Auto-generated from `schemas_openxmlformats_org_officeDocument_2006_bibliography.json`.
//! Target namespace: `http://schemas.openxmlformats.org/officeDocument/2006/bibliography` (prefix `b`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/officeDocument/2006/bibliography";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "b";

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

static ATTRS_SOURCES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":SelectedStyle", property_name: Some("SelectedStyle"), type_name: "StringValue" },
    AttributeInfo { qname: ":StyleName", property_name: Some("StyleName"), type_name: "StringValue" },
    AttributeInfo { qname: ":URI", property_name: Some("Uri"), type_name: "StringValue" },
];
static CHILDREN_SOURCES: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_SourceType/b:Source", property_name: None },
];
static CHILDREN_PERSON: &[ChildInfo] = &[
    ChildInfo { name: "b:ST_String255/b:Last", property_name: None },
    ChildInfo { name: "b:ST_String255/b:First", property_name: None },
    ChildInfo { name: "b:ST_String255/b:Middle", property_name: None },
];
static CHILDREN_NAME_LIST: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_PersonType/b:Person", property_name: None },
];
static CHILDREN_ARTIST: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_BOOK_AUTHOR: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_COMPILER: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_COMPOSER: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_CONDUCTOR: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_COUNSEL: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_DIRECTOR: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_EDITOR: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_INTERVIEWEE: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_INTERVIEWER: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_INVENTOR: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_PRODUCER_NAME: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_TRANSLATOR: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_WRITER: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
];
static CHILDREN_AUTHOR: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
    ChildInfo { name: "b:ST_String255/b:Corporate", property_name: Some("Corporate") },
];
static CHILDREN_PERFORMER: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameListType/b:NameList", property_name: Some("NameList") },
    ChildInfo { name: "b:ST_String255/b:Corporate", property_name: Some("Corporate") },
];
static CHILDREN_AUTHOR_LIST: &[ChildInfo] = &[
    ChildInfo { name: "b:CT_NameType/b:Artist", property_name: Some("Artist") },
    ChildInfo { name: "b:CT_NameOrCorporateType/b:Author", property_name: Some("Author") },
    ChildInfo { name: "b:CT_NameType/b:BookAuthor", property_name: Some("BookAuthor") },
    ChildInfo { name: "b:CT_NameType/b:Compiler", property_name: Some("Compiler") },
    ChildInfo { name: "b:CT_NameType/b:Composer", property_name: Some("Composer") },
    ChildInfo { name: "b:CT_NameType/b:Conductor", property_name: Some("Conductor") },
    ChildInfo { name: "b:CT_NameType/b:Counsel", property_name: Some("Counsel") },
    ChildInfo { name: "b:CT_NameType/b:Director", property_name: Some("Director") },
    ChildInfo { name: "b:CT_NameType/b:Editor", property_name: Some("Editor") },
    ChildInfo { name: "b:CT_NameType/b:Interviewee", property_name: Some("Interviewee") },
    ChildInfo { name: "b:CT_NameType/b:Interviewer", property_name: Some("Interviewer") },
    ChildInfo { name: "b:CT_NameType/b:Inventor", property_name: Some("Inventor") },
    ChildInfo { name: "b:CT_NameOrCorporateType/b:Performer", property_name: Some("Performer") },
    ChildInfo { name: "b:CT_NameType/b:ProducerName", property_name: Some("ProducerName") },
    ChildInfo { name: "b:CT_NameType/b:Translator", property_name: Some("Translator") },
    ChildInfo { name: "b:CT_NameType/b:Writer", property_name: Some("Writer") },
];
static CHILDREN_SOURCE: &[ChildInfo] = &[
    ChildInfo { name: "b:ST_String255/b:AbbreviatedCaseNumber", property_name: Some("AbbreviatedCaseNumber") },
    ChildInfo { name: "b:ST_String255/b:AlbumTitle", property_name: Some("AlbumTitle") },
    ChildInfo { name: "b:CT_AuthorType/b:Author", property_name: Some("AuthorList") },
    ChildInfo { name: "b:ST_String255/b:BookTitle", property_name: Some("BookTitle") },
    ChildInfo { name: "b:ST_String255/b:Broadcaster", property_name: Some("Broadcaster") },
    ChildInfo { name: "b:ST_String255/b:BroadcastTitle", property_name: Some("BroadcastTitle") },
    ChildInfo { name: "b:ST_String255/b:CaseNumber", property_name: Some("CaseNumber") },
    ChildInfo { name: "b:ST_String255/b:ChapterNumber", property_name: Some("ChapterNumber") },
    ChildInfo { name: "b:ST_String255/b:City", property_name: Some("City") },
    ChildInfo { name: "b:ST_String255/b:Comments", property_name: Some("Comments") },
    ChildInfo { name: "b:ST_String255/b:ConferenceName", property_name: Some("ConferenceName") },
    ChildInfo { name: "b:ST_String255/b:CountryRegion", property_name: Some("CountryRegion") },
    ChildInfo { name: "b:ST_String255/b:Court", property_name: Some("Court") },
    ChildInfo { name: "b:ST_String255/b:Day", property_name: Some("Day") },
    ChildInfo { name: "b:ST_String255/b:DayAccessed", property_name: Some("DayAccessed") },
    ChildInfo { name: "b:ST_String255/b:Department", property_name: Some("Department") },
    ChildInfo { name: "b:ST_String255/b:Distributor", property_name: Some("Distributor") },
    ChildInfo { name: "b:ST_String255/b:Edition", property_name: Some("Edition") },
    ChildInfo { name: "b:ST_String255/b:Guid", property_name: Some("GuidString") },
    ChildInfo { name: "b:ST_String255/b:Institution", property_name: Some("Institution") },
    ChildInfo { name: "b:ST_String255/b:InternetSiteTitle", property_name: Some("InternetSiteTitle") },
    ChildInfo { name: "b:ST_String255/b:Issue", property_name: Some("Issue") },
    ChildInfo { name: "b:ST_String255/b:JournalName", property_name: Some("JournalName") },
    ChildInfo { name: "b:ST_String255/b:LCID", property_name: Some("LcId") },
    ChildInfo { name: "b:ST_String255/b:Medium", property_name: Some("Medium") },
    ChildInfo { name: "b:ST_String255/b:Month", property_name: Some("Month") },
    ChildInfo { name: "b:ST_String255/b:MonthAccessed", property_name: Some("MonthAccessed") },
    ChildInfo { name: "b:ST_String255/b:NumberVolumes", property_name: Some("NumberVolumes") },
    ChildInfo { name: "b:ST_String255/b:Pages", property_name: Some("Pages") },
    ChildInfo { name: "b:ST_String255/b:PatentNumber", property_name: Some("PatentNumber") },
    ChildInfo { name: "b:ST_String255/b:PeriodicalTitle", property_name: Some("PeriodicalTitle") },
    ChildInfo { name: "b:ST_String255/b:ProductionCompany", property_name: Some("ProductionCompany") },
    ChildInfo { name: "b:ST_String255/b:PublicationTitle", property_name: Some("PublicationTitle") },
    ChildInfo { name: "b:ST_String255/b:Publisher", property_name: Some("Publisher") },
    ChildInfo { name: "b:ST_String255/b:RecordingNumber", property_name: Some("RecordingNumber") },
    ChildInfo { name: "b:ST_String255/b:RefOrder", property_name: Some("ReferenceOrder") },
    ChildInfo { name: "b:ST_String255/b:Reporter", property_name: Some("Reporter") },
    ChildInfo { name: "b:ST_SourceType/b:SourceType", property_name: Some("SourceType") },
    ChildInfo { name: "b:ST_String255/b:ShortTitle", property_name: Some("ShortTitle") },
    ChildInfo { name: "b:ST_String255/b:StandardNumber", property_name: Some("StandardNumber") },
    ChildInfo { name: "b:ST_String255/b:StateProvince", property_name: Some("StateProvince") },
    ChildInfo { name: "b:ST_String255/b:Station", property_name: Some("Station") },
    ChildInfo { name: "b:ST_String255/b:Tag", property_name: Some("Tag") },
    ChildInfo { name: "b:ST_String255/b:Theater", property_name: Some("Theater") },
    ChildInfo { name: "b:ST_String255/b:ThesisType", property_name: Some("ThesisType") },
    ChildInfo { name: "b:ST_String255/b:Title", property_name: Some("Title") },
    ChildInfo { name: "b:ST_String255/b:Type", property_name: Some("PatentType") },
    ChildInfo { name: "b:ST_String255/b:URL", property_name: Some("UrlString") },
    ChildInfo { name: "b:ST_String255/b:Version", property_name: Some("Version") },
    ChildInfo { name: "b:ST_String255/b:Volume", property_name: Some("Volume") },
    ChildInfo { name: "b:ST_String255/b:Year", property_name: Some("Year") },
    ChildInfo { name: "b:ST_String255/b:YearAccessed", property_name: Some("YearAccessed") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Sources", local_name: "Sources", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SOURCES, children: CHILDREN_SOURCES },
    ElementInfo { class_name: "Person", local_name: "Person", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PERSON },
    ElementInfo { class_name: "Last", local_name: "Last", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "First", local_name: "First", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Middle", local_name: "Middle", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Corporate", local_name: "Corporate", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "AbbreviatedCaseNumber", local_name: "AbbreviatedCaseNumber", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "AlbumTitle", local_name: "AlbumTitle", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "BookTitle", local_name: "BookTitle", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Broadcaster", local_name: "Broadcaster", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "BroadcastTitle", local_name: "BroadcastTitle", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CaseNumber", local_name: "CaseNumber", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ChapterNumber", local_name: "ChapterNumber", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "City", local_name: "City", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Comments", local_name: "Comments", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ConferenceName", local_name: "ConferenceName", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CountryRegion", local_name: "CountryRegion", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Court", local_name: "Court", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Day", local_name: "Day", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "DayAccessed", local_name: "DayAccessed", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Department", local_name: "Department", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Distributor", local_name: "Distributor", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Edition", local_name: "Edition", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "GuidString", local_name: "Guid", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Institution", local_name: "Institution", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "InternetSiteTitle", local_name: "InternetSiteTitle", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Issue", local_name: "Issue", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "JournalName", local_name: "JournalName", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "LcId", local_name: "LCID", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Medium", local_name: "Medium", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Month", local_name: "Month", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "MonthAccessed", local_name: "MonthAccessed", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "NumberVolumes", local_name: "NumberVolumes", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Pages", local_name: "Pages", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PatentNumber", local_name: "PatentNumber", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PeriodicalTitle", local_name: "PeriodicalTitle", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ProductionCompany", local_name: "ProductionCompany", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PublicationTitle", local_name: "PublicationTitle", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Publisher", local_name: "Publisher", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "RecordingNumber", local_name: "RecordingNumber", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ReferenceOrder", local_name: "RefOrder", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Reporter", local_name: "Reporter", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ShortTitle", local_name: "ShortTitle", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "StandardNumber", local_name: "StandardNumber", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "StateProvince", local_name: "StateProvince", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Station", local_name: "Station", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Tag", local_name: "Tag", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Theater", local_name: "Theater", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ThesisType", local_name: "ThesisType", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Title", local_name: "Title", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PatentType", local_name: "Type", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "UrlString", local_name: "URL", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Version", local_name: "Version", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Volume", local_name: "Volume", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Year", local_name: "Year", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "YearAccessed", local_name: "YearAccessed", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "NameList", local_name: "NameList", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NAME_LIST },
    ElementInfo { class_name: "Artist", local_name: "Artist", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ARTIST },
    ElementInfo { class_name: "BookAuthor", local_name: "BookAuthor", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BOOK_AUTHOR },
    ElementInfo { class_name: "Compiler", local_name: "Compiler", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMPILER },
    ElementInfo { class_name: "Composer", local_name: "Composer", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMPOSER },
    ElementInfo { class_name: "Conductor", local_name: "Conductor", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONDUCTOR },
    ElementInfo { class_name: "Counsel", local_name: "Counsel", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COUNSEL },
    ElementInfo { class_name: "Director", local_name: "Director", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DIRECTOR },
    ElementInfo { class_name: "Editor", local_name: "Editor", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EDITOR },
    ElementInfo { class_name: "Interviewee", local_name: "Interviewee", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_INTERVIEWEE },
    ElementInfo { class_name: "Interviewer", local_name: "Interviewer", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_INTERVIEWER },
    ElementInfo { class_name: "Inventor", local_name: "Inventor", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_INVENTOR },
    ElementInfo { class_name: "ProducerName", local_name: "ProducerName", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PRODUCER_NAME },
    ElementInfo { class_name: "Translator", local_name: "Translator", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TRANSLATOR },
    ElementInfo { class_name: "Writer", local_name: "Writer", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WRITER },
    ElementInfo { class_name: "Author", local_name: "Author", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AUTHOR },
    ElementInfo { class_name: "Performer", local_name: "Performer", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PERFORMER },
    ElementInfo { class_name: "AuthorList", local_name: "Author", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AUTHOR_LIST },
    ElementInfo { class_name: "SourceType", local_name: "SourceType", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Source", local_name: "Source", prefix: "b", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SOURCE },
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

/// Create a `<b:Sources>` element (`Sources`).
pub fn sources(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Sources").with_children(children)
}

/// Create a `<b:Person>` element (`Person`).
pub fn person(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Person").with_children(children)
}

/// Create a `<b:Last>` element (`Last`).
pub fn last(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Last").with_text(value)
}

/// Create a `<b:First>` element (`First`).
pub fn first(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "First").with_text(value)
}

/// Create a `<b:Middle>` element (`Middle`).
pub fn middle(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Middle").with_text(value)
}

/// Create a `<b:Corporate>` element (`Corporate`).
pub fn corporate(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Corporate").with_text(value)
}

/// Create a `<b:AbbreviatedCaseNumber>` element (`AbbreviatedCaseNumber`).
pub fn abbreviated_case_number(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "AbbreviatedCaseNumber").with_text(value)
}

/// Create a `<b:AlbumTitle>` element (`AlbumTitle`).
pub fn album_title(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "AlbumTitle").with_text(value)
}

/// Create a `<b:BookTitle>` element (`BookTitle`).
pub fn book_title(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "BookTitle").with_text(value)
}

/// Create a `<b:Broadcaster>` element (`Broadcaster`).
pub fn broadcaster(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Broadcaster").with_text(value)
}

/// Create a `<b:BroadcastTitle>` element (`BroadcastTitle`).
pub fn broadcast_title(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "BroadcastTitle").with_text(value)
}

/// Create a `<b:CaseNumber>` element (`CaseNumber`).
pub fn case_number(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "CaseNumber").with_text(value)
}

/// Create a `<b:ChapterNumber>` element (`ChapterNumber`).
pub fn chapter_number(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "ChapterNumber").with_text(value)
}

/// Create a `<b:City>` element (`City`).
pub fn city(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "City").with_text(value)
}

/// Create a `<b:Comments>` element (`Comments`).
pub fn comments(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Comments").with_text(value)
}

/// Create a `<b:ConferenceName>` element (`ConferenceName`).
pub fn conference_name(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "ConferenceName").with_text(value)
}

/// Create a `<b:CountryRegion>` element (`CountryRegion`).
pub fn country_region(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "CountryRegion").with_text(value)
}

/// Create a `<b:Court>` element (`Court`).
pub fn court(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Court").with_text(value)
}

/// Create a `<b:Day>` element (`Day`).
pub fn day(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Day").with_text(value)
}

/// Create a `<b:DayAccessed>` element (`DayAccessed`).
pub fn day_accessed(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "DayAccessed").with_text(value)
}

/// Create a `<b:Department>` element (`Department`).
pub fn department(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Department").with_text(value)
}

/// Create a `<b:Distributor>` element (`Distributor`).
pub fn distributor(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Distributor").with_text(value)
}

/// Create a `<b:Edition>` element (`Edition`).
pub fn edition(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Edition").with_text(value)
}

/// Create a `<b:Guid>` element (`GuidString`).
pub fn guid_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Guid").with_text(value)
}

/// Create a `<b:Institution>` element (`Institution`).
pub fn institution(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Institution").with_text(value)
}

/// Create a `<b:InternetSiteTitle>` element (`InternetSiteTitle`).
pub fn internet_site_title(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "InternetSiteTitle").with_text(value)
}

/// Create a `<b:Issue>` element (`Issue`).
pub fn issue(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Issue").with_text(value)
}

/// Create a `<b:JournalName>` element (`JournalName`).
pub fn journal_name(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "JournalName").with_text(value)
}

/// Create a `<b:LCID>` element (`LcId`).
pub fn lc_id(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "LCID").with_text(value)
}

/// Create a `<b:Medium>` element (`Medium`).
pub fn medium(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Medium").with_text(value)
}

/// Create a `<b:Month>` element (`Month`).
pub fn month(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Month").with_text(value)
}

/// Create a `<b:MonthAccessed>` element (`MonthAccessed`).
pub fn month_accessed(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "MonthAccessed").with_text(value)
}

/// Create a `<b:NumberVolumes>` element (`NumberVolumes`).
pub fn number_volumes(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "NumberVolumes").with_text(value)
}

/// Create a `<b:Pages>` element (`Pages`).
pub fn pages(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Pages").with_text(value)
}

/// Create a `<b:PatentNumber>` element (`PatentNumber`).
pub fn patent_number(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "PatentNumber").with_text(value)
}

/// Create a `<b:PeriodicalTitle>` element (`PeriodicalTitle`).
pub fn periodical_title(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "PeriodicalTitle").with_text(value)
}

/// Create a `<b:ProductionCompany>` element (`ProductionCompany`).
pub fn production_company(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "ProductionCompany").with_text(value)
}

/// Create a `<b:PublicationTitle>` element (`PublicationTitle`).
pub fn publication_title(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "PublicationTitle").with_text(value)
}

/// Create a `<b:Publisher>` element (`Publisher`).
pub fn publisher(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Publisher").with_text(value)
}

/// Create a `<b:RecordingNumber>` element (`RecordingNumber`).
pub fn recording_number(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "RecordingNumber").with_text(value)
}

/// Create a `<b:RefOrder>` element (`ReferenceOrder`).
pub fn reference_order(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "RefOrder").with_text(value)
}

/// Create a `<b:Reporter>` element (`Reporter`).
pub fn reporter(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Reporter").with_text(value)
}

/// Create a `<b:ShortTitle>` element (`ShortTitle`).
pub fn short_title(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "ShortTitle").with_text(value)
}

/// Create a `<b:StandardNumber>` element (`StandardNumber`).
pub fn standard_number(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "StandardNumber").with_text(value)
}

/// Create a `<b:StateProvince>` element (`StateProvince`).
pub fn state_province(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "StateProvince").with_text(value)
}

/// Create a `<b:Station>` element (`Station`).
pub fn station(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Station").with_text(value)
}

/// Create a `<b:Tag>` element (`Tag`).
pub fn tag(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Tag").with_text(value)
}

/// Create a `<b:Theater>` element (`Theater`).
pub fn theater(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Theater").with_text(value)
}

/// Create a `<b:ThesisType>` element (`ThesisType`).
pub fn thesis_type(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "ThesisType").with_text(value)
}

/// Create a `<b:Title>` element (`Title`).
pub fn title(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Title").with_text(value)
}

/// Create a `<b:Type>` element (`PatentType`).
pub fn patent_type(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Type").with_text(value)
}

/// Create a `<b:URL>` element (`UrlString`).
pub fn url_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "URL").with_text(value)
}

/// Create a `<b:Version>` element (`Version`).
pub fn version(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Version").with_text(value)
}

/// Create a `<b:Volume>` element (`Volume`).
pub fn volume(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Volume").with_text(value)
}

/// Create a `<b:Year>` element (`Year`).
pub fn year(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Year").with_text(value)
}

/// Create a `<b:YearAccessed>` element (`YearAccessed`).
pub fn year_accessed(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "YearAccessed").with_text(value)
}

/// Create a `<b:NameList>` element (`NameList`).
pub fn name_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "NameList").with_children(children)
}

/// Create a `<b:Artist>` element (`Artist`).
pub fn artist(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Artist").with_children(children)
}

/// Create a `<b:BookAuthor>` element (`BookAuthor`).
pub fn book_author(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "BookAuthor").with_children(children)
}

/// Create a `<b:Compiler>` element (`Compiler`).
pub fn compiler(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Compiler").with_children(children)
}

/// Create a `<b:Composer>` element (`Composer`).
pub fn composer(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Composer").with_children(children)
}

/// Create a `<b:Conductor>` element (`Conductor`).
pub fn conductor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Conductor").with_children(children)
}

/// Create a `<b:Counsel>` element (`Counsel`).
pub fn counsel(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Counsel").with_children(children)
}

/// Create a `<b:Director>` element (`Director`).
pub fn director(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Director").with_children(children)
}

/// Create a `<b:Editor>` element (`Editor`).
pub fn editor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Editor").with_children(children)
}

/// Create a `<b:Interviewee>` element (`Interviewee`).
pub fn interviewee(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Interviewee").with_children(children)
}

/// Create a `<b:Interviewer>` element (`Interviewer`).
pub fn interviewer(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Interviewer").with_children(children)
}

/// Create a `<b:Inventor>` element (`Inventor`).
pub fn inventor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Inventor").with_children(children)
}

/// Create a `<b:ProducerName>` element (`ProducerName`).
pub fn producer_name(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "ProducerName").with_children(children)
}

/// Create a `<b:Translator>` element (`Translator`).
pub fn translator(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Translator").with_children(children)
}

/// Create a `<b:Writer>` element (`Writer`).
pub fn writer(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Writer").with_children(children)
}

/// Create a `<b:Author>` element (`Author`).
pub fn author(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Author").with_children(children)
}

/// Create a `<b:Performer>` element (`Performer`).
pub fn performer(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Performer").with_children(children)
}

/// Create a `<b:Author>` element (`AuthorList`).
pub fn author_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Author").with_children(children)
}

/// Create a `<b:SourceType>` element (`SourceType`).
pub fn source_type(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "SourceType").with_text(value)
}

/// Create a `<b:Source>` element (`Source`).
pub fn source(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("b", NAMESPACE_URI, "Source").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 78;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 76;
