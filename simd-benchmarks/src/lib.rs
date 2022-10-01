#![cfg_attr(feature = "avx512", feature(avx512_target_feature))]
#![cfg_attr(feature = "avx512", feature(stdsimd))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod depth;
pub mod discrepancy;
pub mod find_byte;
pub mod sequences;
