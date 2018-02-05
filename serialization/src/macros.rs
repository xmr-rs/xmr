// TODO: Improve macro syntax
#[macro_export]
macro_rules! serialize2 {
    ($struct_name:ident { $( $name:ident -> ($($discriminator:tt)*) ,)* }) => {
        impl $crate::serializer::Serialize for $struct_name {
            fn serialize<T: $crate::serializer::Serializer>(&self, serializer: &mut T) {
                $(
                    serialize2!(__internal self, serializer, $name -> $($discriminator)*);
                )*
            }
        }
    };
    (__internal $self:ident, $serializer:ident, $field_name:ident -> num) => {
        {
            $serializer.tag(stringify!($field_name));
            $serializer.serialize_num($self.$field_name);
        }
    };
    (__internal $self:ident, $serializer:ident, $field_name:ident -> varint) => {
        {
            $serializer.tag(stringify!($field_name));
            $serializer.serialize_varint($self.$field_name);
        }
    };
    (__internal $self:ident, $serializer:ident, $field_name:ident -> uvarint) => {
        {
            $serializer.tag(stringify!($field_name));
            $serializer.serialize_uvarint($self.$field_name);
        }
    };

    (__internal $self:ident, $serializer:ident, $field_name:ident -> blob) => {
        {
            $serializer.tag(stringify!($field_name));
            $serializer.serialize_blob(&$self.$field_name);
        }
    };
}
