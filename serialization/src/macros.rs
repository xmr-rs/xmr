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
        impl $crate::ser::Serialize for $struct_name {
            fn serialize<T: $crate::ser::Serializer>(&self, serializer: &mut T) {
                $(
                    serialize2!(__serialize self, serializer, $name -> $($discriminator)*);
                )*
            }
        }

        impl $crate::de::Deserialize for $struct_name {
            fn deserialize<T: $crate::de::Deserializer>(deserializer: &mut T) -> Self {
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
    (__serialize $self:ident, $serializer:ident, $field_name:ident -> struct) => {
        {
            $serializer.serialize_struct(&$self.$field_name);
        }
    };
    (__serialize $self:ident, $serializer:ident, $field_name:ident -> array) => {
        {
            $serializer.serialize_array(&$self.$field_name);
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
    (__deserialize $self:ident, $deserializer:ident, $field_name:ident -> struct) => {
        {
            $self.$field_name = $deserializer.deserialize_struct();
        }
    };
    (__deserialize $self:ident, $deserializer:ident, $field_name:ident -> array) => {
        {
            $self.$field_name = $deserializer.deserialize_array();
        }
    };
}

#[macro_export]
macro_rules! serialize2_variant {
    ($enum_name:ident { $( $variant:path => ($deser:expr, $tag:expr) ,)+ }) => {
        impl $crate::ser::Serialize for $enum_name {
            fn serialize<T: $crate::ser::Serializer>(&self, serializer: &mut T) {
                match self {
                $(
                    &$variant(ref v) => {
                        serializer.serialize_num(u8::from($tag));
                        serializer.serialize_struct(v);
                    }
                )+
                }

            }
        }

        impl $crate::de::Deserialize for $enum_name {
            fn deserialize<T: $crate::de::Deserializer>(deserializer: &mut T) -> $enum_name {
                let tag: u8 = deserializer.deserialize_num();
                match tag {
                $(
                    $tag => $variant($deser(deserializer)),
                )+
                    // TODO: again throw an error and stop panicking around there.
                    _ => panic!("invalid varian tag"),
                }
            }
        }
    }
}
