// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{convert::TryInto, ops::Range};

use crate::{Datom, DatomType, Index, Value, ID};

const fn u64_byte_count() -> usize {
    0u64.to_be_bytes().len()
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

/**
Create a range encompassing every possible datom for a given entity
in the [EAVT index](crate::Index::EAVT)
*/
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

/**
Create a range encompassing every possible [datom](crate::Datom) for
a given entity and attribute in the [EAVT index](crate::Index::EAVT)
*/
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

/**
Create a range encompassing every possible datom for a given attribute
in the [AVET index](crate::Index::AVET)
*/
pub fn avet_attribute_range(eid: ID) -> Range<[u8; 17]> {
    let mut from = [0; 17];
    let mut to = [0; 17];
    from[0] = Index::AVET.byte();
    to[0] = Index::AVET.byte();
    let eid_bytes: [u8; 16] = eid.into();
    let eid_u128 = u128::from_be_bytes(eid_bytes);
    let to_u128 = eid_u128 + 1;
    let to_bytes = to_u128.to_be_bytes();
    from[1..].copy_from_slice(&eid_bytes);
    to[1..].copy_from_slice(&to_bytes);
    from..to
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

/// Deserialize a [datom](crate::Datom) from a given [index](crate::Index)
pub fn deserialize(bytes: &[u8], index: Index) -> Option<Datom> {
    match index {
        Index::EAVT => deserialize_eavt(bytes),
        Index::AEVT => deserialize_aevt(bytes),
        Index::AVET => deserialize_avet(bytes),
        Index::VAET => deserialize_vaet(bytes),
    }
}

/**
Deserialize a [datom](crate::Datom) from any [index](crate::Index)

```
use datom::serial::*;
use datom::{Datom, ID, DatomType, Index::*};
let my_datom = Datom {
    entity: ID::new(),
    attribute: ID::new(),
    value: "Val".into(),
    t: 0,
    datom_type: DatomType::Addition
};
let eavt = serialize(&my_datom, EAVT);
let aevt = serialize(&my_datom, AEVT);
let avet = serialize(&my_datom, AVET);
let vaet = serialize(&my_datom, VAET);
assert_eq!(deserialize_unknown(&eavt), Some((my_datom.clone(), EAVT)));
assert_eq!(deserialize_unknown(&aevt), Some((my_datom.clone(), AEVT)));
assert_eq!(deserialize_unknown(&avet), Some((my_datom.clone(), AVET)));
assert_eq!(deserialize_unknown(&vaet), Some((my_datom.clone(), VAET)));
```
*/
pub fn deserialize_unknown(bytes: &[u8]) -> Option<(Datom, Index)> {
    let (index_byte, _) = deserialize_byte(bytes);
    let index = Index::from_byte(index_byte);
    Some((deserialize(bytes, index)?, index))
}
