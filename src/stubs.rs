// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later
#![allow(non_camel_case_types)]
use crate::shared::*;
use crate::shared_memory::*;
use crate::shared_translation::*;
use crate::shared_vmsa::*;
/*
                AArch64S1Walk(fault, walkparams, va, regime, accdesc, 128);
            SetInGuardedPage(walkstate.guardedpage == '1');
        if AArch64S1HasAlignmentFault(accdesc, aligned, walkparams.ntlsmd, walkstate.memattrs) {
        fault = AArch64S1CheckPermissions(fault, regime, walkstate, walkparams, accdesc);
        if walkparams.ha == '1' && AArch64SettingAccessFlagPermitted(fault) {
        if (AArch64SettingDirtyStatePermitted(fault) &&
            let mut descaccess = CreateAccDescTTEUpdate(accdesc);
    let oa = StageOA(va, walkparams.d128, walkparams.tgx, walkstate);
        memattrs = NormalNCMemAttr();
              !ConstrainUnpredictableBool(Unpredictable_S1CTAGGED)) {
    let mut ipa = CreateAddressDescriptor(va, oa, memattrs);
    ipa.mecid = AArch64S1OutputMECID(walkparams, regime, varange, ipa.paddress.paspace,
                                      descriptor);
*/

pub fn AArch64S1ICacheEnabled(_: Regime) -> bool {
    true
}

pub fn AArch64S1DCacheEnabled(_: Regime) -> bool {
    true
}

pub fn IsFeatureImplemented(_feat: &str) -> bool {
    true
}

pub fn AArch64S1Enabled(_: Regime, _: AccessType) -> bool {
    true
}

pub fn AArch64S1DisabledOutput(
    _fault: FaultRecord,
    _regime: Regime,
    _va: u64,
    _accdesc: AccessDescriptor,
    _aligned: bool,
) -> (FaultRecord, AddressDescriptor) {
    todo!()
}

pub fn AArch64S1TxSZFaults(_regime: Regime, _walkparams: S1TTWParams) -> bool {
    true
}
pub fn AArch64GetS1TTWParams(_: Regime, _: SecurityState, _va: u64) -> S1TTWParams {
    todo!()
}

pub fn AArch64S1MinTxSZ(
    _: Regime,
    _walkparamsd128: u64,
    _walkparamsds: u64,
    _walkparamstgx: TGx,
) -> u64 {
    todo!()
}

pub fn AArch64MaxTxSZ(_walkparamstgx: TGx) -> u64 {
    todo!()
}
pub fn AArch64VAIsOutOfRange(
    _va: u64,
    _acctype: AccessType,
    _regime: Regime,
    _walkparams: S1TTWParams,
) -> bool {
    todo!()
}

pub fn SetInGuardedPage(_cond: bool) {
    todo!()
}

pub fn AArch64S1HasAlignmentFault(
    _accdesc: AccessDescriptor,
    _aligned: bool,
    _walkparamsntlsmd: u64,
    _walkstatememattrs: MemoryAttributes,
) -> bool {
    todo!()
}

pub fn AArch64S1CheckPermissions(
    _fault: FaultRecord,
    _regime: Regime,
    _walkstate: TTWState,
    _walkparams: S1TTWParams,
    _accdesc: AccessDescriptor,
) -> FaultRecord {
    todo!()
}

pub fn AArch64SettingAccessFlagPermitted(_fault: FaultRecord) -> bool {
    todo!()
}

pub fn AArch64SettingDirtyStatePermitted(_fault: FaultRecord) -> bool {
    todo!()
}

pub fn SecureOnlyImplementation() -> bool {
    // TODO
    false
}

/// Library pseudocode for shared/functions/system/ELUsingAArch32
/// ELUsingAArch32()
/// ================
pub fn ELUsingAArch32(_el: PrivilegeLevel) -> bool {
    //return ELStateUsingAArch32(el, IsSecureBelowEL3());
    false
}

mycelium_bitfield::bitfield! {
    pub struct SCR_EL3_REG<u64> {
        pub const NS = 1;
        //pub const IRQ = 1;
        // TODO
    }
}

pub static SCR_EL3: SCR_EL3_REG = SCR_EL3_REG::new();

mycelium_bitfield::bitfield! {
    pub struct SCR_REG<u32> {
        pub const NS = 1;
        //pub const IRQ = 1;
        // TODO
    }
}

pub static SCR: SCR_REG = SCR_REG::new();

/// Library pseudocode for aarch64/translation/vmsa_walk/AArch64.S1Walk
/// AArch64.S1Walk()
/// ================
/// Traverse stage 1 translation tables obtaining the final descriptor
/// as well as the address leading to that descriptor
pub fn AArch64S1Walk_128(
    _fault_in: FaultRecord,
    _walkparams: S1TTWParams,
    _va: u64,
    _regime: Regime,
    _accdesc: AccessDescriptor,
) -> (FaultRecord, AddressDescriptor, TTWState, u128) {
    todo!()
}
