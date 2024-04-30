// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later
use crate::shared_vmsa::*;
pub static PSTATE: ProcState = ProcState::new();

// /// Library pseudocode for shared/functions/system/PhysicalCountInt

// // PhysicalCountInt()
// // ==================
// // Returns the integral part of physical count value of the System counter.

// bits(64) PhysicalCountInt()
//     return PhysicalCount<87:24>;

// Library pseudocode for shared/functions/system/ProcState

mycelium_bitfield::bitfield! {
    pub struct ProcState<u64> {
        /// Negative condition flag
        pub const N=1;
        /// Zero condition flag
        pub const Z=1;
        /// Carry condition flag
        pub const C=1;
        /// Overflow condition flag
        pub const V=1;
        /// Debug mask bit                     [AArch64 only]
        pub const D=1;
        /// SError interrupt mask bit
        pub const A=1;
        /// IRQ mask bit
        pub const I=1;
        /// FIQ mask bit
        pub const F=1;
        /// Privileged Access Never Bit        [v8.1]
        pub const PAN=1;
        /// User Access Override               [v8.2]
        pub const UAO=1;
        /// Data Independent Timing            [v8.4]
        pub const DIT=1;
        /// Tag Check Override                 [v8.5, AArch64 only]
        pub const TCO=1;
        /// Branch Type                        [v8.5]
        pub const BTYPE=2;
        /// Interrupt mask bit
        pub const ALLINT=1;
        /// Software step bit
        pub const SS=1;
        /// Illegal Execution state bit
        pub const IL=1;
        /// Exception level
        pub const EL=2;
        /// not Register Width: 0=64, 1=32
        pub const nRW=1;
        /// Stack pointer select: 0=SP0, 1=SPx [AArch64 only]
        pub const SP=1;
        /// Cumulative saturation flag         [AArch32 only]
        pub const Q=1;
        /// Greater than or Equal flags        [AArch32 only]
        pub const GE=4;
        /// Speculative Store Bypass Safe
        pub const SSBS=1;
        /// If-then bits, RES0 in CPSR         [AArch32 only]
        pub const IT=8;
        /// J bit, RES0                        [AArch32 only, RES0 in SPSR and CPSR]
        pub const J=1;
        /// T32 bit, RES0 in CPSR              [AArch32 only]
        pub const T=1;
        /// Endianness bit                     [AArch32 only]
        pub const E=1;
        /// Mode field                         [AArch32 only]
        pub const M=5;
    }
}

impl ProcState {
    pub fn get_EL(&self) -> PrivilegeLevel {
        match self.get(Self::EL) {
            v if v == (EL0 as u64) => PrivilegeLevel::PL0,
            v if v == (EL1 as u64) => PrivilegeLevel::PL1,
            v if v == (EL2 as u64) => PrivilegeLevel::PL2,
            // TODO when EL3  return if !HaveAArch64() then PL1 else PL3;
            v if v == (EL3 as u64) => PrivilegeLevel::PL3,
            _ => unreachable!(),
        }
    }
}

/// Library pseudocode for shared/functions/system/SecurityState

/// SecurityState
/// =============
/// The Security state of an execution context
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SecurityState {
    SS_NonSecure,
    SS_Root,
    SS_Realm,
    SS_Secure,
}

/// Library pseudocode for shared/functions/system/EL2Enabled
/// EL2Enabled()
/// ============
/// Returns TRUE if EL2 is present and executing
/// - with the PE in Non-secure state when Non-secure EL2 is implemented, or
/// - with the PE in Realm state when Realm EL2 is implemented, or
/// - with the PE in Secure state when Secure EL2 is implemented and enabled, or
/// - when EL3 is not implemented.
pub fn EL2Enabled() -> bool {
    // FIXME HaveEL(EL2) && (!HaveEL(EL3) || SCR_curr[].NS == '1' || IsSecureEL2Enabled())
    HaveEL(EL2) && (!HaveEL(EL3) || IsSecureEL2Enabled())
}

///Library pseudocode for shared/functions/system/HaveEL
/// HaveEL()
/// ========
/// Return TRUE if Exception level 'el' is supported
pub fn HaveEL(el: PrivilegeLevel) -> bool {
    todo!()

    // case el of
    // when EL1,EL0
    // return TRUE; // EL1 and EL0 must exist
    // when EL2
    // return IsFeatureImplemented(FEAT_AA64EL2) || IsFeatureImplemented(FEAT_AA32EL2);
    // when EL3
    // return IsFeatureImplemented(FEAT_AA64EL3) || IsFeatureImplemented(FEAT_AA32EL3);
    // otherwise
    // Unreachable();
}

/// Library pseudocode for shared/functions/system/IsSecureEL2Enabled
/// // IsSecureEL2Enabled()
/// // ====================
/// // Returns TRUE if Secure EL2 is enabled, FALSE otherwise.
pub fn IsSecureEL2Enabled() -> bool {
    false
    // if HaveEL(EL2) && IsFeatureImplemented(FEAT_SEL2) then
    // if HaveEL(EL3) then
    // if !ELUsingAArch32(EL3) && SCR_EL3.EEL2 == '1' then
    // return TRUE;
    // else
    // return FALSE;
    // else
    // return SecureOnlyImplementation();
    // else
    // return FALSE;
}
