// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later
#![allow(non_camel_case_types)]

use crate::shared_memory::{FaultRecord, FullAddress, MemoryAttributes};

pub enum ATAccess {
    ATAccess_Read,
    ATAccess_Write,
    ATAccess_ReadPAN,
    ATAccess_WritePAN,
}

/// Library pseudocode for shared/translation/vmsa/AddressDescriptor

pub struct AddressDescriptor {
    /// fault.statuscode indicates whether the address is valid
    fault: FaultRecord,
    memattrs: MemoryAttributes,
    paddress: FullAddress,
    /// bits(64)
    vaddress: u64,
}

/// Library pseudocode for shared/translation/vmsa/Regime

/// Regime
/// ======
/// Translation regimes
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Regime {
    /// EL3
    Regime_EL3,
    /// EL3&0 (PL1&0 when EL3 is AArch32)
    Regime_EL30,
    /// EL2
    Regime_EL2,
    /// EL2&0
    Regime_EL20,
    /// EL1&0
    Regime_EL10,
}
