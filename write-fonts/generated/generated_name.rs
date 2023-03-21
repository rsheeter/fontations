// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [Naming table version 1](https://docs.microsoft.com/en-us/typography/opentype/spec/name#naming-table-version-1)
#[derive(Clone, Debug, Default)]
pub struct Name {
    /// The name records where count is the number of records.
    pub name_record: BTreeSet<NameRecord>,
    /// The language-tag records where langTagCount is the number of records.
    pub lang_tag_record: Option<Vec<LangTagRecord>>,
}

impl Name {
    /// Construct a new `Name`
    #[allow(clippy::useless_conversion)]
    pub fn new(name_record: BTreeSet<NameRecord>) -> Self {
        Self {
            name_record: name_record.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

impl FontWrite for Name {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = self.compute_version() as u16;
        version.write_into(writer);
        (array_len(&self.name_record).unwrap() as u16).write_into(writer);
        (self.compute_storage_offset() as u16).write_into(writer);
        writer.adjust_offsets(self.compute_storage_offset() as u32, |writer| {
            self.name_record.write_into(writer);
        });
        version
            .compatible(1)
            .then(|| (array_len(&self.lang_tag_record).unwrap() as u16).write_into(writer));
        writer.adjust_offsets(self.compute_storage_offset() as u32, |writer| {
            version.compatible(1).then(|| {
                self.lang_tag_record
                    .as_ref()
                    .expect("missing versioned field should have failed validation")
                    .write_into(writer)
            });
        });
    }
}

impl Validate for Name {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Name", |ctx| {
            let version: u16 = self.compute_version();
            ctx.in_field("name_record", |ctx| {
                if self.name_record.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.name_record.validate_impl(ctx);
            });
            ctx.in_field("lang_tag_record", |ctx| {
                if version.compatible(1) && self.lang_tag_record.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
                if self.lang_tag_record.is_some()
                    && self.lang_tag_record.as_ref().unwrap().len() > (u16::MAX as usize)
                {
                    ctx.report("array exceeds max length");
                }
                self.lang_tag_record.validate_impl(ctx);
            });
        })
    }
}

impl TopLevelTable for Name {
    const TAG: Tag = Tag::new(b"name");
}

impl<'a> FromObjRef<read_fonts::tables::name::Name<'a>> for Name {
    fn from_obj_ref(obj: &read_fonts::tables::name::Name<'a>, _: FontData) -> Self {
        let offset_data = obj.string_data();
        Name {
            name_record: obj.name_record().to_owned_obj(offset_data),
            lang_tag_record: obj.lang_tag_record().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::name::Name<'a>> for Name {}

impl<'a> FontRead<'a> for Name {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::name::Name as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// Part of [Name]
#[derive(Clone, Debug, Default)]
pub struct LangTagRecord {
    /// Language-tag string offset from start of storage area (in
    /// bytes).
    pub lang_tag: OffsetMarker<String>,
}

impl LangTagRecord {
    /// Construct a new `LangTagRecord`
    #[allow(clippy::useless_conversion)]
    pub fn new(lang_tag: OffsetMarker<String>) -> Self {
        Self {
            lang_tag: lang_tag.into(),
        }
    }
}

impl FontWrite for LangTagRecord {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (self.compile_name_string()).write_into(writer);
    }
}

impl Validate for LangTagRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::name::LangTagRecord> for LangTagRecord {
    fn from_obj_ref(obj: &read_fonts::tables::name::LangTagRecord, offset_data: FontData) -> Self {
        LangTagRecord {
            lang_tag: obj.lang_tag(offset_data).to_owned_table(),
        }
    }
}

///[Name Records](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-records)
#[derive(Clone, Debug, Default)]
pub struct NameRecord {
    /// Platform ID.
    pub platform_id: u16,
    /// Platform-specific encoding ID.
    pub encoding_id: u16,
    /// Language ID.
    pub language_id: u16,
    /// Name ID.
    pub name_id: NameId,
    /// String offset from start of storage area (in bytes).
    pub string: OffsetMarker<String>,
}

impl NameRecord {
    /// Construct a new `NameRecord`
    #[allow(clippy::useless_conversion)]
    pub fn new(
        platform_id: u16,
        encoding_id: u16,
        language_id: u16,
        name_id: NameId,
        string: OffsetMarker<String>,
    ) -> Self {
        Self {
            platform_id,
            encoding_id,
            language_id,
            name_id,
            string: string.into(),
        }
    }
}

impl FontWrite for NameRecord {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.platform_id.write_into(writer);
        self.encoding_id.write_into(writer);
        self.language_id.write_into(writer);
        self.name_id.write_into(writer);
        (self.compile_name_string()).write_into(writer);
    }
}

impl Validate for NameRecord {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("NameRecord", |ctx| {
            ctx.in_field("string", |ctx| {
                self.validate_string_data(ctx);
            });
        })
    }
}

impl FromObjRef<read_fonts::tables::name::NameRecord> for NameRecord {
    fn from_obj_ref(obj: &read_fonts::tables::name::NameRecord, offset_data: FontData) -> Self {
        NameRecord {
            platform_id: obj.platform_id(),
            encoding_id: obj.encoding_id(),
            language_id: obj.language_id(),
            name_id: obj.name_id(),
            string: obj.string(offset_data).to_owned_table(),
        }
    }
}
