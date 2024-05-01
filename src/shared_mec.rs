// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later
#![allow(non_camel_case_types)]

use crate::shared::*;
use crate::shared_memory::*;
use crate::shared_mpam::MPAMinfo;
use crate::shared_translation::*;
use crate::shared_vmsa::*;

// Library pseudocode for aarch64/functions/mec/AArch64.S1AMECFault

// // AArch64.S1AMECFault()
// // =====================
// // Returns TRUE if a Translation fault should occur for Realm EL2 and Realm EL2&0
// // stage 1 translated addresses to Realm PA space.

// boolean AArch64.S1AMECFault(S1TTWParams walkparams, PASpace paspace, Regime regime,
//                            bits(N) descriptor)
//    assert N IN {64,128};
//    bit descriptor_amec = if walkparams.d128 == '1' then descriptor<108> else descriptor<63>;

//    return (walkparams.<emec,amec> == '10' &&
//            regime IN {Regime_EL2, Regime_EL20} &&
//            paspace == PAS_Realm &&
//            descriptor_amec == '1');

// Library pseudocode for aarch64/functions/mec/AArch64.S1DisabledOutputMECID

// // AArch64.S1DisabledOutputMECID()
// // ===============================
// // Returns the output MECID when stage 1 address translation is disabled.

// bits(16) AArch64.S1DisabledOutputMECID(S1TTWParams walkparams, Regime regime, PASpace paspace)
//    if walkparams.emec == '0' then
//        return DEFAULT_MECID;

//    if !(regime IN {Regime_EL2, Regime_EL20, Regime_EL10}) then
//        return DEFAULT_MECID;

//    if paspace != PAS_Realm then
//        return DEFAULT_MECID;

//    if regime == Regime_EL10 then
//        return VMECID_P_EL2.MECID;
//    else
//        return MECID_P0_EL2.MECID;

/// Library pseudocode for aarch64/functions/mec/AArch64.S1OutputMECID

/// AArch64.S1OutputMECID()
/// =======================
/// Returns the output MECID when stage 1 address translation is enabled.

pub fn AArch64S1OutputMECID_64(
    walkparams: S1TTWParams,
    _regime: Regime,
    _varange: VARange,
    paspace: PASpace,
    _descriptor: u64,
) -> u16 {
    if walkparams.bitfield.get(S1TTWParamsBits::emec) == 0 {
        return DEFAULT_MECID;
    }

    if paspace != PASpace::PAS_Realm {
        return DEFAULT_MECID;
    }

    todo!()
    // let descriptor_amec = if walkparams.d128 == 1 { descriptor<108> } else { descriptor<63> };
    // match regime {
    //     Regime::Regime_EL3 =>
    //         return MECID_RL_A_EL3.MECID,
    //     Regime::Regime_EL2 =>
    //         if descriptor_amec == 0 {
    //             return MECID_P0_EL2.MECID;
    //         }
    //         else {
    //             return MECID_A0_EL2.MECID;
    //         }
    //     Regime::Regime_EL20 =>
    //         if varange == VARange_LOWER {
    //             if descriptor_amec == 0 {
    //                 return MECID_P0_EL2.MECID;
    //             } else {
    //                 return MECID_A0_EL2.MECID;
    //             }
    //         } else {
    //             if descriptor_amec == '0' {
    //                 return MECID_P1_EL2.MECID;
    //             }else {
    //                 return MECID_A1_EL2.MECID;
    //             }
    //         }
    //     Regime::Regime_EL10 =>
    //         return VMECID_P_EL2.MECID,
    //     Regime::Regime_EL30 => unreachable!(),
    // }
}

pub fn AArch64S1OutputMECID_128(
    walkparams: S1TTWParams,
    _regime: Regime,
    _varange: VARange,
    paspace: PASpace,
    _descriptor: u128,
) -> u16 {
    if walkparams.bitfield.get(S1TTWParamsBits::emec) == 0 {
        return DEFAULT_MECID;
    }

    if paspace != PASpace::PAS_Realm {
        return DEFAULT_MECID;
    }

    todo!()
    // let descriptor_amec = if walkparams.bitfield.get(S1TTWParamsBits::d128) == 1 { descriptor<108> } else { descriptor<63> };
    // match regime {
    //     Regime::Regime_EL3 =>
    //         return MECID_RL_A_EL3.MECID,
    //     Regime::Regime_EL2 =>
    //         if descriptor_amec == 0 {
    //             return MECID_P0_EL2.MECID;
    //         }
    //         else {
    //             return MECID_A0_EL2.MECID;
    //         }
    //     Regime::Regime_EL20 =>
    //         if varange == VARange_LOWER {
    //             if descriptor_amec == 0 {
    //                 return MECID_P0_EL2.MECID;
    //             } else {
    //                 return MECID_A0_EL2.MECID;
    //             }
    //         } else {
    //             if descriptor_amec == '0' {
    //                 return MECID_P1_EL2.MECID;
    //             }else {
    //                 return MECID_A1_EL2.MECID;
    //             }
    //         }
    //     Regime::Regime_EL10 =>
    //         return VMECID_P_EL2.MECID,
    //     Regime::Regime_EL30 => unreachable!(),
    // }
}

// Library pseudocode for aarch64/functions/mec/AArch64.S2OutputMECID

// // AArch64.S2OutputMECID()
// // =======================
// // Returns the output MECID for stage 2 address translation.

// bits(16) AArch64.S2OutputMECID(S2TTWParams walkparams, PASpace paspace, bits(N) descriptor)
//    assert N IN {64,128};

//    if walkparams.emec == '0' then
//        return DEFAULT_MECID;

//    if paspace != PAS_Realm then
//        return DEFAULT_MECID;

//    bit descriptor_amec = if walkparams.d128 == '1' then descriptor<108> else descriptor<63>;
//    if descriptor_amec == '0' then
//        return VMECID_P_EL2.MECID;
//    else
//        return VMECID_A_EL2.MECID;

// Library pseudocode for aarch64/functions/mec/AArch64.TTWalkMECID

// // AArch64.TTWalkMECID()
// // =====================
// // Returns the associated MECID for the translation table walk of the given
// // translation regime and Security state.

// bits(16) AArch64.TTWalkMECID(bit emec, Regime regime, SecurityState ss)
//    if emec == '0' then
//        return DEFAULT_MECID;

//    if ss != SS_Realm then
//        return DEFAULT_MECID;

//    case regime of
//        when Regime_EL2
//            return MECID_P0_EL2.MECID;
//        when Regime_EL20
//            if TCR_EL2.A1 == '0' then
//                return MECID_P1_EL2.MECID;
//            else
//                return MECID_P0_EL2.MECID;
//        // This applies to stage 1 and stage 2 translation table walks for
//        // Realm EL1&0, but the stage 2 translation for a stage 1 walk
//        // might later override the MECID according to AMEC configuration.
//        when Regime_EL10
//            return VMECID_P_EL2.MECID;
//        otherwise
//            Unreachable();

/// Library pseudocode for aarch64/functions/mec/DEFAULT_MECID
pub const DEFAULT_MECID: u16 = 0_16;
