// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [Naming table version 1](https://docs.microsoft.com/en-us/typography/opentype/spec/name#naming-table-version-1)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct NameMarker {
    name_record_byte_len: usize,
    lang_tag_count_byte_start: Option<usize>,
    lang_tag_record_byte_start: Option<usize>,
    lang_tag_record_byte_len: Option<usize>,
}

impl NameMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn count_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn storage_offset_byte_range(&self) -> Range<usize> {
        let start = self.count_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn name_record_byte_range(&self) -> Range<usize> {
        let start = self.storage_offset_byte_range().end;
        start..start + self.name_record_byte_len
    }
    fn lang_tag_count_byte_range(&self) -> Option<Range<usize>> {
        let start = self.lang_tag_count_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn lang_tag_record_byte_range(&self) -> Option<Range<usize>> {
        let start = self.lang_tag_record_byte_start?;
        Some(start..start + self.lang_tag_record_byte_len?)
    }
}

impl TopLevelTable for Name<'_> {
    /// `name`
    const TAG: Tag = Tag::new(b"name");
}

impl<'a> FontRead<'a> for Name<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let version: u16 = cursor.read()?;
        let count: u16 = cursor.read()?;
        cursor.advance::<u16>();
        let name_record_byte_len = count as usize * NameRecord::RAW_BYTE_LEN;
        cursor.advance_by(name_record_byte_len);
        let lang_tag_count_byte_start = version
            .compatible(1)
            .then(|| cursor.position())
            .transpose()?;
        let lang_tag_count = version
            .compatible(1)
            .then(|| cursor.read::<u16>())
            .transpose()?
            .unwrap_or(0);
        let lang_tag_record_byte_start = version
            .compatible(1)
            .then(|| cursor.position())
            .transpose()?;
        let lang_tag_record_byte_len = version
            .compatible(1)
            .then_some(lang_tag_count as usize * LangTagRecord::RAW_BYTE_LEN);
        if let Some(value) = lang_tag_record_byte_len {
            cursor.advance_by(value);
        }
        cursor.finish(NameMarker {
            name_record_byte_len,
            lang_tag_count_byte_start,
            lang_tag_record_byte_start,
            lang_tag_record_byte_len,
        })
    }
}

/// [Naming table version 1](https://docs.microsoft.com/en-us/typography/opentype/spec/name#naming-table-version-1)
pub type Name<'a> = TableRef<'a, NameMarker>;

impl<'a> Name<'a> {
    /// Table version number (0 or 1)
    pub fn version(&self) -> u16 {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of name records.
    pub fn count(&self) -> u16 {
        let range = self.shape.count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset to start of string storage (from start of table).
    pub fn storage_offset(&self) -> u16 {
        let range = self.shape.storage_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The name records where count is the number of records.
    pub fn name_record(&self) -> &'a [NameRecord] {
        let range = self.shape.name_record_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Number of language-tag records.
    pub fn lang_tag_count(&self) -> Option<u16> {
        let range = self.shape.lang_tag_count_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// The language-tag records where langTagCount is the number of records.
    pub fn lang_tag_record(&self) -> Option<&'a [LangTagRecord]> {
        let range = self.shape.lang_tag_record_byte_range()?;
        Some(self.data.read_array(range).unwrap())
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Name<'a> {
    fn type_name(&self) -> &str {
        "Name"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        let version = self.version();
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("count", self.count())),
            2usize => Some(Field::new("storage_offset", self.storage_offset())),
            3usize => Some(Field::new(
                "name_record",
                traversal::FieldType::array_of_records(
                    stringify!(NameRecord),
                    self.name_record(),
                    self.string_data(),
                ),
            )),
            4usize if version.compatible(1) => {
                Some(Field::new("lang_tag_count", self.lang_tag_count().unwrap()))
            }
            5usize if version.compatible(1) => Some(Field::new(
                "lang_tag_record",
                traversal::FieldType::array_of_records(
                    stringify!(LangTagRecord),
                    self.lang_tag_record().unwrap(),
                    self.string_data(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Name<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Part of [Name]
#[derive(Clone, Debug)]
#[repr(C)]
#[repr(packed)]
pub struct LangTagRecord {
    /// Language-tag string length (in bytes)
    pub length: BigEndian<u16>,
    /// Language-tag string offset from start of storage area (in
    /// bytes).
    pub lang_tag_offset: BigEndian<Offset16>,
}

impl LangTagRecord {
    /// Language-tag string length (in bytes)
    pub fn length(&self) -> u16 {
        self.length.get()
    }

    /// Language-tag string offset from start of storage area (in
    /// bytes).
    pub fn lang_tag_offset(&self) -> Offset16 {
        self.lang_tag_offset.get()
    }
}

impl FixedSize for LangTagRecord {
    const RAW_BYTE_LEN: usize = u16::RAW_BYTE_LEN + Offset16::RAW_BYTE_LEN;
}

#[cfg(feature = "traversal")]
impl<'a> SomeRecord<'a> for LangTagRecord {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "LangTagRecord",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("length", self.length())),
                1usize => Some(Field::new("lang_tag_offset", self.traverse_lang_tag(_data))),
                _ => None,
            }),
            data,
        }
    }
}

///[Name Records](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-records)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
#[repr(packed)]
pub struct NameRecord {
    /// Platform ID.
    pub platform_id: BigEndian<u16>,
    /// Platform-specific encoding ID.
    pub encoding_id: BigEndian<u16>,
    /// Language ID.
    pub language_id: BigEndian<u16>,
    /// Name ID.
    pub name_id: BigEndian<NameId>,
    /// String length (in bytes).
    pub length: BigEndian<u16>,
    /// String offset from start of storage area (in bytes).
    pub string_offset: BigEndian<Offset16>,
}

impl NameRecord {
    /// Platform ID.
    pub fn platform_id(&self) -> u16 {
        self.platform_id.get()
    }

    /// Platform-specific encoding ID.
    pub fn encoding_id(&self) -> u16 {
        self.encoding_id.get()
    }

    /// Language ID.
    pub fn language_id(&self) -> u16 {
        self.language_id.get()
    }

    /// Name ID.
    pub fn name_id(&self) -> NameId {
        self.name_id.get()
    }

    /// String length (in bytes).
    pub fn length(&self) -> u16 {
        self.length.get()
    }

    /// String offset from start of storage area (in bytes).
    pub fn string_offset(&self) -> Offset16 {
        self.string_offset.get()
    }
}

impl FixedSize for NameRecord {
    const RAW_BYTE_LEN: usize = u16::RAW_BYTE_LEN
        + u16::RAW_BYTE_LEN
        + u16::RAW_BYTE_LEN
        + NameId::RAW_BYTE_LEN
        + u16::RAW_BYTE_LEN
        + Offset16::RAW_BYTE_LEN;
}

#[cfg(feature = "traversal")]
impl<'a> SomeRecord<'a> for NameRecord {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "NameRecord",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("platform_id", self.platform_id())),
                1usize => Some(Field::new("encoding_id", self.encoding_id())),
                2usize => Some(Field::new("language_id", self.language_id())),
                3usize => Some(Field::new("name_id", self.name_id())),
                4usize => Some(Field::new("length", self.length())),
                5usize => Some(Field::new("string_offset", self.traverse_string(_data))),
                _ => None,
            }),
            data,
        }
    }
}
