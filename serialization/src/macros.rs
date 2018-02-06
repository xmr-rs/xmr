// Greensputh's tenth rule anywhere?

// TODO: Improve macro syntax
/// A macro to craft `Serialize` and `Deserialize` routines for an structure.
///
/// # Usage:
///
/// `serialize!2 { field -> (specifier), ...,  }`
///
/// `specifier` should be one of the valid identifiers found below.
///
/// # Notes:
///
/// The field list should end with a trailing comma.
///
/// # Example:
///
/// ```rust
/// #[derive(Debug)]
/// pub struct TransactionHistory {
///     pub id: u32,
///     pub extra: u8,
/// }
/// 
/// serialize2! {
///     TransactionHistory {
///         id -> (num),
///         // note the trailing comma.
///         extra -> (num),
///     }
/// }
/// ```
///
/// ## Specifier list
///
/// - `num`: a number it can be signed or unsigned.
/// - `varint`: a signed variable-length integer.
/// - ``
#[macro_export]
macro_rules! serialize2 {
    ($struct_name:ident { $( $name:ident -> ($($discriminator:tt)*) ,)* }) => {
        impl $crate::serializer::Serialize for $struct_name {
            fn serialize<T: $crate::serializer::Serializer>(&self, serializer: &mut T) {
                $(
                    serialize2!(__serialize self, serializer, $name -> $($discriminator)*);
                )*
            }
        }

        impl $crate::deserializer::Deserialize for $struct_name {
            fn deserialize<'buf, T: $crate::deserializer::Deserializer<'buf>>(deserializer: &'buf mut T) -> Self {
                let mut st = Self::default();
                $(
                    serialize2!(__deserialize st, deserializer, $name -> $($discriminator)*);
                )*
                st
            }
        }
    };
    (__serialize $self:ident, $serializer:ident, $field_name:ident -> num) => {
        {
            $serializer.serialize_num($self.$field_name);
        }
    };
    (__serialize $self:ident, $serializer:ident, $field_name:ident -> varint) => {
        {
            $serializer.serialize_varint($self.$field_name);
        }
    };
    (__serialize $self:ident, $serializer:ident, $field_name:ident -> uvarint) => {
        {
            $serializer.serialize_uvarint($self.$field_name);
        }
    };

    (__serialize $self:ident, $serializer:ident, $field_name:ident -> blob) => {
        {
            $serializer.serialize_blob(&$self.$field_name);
        }
    };
    (__deserialize $self:ident, $deserializer:ident, $field_name:ident -> num) => {
        {
            $self.$field_name= $deserializer.deserialize_num();
        }
    };
    (__deserialize $self:ident, $deserializer:ident, $field_name:ident -> varint) => {
        {
            $self.$field_name = $deserializer.deserialize_varint();
        }
    };
    (__deserialize $self:ident, $deserializer:ident, $field_name:ident -> uvarint) => {
        {
            $self.$field_name = $deserializer.deserialize_uvarint();
        }
    };

    (__deserialize $self:ident, $deserializer:ident, $field_name:ident -> blob) => {
        {
            $self.$field_name = $deserializer.deserialize_blob();
        }
    };
}
