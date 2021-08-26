// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::ops::Range;

use chrono::{TimeZone, Utc};

use crate::{Datom, DatomType, Index, TransactionRecord, Value, ID};

const fn u64_byte_count() -> usize {
    0u64.to_be_bytes().len()
}

const fn i64_byte_count() -> usize {
    0i64.to_be_bytes().len()
}

fn serialize_v(v: &Value) -> Vec<u8> {
    let mut v_bytes = v.bytes();
    let byte_count = v_bytes.len() as u64;
    let mut vec = byte_count.to_be_bytes().to_vec();
    vec.append(&mut v_bytes);
    vec
}

fn deserialize_byte(bytes: &[u8]) -> (u8, &[u8]) {
    (bytes[0], &bytes[1..])
}

fn deserialize_id(bytes: &[u8]) -> Option<(ID, &[u8])> {
    let sized_bytes: [u8; 16] = bytes[0..16].try_into().ok()?;
    Some((sized_bytes.into(), &bytes[16..]))
}

fn deserialize_u64(bytes: &[u8]) -> Option<(u64, &[u8])> {
    let u = u64::from_be_bytes(bytes[0..u64_byte_count()].try_into().ok()?);
    Some((u, &bytes[u64_byte_count()..]))
}

fn deserialize_i64(bytes: &[u8]) -> Option<(i64, &[u8])> {
    let u = i64::from_be_bytes(bytes[0..i64_byte_count()].try_into().ok()?);
    Some((u, &bytes[i64_byte_count()..]))
}

fn deserialize_v(bytes: &[u8]) -> Option<(Value, &[u8])> {
    let (byte_count, bytes) = deserialize_u64(bytes)?;
    let byte_count = byte_count as usize;
    let v = Value::from_bytes(&bytes[0..byte_count])?;
    Some((v, &bytes[byte_count..]))
}

fn deserialize_datom_type(bytes: &[u8]) -> (DatomType, &[u8]) {
    let byte = bytes[0];
    let datom_type = DatomType::from_byte(byte);
    (datom_type, &bytes[1..])
}

/// Serialize a [datom](crate::Datom) in entity-attribute-value-t order
pub fn serialize_eavt(datom: &Datom) -> Vec<u8> {
    let mut v = vec![Index::EAVT.byte()];
    v.append(&mut <[u8; 16]>::from(datom.entity).to_vec());
    v.append(&mut <[u8; 16]>::from(datom.attribute).to_vec());
    v.append(&mut serialize_v(&datom.value));
    v.append(&mut datom.t.to_be_bytes().to_vec());
    v.push(datom.datom_type.byte());
    v
}

/// Serialize a [datom](crate::Datom) in attribute-entity-value-t order
pub fn serialize_aevt(datom: &Datom) -> Vec<u8> {
    let mut v = vec![Index::AEVT.byte()];
    v.append(&mut <[u8; 16]>::from(datom.attribute).to_vec());
    v.append(&mut <[u8; 16]>::from(datom.entity).to_vec());
    v.append(&mut serialize_v(&datom.value));
    v.append(&mut datom.t.to_be_bytes().to_vec());
    v.push(datom.datom_type.byte());
    v
}

/// Serialize a [datom](crate::Datom) in attribute-value-entity-t order
pub fn serialize_avet(datom: &Datom) -> Vec<u8> {
    let mut v = vec![Index::AVET.byte()];
    v.append(&mut <[u8; 16]>::from(datom.attribute).to_vec());
    v.append(&mut serialize_v(&datom.value));
    v.append(&mut <[u8; 16]>::from(datom.entity).to_vec());
    v.append(&mut datom.t.to_be_bytes().to_vec());
    v.push(datom.datom_type.byte());
    v
}

/// Serialize a [datom](crate::Datom) in value-attribute-entity-t order
pub fn serialize_vaet(datom: &Datom) -> Vec<u8> {
    let mut v = vec![Index::VAET.byte()];
    v.append(&mut serialize_v(&datom.value));
    v.append(&mut <[u8; 16]>::from(datom.attribute).to_vec());
    v.append(&mut <[u8; 16]>::from(datom.entity).to_vec());
    v.append(&mut datom.t.to_be_bytes().to_vec());
    v.push(datom.datom_type.byte());
    v
}

/// Serialize a [TransactionRecord]
pub fn serialize_tr(tr: &TransactionRecord) -> Vec<u8> {
    let t_bytes = tr.t.to_be_bytes();
    let timestamp_bytes = tr.timestamp.timestamp_millis().to_be_bytes();
    let mut v = [0; 1 + u64_byte_count() + i64_byte_count()];
    v[0] = 255;
    v[1..t_bytes.len() + 1].copy_from_slice(&t_bytes);
    v[t_bytes.len() + 1..].copy_from_slice(&timestamp_bytes);
    v.to_vec()
}

/// Create a range encompassing an entire index
///
/// ```
/// use datom::{serial, Index};
/// assert_eq!(serial::index_range(Index::EAVT), [0]..[1]);
/// ```
pub const fn index_range(index: Index) -> Range<[u8; 1]> {
    [index.byte()]..[index.byte() + 1]
}

/// Create a range encompassing every possible datom for a given entity
/// in the [EAVT index](crate::Index::EAVT)
///
/// ```
/// use datom::{serial, ID};
/// let id = ID::null();
/// let from = [0u8; 17];
/// let mut to = from;
/// to[16] = 1;
/// assert_eq!(serial::eavt_entity_range(id), from..to);
/// ```
pub fn eavt_entity_range(eid: ID) -> Range<[u8; 17]> {
    let mut from = [0; 17];
    let mut to = [0; 17];
    from[0] = Index::EAVT.byte();
    to[0] = Index::EAVT.byte();
    let eid_bytes: [u8; 16] = eid.into();
    let eid_u128 = u128::from_be_bytes(eid_bytes);
    let to_u128 = eid_u128 + 1;
    let to_bytes = to_u128.to_be_bytes();
    from[1..].copy_from_slice(&eid_bytes);
    to[1..].copy_from_slice(&to_bytes);
    from..to
}

/// Create a range encompassing every possible [datom](crate::Datom) for
/// a given entity and attribute in the [EAVT index](crate::Index::EAVT)
pub fn eavt_entity_attribute_range(eid: ID, aid: ID) -> Range<[u8; 33]> {
    let mut base = [0; 33];
    base[0] = Index::EAVT.byte();
    base[1..17].copy_from_slice(&<[u8; 16]>::from(eid));
    let aid_bytes = <[u8; 16]>::from(aid);
    let aid_u128 = u128::from_be_bytes(aid_bytes);
    let to_u128 = aid_u128 + 1;
    let to_bytes = to_u128.to_be_bytes();
    let mut from = base;
    from[17..].copy_from_slice(&aid_bytes);
    let mut to = base;
    to[17..].copy_from_slice(&to_bytes);
    from..to
}

/// Create a range encompassing every possible datom for a given
/// attribute and value in the [AVET index](crate::Index::AVET)
pub fn avet_attribute_value_range(eid: ID, val: Value) -> Range<Vec<u8>> {
    let mut val_serialized = serialize_v(&val);
    let mut from = vec![0u8; 17];
    from[0] = Index::AVET.byte();
    let eid_bytes: [u8; 16] = eid.into();
    from[1..17].copy_from_slice(&eid_bytes);
    from.append(&mut val_serialized);
    let mut to = from.clone();
    let mut i = to.len() - 1;
    while i > 0 {
        match to[i].checked_add(1) {
            Some(x) => {
                to[i] = x;
                break;
            }
            None => {
                i -= 1;
            }
        }
    }
    from..to
}

/// Create a range encompassing every possible datom for a given
/// value and attribute in the [VAET index](crate::Index::VAET)
pub fn vaet_value_attribute_range(val: Value, eid: ID) -> Range<Vec<u8>> {
    let mut val_serialized = serialize_v(&val);
    let mut from = Vec::with_capacity(1 + val_serialized.len() + 16);
    from.push(Index::VAET.byte());
    let eid_bytes: [u8; 16] = eid.into();
    from.append(&mut val_serialized);
    from.extend_from_slice(&eid_bytes);
    let mut to = from.clone();
    let mut i = to.len() - 1;
    while i > 0 {
        match to[i].checked_add(1) {
            Some(x) => {
                to[i] = x;
                break;
            }
            None => {
                i -= 1;
            }
        }
    }
    from..to
}

/// Create a range encompassing every transaction result
pub fn tr_range() -> Range<Vec<u8>> {
    vec![255]..vec![255; 1 + u64_byte_count() + i64_byte_count()]
}

/// Convert a range of arrays to a range of slices
pub fn range_slice<T, const N: usize>(r: &'_ Range<[T; N]>) -> Range<&'_ [T]> {
    &r.start..&r.end
}

/// Convert a range of [Vec]s to a range of slices
pub fn vec_range_slice<T>(r: &'_ Range<Vec<T>>) -> Range<&'_ [T]> {
    &r.start..&r.end
}

/// Serialize a [datom](crate::Datom) for a given [index](crate::Index)
pub fn serialize(datom: &Datom, index: Index) -> Vec<u8> {
    match index {
        Index::EAVT => serialize_eavt(datom),
        Index::AEVT => serialize_aevt(datom),
        Index::AVET => serialize_avet(datom),
        Index::VAET => serialize_vaet(datom),
    }
}

/**
Deserialize a [datom](crate::Datom) in entity-attribute-value-t
order
*/
pub fn deserialize_eavt(bytes: &[u8]) -> Option<Datom> {
    let (_, bytes) = deserialize_byte(bytes);
    let (entity, bytes) = deserialize_id(bytes)?;
    let (attribute, bytes) = deserialize_id(bytes)?;
    let (value, bytes) = deserialize_v(bytes)?;
    let (t, bytes) = deserialize_u64(bytes)?;
    let (datom_type, _) = deserialize_datom_type(bytes);
    Some(Datom {
        entity,
        attribute,
        value,
        t,
        datom_type,
    })
}

/// Deserialize a [datom](crate::Datom) in attribute-entity-value-t order
pub fn deserialize_aevt(bytes: &[u8]) -> Option<Datom> {
    let (_, bytes) = deserialize_byte(bytes);
    let (attribute, bytes) = deserialize_id(bytes)?;
    let (entity, bytes) = deserialize_id(bytes)?;
    let (value, bytes) = deserialize_v(bytes)?;
    let (t, bytes) = deserialize_u64(bytes)?;
    let (datom_type, _) = deserialize_datom_type(bytes);
    Some(Datom {
        entity,
        attribute,
        value,
        t,
        datom_type,
    })
}

/// Deserialize a [datom](crate::Datom) in attribute-value-entity-t order
pub fn deserialize_avet(bytes: &[u8]) -> Option<Datom> {
    let (_, bytes) = deserialize_byte(bytes);
    let (attribute, bytes) = deserialize_id(bytes)?;
    let (value, bytes) = deserialize_v(bytes)?;
    let (entity, bytes) = deserialize_id(bytes)?;
    let (t, bytes) = deserialize_u64(bytes)?;
    let (datom_type, _) = deserialize_datom_type(bytes);
    Some(Datom {
        entity,
        attribute,
        value,
        t,
        datom_type,
    })
}

/// Deserialize a [datom](crate::Datom) in value-attribute-entity-t order
pub fn deserialize_vaet(bytes: &[u8]) -> Option<Datom> {
    let (_, bytes) = deserialize_byte(bytes);
    let (value, bytes) = deserialize_v(bytes)?;
    let (attribute, bytes) = deserialize_id(bytes)?;
    let (entity, bytes) = deserialize_id(bytes)?;
    let (t, bytes) = deserialize_u64(bytes)?;
    let (datom_type, _) = deserialize_datom_type(bytes);
    Some(Datom {
        entity,
        attribute,
        value,
        t,
        datom_type,
    })
}

/// Deserialize a [TransactionRecord]
pub fn deserialize_tr(bytes: &[u8]) -> Option<TransactionRecord> {
    let (_, bytes) = deserialize_byte(bytes);
    let (t, bytes) = deserialize_u64(bytes)?;
    let (ts_millis, _) = deserialize_i64(bytes)?;
    Some(TransactionRecord {
        t,
        timestamp: Utc.timestamp_millis(ts_millis),
    })
}

/// Deserialize a [datom](crate::Datom) from a given [index](crate::Index)
pub fn deserialize(bytes: &[u8], index: Index) -> Option<Datom> {
    match index {
        Index::EAVT => deserialize_eavt(bytes),
        Index::AEVT => deserialize_aevt(bytes),
        Index::AVET => deserialize_avet(bytes),
        Index::VAET => deserialize_vaet(bytes),
    }
}

/// Deserialize a [datom](crate::Datom) from any [index](crate::Index)
///
/// ```
/// use datom::serial::*;
/// use datom::{Datom, ID, DatomType, Index::*};
/// let my_datom = Datom {
///     entity: ID::new(),
///     attribute: ID::new(),
///     value: "Val".into(),
///     t: 0,
///     datom_type: DatomType::Addition
/// };
/// let eavt = serialize(&my_datom, EAVT);
/// let aevt = serialize(&my_datom, AEVT);
/// let avet = serialize(&my_datom, AVET);
/// let vaet = serialize(&my_datom, VAET);
/// assert_eq!(deserialize_unknown(&eavt), Some((my_datom.clone(), EAVT)));
/// assert_eq!(deserialize_unknown(&aevt), Some((my_datom.clone(), AEVT)));
/// assert_eq!(deserialize_unknown(&avet), Some((my_datom.clone(), AVET)));
/// assert_eq!(deserialize_unknown(&vaet), Some((my_datom.clone(), VAET)));
/// ```
pub fn deserialize_unknown(bytes: &[u8]) -> Option<(Datom, Index)> {
    let (index_byte, _) = deserialize_byte(bytes);
    let index = Index::from_byte(index_byte);
    Some((deserialize(bytes, index)?, index))
}
