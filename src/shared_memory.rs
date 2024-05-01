// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later
#![allow(non_camel_case_types)]

use crate::shared::*;
use crate::shared_mpam::{GenMPAMCurEL, MPAMinfo};
use crate::shared_vmsa::*;

/// Library pseudocode for shared/functions/memory/Fault
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Fault {
    Fault_None,
    Fault_AccessFlag,
    Fault_Alignment,
    Fault_Background,
    Fault_Domain,
    Fault_Permission,
    Fault_Translation,
    Fault_AddressSize,
    Fault_SyncExternal,
    Fault_SyncExternalOnWalk,
    Fault_SyncParity,
    Fault_SyncParityOnWalk,
    Fault_AsyncParity,
    Fault_AsyncExternal,
    Fault_Debug,
    Fault_TLBConflict,
    Fault_BranchTarget,
    Fault_HWUpdateAccessFlag,
    Fault_Lockdown,
    Fault_Exclusive,
    Fault_ICacheMaint,
}

/// Library pseudocode for shared/functions/memory/FaultRecord

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FaultRecord {
    // Fault Status
    pub statuscode: Fault,
    // Type of access that faulted
    pub acctype: AccessType,
    // Intermediate physical address
    pub ipaddress: FullAddress,
    // Is on a Stage 1 translation table walk
    pub s2fs1walk: bool,
    // TRUE for a write, FALSE for a read
    pub write: bool,
    // For translation, access flag and permission faults
    pub level: u64,
    // IMPLEMENTATION DEFINED bit syndrome for External aborts
    pub extflag: bool,
    // Is a Stage 2 abort
    pub secondstage: bool,
    // Domain number, AArch32 only
    pub domain: u8,
    // [Armv8.2 RAS] AArch32 AET or AArch64 SET
    pub errortype: u8,
    // Debug method of entry, from AArch32 only
    pub debugmoe: u8,
}

impl FaultRecord {
    /// Library pseudocode for shared/translation/faults/NoFault
    /// Return a clear fault record indicating no faults have occured
    pub fn NoFault() -> Self {
        // FaultRecord fault;

        // fault.statuscode  = Fault_None;
        // fault.accessdesc  = AccessDescriptor UNKNOWN;
        // fault.secondstage = FALSE;
        // fault.s2fs1walk   = FALSE;
        // fault.dirtybit    = FALSE;
        // fault.overlay     = FALSE;
        // fault.toplevel    = FALSE;
        // fault.assuredonly = FALSE;
        // fault.s1tagnotdata = FALSE;
        // fault.tagaccess   = FALSE;
        // fault.gpcfs2walk  = FALSE;
        // fault.gpcf        = GPCNoFault();
        // fault.hdbssf      = FALSE;

        // return fault;
        todo!()
    }

    /// NoFault()
    /// =========
    /// Return a clear fault record indicating no faults have occured for a specific access

    pub fn NoFaultForAccess(_accdesc: AccessDescriptor) -> Self {
        // FaultRecord NoFault(AccessDescriptor accdesc)
        //     FaultRecord fault;

        //     fault.statuscode  = Fault_None;
        //     fault.accessdesc  = accdesc;
        //     fault.secondstage = FALSE;
        //     fault.s2fs1walk   = FALSE;
        //     fault.dirtybit    = FALSE;
        //     fault.overlay     = FALSE;
        //     fault.toplevel    = FALSE;
        //     fault.assuredonly = FALSE;
        //     fault.s1tagnotdata = FALSE;
        //     fault.tagaccess   = FALSE;
        //     fault.write       = !accdesc.read && accdesc.write;
        //     fault.gpcfs2walk  = FALSE;
        //     fault.gpcf        = GPCNoFault();
        //     fault.hdbssf      = FALSE;

        //     return fault;
        todo!()
    }
}

/// Library pseudocode for shared/functions/memory/AccessType
/// AccessType
/// ==========
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccessType {
    /// Instruction FETCH
    AccessType_IFETCH,
    /// Software load/store to a General Purpose Register
    AccessType_GPR,
    /// Software ASIMD extension load/store instructions
    AccessType_ASIMD,
    /// Software SVE load/store instructions
    AccessType_SVE,
    /// Software SME load/store instructions
    AccessType_SME,
    /// Sysop IC
    AccessType_IC,
    /// Sysop DC (not DC {Z,G,GZ}VA)
    AccessType_DC,
    /// Sysop DC {Z,G,GZ}VA
    AccessType_DCZero,
    /// Sysop AT
    AccessType_AT,
    /// NV2 memory redirected access
    AccessType_NV2,
    /// Statistical Profiling buffer access
    AccessType_SPE,
    /// Guarded Control Stack access
    AccessType_GCS,
    /// Trace Buffer access
    AccessType_TRBE,
    /// Granule Protection Table Walk
    AccessType_GPTW,
    /// Translation Table Walk
    AccessType_TTW,
}

/// Library pseudocode for shared/functions/memory/AccessDescriptor

/// # AccessDescriptor
///
/// Memory access or translation invocation details that steer architectural behavior
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AccessDescriptor {
    pub acctype: AccessType,
    /// Acting EL for the access bits(2}
    pub el: PrivilegeLevel,
    /// Acting Security State for the access
    pub ss: SecurityState,
    /// Acquire with Sequential Consistency
    pub acqsc: bool,
    /// FEAT_LRCPC: Acquire with Processor Consistency
    pub acqpc: bool,
    /// Release with Sequential Consistency
    pub relsc: bool,
    /// FEAT_LOR: Acquire/Release with limited ordering
    pub limitedordered: bool,
    /// Access has Exclusive semantics
    pub exclusive: bool,
    /// FEAT_LSE: Atomic read-modify-write access
    pub atomicop: bool,
    /// FEAT_LSE: The modification operation in the 'atomicop' access
    pub modop: MemAtomicOp,
    /// Hints the access is non-temporal
    pub nontemporal: bool,
    /// Read from memory or only require read permissions
    pub read: bool,
    /// Write to memory or only require write permissions
    pub write: bool,
    /// DC/IC: Cache operation
    pub cacheop: CacheOp,
    /// DC/IC: Scope of cache operation
    pub opscope: CacheOpScope,
    /// DC/IC: Type of target cache
    pub cachetype: CacheType,
    /// FEAT_PAN: The access is subject to PSTATE.PAN
    pub pan: bool,
    /// FEAT_TME: Access is part of a transaction
    pub transactional: bool,
    /// SVE: Non-faulting load
    pub nonfault: bool,
    /// SVE: First-fault load
    pub firstfault: bool,
    /// SVE: First-fault load for the first active element
    pub first: bool,
    /// SVE: Contiguous load/store not gather load/scatter store
    pub contiguous: bool,
    /// SME: Access made by PE while in streaming SVE mode
    pub streamingsve: bool,
    /// FEAT_LS64: Accesses by accelerator support loads/stores
    pub ls64: bool,
    /// FEAT_MOPS: Memory operation (CPY/SET) accesses
    pub mops: bool,
    /// FEAT_THE: Read-Check-Write access
    pub rcw: bool,
    /// FEAT_THE: Read-Check-Write Software access
    pub rcws: bool,
    /// FEAT_THE: Translation table walk access for TTB address
    pub toplevel: bool,
    /// FEAT_THE: The corresponding TTBR supplying the TTB
    pub varange: VARange,
    /// A32 Load/Store Multiple Data access
    pub a32lsmd: bool,
    /// FEAT_MTE2: Access is tag checked
    pub tagchecked: bool,
    /// FEAT_MTE: Access targets the tag bits
    pub tagaccess: bool,
    /// FEAT_MTE: Accesses that store Allocation tags to Device memory are CONSTRAINED UNPREDICTABLE
    pub devstoreunpred: bool,
    /// Access represents a Load/Store pair access
    pub ispair: bool,
    /// FEAT_LRCPC3: Highest address is accessed first
    pub highestaddressfirst: bool,
    /// FEAT_MPAM: MPAM information
    pub mpam: MPAMinfo,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FullAddress {
    pub paspace: PASpace,
    /// bits(52}
    pub address: u64,
}

/// Library pseudocode for shared/functions/memory/MemAttrHints

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemAttrHints {
    /// bits(2}
    pub attrs: MemAttr, // See MemAttr_*, Cacheability attributes
    /// bits(2}
    pub hints: MemHint, // See MemHint_*, Allocation hints
    pub transient: bool,
}

/// Library pseudocode for shared/functions/memory/MemType

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemType {
    MemType_Normal,
    MemType_Device,
}

/// Library pseudocode for shared/functions/memory/Permissions

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Permissions {
    /// Stage 1 hierarchical access permissions bits(2}
    pub ap_table: u8,
    /// Stage 1 hierarchical execute-never for single EL regimes bit
    pub xn_table: u8,
    /// Stage 1 hierarchical privileged execute-never bit
    pub pxn_table: u8,
    /// Stage 1 hierarchical unprivileged execute-never bit
    pub uxn_table: u8,
    /// Stage 1 access permissions bits(3}
    pub ap: u8,
    /// Stage 1 execute-never for single EL regimes bit
    pub xn: u8,
    /// Stage 1 unprivileged execute-never bit
    pub uxn: u8,
    /// Stage 1 privileged execute-never bit
    pub pxn: u8,
    /// Stage 2 access permissions bits(2}
    pub s2ap: u8,
    /// Stage 2 extended execute-never bit
    pub s2xnx: u8,
    /// Stage 2 execute-never bit
    pub s2xn: u8,
}

// Library pseudocode for shared/functions/memory/PhysMemRead

// // Returns the value read from memory, and a status.
// // Returned value is UNKNOWN if an external abort occurred while reading the
// // memory.
// // Otherwise the PhysMemRetStatus statuscode is Fault_None.
// (PhysMemRetStatus, bits(8*size)) PhysMemRead(AddressDescriptor desc, integer size,
//                                              AccessDescriptor accdesc);

/// Library pseudocode for shared/functions/memory/PhysMemRetStatus

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PhysMemRetStatus {
    /// Fault Status
    pub statuscode: Fault,
    /// IMPLEMENTATION DEFINED syndrome for External aborts bit
    pub extflag: u8,
    /// optional error state returned on a physical memory access bits(2}
    pub errortype: u8,
    /// status of 64B store bits(64}
    pub store64bstatus: u64,
    /// Type of access that faulted
    pub acctype: AccessType,
}

// Library pseudocode for shared/functions/memory/PhysMemWrite

// Writes the value to memory, and returns the status of the write. If there is an external abort
// on the write, the PhysMemRetStatus indicates this. Otherwise the statuscode of PhysMemRetStatus
// is Fault_None. PhysMemRetStatus PhysMemWrite(AddressDescriptor desc, integer size,
// AccessDescriptor accdesc, bits(8*size) value);

/// Library pseudocode for shared/functions/memory/PrefetchHint

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PrefetchHint {
    Prefetch_READ,
    Prefetch_WRITE,
    Prefetch_EXEC,
}

/// Library pseudocode for shared/functions/memory/Shareability

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Shareability {
    Shareability_NSH,
    Shareability_ISH,
    Shareability_OSH,
}

/// Library pseudocode for shared/translation/attrs/EffectiveShareability
/// EffectiveShareability()
/// =======================
/// Force Outer Shareability on Device and Normal iNCoNC memory
pub fn EffectiveShareability(memattrs: MemoryAttributes) -> Shareability {
    if memattrs.memtype == MemType::MemType_Device
        || (memattrs.inner.attrs == MemAttr::MemAttr_NC
            && memattrs.outer.attrs == MemAttr::MemAttr_NC)
    {
        return Shareability::Shareability_OSH;
    }

    memattrs.shareability
}

/// Library pseudocode for shared/functions/memory/VARange

/// VARange
/// =======
/// Virtual address ranges

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum VARange {
    #[default]
    VARange_LOWER,
    VARange_UPPER,
}

/// Library pseudocode for shared/functions/memory/DeviceType

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviceType {
    #[default]
    DeviceType_GRE,
    DeviceType_nGRE,
    DeviceType_nGnRE,
    DeviceType_nGnRnE,
}

/// Library pseudocode for shared/functions/memory/MemAtomicOp

/// MemAtomicOp
/// ===========
/// Atomic data processing instruction types.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemAtomicOp {
    #[default]
    MemAtomicOp_GCSSS1,
    MemAtomicOp_ADD,
    MemAtomicOp_BIC,
    MemAtomicOp_EOR,
    MemAtomicOp_ORR,
    MemAtomicOp_SMAX,
    MemAtomicOp_SMIN,
    MemAtomicOp_UMAX,
    MemAtomicOp_UMIN,
    MemAtomicOp_SWP,
    MemAtomicOp_CAS,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum CacheOp {
    #[default]
    CacheOp_Clean,
    CacheOp_Invalidate,
    CacheOp_CleanInvalidate,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum CacheOpScope {
    #[default]
    CacheOpScope_SetWay,
    CacheOpScope_PoU,
    CacheOpScope_PoC,
    CacheOpScope_PoE,
    CacheOpScope_PoP,
    CacheOpScope_PoDP,
    CacheOpScope_PoPA,
    CacheOpScope_ALLU,
    CacheOpScope_ALLUIS,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum CacheType {
    #[default]
    CacheType_Data,
    CacheType_Tag,
    CacheType_Data_Tag,
    CacheType_Instruction,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CachePASpace {
    CPAS_NonSecure,
    CPAS_Any, // Applicable only for DC *SW / IC IALLU* in Root state:
    // match entries from any PA Space
    CPAS_RealmNonSecure, // Applicable only for DC *SW / IC IALLU* in Realm state:
    // match entries from Realm or Non-Secure PAS
    CPAS_Realm,
    CPAS_Root,
    CPAS_SecureNonSecure, // Applicable only for DC *SW / IC IALLU* in Secure state:
    // match entries from Secure or Non-Secure PAS
    CPAS_Secure,
}

/// Library pseudocode for shared/functions/memory/MemOp

/// MemOp
/// =====
/// Memory access instruction types.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemOp {
    MemOp_LOAD,
    MemOp_STORE,
    MemOp_PREFETCH,
}

/// Library pseudocode for shared/functions/memory/Memory

/// Memory Tag type
/// ===============

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemTagType {
    MemTag_Untagged,
    MemTag_AllocationTagged,
    MemTag_CanonicallyTagged,
}

/// Library pseudocode for shared/functions/memory/MemoryAttributes
/// MemoryAttributes
/// ================
/// Memory attributes descriptor
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemoryAttributes {
    pub memtype: MemType,
    // For Device memory types
    pub device: DeviceType,
    // Inner hints and attributes
    pub inner: MemAttrHints,
    // Outer hints and attributes
    pub outer: MemAttrHints,
    // Shareability attribute
    pub shareability: Shareability,
    // MTE tag type for this memory.
    pub tags: MemTagType,
    // Allocation Tag access permission
    pub notagaccess: bool,
    // XS attribute bit
    pub xs: bool,
}

/// Library pseudocode for shared/functions/memory/Cacheability
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemAttr {
    #[default]
    // Non-cacheable
    MemAttr_NC = 0b00,
    // Write-through
    MemAttr_WT = 0b10,
    // Write-back
    MemAttr_WB = 0b11,
}

/// Library pseudocode for shared/functions/memory/Allocation
/// bits(2)
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemHint {
    #[default]
    /// No Read-Allocate, No Write-Allocate
    MemHint_No = 0b00,
    /// No Read-Allocate, Write-Allocate
    MemHint_WA = 0b01,
    /// Read-Allocate, No Write-Allocate
    MemHint_RA = 0b10,
    /// Read-Allocate, Write-Allocate
    MemHint_RWA = 0b11,
}

/// Library pseudocode for shared/functions/memory/NewAccDesc
/// NewAccDesc()
/// ============
/// Create a new AccessDescriptor with initialised fields
pub fn NewAccDesc(acctype: AccessType) -> AccessDescriptor {
    let accdesc: AccessDescriptor = AccessDescriptor {
        acctype,
        el: PSTATE.get_EL(),
        ss: SecurityStateAtEL(PSTATE.get_EL()),
        acqsc: false,
        acqpc: false,
        relsc: false,
        limitedordered: false,
        exclusive: false,
        rcw: false,
        rcws: false,
        atomicop: false,
        nontemporal: false,
        read: false,
        write: false,
        pan: false,
        nonfault: false,
        firstfault: false,
        first: false,
        contiguous: false,
        streamingsve: false,
        ls64: false,
        mops: false,
        a32lsmd: false,
        tagchecked: false,
        tagaccess: false,
        devstoreunpred: false,
        transactional: false,
        mpam: GenMPAMCurEL(acctype),
        ispair: false,
        highestaddressfirst: false,
        cacheop: CacheOp::default(),
        cachetype: CacheType::default(),
        modop: MemAtomicOp::default(),
        opscope: CacheOpScope::default(),
        toplevel: false,
        varange: VARange::default(),
    };
    accdesc
}

/// Library pseudocode for shared/functions/memory/PASpace
/// PASpace
/// =======
/// Physical address spaces
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PASpace {
    PAS_NonSecure,
    PAS_Secure,
    PAS_Root,
    PAS_Realm,
}

/// Library pseudocode for shared/functions/memory/CreateAccDescTTEUpdate
/// CreateAccDescTTEUpdate()
/// ========================
/// Access descriptor for translation table entry HW update
pub fn CreateAccDescTTEUpdate(accdesc_in: AccessDescriptor) -> AccessDescriptor {
    let mut accdesc: AccessDescriptor = NewAccDesc(AccessType::AccessType_TTW);
    accdesc.el = accdesc_in.el;
    accdesc.ss = accdesc_in.ss;
    accdesc.atomicop = true;
    accdesc.modop = MemAtomicOp::MemAtomicOp_CAS;
    accdesc.read = true;
    accdesc.write = true;
    accdesc.mpam = accdesc_in.mpam;
    accdesc
}

/// Library pseudocode for shared/translation/attrs/NormalNCMemAttr
/// NormalNCMemAttr()
/// =================
/// Normal Non-cacheable memory attributes
pub fn NormalNCMemAttr() -> MemoryAttributes {
    let non_cacheable: MemAttrHints = MemAttrHints {
        attrs: MemAttr::MemAttr_NC,
        hints: Default::default(),
        transient: Default::default(),
    };
    MemoryAttributes {
        memtype: MemType::MemType_Normal,
        device: DeviceType::default(),
        outer: non_cacheable,
        inner: non_cacheable,
        shareability: Shareability::Shareability_OSH,
        tags: MemTagType::MemTag_Untagged,
        notagaccess: false,
        xs: false,
    }
}
