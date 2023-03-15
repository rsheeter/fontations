//! An fvar InstanceRecord

use types::{BigEndian, Fixed, FixedSize};

#[cfg(feature = "traversal")]
use crate::traversal::{Field, RecordResolver, SomeRecord};
use crate::{ComputeSize, FontData, FontReadWithArgs, ReadArgs, ReadError};

/// The [InstanceRecord](https://learn.microsoft.com/en-us/typography/opentype/spec/fvar#instancerecord)
#[derive(Clone, Debug)]
pub struct InstanceRecord<'a> {
    /// The name ID for entries in the 'name' table that provide subfamily names for this instance.
    pub subfamily_name_id: u16,
    /// Reserved for future use — set to 0.
    pub flags: u16,
    /// The coordinates array for this instance.
    pub coordinates: &'a [BigEndian<Fixed>],
    /// Optional. The name ID for entries in the 'name' table that provide PostScript names for this instance.
    pub post_script_name_id: Option<u16>,
}

/// The [ReadArgs] args for [InstanceRecord].
#[derive(Clone, Copy, Debug)]
pub struct InstanceRecordArgs {
    pub axis_count: u16,
    pub instance_size: u16,
}

impl ReadArgs for InstanceRecord<'_> {
    type Args = InstanceRecordArgs;
}

impl<'a> FontReadWithArgs<'a> for InstanceRecord<'a> {
    fn read_with_args(data: FontData<'a>, args: &Self::Args) -> Result<Self, ReadError> {
        let InstanceRecordArgs {
            axis_count,
            instance_size,
        } = *args;
        let axis_count = axis_count as usize;
        let mut cursor = data.cursor();
        let subfamily_name_id = cursor.read()?;
        let flags = cursor.read()?;
        let coordinates = cursor.read_array(axis_count)?;
        // Size of common fields (subfamily_name_id and flags) plus axis coordinates.
        let common_byte_len = u16::RAW_BYTE_LEN * 2 + (axis_count * Fixed::RAW_BYTE_LEN);
        // The instance contains a post_script_name_id field if the instance size is greater than
        // or equal to the common size plus the 2 bytes for the optional field.
        let has_post_script_name_id = instance_size as usize >= common_byte_len + u16::RAW_BYTE_LEN;
        let post_script_name_id = if has_post_script_name_id {
            Some(cursor.read()?)
        } else {
            None
        };
        Ok(InstanceRecord {
            subfamily_name_id,
            flags,
            coordinates,
            post_script_name_id,
        })
    }
}

impl ComputeSize for InstanceRecord<'_> {
    #[inline]
    fn compute_size(args: &InstanceRecordArgs) -> usize {
        args.instance_size as usize
    }
}

#[cfg(feature = "traversal")]
impl<'a> InstanceRecord<'a> {
    pub(crate) fn get_field(&self, idx: usize, _data: FontData<'a>) -> Option<Field<'a>> {
        match idx {
            0 => Some(Field::new("subfamily_name_id", self.subfamily_name_id)),
            1 => Some(Field::new("flags", self.flags)),
            2 => Some(Field::new("coordinates", self.coordinates)),
            3 => Some(Field::new("post_script_name_id", self.post_script_name_id?)),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeRecord<'a> for InstanceRecord<'a> {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "InstanceRecord",
            data,
            get_field: Box::new(move |idx, data| self.get_field(idx, data)),
        }
    }
}
