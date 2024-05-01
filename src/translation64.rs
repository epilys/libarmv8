// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later
#![allow(non_camel_case_types)]

use std::mem::MaybeUninit;

use crate::shared::*;
use crate::shared_mec::*;
use crate::shared_memory::*;
use crate::shared_translation::*;
use crate::shared_vmsa::*;
use crate::stubs::*;

/// Library pseudocode for aarch64/translation/vmsa_translation/AArch64FullTranslate
///
/// Address translation as specified by VMSA
/// Alignment check NOT due to memory type is expected to be done before translation
pub fn AArch64FullTranslate(
    va: u64,
    accdesc: AccessDescriptor,
    aligned: bool,
) -> AddressDescriptor {
    let regime: Regime = TranslationRegime(accdesc.el);
    let fault = FaultRecord::NoFaultForAccess(accdesc);

    let _ipa: AddressDescriptor;
    let (fault, ipa) = AArch64S1Translate(fault, regime, va, aligned, accdesc);

    if !matches!(fault.statuscode, Fault::Fault_None) {
        return CreateFaultyAddressDescriptor(va, fault);
    }

    // if accdesc.ss == SS_Realm {
    //     assert!(EL2Enabled());
    // }
    if regime == Regime::Regime_EL10 && EL2Enabled() {
        let s1aarch64 = true;
        let (fault, pa) = AArch64S2Translate(fault, ipa, s1aarch64, aligned, accdesc);

        if !matches!(fault.statuscode, Fault::Fault_None) {
            return CreateFaultyAddressDescriptor(va, fault);
        }
        return pa;
    }
    ipa
}

/// Library pseudocode for aarch64/translation/vmsa_translation/AArch64S1Translate
///
/// AArch64S1Translate()
/// =====================
/// Translate VA to IPA/PA depending on the regime
pub fn AArch64S1Translate(
    fault_in: FaultRecord,
    regime: Regime,
    va: u64,
    aligned: bool,
    accdesc: AccessDescriptor,
) -> (FaultRecord, AddressDescriptor) {
    let mut fault: FaultRecord = fault_in;
    // // Prepare fault fields in case a fault is detected
    fault.secondstage = false;
    fault.s2fs1walk = false;
    if !AArch64S1Enabled(regime, accdesc.acctype) {
        return AArch64S1DisabledOutput(fault, regime, va, accdesc, aligned);
    }
    let walkparams = AArch64GetS1TTWParams(regime, accdesc.ss, va);
    let s1mintxsz = AArch64S1MinTxSZ(
        regime,
        walkparams.get_d128(),
        walkparams.get_ds(),
        walkparams.get_tgx(),
    );
    let s1maxtxsz = AArch64MaxTxSZ(walkparams.get_tgx());

    if AArch64S1TxSZFaults(regime, walkparams) {
        fault.statuscode = Fault::Fault_Translation;
        fault.level = 0;
        return (fault, AddressDescriptor::UNKNOWN);
    } else if walkparams.get_txsz() < s1mintxsz {
        //walkparams.txsz = s1mintxsz<5:0>;
    } else if walkparams.get_txsz() > s1maxtxsz {
        //walkparams.txsz = s1maxtxsz<5:0>;
    }

    if AArch64VAIsOutOfRange(va, accdesc.acctype, regime, walkparams) {
        fault.statuscode = Fault::Fault_Translation;
        fault.level = 0;
        return (fault, AddressDescriptor::UNKNOWN);
    }

    if accdesc.el == EL0 && walkparams.get_e0pd() == 1 {
        fault.statuscode = Fault::Fault_Translation;
        fault.level = 0;
        return (fault, AddressDescriptor::UNKNOWN);
    }

    if IsFeatureImplemented("FEAT_TME")
        && accdesc.el == EL0
        && walkparams.get_nfd() == 1
        && accdesc.transactional
    {
        fault.statuscode = Fault::Fault_Translation;
        fault.level = 0;
        return (fault, AddressDescriptor::UNKNOWN);
    }

    if IsFeatureImplemented("FEAT_SVE")
        && accdesc.el == EL0
        && walkparams.get_nfd() == 1
        && ((accdesc.nonfault && accdesc.contiguous)
            || (accdesc.firstfault && !accdesc.first && !accdesc.contiguous))
    {
        fault.statuscode = Fault::Fault_Translation;
        fault.level = 0;
        return (fault, AddressDescriptor::UNKNOWN);
    }

    let mut descipaddr: AddressDescriptor = unsafe { MaybeUninit::zeroed().assume_init_read() };
    let mut walkstate: TTWState = unsafe { MaybeUninit::zeroed().assume_init_read() };
    let mut descriptor: u128 = 0;
    let mut new_desc: u128;
    let mem_desc: u128 = 0;
    loop {
        if walkparams.get_d128() == 1 {
            (fault, descipaddr, walkstate, descriptor) =
                AArch64S1Walk_128(fault, walkparams, va, regime, accdesc);
        } else {
            //(fault, descipaddr, walkstate, descriptor<63:0>) = AArch64S1Walk(fault, walkparams, va, regime, accdesc, 64);
        }
        //descriptor<127:64> = Zeros(64);
        if fault.statuscode != Fault::Fault_None {
            return (fault, AddressDescriptor::UNKNOWN);
        }
        if accdesc.acctype == AccessType::AccessType_IFETCH {
            // Flag the fetched instruction is from a guarded page
            // FIXME walkstate might be uninitialized SetInGuardedPage(walkstate.guardedpage);
        }
        if AArch64S1HasAlignmentFault(
            accdesc,
            aligned,
            walkparams.get_ntlsmd(),
            walkstate.memattrs,
        ) {
            fault.statuscode = Fault::Fault_Alignment;
        }
        if fault.statuscode == Fault::Fault_None {
            fault = AArch64S1CheckPermissions(fault, regime, walkstate, walkparams, accdesc);
        }
        new_desc = descriptor;
        if walkparams.get_ha() == 1 && AArch64SettingAccessFlagPermitted(fault) {
            // Set descriptor AF bit
            // TODO new_desc<10> = 0b1;
        }

        // If HW update of dirty bit is enabled, the walk state permissions
        // will already reflect a configuration permitting writes.
        // The update of the descriptor occurs only if the descriptor bits in
        // memory do not reflect that and the access instigates a write.

        if AArch64SettingDirtyStatePermitted(fault)
            && walkparams.get_ha() == 1
            && walkparams.get_hd() == 1
            && (walkparams.get_pie() == 1/* FIXME || descriptor<51> == 0b1 */)
            && accdesc.write
            && !([
                AccessType::AccessType_AT,
                AccessType::AccessType_IC,
                AccessType::AccessType_DC,
            ]
            .contains(&accdesc.acctype))
        {
            // Clear descriptor AP[2]/nDirty bit permitting stage 1 writes
            // TODO new_desc<7> = 0b0;
        }

        // Either the access flag was clear or AP[2]/nDirty is set
        if new_desc != descriptor {
            let descpaddr: AddressDescriptor;
            let descaccess = CreateAccDescTTEUpdate(accdesc);
            if regime == Regime::Regime_EL10 && EL2Enabled() {
                let s2fault: FaultRecord;
                let s1aarch64 = true;
                let s2aligned = true;
                let v: (FaultRecord, AddressDescriptor) =
                    AArch64S2Translate(fault, descipaddr, s1aarch64, s2aligned, descaccess);
                s2fault = v.0;
                descpaddr = v.1;

                if s2fault.statuscode != Fault::Fault_None {
                    return (s2fault, AddressDescriptor::UNKNOWN);
                }
            } else {
                descpaddr = descipaddr;
            }
            if walkparams.get_d128() == 1 {
                //(fault, mem_desc) = AArch64MemSwapTableDesc(fault, descriptor, new_desc, walkparams.get_ee(), descaccess, descpaddr);
            } else {
                //(fault, mem_desc<63:0>) = AArch64MemSwapTableDesc(fault, descriptor<63:0>, new_desc<63:0>, walkparams.get_ee(), descaccess, descpaddr);
                //mem_desc<127:64> = Zeros(64);
            }
        }
        if new_desc == descriptor || mem_desc == new_desc {
            break;
        }
    }
    if fault.statuscode != Fault::Fault_None {
        return (fault, AddressDescriptor::UNKNOWN);
    }

    // // Output Address
    let oa = StageOA(va, walkparams.get_d128(), walkparams.get_tgx(), walkstate);
    let mut memattrs: MemoryAttributes;
    if accdesc.acctype == AccessType::AccessType_IFETCH
        && (walkstate.memattrs.memtype == MemType::MemType_Device
            || !AArch64S1ICacheEnabled(regime))
    {
        // Treat memory attributes as Normal Non-Cacheable
        memattrs = NormalNCMemAttr();
        memattrs.xs = walkstate.memattrs.xs;
    } else if accdesc.acctype != AccessType::AccessType_IFETCH
        && !AArch64S1DCacheEnabled(regime)
        && walkstate.memattrs.memtype == MemType::MemType_Normal
    {
        // Treat memory attributes as Normal Non-Cacheable
        memattrs = NormalNCMemAttr();
        memattrs.xs = walkstate.memattrs.xs;

        // The effect of SCTLR_ELx.C when 0b0 is Constrained UNPREDICTABLE
        // on the Tagged attribute
        // if (IsFeatureImplemented("FEAT_MTE2") &&
        //       walkstate.memattrs.tags == MemTagType::MemTag_AllocationTagged &&
        //       !ConstrainUnpredictableBool(Unpredictable_S1CTAGGED)) {
        //     memattrs.tags = MemTagType::MemTag_Untagged;
        //     }
    } else {
        memattrs = walkstate.memattrs;
    }

    // Shareability value of stage 1 translation subject to stage 2 is IMPLEMENTATION DEFINED
    // to be either effective value or descriptor value
    // if (regime == Regime::Regime_EL10 && EL2Enabled() && HCR_EL2.VM == 0b1 && !(boolean IMPLEMENTATION_DEFINED "Apply effective shareability at stage 1")) then
    // memattrs.shareability = walkstate.memattrs.shareability;
    // else
    // memattrs.shareability = EffectiveShareability(memattrs);
    memattrs.shareability = EffectiveShareability(memattrs);

    if accdesc.ls64 && memattrs.memtype == MemType::MemType_Normal {
        if memattrs.inner.attrs != MemAttr::MemAttr_NC
            || memattrs.outer.attrs != MemAttr::MemAttr_NC
        {
            fault.statuscode = Fault::Fault_Exclusive;
            return (fault, AddressDescriptor::UNKNOWN);
        }
    }

    let mut ipa = CreateAddressDescriptor(va, oa, memattrs);
    ipa.s1assured = walkstate.s1assured;
    let varange = AArch64GetVARange(va);
    ipa.mecid = AArch64S1OutputMECID_128(
        walkparams,
        regime,
        varange,
        ipa.paddress.paspace,
        descriptor,
    );
    return (fault, ipa);
}

/// Library pseudocode for aarch64/translation/vmsa_translation/AArch64S2Translate
/// AArch64S2Translate()
/// =====================
/// Translate stage 1 IPA to PA and combine memory attributes
pub fn AArch64S2Translate(
    _fault_in: FaultRecord,
    _ipa: AddressDescriptor,
    _s1aarch64: bool,
    _aligned: bool,
    _accdesc: AccessDescriptor,
) -> (FaultRecord, AddressDescriptor) {
    todo!()
    // walkparams = AArch64GetS2TTWParams(accdesc.ss, ipa.paddress.paspace, s1aarch64);
    // FaultRecord fault = fault_in;
    // bool s2fs1mro;
    // // Prepare fault fields in case a fault is detected
    // fault.statuscode = Fault::Fault_None; // Ignore any faults from stage 1
    // fault.dirtybit = FALSE;
    // fault.overlay = FALSE;
    // fault.tagaccess = FALSE;
    // fault.s1tagnotdata = FALSE;
    // fault.secondstage = TRUE;
    // fault.s2fs1walk = accdesc.acctype == AccessType_TTW;
    // fault.ipaddress = ipa.paddress;
    // if walkparams.vm != 0b1 then
    // // Stage 2 translation is disabled
    // return (fault, ipa);
    // constant integer s2mintxsz = AArch64S2MinTxSZ(walkparams.d128, walkparams.ds,
    // walkparams.tgx, s1aarch64);
    // constant integer s2maxtxsz = AArch64MaxTxSZ(walkparams.tgx);
    // if AArch64S2TxSZFaults(walkparams, s1aarch64) then
    // fault.statuscode = Fault::Fault_Translation;
    // fault.level = 0;
    // return (fault, AddressDescriptor::UNKNOWN);
    // elsif UInt(walkparams.txsz) < s2mintxsz then
    // walkparams.txsz = s2mintxsz<5:0>;
    // elsif UInt(walkparams.txsz) > s2maxtxsz then
    // walkparams.txsz = s2maxtxsz<5:0>;
    // if (walkparams.d128 == 0b0 &&
    // (AArch64S2InvalidSL(walkparams) || AArch64S2InconsistentSL(walkparams))) then
    // fault.statuscode = Fault::Fault_Translation;
    // fault.level = 0;
    // return (fault, AddressDescriptor::UNKNOWN);
    // if AArch64IPAIsOutOfRange(ipa.paddress.address, walkparams) then
    // fault.statuscode = Fault::Fault_Translation;
    // fault.level = 0;
    // return (fault, AddressDescriptor::UNKNOWN);
    // AddressDescriptor descpaddr;
    // TTWState walkstate;
    // bits(128) descriptor;
    // bits(128) new_desc;
    // bits(128) mem_desc;
    // repeat
    // if walkparams.d128 == 0b1 then
    // (fault, descpaddr, walkstate, descriptor) = AArch64S2Walk(fault, ipa, walkparams,
    // accdesc, 128);
    // else
    // (fault, descpaddr, walkstate, descriptor<63:0>) = AArch64S2Walk(fault, ipa,
    // walkparams, accdesc,
    // 64);
    // descriptor<127:64> = Zeros(64);
    // if fault.statuscode != Fault::Fault_None then
    // return (fault, AddressDescriptor::UNKNOWN);
    // if AArch64S2HasAlignmentFault(accdesc, aligned, walkstate.memattrs) then
    // fault.statuscode = Fault::Fault_Alignment;
    // if fault.statuscode == Fault::Fault_None then
    // (fault, s2fs1mro) = AArch64S2CheckPermissions(fault, walkstate, walkparams, ipa,
    // accdesc);
    // new_desc = descriptor;
    // if walkparams.ha == 0b1 && AArch64SettingAccessFlagPermitted(fault) then
    // // Set descriptor AF bit
    // new_desc<10> = 0b1;
    // // If HW update of dirty bit is enabled, the walk state permissions
    // // will already reflect a configuration permitting writes.
    // // The update of the descriptor occurs only if the descriptor bits in
    // // memory do not reflect that and the access instigates a write.
    // if (AArch64SettingDirtyStatePermitted(fault) &&
    // walkparams.ha == 0b1 &&
    // walkparams.hd == 0b1 &&
    // (walkparams.s2pie == 0b1 || descriptor<51> == 0b1) &&
    // accdesc.write &&
    // !(accdesc.acctype IN {AccessType_AT, AccessType_IC, AccessType_DC})) then
    // // Set descriptor S2AP[1]/Dirty bit permitting stage 2 writes
    // new_desc<7> = 0b1;
    // // Either the access flag was clear or S2AP[1]/Dirty is clear
    // if new_desc != descriptor then
    // if walkparams.hdbss == 0b1 && descriptor<7> == 0b0 && new_desc<7> == 0b1 then
    // fault = AppendToHDBSS(fault, ipa.paddress, accdesc, walkparams, walkstate.level);
    // // If an error, other than a synchronous External abort, occurred on the HDBSS update,
    // // stage 2 hardware update of dirty state is not permitted.
    // if (HDBSSPROD_EL2.FSC != 0b101000 &&
    // (!fault.hdbssf || IsExternalAbort(fault.statuscode))) then
    // AccessDescriptor descaccess = CreateAccDescTTEUpdate(accdesc);
    // if walkparams.d128 == 0b1 then
    // (fault, mem_desc) = AArch64MemSwapTableDesc(fault, descriptor, new_desc,
    // walkparams.ee, descaccess,
    // descpaddr);
    // else
    // (fault, mem_desc<63:0>) = AArch64MemSwapTableDesc(fault, descriptor<63:0>,
    // new_desc<63:0>,
    // walkparams.ee,
    // descaccess, descpaddr);
    // mem_desc<127:64> = Zeros(64);
    // if fault.statuscode != Fault::Fault_None then
    // return (fault, AddressDescriptor::UNKNOWN);
    // until new_desc == descriptor || mem_desc == new_desc;
    // if fault.statuscode != Fault::Fault_None then
    // return (fault, AddressDescriptor::UNKNOWN);
    // ipa_64 = ZeroExtend(ipa.paddress.address, 64);
    // // Output Address
    // oa = StageOA(ipa_64, walkparams.d128, walkparams.tgx, walkstate);
    // MemoryAttributes s2_memattrs;
    // if ((accdesc.acctype == AccessType_TTW &&
    // walkstate.memattrs.memtype == MemType_Device && walkparams.ptw == 0b0) ||
    // (accdesc.acctype == AccessType_IFETCH &&
    // (walkstate.memattrs.memtype == MemType_Device || HCR_EL2.ID == 0b1)) ||
    // (accdesc.acctype != AccessType_IFETCH &&
    // walkstate.memattrs.memtype == MemType_Normal && !S2DCacheEnabled())) then
    // // Treat memory attributes as Normal Non-Cacheable
    // s2_memattrs = NormalNCMemAttr();
    // s2_memattrs.xs = walkstate.memattrs.xs;
    // else
    // s2_memattrs = walkstate.memattrs;
    // if accdesc.ls64 && s2_memattrs.memtype == MemType_Normal then
    // if s2_memattrs.inner.attrs != MemAttr_NC || s2_memattrs.outer.attrs != MemAttr_NC then
    // fault.statuscode = Fault::Fault_Exclusive;
    // return (fault, AddressDescriptor::UNKNOWN);
    // s2aarch64 = TRUE;
    // MemoryAttributes memattrs;
    // if walkparams.fwb == 0b0 then
    // memattrs = S2CombineS1MemAttrs(ipa.memattrs, s2_memattrs, s2aarch64);
    // else
    // memattrs = s2_memattrs;
    // pa = CreateAddressDescriptor(ipa.vaddress, oa, memattrs);
    // pa.s2fs1mro = s2fs1mro;
    // pa.mecid = AArch64S2OutputMECID(walkparams, pa.paddress.paspace, descriptor);
    // return (fault, pa);
}
