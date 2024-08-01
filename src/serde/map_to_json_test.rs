use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
macro_rules! treemap {
    () => {
        BTreeMap::new()
    };
    ($($k:expr => $v:expr),+ $(,)?) => {
        {
            let mut m = BTreeMap::new();
            $(
                m.insert($k, $v);
            )+
            m
        }
    };
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
struct F32Bits(u32);
impl Serialize for F32Bits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_f32(f32::from_bits(self.0))
    }
}


pub fn test(){


    // We store float bits so that we can derive Ord, and other traits. In a
    // real context the code might involve a crate like ordered-float.

    let map = treemap!(F32Bits(f32::INFINITY.to_bits()) => "x".to_owned());
    let result = serde_json::to_string(&map);
    dbg!(result);
    assert!(serde_json::to_string(&map).is_err());
    assert!(serde_json::to_value(map).is_err());

    let map = treemap!(F32Bits(f32::NEG_INFINITY.to_bits()) => "x".to_owned());
    assert!(serde_json::to_string(&map).is_err());
    assert!(serde_json::to_value(map).is_err());

    let map = treemap!(F32Bits(f32::NAN.to_bits()) => "x".to_owned());
    assert!(serde_json::to_string(&map).is_err());
    assert!(serde_json::to_value(map).is_err());

}