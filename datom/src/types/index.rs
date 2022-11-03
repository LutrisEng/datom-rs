// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

/**
The four indices used in the underlying data store. The names refer
to the serialization order, and by extension the sort order.

These indices are the same as are used in Datomic, and are described
in [Datomic's documentation] in great detail.

[Datomic's documentation]: https://docs.datomic.com/on-prem/query/indexes.html
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum Index {
    /**
    Entity-Attribute-Value-T index

    Provides information on an entity-by-entity basis.
    */
    EAVT,
    /**
    Attribute-Entity-Value-T index

    Provides access to all instances of each attribute.
    */
    AEVT,
    /**
    Attribute-Value-Entity-T index

    Provides efficient access to unique entities
    */
    AVET,
    /**
    Value-Attribute-Entity-T index

    Also known as the reverse index. Provides relationship access in
    reverse. For example, if a page has its owner as an attribute
    but not vice versa, this index allows a query to walk backwards
    from owner to page.
    */
    VAET,
}

impl Index {
    /// Map the [Index] to its byte representation
    pub const fn byte(&self) -> u8 {
        match self {
            Self::EAVT => 0,
            Self::AEVT => 1,
            Self::AVET => 2,
            Self::VAET => 3,
        }
    }

    /// Map byte representation to an [Index]
    pub const fn from_byte(b: u8) -> Self {
        match b {
            0 => Self::EAVT,
            1 => Self::AEVT,
            2 => Self::AVET,
            3 => Self::VAET,
            _ => panic!("invalid index"),
        }
    }
}
