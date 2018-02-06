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
    };
    (__serialize $self:ident, $serializer:ident, $field_name:ident -> num) => {
        {
            $serializer.tag(stringify!($field_name));
            $serializer.serialize_num($self.$field_name);
        }
    };
    (__serialize $self:ident, $serializer:ident, $field_name:ident -> varint) => {
        {
            $serializer.tag(stringify!($field_name));
            $serializer.serialize_varint($self.$field_name);
        }
    };
    (__serialize $self:ident, $serializer:ident, $field_name:ident -> uvarint) => {
        {
            $serializer.tag(stringify!($field_name));
            $serializer.serialize_uvarint($self.$field_name);
        }
    };

    (__serialize $self:ident, $serializer:ident, $field_name:ident -> blob) => {
        {
            $serializer.tag(stringify!($field_name));
            $serializer.serialize_blob(&$self.$field_name);
        }
    };
}
