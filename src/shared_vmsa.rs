// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later
use std::mem::MaybeUninit;

use crate::shared::*;
use crate::shared_memory::*;
use crate::shared_translation::*;

/// Library pseudocode for shared/translation/vmsa/AddressDescriptor

pub const FINAL_LEVEL: u64 = 3;

/// AddressDescriptor
/// =================
/// Descriptor used to access the underlying memory array.

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AddressDescriptor {
    /// fault.statuscode indicates whether the address is valid
    pub fault: FaultRecord,
    pub memattrs: MemoryAttributes,
    pub paddress: FullAddress,
    /// Stage 1 Assured Translation Property
    pub s1assured: bool,
    /// Stage 2 MRO permission for Stage 1
    pub s2fs1mro: bool,
    /// FEAT_MEC: Memory Encryption Context ID bits(16)
    pub mecid: u16,
    /// bits(64)
    pub vaddress: u64,
}

impl AddressDescriptor {
    pub const UNKNOWN: Self = unsafe { MaybeUninit::zeroed().assume_init_read() };
}

/// Library pseudocode for shared/translation/vmsa/ContiguousSize
/// ContiguousSize()
/// ================
/// Return the number of entries log 2 marking a contiguous output range
pub fn ContiguousSize(d128: u64, tgx: TGx, level: u64) -> u64 {
    if d128 == 1 {
        match tgx {
            TGx::TGx_4KB => {
                //assert level IN {1, 2, 3};
                return if level == 1 { 2 } else { 4 };
            }
            TGx::TGx_16KB => {
                //assert level IN {1, 2, 3};
                if level == 1 {
                    return 2;
                } else if level == 2 {
                    return 4;
                } else {
                    return 6;
                }
            }
            TGx::TGx_64KB => {
                //assert level IN {2, 3};
                return if level == 2 { 6 } else { 4 };
            }
        }
    } else {
        match tgx {
            TGx::TGx_4KB => {
                //assert level IN {1, 2, 3};
                return 4;
            }
            TGx::TGx_16KB => {
                //assert level IN {2, 3};
                return if level == 2 { 5 } else { 7 };
            }
            TGx::TGx_64KB => {
                //assert level IN {2, 3};
                return 5;
            }
        }
    }
}

/// Library pseudocode for shared/translation/vmsa/CreateAddressDescriptor
/// CreateAddressDescriptor()
/// =========================
/// Set internal members for address descriptor type to valid values
pub fn CreateAddressDescriptor(
    va: u64,
    pa: FullAddress,
    memattrs: MemoryAttributes,
) -> AddressDescriptor {
    AddressDescriptor {
        paddress: pa,
        vaddress: va,
        memattrs,
        fault: FaultRecord::NoFault(),
        s1assured: false,
        s2fs1mro: false,
        mecid: 0,
    }
}

/// Library pseudocode for shared/translation/vmsa/CreateFaultyAddressDescriptor

/// CreateFaultyAddressDescriptor()
/// ===============================
/// Set internal members for address descriptor type with values indicating error

pub fn CreateFaultyAddressDescriptor(_va: u64, _fault: FaultRecord) -> AddressDescriptor {
    todo!()
    // AddressDescriptor CreateFaultyAddressDescriptor(bits(64) va, FaultRecord fault)
    //    AddressDescriptor addrdesc;

    //    addrdesc.vaddress = va;
    //    addrdesc.fault    = fault;

    //    return addrdesc;
}

// Library pseudocode for shared/translation/vmsa/DecodePASpace

// // DecodePASpace()
// // ===============
// // Decode the target PA Space

// PASpace DecodePASpace (bit nse, bit ns)
//    case nse:ns of
//        when '00'   return PAS_Secure;
//        when '01'   return PAS_NonSecure;
//        when '10'   return PAS_Root;
//        when '11'   return PAS_Realm;

// Library pseudocode for shared/translation/vmsa/DescriptorType

// // DescriptorType
// // ==============
// // Translation table descriptor formats

// enumeration DescriptorType {
//    DescriptorType_Table,
//    DescriptorType_Leaf,
//    DescriptorType_Invalid
// };

// Library pseudocode for shared/translation/vmsa/Domains

// constant bits(2) Domain_NoAccess = '00';
// constant bits(2) Domain_Client   = '01';
// constant bits(2) Domain_Manager  = '11';

// Library pseudocode for shared/translation/vmsa/FetchDescriptor

// // FetchDescriptor()
// // =================
// // Fetch a translation table descriptor

// (FaultRecord, bits(N)) FetchDescriptor(bit ee, AddressDescriptor walkaddress,
//                                       AccessDescriptor walkaccess, FaultRecord fault_in,
//                                       integer N)
//    // 32-bit descriptors for AArch32 Short-descriptor format
//    // 64-bit descriptors for AArch64 or AArch32 Long-descriptor format
//    // 128-bit descriptors for AArch64 when FEAT_D128 is set and {V}TCR_ELx.d128 is set
//    assert N == 32 || N == 64 || N == 128;
//    bits(N) descriptor;
//    FaultRecord fault = fault_in;

//    if IsFeatureImplemented(FEAT_RME) then
//        fault.gpcf = GranuleProtectionCheck(walkaddress, walkaccess);
//        if fault.gpcf.gpf != GPCF_None then
//            fault.statuscode = Fault_GPCFOnWalk;
//            fault.paddress   = walkaddress.paddress;
//            fault.gpcfs2walk = fault.secondstage;
//            return (fault, bits(N) UNKNOWN);

//    PhysMemRetStatus memstatus;
//    (memstatus, descriptor) = PhysMemRead(walkaddress, N DIV 8, walkaccess);
//    if IsFault(memstatus) then
//        boolean iswrite = FALSE;
//        fault = HandleExternalTTWAbort(memstatus, iswrite, walkaddress,
//                                       walkaccess, N DIV 8, fault);
//        if IsFault(fault.statuscode) then
//            return (fault, bits(N) UNKNOWN);

//    if ee == '1' then
//        descriptor = BigEndianReverse(descriptor);

//    return (fault, descriptor);

// Library pseudocode for shared/translation/vmsa/HasUnprivileged

// // HasUnprivileged()
// // =================
// // Returns whether a translation regime serves EL0 as well as a higher EL

// boolean HasUnprivileged(Regime regime)
//    return (regime IN {
//        Regime_EL20,
//        Regime_EL30,
//        Regime_EL10
//    });

// Library pseudocode for shared/translation/vmsa/Regime

// // Regime
// // ======
// // Translation regimes

// enumeration Regime {
//    Regime_EL3,            // EL3
//    Regime_EL30,           // EL3&0 (PL1&0 when EL3 is AArch32)
//    Regime_EL2,            // EL2
//    Regime_EL20,           // EL2&0
//    Regime_EL10            // EL1&0
// };

// Library pseudocode for shared/translation/vmsa/RegimeUsingAArch32

// // RegimeUsingAArch32()
// // ====================
// // Determine if the EL controlling the regime executes in AArch32 state

// boolean RegimeUsingAArch32(Regime regime)
//    case regime of
//        when Regime_EL10 return ELUsingAArch32(EL1);
//        when Regime_EL30 return TRUE;
//        when Regime_EL20 return FALSE;
//        when Regime_EL2  return ELUsingAArch32(EL2);
//        when Regime_EL3  return FALSE;

// type S1TTWParams is (
// // A64-VMSA exclusive parameters
//    bit         ha,         // TCR_ELx.HA
//    bit         hd,         // TCR_ELx.HD
//    bit         tbi,        // TCR_ELx.TBI{x}
//    bit         tbid,       // TCR_ELx.TBID{x}
//    bit         nfd,        // TCR_EL1.NFDx or TCR_EL2.NFDx when HCR_EL2.E2H == '1'
//    bit         e0pd,       // TCR_EL1.E0PDx or TCR_EL2.E0PDx when HCR_EL2.E2H == '1'
//    bit         d128,       // TCR_ELx.D128
//    bit         aie,         // (TCR2_ELx/TCR_EL3).AIE
//    MAIRType    mair2,       // MAIR2_ELx
//    bit         ds,         // TCR_ELx.DS
//    bits(3)     ps,         // TCR_ELx.{I}PS
//    bits(6)     txsz,       // TCR_ELx.TxSZ
//    bit         epan,       // SCTLR_EL1.EPAN or SCTLR_EL2.EPAN when HCR_EL2.E2H == '1'
//    bit         dct,        // HCR_EL2.DCT
//    bit         nv1,        // HCR_EL2.NV1
//    bit         cmow,       // SCTLR_EL1.CMOW or SCTLR_EL2.CMOW when HCR_EL2.E2H == '1'
//    bit         pnch,       // TCR{2}_ELx.PnCH
//    bit         disch,      // TCR{2}_ELx.DisCH
//    bit         haft,       // TCR{2}_ELx.HAFT
//    bit         mtx,        // TCR_ELx.MTX{y}
//    bits(2)     skl,        // TCR_ELx.SKL
//    bit         pie,        // TCR2_ELx.PIE or TCR_EL3.PIE
//    S1PIRType   pir,        // PIR_ELx
//    S1PIRType   pire0,      // PIRE0_EL1 or PIRE0_EL2 when HCR_EL2.E2H == '1'
//    bit         emec,       // SCTLR2_EL2.EMEC or SCTLR2_EL3.EMEC
//    bit         amec,       // TCR2_EL2.AMEC0 or TCR2_EL2.AMEC1 when HCR_EL2.E2H == '1'
//    bit         fng,        // TCR2_EL1.FNGx or TCR2_EL2.FNGx when HCR_EL2.E2H == '1'

// // A32-VMSA exclusive parameters
//    bits(3)     t0sz,       // TTBCR.T0SZ
//    bits(3)     t1sz,       // TTBCR.T1SZ
//    bit         uwxn,       // SCTLR.UWXN

// // Parameters common to both A64-VMSA & A32-VMSA (A64/A32)
//    TGx         tgx,        // TCR_ELx.TGx      / Always TGx_4KB
//    bits(2)     irgn,       // TCR_ELx.IRGNx    / TTBCR.IRGNx or HTCR.IRGN0
//    bits(2)     orgn,       // TCR_ELx.ORGNx    / TTBCR.ORGNx or HTCR.ORGN0
//    bits(2)     sh,         // TCR_ELx.SHx      / TTBCR.SHx or HTCR.SH0
//    bit         hpd,        // TCR_ELx.HPD{x}   / TTBCR2.HPDx or HTCR.HPD
//    bit         ee,         // SCTLR_ELx.EE     / SCTLR.EE or HSCTLR.EE
//    bit         wxn,        // SCTLR_ELx.WXN    / SCTLR.WXN or HSCTLR.WXN
//    bit         ntlsmd,     // SCTLR_ELx.nTLSMD / SCTLR.nTLSMD or HSCTLR.nTLSMD
//    bit         dc,         // HCR_EL2.DC       / HCR.DC
//    bit         sif,        // SCR_EL3.SIF      / SCR.SIF
//    MAIRType    mair        // MAIR_ELx         / MAIR1:MAIR0 or HMAIR1:HMAIR0
// )
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct S1TTWParams {
    pub bitfield: S1TTWParamsBits,
    /// MAIR2_ELx
    pub mair2: u64,
    /// PIR_ELx
    pub pir: u64,
    /// PIRE0_EL1 or PIRE0_EL2 when HCR_EL2.E2H == '1'
    pub pire0: u64,
    /// TCR_ELx.TGx      / Always TGx_4KB
    pub tgx: TGx,
    /// MAIR_ELx         / MAIR1:MAIR0 or HMAIR1:HMAIR0
    pub mair: u64, /*MAIRType*/
}

macro_rules! getter {
    ($($getter:tt $ident:tt),*$(,)*) => {
        $(pub fn $getter(&self) -> u64 {
            self.bitfield.get(S1TTWParamsBits::$ident)
        })*
    };
}
impl S1TTWParams {
    getter! {
        get_ha ha,
        get_hd hd,
        get_tbi tbi,
        get_tbid tbid,
        get_nfd nfd,
        get_e0pd e0pd,
        get_d128 d128,
        get_aie aie,
        get_ds ds,
        get_ps ps,
        get_txsz txsz,
        get_epan epan,
        get_dct dct,
        get_nv1 nv1,
        get_cmow cmow,
        get_pnch pnch,
        get_disch disch,
        get_haft haft,
        get_mtx mtx,
        get_skl skl,
        get_pie pie,
        get_emec emec,
        get_amec amec,
        get_fng fng,
        get_irgn irgn,
        get_orgn orgn,
        get_sh sh,
        get_hpd hpd,
        get_ee ee,
        get_wxn wxn,
        get_ntlsmd ntlsmd,
        get_dc dc,
        get_sif sif,

    }

    pub const fn get_tgx(&self) -> TGx {
        TGx::TGx_4KB
    }
}

mycelium_bitfield::bitfield! {
    /// Library pseudocode for shared/translation/vmsa/S1TTWParams
    /// S1TTWParams
    /// ===========
    /// Register fields corresponding to stage 1 translation
    /// For A32-VMSA, if noted, they correspond to A32-LPAE (Long descriptor format)
    #[derive(Eq, PartialEq)]
    pub struct S1TTWParamsBits<u64> {
        // A64-VMSA exclusive parameters
        pub const ha = 1;         // TCR_ELx.HA
        pub const hd = 1;         // TCR_ELx.HD
        pub const tbi = 1;        // TCR_ELx.TBI{x}
        pub const tbid = 1;       // TCR_ELx.TBID{x}
        pub const nfd = 1;        // TCR_EL1.NFDx or TCR_EL2.NFDx when HCR_EL2.E2H == '1'
        pub const e0pd = 1;       // TCR_EL1.E0PDx or TCR_EL2.E0PDx when HCR_EL2.E2H == '1'
        pub const d128 = 1;       // TCR_ELx.D128
        pub const aie = 1;         // (TCR2_ELx/TCR_EL3).AIE
        pub const ds = 1;         // TCR_ELx.DS
        pub const ps = 3;         // TCR_ELx.{I}PS
        pub const txsz = 6;       // TCR_ELx.TxSZ
        pub const epan = 1;       // SCTLR_EL1.EPAN or SCTLR_EL2.EPAN when HCR_EL2.E2H == '1'
        pub const dct = 1;        // HCR_EL2.DCT
        pub const nv1 = 1;        // HCR_EL2.NV1
        pub const cmow = 1;       // SCTLR_EL1.CMOW or SCTLR_EL2.CMOW when HCR_EL2.E2H == '1'
        pub const pnch = 1;       // TCR{2}_ELx.PnCH
        pub const disch = 1;      // TCR{2}_ELx.DisCH
        pub const haft = 1;       // TCR{2}_ELx.HAFT
        pub const mtx = 1;        // TCR_ELx.MTX{y}
        pub const skl = 2;        // TCR_ELx.SKL
        pub const pie = 1;        // TCR2_ELx.PIE or TCR_EL3.PIE
        pub const emec = 1;       // SCTLR2_EL2.EMEC or SCTLR2_EL3.EMEC
        pub const amec = 1;       // TCR2_EL2.AMEC0 or TCR2_EL2.AMEC1 when HCR_EL2.E2H == '1'
        pub const fng = 1;        // TCR2_EL1.FNGx or TCR2_EL2.FNGx when HCR_EL2.E2H == '1'

    // // A32-VMSA exclusive parameters
    //    bits(3)     t0sz,       // TTBCR.T0SZ
    //    bits(3)     t1sz,       // TTBCR.T1SZ
    //    bit         uwxn,       // SCTLR.UWXN

    // // Parameters common to both A64-VMSA & A32-VMSA (A64/A32)
    pub const     irgn = 2;       // TCR_ELx.IRGNx    / TTBCR.IRGNx or HTCR.IRGN0
    pub const orgn = 2;       // TCR_ELx.ORGNx    / TTBCR.ORGNx or HTCR.ORGN0
    pub const sh=2;         // TCR_ELx.SHx      / TTBCR.SHx or HTCR.SH0
    pub const         hpd=1;        // TCR_ELx.HPD{x}   / TTBCR2.HPDx or HTCR.HPD
    pub const         ee=1;         // SCTLR_ELx.EE     / SCTLR.EE or HSCTLR.EE
    pub const         wxn=1;        // SCTLR_ELx.WXN    / SCTLR.WXN or HSCTLR.WXN
    pub const         ntlsmd=1;     // SCTLR_ELx.nTLSMD / SCTLR.nTLSMD or HSCTLR.nTLSMD
    pub const         dc=1;         // HCR_EL2.DC       / HCR.DC
    pub const         sif=1;        // SCR_EL3.SIF      / SCR.SIF
    }
}

// Library pseudocode for shared/translation/vmsa/S2TTWParams

// // S2TTWParams
// // ===========
// // Register fields corresponding to stage 2 translation.

// type S2TTWParams is (
// // A64-VMSA exclusive parameters
//    bit         ha,         // VTCR_EL2.HA
//    bit         hd,         // VTCR_EL2.HD
//    bit         sl2,        // V{S}TCR_EL2.SL2
//    bit         ds,         // VTCR_EL2.DS
//    bit         d128,       // VTCR_ELx.D128
//    bit         sw,         // VSTCR_EL2.SW
//    bit         nsw,        // VTCR_EL2.NSW
//    bit         sa,         // VSTCR_EL2.SA
//    bit         nsa,        // VTCR_EL2.NSA
//    bits(3)     ps,         // VTCR_EL2.PS
//    bits(6)     txsz,       // V{S}TCR_EL2.T0SZ
//    bit         fwb,        // HCR_EL2.FWB
//    bit         cmow,       // HCRX_EL2.CMOW
//    bits(2)     skl,        // VTCR_EL2.SKL
//    bit         s2pie,      // VTCR_EL2.S2PIE
//    S2PIRType   s2pir,      // S2PIR_EL2
//    bit         tl0,        // VTCR_EL2.TL0
//    bit         tl1,        // VTCR_EL2.TL1
//    bit         assuredonly,// VTCR_EL2.AssuredOnly
//    bit         haft,       // VTCR_EL2.HAFT
//    bit         emec,       // SCTLR2_EL2.EMEC
//    bit         hdbss,      // VTCR_EL2.HDBSS

// // A32-VMSA exclusive parameters
//    bit         s,          // VTCR.S
//    bits(4)     t0sz,       // VTCR.T0SZ

// // Parameters common to both A64-VMSA & A32-VMSA if implemented (A64/A32)
//    TGx         tgx,        // V{S}TCR_EL2.TG0  / Always TGx_4KB
//    bits(2)     sl0,        // V{S}TCR_EL2.SL0  / VTCR.SL0
//    bits(2)     irgn,       // VTCR_EL2.IRGN0   / VTCR.IRGN0
//    bits(2)     orgn,       // VTCR_EL2.ORGN0   / VTCR.ORGN0
//    bits(2)     sh,         // VTCR_EL2.SH0     / VTCR.SH0
//    bit         ee,         // SCTLR_EL2.EE     / HSCTLR.EE
//    bit         ptw,        // HCR_EL2.PTW      / HCR.PTW
//    bit         vm          // HCR_EL2.VM       / HCR.VM
// )

/// Library pseudocode for shared/translation/vmsa/SDFType

/// SDFType
/// =======
/// Short-descriptor format type

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SDFType {
    SDFType_Table,
    SDFType_Invalid,
    SDFType_Supersection,
    SDFType_Section,
    SDFType_LargePage,
    SDFType_SmallPage,
}

// Library pseudocode for shared/translation/vmsa/SecurityStateForRegime

// // SecurityStateForRegime()
// // ========================
// // Return the Security State of the given translation regime

// SecurityState SecurityStateForRegime(Regime regime)
//    case regime of
//        when Regime_EL3     return SecurityStateAtEL(EL3);
//        when Regime_EL30    return SS_Secure; // A32 EL3 is always Secure
//        when Regime_EL2     return SecurityStateAtEL(EL2);
//        when Regime_EL20    return SecurityStateAtEL(EL2);
//        when Regime_EL10    return SecurityStateAtEL(EL1);

/// Library pseudocode for shared/translation/vmsa/StageOA
/// StageOA()
/// =========
/// Given the final walk state (a page or block descriptor), map the untranslated
/// input address bits to the output address
pub fn StageOA(_ia: u64, d128: u64, tgx: TGx, walkstate: TTWState) -> FullAddress {
    // Output Address
    let mut oa: FullAddress = unsafe { MaybeUninit::zeroed().assume_init_read() };
    let csize: u64;

    let tsize = TranslationSize(d128, tgx, walkstate.level);
    csize = if walkstate.contiguous {
        ContiguousSize(d128, tgx, walkstate.level)
    } else {
        0
    };

    let _ia_msb = tsize + csize;
    oa.paspace = walkstate.baseaddress.paspace;
    //oa.address = walkstate.baseaddress.address<55:ia_msb>:ia<ia_msb-1:0>;

    oa
}

/// Library pseudocode for shared/translation/vmsa/TGx
/// // TGx
/// // ===
/// // Translation granules sizes
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub enum TGx {
    #[default]
    TGx_4KB,
    TGx_16KB,
    TGx_64KB,
}

/// Library pseudocode for shared/translation/vmsa/TGxGranuleBits
/// TGxGranuleBits()
/// ================
/// Retrieve the address size, in bits, of a granule
pub fn TGxGranuleBits(tgx: TGx) -> u64 {
    match tgx {
        TGx::TGx_4KB => 12,
        TGx::TGx_16KB => 14,
        TGx::TGx_64KB => 16,
    }
}

// Library pseudocode for shared/translation/vmsa/TLBContext

// // TLBContext
// // ==========
// // Translation context compared on TLB lookups and invalidations, promoting a TLB hit on match

// type TLBContext is (
//    SecurityState ss,
//    Regime        regime,
//    bits(16)      vmid,
//    bits(16)      asid,
//    bit           nG,
//    PASpace       ipaspace, // Used in stage 2 lookups & invalidations only
//    boolean       includes_s1,
//    boolean       includes_s2,
//    boolean       includes_gpt,
//    bits(64)      ia,       // Input Address
//    TGx           tg,
//    bit           cnp,
//    integer       level,    // Assist TLBI level hints (FEAT_TTL)
//    boolean       isd128,
//    bit           xs        // XS attribute (FEAT_XS)
// )

// Library pseudocode for shared/translation/vmsa/TLBRecord

// // TLBRecord
// // =========
// // Translation output as a TLB payload

// type TLBRecord is (
//    TLBContext  context,
//    TTWState    walkstate,
//    integer     blocksize,    // Number of bits directly mapped from IA to OA
//    integer     contigsize,   // Number of entries log 2 marking a contiguous output range
//    bits(128)   s1descriptor, // Stage 1 leaf descriptor in memory (valid if the TLB caches stage 1)
//    bits(128)   s2descriptor  // Stage 2 leaf descriptor in memory (valid if the TLB caches stage 2)
// )

/// Library pseudocode for shared/translation/vmsa/TTWState

/// TTWState
/// ========
/// Translation table walk state
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TTWState {
    pub istable: bool,
    pub level: u64,
    pub baseaddress: FullAddress,
    /// bit
    pub contiguous: bool,
    /// Stage 1 Assured Translation Property
    pub s1assured: bool,
    /// Stage 2 AssuredOnly attribute bit
    pub s2assuredonly: bool,
    /// Stage 1 Disable Contiguous Hint bit
    pub disch: bool,
    /// bit
    pub nG: bool,
    /// bit
    pub guardedpage: bool,
    /// AArch32 Short-descriptor format walk only
    pub sdftype: SDFType,
    /// AArch32 Short-descriptor format walk only bits(4)
    pub domain: u8,
    pub memattrs: MemoryAttributes,
    pub permissions: Permissions,
}

/// Library pseudocode for shared/functions/system/EL0

pub const EL3: PrivilegeLevel = PrivilegeLevel::PL3;
pub const EL2: PrivilegeLevel = PrivilegeLevel::PL2;
pub const EL1: PrivilegeLevel = PrivilegeLevel::PL1;
pub const EL0: PrivilegeLevel = PrivilegeLevel::PL0;

/// Library pseudocode for shared/functions/system/PrivilegeLevel

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PrivilegeLevel {
    PL3 = 0b11,
    PL2 = 0b10,
    PL1 = 0b01,
    PL0 = 0b00,
}

/// Library pseudocode for shared/translation/vmsa/TranslationRegime

/// TranslationRegime()
/// ===================
/// Select the translation regime given the target EL and PE state

pub fn TranslationRegime(_el: PrivilegeLevel) -> Regime {
    todo!()
    // match el {
    //     self::EL3 if ELUsingAArch32(EL3) => Regime::Regime_EL30,
    //     self::EL3 => Regime::Regime_EL3,
    //     self::EL2 if ELIsInHost(EL2) => Regime::Regime_EL20,
    //     self::EL2 => Regime::Regime_EL2,
    //     self::EL1 => Regime::Regime_EL10,
    //     self::EL0 if CurrentSecurityState() == SecurityState::SS_Secure && ELUsingAArch32(EL3) => {
    //         Regime::Regime_EL30
    //     }
    //     self::EL0 if ELIsInHost(EL0) => Regime::Regime_EL20,
    //     self::EL0 => Regime::Regime_EL10,
    // }
}

/// Library pseudocode for shared/translation/vmsa/TranslationSize
/// TranslationSize()
/// =================
/// Compute the number of bits directly mapped from the input address
/// to the output address
pub fn TranslationSize(d128: u64, tgx: TGx, level: u64) -> u64 {
    let granulebits = TGxGranuleBits(tgx);
    let descsizelog2 = if d128 == 1 { 4 } else { 3 };
    let blockbits = (FINAL_LEVEL - level) * (granulebits - descsizelog2);

    granulebits + blockbits
}

// Library pseudocode for shared/translation/vmsa/UseASID

// // UseASID()
// // =========
// // Determine whether the translation context for the access requires ASID or is a global entry

// boolean UseASID(TLBContext accesscontext)
//    return HasUnprivileged(accesscontext.regime);

// Library pseudocode for shared/translation/vmsa/UseVMID

// // UseVMID()
// // =========
// // Determine whether the translation context for the access requires VMID to match a TLB entry

// boolean UseVMID(TLBContext accesscontext)
//    return accesscontext.regime == Regime_EL10 && EL2Enabled();

mod walkparams {
    use super::*;

    /// AArch64.GetVARange()
    /// ====================
    /// Determines if the VA that is to be translated lies in LOWER or UPPER address range.
    pub fn AArch64GetVARange(va: u64) -> VARange {
        //if va<55> == '0' {
        if (va & (1 << 55)) == 0 {
            return VARange::VARange_LOWER;
        }
        VARange::VARange_UPPER
    }
}

pub use walkparams::*;
