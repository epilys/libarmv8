// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later
#![allow(non_camel_case_types)]

use crate::shared::*;
use crate::shared_memory::*;
use crate::shared_translation::*;
use crate::shared_vmsa::*;

/// Library pseudocode for aarch64/translation/vmsa_translation/AArch64.FullTranslate
///
/// Address translation as specified by VMSA
/// Alignment check NOT due to memory type is expected to be done before translation
pub fn AArch64FullTranslate(
    va: u64,
    accdesc: AccessDescriptor,
    aligned: bool,
) -> AddressDescriptor {
    let regime: Regime = TranslationRegime(accdesc.el);
    let mut fault = FaultRecord::NoFaultForAccess(accdesc);

    let ipa: AddressDescriptor;
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

/// Library pseudocode for aarch64/translation/vmsa_translation/AArch64.S1Translate
///
/// AArch64.S1Translate()
/// =====================
/// Translate VA to IPA/PA depending on the regime
pub fn AArch64S1Translate(
    fault_in: FaultRecord,
    regime: Regime,
    va: u64,
    aligned: bool,
    accdesc: AccessDescriptor,
) -> (FaultRecord, AddressDescriptor) {
    todo!()
    // FaultRecord fault = fault_in;
    // // Prepare fault fields in case a fault is detected
    // fault.secondstage = FALSE;
    // fault.s2fs1walk = FALSE;
    // if !AArch64.S1Enabled(regime, accdesc.acctype) then
    // return AArch64.S1DisabledOutput(fault, regime, va, accdesc, aligned);
    // walkparams = AArch64.GetS1TTWParams(regime, accdesc.ss, va);
    // constant integer s1mintxsz = AArch64.S1MinTxSZ(regime, walkparams.d128,
    // walkparams.ds, walkparams.tgx);
    // constant integer s1maxtxsz = AArch64.MaxTxSZ(walkparams.tgx);
    // if AArch64.S1TxSZFaults(regime, walkparams) then
    // fault.statuscode = Fault_Translation;
    // fault.level = 0;
    // return (fault, AddressDescriptor UNKNOWN);
    // elsif UInt(walkparams.txsz) < s1mintxsz then
    // walkparams.txsz = s1mintxsz<5:0>;
    // elsif UInt(walkparams.txsz) > s1maxtxsz then
    // walkparams.txsz = s1maxtxsz<5:0>;
    // if AArch64.VAIsOutOfRange(va, accdesc.acctype, regime, walkparams) then
    // fault.statuscode = Fault_Translation;
    // fault.level = 0;
    // return (fault, AddressDescriptor UNKNOWN);
    // if accdesc.el == EL0 && walkparams.e0pd == '1' then
    // fault.statuscode = Fault_Translation;
    // fault.level = 0;
    // return (fault, AddressDescriptor UNKNOWN);
    // if (IsFeatureImplemented(FEAT_TME) && accdesc.el == EL0 && walkparams.nfd == '1' &&
    // accdesc.transactional) then
    // fault.statuscode = Fault_Translation;
    // fault.level = 0;
    // return (fault, AddressDescriptor UNKNOWN);
    // if (IsFeatureImplemented(FEAT_SVE) && accdesc.el == EL0 && walkparams.nfd == '1' &&
    // ((accdesc.nonfault && accdesc.contiguous) ||
    // (accdesc.firstfault && !accdesc.first && !accdesc.contiguous))) then
    // fault.statuscode = Fault_Translation;
    // fault.level = 0;
    // return (fault, AddressDescriptor UNKNOWN);
    // AddressDescriptor descipaddr;
    // TTWState walkstate;
    // bits(128) descriptor;
    // bits(128) new_desc;
    // bits(128) mem_desc;
    // repeat
    // if walkparams.d128 == '1' then
    // (fault, descipaddr, walkstate, descriptor) = AArch64.S1Walk(fault, walkparams, va,
    // regime, accdesc, 128);
    // else
    // (fault, descipaddr, walkstate, descriptor<63:0>) = AArch64.S1Walk(fault, walkparams,
    // va, regime, accdesc,
    // 64);
    // descriptor<127:64> = Zeros(64);
    // if fault.statuscode != Fault_None then
    // return (fault, AddressDescriptor UNKNOWN);
    // if accdesc.acctype == AccessType_IFETCH then
    // // Flag the fetched instruction is from a guarded page
    // SetInGuardedPage(walkstate.guardedpage == '1');
    // if AArch64.S1HasAlignmentFault(accdesc, aligned, walkparams.ntlsmd,
    // walkstate.memattrs) then
    // fault.statuscode = Fault_Alignment;
    // if fault.statuscode == Fault_None then
    // fault = AArch64.S1CheckPermissions(fault, regime, walkstate, walkparams, accdesc);
    // new_desc = descriptor;
    // if walkparams.ha == '1' && AArch64.SettingAccessFlagPermitted(fault) then
    // // Set descriptor AF bit
    // new_desc<10> = '1';
    // // If HW update of dirty bit is enabled, the walk state permissions
    // // will already reflect a configuration permitting writes.
    // // The update of the descriptor occurs only if the descriptor bits in
    // // memory do not reflect that and the access instigates a write.
    // if (AArch64.SettingDirtyStatePermitted(fault) &&
    // walkparams.ha == '1' &&
    // walkparams.hd == '1' &&
    // (walkparams.pie == '1' || descriptor<51> == '1') &&
    // accdesc.write &&
    // !(accdesc.acctype IN {AccessType_AT, AccessType_IC, AccessType_DC})) then
    // // Clear descriptor AP[2]/nDirty bit permitting stage 1 writes
    // new_desc<7> = '0';
    // // Either the access flag was clear or AP[2]/nDirty is set
    // if new_desc != descriptor then
    // AddressDescriptor descpaddr;
    // descaccess = CreateAccDescTTEUpdate(accdesc);
    // if regime == Regime_EL10 && EL2Enabled() then
    // FaultRecord s2fault;
    // s1aarch64 = TRUE;
    // s2aligned = TRUE;
    // (s2fault, descpaddr) = AArch64.S2Translate(fault, descipaddr, s1aarch64, s2aligned,
    // descaccess);
    // if s2fault.statuscode != Fault_None then
    // return (s2fault, AddressDescriptor UNKNOWN);
    // else
    // descpaddr = descipaddr;
    // if walkparams.d128 == '1' then
    // (fault, mem_desc) = AArch64.MemSwapTableDesc(fault, descriptor, new_desc,
    // walkparams.ee, descaccess, descpaddr);
    // else
    // (fault, mem_desc<63:0>) = AArch64.MemSwapTableDesc(fault, descriptor<63:0>,
    // new_desc<63:0>, walkparams.ee,
    // descaccess, descpaddr);
    // mem_desc<127:64> = Zeros(64);
    // until new_desc == descriptor || mem_desc == new_desc;
    // if fault.statuscode != Fault_None then
    // return (fault, AddressDescriptor UNKNOWN);
    // // Output Address
    // oa = StageOA(va, walkparams.d128, walkparams.tgx, walkstate);
    // MemoryAttributes memattrs;
    // if (accdesc.acctype == AccessType_IFETCH &&
    // (walkstate.memattrs.memtype == MemType_Device || !AArch64.S1ICacheEnabled(regime))) then
    // // Treat memory attributes as Normal Non-Cacheable
    // memattrs = NormalNCMemAttr();
    // memattrs.xs = walkstate.memattrs.xs;
    // elsif (accdesc.acctype != AccessType_IFETCH && !AArch64.S1DCacheEnabled(regime) &&
    // walkstate.memattrs.memtype == MemType_Normal) then
    // // Treat memory attributes as Normal Non-Cacheable
    // memattrs = NormalNCMemAttr();
    // memattrs.xs = walkstate.memattrs.xs;
    // // The effect of SCTLR_ELx.C when '0' is Constrained UNPREDICTABLE
    // // on the Tagged attribute
    // if (IsFeatureImplemented(FEAT_MTE2) &&
    // walkstate.memattrs.tags == MemTag_AllocationTagged &&
    // !ConstrainUnpredictableBool(Unpredictable_S1CTAGGED)) then
    // memattrs.tags = MemTag_Untagged;
    // else
    // memattrs = walkstate.memattrs;
    // // Shareability value of stage 1 translation subject to stage 2 is IMPLEMENTATION DEFINED
    // // to be either effective value or descriptor value
    // if (regime == Regime_EL10 && EL2Enabled() && HCR_EL2.VM == '1' &&
    // !(bool IMPLEMENTATION_DEFINED "Apply effective shareability at stage 1")) then
    // memattrs.shareability = walkstate.memattrs.shareability;
    // else
    // memattrs.shareability = EffectiveShareability(memattrs);
    // if accdesc.ls64 && memattrs.memtype == MemType_Normal then
    // if memattrs.inner.attrs != MemAttr_NC || memattrs.outer.attrs != MemAttr_NC then
    // fault.statuscode = Fault_Exclusive;
    // return (fault, AddressDescriptor UNKNOWN);
    // ipa = CreateAddressDescriptor(va, oa, memattrs);
    // ipa.s1assured = walkstate.s1assured;
    // varange = AArch64.GetVARange(va);
    // ipa.mecid = AArch64.S1OutputMECID(walkparams, regime, varange, ipa.paddress.paspace,
    // descriptor);
    // return (fault, ipa);
}

/// Library pseudocode for aarch64/translation/vmsa_translation/AArch64.S2Translate
/// AArch64.S2Translate()
/// =====================
/// Translate stage 1 IPA to PA and combine memory attributes
pub fn AArch64S2Translate(
    fault_in: FaultRecord,
    ipa: AddressDescriptor,
    s1aarch64: bool,
    aligned: bool,
    accdesc: AccessDescriptor,
) -> (FaultRecord, AddressDescriptor) {
    todo!()
    // walkparams = AArch64.GetS2TTWParams(accdesc.ss, ipa.paddress.paspace, s1aarch64);
    // FaultRecord fault = fault_in;
    // bool s2fs1mro;
    // // Prepare fault fields in case a fault is detected
    // fault.statuscode = Fault_None; // Ignore any faults from stage 1
    // fault.dirtybit = FALSE;
    // fault.overlay = FALSE;
    // fault.tagaccess = FALSE;
    // fault.s1tagnotdata = FALSE;
    // fault.secondstage = TRUE;
    // fault.s2fs1walk = accdesc.acctype == AccessType_TTW;
    // fault.ipaddress = ipa.paddress;
    // if walkparams.vm != '1' then
    // // Stage 2 translation is disabled
    // return (fault, ipa);
    // constant integer s2mintxsz = AArch64.S2MinTxSZ(walkparams.d128, walkparams.ds,
    // walkparams.tgx, s1aarch64);
    // constant integer s2maxtxsz = AArch64.MaxTxSZ(walkparams.tgx);
    // if AArch64.S2TxSZFaults(walkparams, s1aarch64) then
    // fault.statuscode = Fault_Translation;
    // fault.level = 0;
    // return (fault, AddressDescriptor UNKNOWN);
    // elsif UInt(walkparams.txsz) < s2mintxsz then
    // walkparams.txsz = s2mintxsz<5:0>;
    // elsif UInt(walkparams.txsz) > s2maxtxsz then
    // walkparams.txsz = s2maxtxsz<5:0>;
    // if (walkparams.d128 == '0' &&
    // (AArch64.S2InvalidSL(walkparams) || AArch64.S2InconsistentSL(walkparams))) then
    // fault.statuscode = Fault_Translation;
    // fault.level = 0;
    // return (fault, AddressDescriptor UNKNOWN);
    // if AArch64.IPAIsOutOfRange(ipa.paddress.address, walkparams) then
    // fault.statuscode = Fault_Translation;
    // fault.level = 0;
    // return (fault, AddressDescriptor UNKNOWN);
    // AddressDescriptor descpaddr;
    // TTWState walkstate;
    // bits(128) descriptor;
    // bits(128) new_desc;
    // bits(128) mem_desc;
    // repeat
    // if walkparams.d128 == '1' then
    // (fault, descpaddr, walkstate, descriptor) = AArch64.S2Walk(fault, ipa, walkparams,
    // accdesc, 128);
    // else
    // (fault, descpaddr, walkstate, descriptor<63:0>) = AArch64.S2Walk(fault, ipa,
    // walkparams, accdesc,
    // 64);
    // descriptor<127:64> = Zeros(64);
    // if fault.statuscode != Fault_None then
    // return (fault, AddressDescriptor UNKNOWN);
    // if AArch64.S2HasAlignmentFault(accdesc, aligned, walkstate.memattrs) then
    // fault.statuscode = Fault_Alignment;
    // if fault.statuscode == Fault_None then
    // (fault, s2fs1mro) = AArch64.S2CheckPermissions(fault, walkstate, walkparams, ipa,
    // accdesc);
    // new_desc = descriptor;
    // if walkparams.ha == '1' && AArch64.SettingAccessFlagPermitted(fault) then
    // // Set descriptor AF bit
    // new_desc<10> = '1';
    // // If HW update of dirty bit is enabled, the walk state permissions
    // // will already reflect a configuration permitting writes.
    // // The update of the descriptor occurs only if the descriptor bits in
    // // memory do not reflect that and the access instigates a write.
    // if (AArch64.SettingDirtyStatePermitted(fault) &&
    // walkparams.ha == '1' &&
    // walkparams.hd == '1' &&
    // (walkparams.s2pie == '1' || descriptor<51> == '1') &&
    // accdesc.write &&
    // !(accdesc.acctype IN {AccessType_AT, AccessType_IC, AccessType_DC})) then
    // // Set descriptor S2AP[1]/Dirty bit permitting stage 2 writes
    // new_desc<7> = '1';
    // // Either the access flag was clear or S2AP[1]/Dirty is clear
    // if new_desc != descriptor then
    // if walkparams.hdbss == '1' && descriptor<7> == '0' && new_desc<7> == '1' then
    // fault = AppendToHDBSS(fault, ipa.paddress, accdesc, walkparams, walkstate.level);
    // // If an error, other than a synchronous External abort, occurred on the HDBSS update,
    // // stage 2 hardware update of dirty state is not permitted.
    // if (HDBSSPROD_EL2.FSC != '101000' &&
    // (!fault.hdbssf || IsExternalAbort(fault.statuscode))) then
    // AccessDescriptor descaccess = CreateAccDescTTEUpdate(accdesc);
    // if walkparams.d128 == '1' then
    // (fault, mem_desc) = AArch64.MemSwapTableDesc(fault, descriptor, new_desc,
    // walkparams.ee, descaccess,
    // descpaddr);
    // else
    // (fault, mem_desc<63:0>) = AArch64.MemSwapTableDesc(fault, descriptor<63:0>,
    // new_desc<63:0>,
    // walkparams.ee,
    // descaccess, descpaddr);
    // mem_desc<127:64> = Zeros(64);
    // if fault.statuscode != Fault_None then
    // return (fault, AddressDescriptor UNKNOWN);
    // until new_desc == descriptor || mem_desc == new_desc;
    // if fault.statuscode != Fault_None then
    // return (fault, AddressDescriptor UNKNOWN);
    // ipa_64 = ZeroExtend(ipa.paddress.address, 64);
    // // Output Address
    // oa = StageOA(ipa_64, walkparams.d128, walkparams.tgx, walkstate);
    // MemoryAttributes s2_memattrs;
    // if ((accdesc.acctype == AccessType_TTW &&
    // walkstate.memattrs.memtype == MemType_Device && walkparams.ptw == '0') ||
    // (accdesc.acctype == AccessType_IFETCH &&
    // (walkstate.memattrs.memtype == MemType_Device || HCR_EL2.ID == '1')) ||
    // (accdesc.acctype != AccessType_IFETCH &&
    // walkstate.memattrs.memtype == MemType_Normal && !S2DCacheEnabled())) then
    // // Treat memory attributes as Normal Non-Cacheable
    // s2_memattrs = NormalNCMemAttr();
    // s2_memattrs.xs = walkstate.memattrs.xs;
    // else
    // s2_memattrs = walkstate.memattrs;
    // if accdesc.ls64 && s2_memattrs.memtype == MemType_Normal then
    // if s2_memattrs.inner.attrs != MemAttr_NC || s2_memattrs.outer.attrs != MemAttr_NC then
    // fault.statuscode = Fault_Exclusive;
    // return (fault, AddressDescriptor UNKNOWN);
    // s2aarch64 = TRUE;
    // MemoryAttributes memattrs;
    // if walkparams.fwb == '0' then
    // memattrs = S2CombineS1MemAttrs(ipa.memattrs, s2_memattrs, s2aarch64);
    // else
    // memattrs = s2_memattrs;
    // pa = CreateAddressDescriptor(ipa.vaddress, oa, memattrs);
    // pa.s2fs1mro = s2fs1mro;
    // pa.mecid = AArch64.S2OutputMECID(walkparams, pa.paddress.paspace, descriptor);
    // return (fault, pa);
}
