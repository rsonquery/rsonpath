use crate::{
    automaton::Automaton,
    engine::{main::MainEngine, Compiler as _},
};
use serde::{
    de::{self, Visitor},
    ser::SerializeTuple as _,
    Deserialize, Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
enum BinaryVersion {
    /// Placeholder for any version in the past, used for tests.
    Past,
    /// Introduced binary serialization in v0.9.4.
    V1,
}

impl de::Expected for BinaryVersion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Past => write!(formatter, "Past"),
            Self::V1 => write!(formatter, "v0.9.4"),
        }
    }
}

impl Serialize for MainEngine {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tuple_ser = serializer.serialize_tuple(2)?;
        tuple_ser.serialize_element(&BinaryVersion::V1)?;
        tuple_ser.serialize_element(&self.automaton())?;
        tuple_ser.end()
    }
}

impl<'de> Deserialize<'de> for MainEngine {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let automaton = deserializer.deserialize_tuple(2, EngineVisitor)?;
        Ok(Self::from_compiled_query(automaton))
    }
}

struct EngineVisitor;

impl<'de> Visitor<'de> for EngineVisitor {
    type Value = Automaton;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "the binary version and the Automaton")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let version = seq.next_element::<BinaryVersion>()?;
        match version {
            Some(BinaryVersion::V1) => (),
            Some(v) => return Err(de::Error::custom(format!("binary version {v:?} is incompatible"))),
            None => return Err(de::Error::missing_field("version")),
        }
        let automaton = seq.next_element::<Automaton>()?;
        match automaton {
            Some(a) => Ok(a),
            None => Err(de::Error::missing_field("automaton")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        automaton::Automaton,
        engine::{Compiler, RsonpathEngine},
    };
    use serde::{ser::SerializeTuple, Serialize, Serializer};
    use std::error::Error;

    #[test]
    fn automaton_round_trip() -> Result<(), Box<dyn Error>> {
        let query_str = "$..phoneNumbers[*].number";
        let query = rsonpath_syntax::parse(query_str)?;
        let automaton = Automaton::new(&query)?;
        let engine = RsonpathEngine::from_compiled_query(automaton.clone());

        let json_string = serde_json::to_string(&engine)?;

        let round_trip: RsonpathEngine = serde_json::from_str(&json_string)?;

        assert_eq!(&automaton, round_trip.automaton());

        Ok(())
    }

    #[test]
    fn deserializing_from_older_version() -> Result<(), Box<dyn Error>> {
        struct OldEngine {
            automaton: Automaton,
        }
        impl Serialize for OldEngine {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut tuple_ser = serializer.serialize_tuple(2)?;
                tuple_ser.serialize_element(&BinaryVersion::Past)?;
                tuple_ser.serialize_element(&self.automaton)?;
                tuple_ser.end()
            }
        }

        let query_str = "$..phoneNumbers[*].number";
        let query = rsonpath_syntax::parse(query_str)?;
        let automaton = Automaton::new(&query)?;
        let engine = OldEngine { automaton };

        let json_string = serde_json::to_string(&engine)?;

        match serde_json::from_str::<RsonpathEngine>(&json_string) {
            Ok(_) => panic!("expected error"),
            Err(e) => assert!(e.to_string().contains("binary version Past is incompatible")),
        }

        Ok(())
    }

    mod proptests {
        use super::{Automaton, Compiler, RsonpathEngine};
        use pretty_assertions::assert_eq;
        use proptest::prelude::*;
        use rsonpath_syntax_proptest::{ArbitraryJsonPathQuery, ArbitraryJsonPathQueryParams};

        proptest! {
            #[test]
            fn main_engine_cbor_roundtrips(ArbitraryJsonPathQuery { parsed, .. } in prop::arbitrary::any_with::<ArbitraryJsonPathQuery>(
                ArbitraryJsonPathQueryParams {
                    only_rsonpath_supported_subset: true,
                    ..Default::default()
                }
            )) {
                use std::io;
                struct ReadBuf<'a> {
                    buf: &'a [u8],
                    idx: usize,
                }
                impl<'a> io::Read for &mut ReadBuf<'a> {
                    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
                        let len = std::cmp::min(self.buf.len() - self.idx, buf.len());
                        buf.copy_from_slice(&self.buf[self.idx..self.idx + len]);
                        self.idx += len;
                        Ok(len)
                    }
                }

                let automaton = match Automaton::new(&parsed) {
                    Ok(x) => Ok(x),
                    Err(crate::automaton::error::CompilerError::QueryTooComplex(_)) => Err(TestCaseError::Reject("query too complex".into())),
                    Err(e) => Err(e.into()),
                }?;
                let engine = RsonpathEngine::from_compiled_query(automaton.clone());

                let mut buf = vec![];
                ciborium::into_writer(&engine, &mut buf)?;

                let mut read = ReadBuf { buf: &buf, idx: 0 };
                let engine_deser = ciborium::from_reader::<RsonpathEngine, _>(&mut read)?;

                assert_eq!(&automaton, engine_deser.automaton());
            }

            #[test]
            fn main_engine_json_roundtrips(ArbitraryJsonPathQuery { parsed, .. } in prop::arbitrary::any_with::<ArbitraryJsonPathQuery>(
                ArbitraryJsonPathQueryParams {
                    only_rsonpath_supported_subset: true,
                    ..Default::default()
                }
            )) {
                let automaton = match Automaton::new(&parsed) {
                    Ok(x) => Ok(x),
                    Err(crate::automaton::error::CompilerError::QueryTooComplex(_)) => Err(TestCaseError::Reject("query too complex".into())),
                    Err(e) => Err(e.into()),
                }?;
                let engine = RsonpathEngine::from_compiled_query(automaton.clone());

                let json_str = serde_json::to_string(&engine)?;
                let engine_deser = serde_json::from_str::<RsonpathEngine>(&json_str)?;

                assert_eq!(&automaton, engine_deser.automaton());
            }

            #[test]
            fn main_engine_message_pack_roundtrips(ArbitraryJsonPathQuery { parsed, .. } in prop::arbitrary::any_with::<ArbitraryJsonPathQuery>(
                ArbitraryJsonPathQueryParams {
                    only_rsonpath_supported_subset: true,
                    ..Default::default()
                }
            )) {
                let automaton = match Automaton::new(&parsed) {
                    Ok(x) => Ok(x),
                    Err(crate::automaton::error::CompilerError::QueryTooComplex(_)) => Err(TestCaseError::Reject("query too complex".into())),
                    Err(e) => Err(e.into()),
                }?;
                let engine = RsonpathEngine::from_compiled_query(automaton.clone());

                let buf = rmp_serde::to_vec(&engine)?;
                let engine_deser = rmp_serde::from_slice::<RsonpathEngine>(&buf)?;

                assert_eq!(&automaton, engine_deser.automaton());
            }
        }
    }
}
