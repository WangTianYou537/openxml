//! OPC (Open Packaging Conventions) layer.
//!
//! An OPC package is a ZIP archive with:
//! - `[Content_Types].xml` — maps parts to MIME content types
//! - `_rels/.rels` — package-level relationships
//! - part streams (e.g. `word/document.xml`) and optional part relationships

mod cfb;
mod content_types;
mod custom_properties;
mod extended_properties;
mod flat_opc;
mod media;
mod package;
mod part_uri;
mod properties;
mod relationships;
mod uri;

pub use cfb::{inspect_vba_project, CfbEntry, CfbFile, CfbObjectType};
pub use content_types::{ContentTypeOverride, ContentTypes};
pub use custom_properties::{
    CustomProperties, CustomProperty, CustomPropertyValue, CUSTOM_PROP_FMTID,
};
pub use extended_properties::ExtendedProperties;
pub use flat_opc::{from_flat_opc, progid, to_flat_opc};
pub use media::{add_media_part, media_rel, MediaKind, MediaPartInfo};
pub use package::{CompressionOption, OpcPackage, PackageMode};
// Lazy open helpers are methods on OpcPackage: open_lazy / open_bytes_lazy / open_reader_lazy.
pub use part_uri::{PartUriHelper, RelatedPart};
pub use properties::PackageProperties;
pub use relationships::{Relationship, RelationshipTargetMode, Relationships};
pub use uri::{pack_uri, resolve_uri, PackUri};
