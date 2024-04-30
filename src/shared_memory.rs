// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later
#![allow(non_camel_case_types)]

use crate::shared::*;
use crate::shared_mpam::MPAMinfo;
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

pub struct FaultRecord {
    // Fault Status
    pub statuscode: Fault,
    // Type of access that faulted
    pub acctype: AccType,
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

    pub fn NoFaultForAccess(accdesc: AccessDescriptor) -> Self {
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

/// Library pseudocode for shared/functions/memory/AccType

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccType {
    /// Normal loads and stores
    AccType_NORMAL,
    /// Streaming loads and stores
    AccType_STREAM,
    /// Vector loads and stores
    AccType_VEC,
    /// Streaming vector loads and stores
    AccType_VECSTREAM,
    /// Scalable vector loads and stores
    AccType_SVE,
    /// Scalable vector streaming loads and stores
    AccType_SVESTREAM,
    /// Streaming unprivileged loads and stores
    AccType_UNPRIVSTREAM,
    /// Load and store multiple
    AccType_A32LSMD,
    /// Atomic loads and stores
    AccType_ATOMIC,
    AccType_ATOMICRW,
    /// Load-Acquire and Store-Release
    AccType_ORDERED,
    AccType_ORDEREDRW,
    /// Load-Acquire and Store-Release with atomic access
    AccType_ORDEREDATOMIC,
    AccType_ORDEREDATOMICRW,
    /// Atomic 64-byte loads and stores
    AccType_ATOMICLS64,
    /// Load-LOAcquire and Store-LORelease
    AccType_LIMITEDORDERED,
    /// Load and store unprivileged
    AccType_UNPRIV,
    /// Instruction fetch
    AccType_IFETCH,
    /// Translation table walk
    AccType_TTW,
    /// Non-faulting loads
    AccType_NONFAULT,
    /// Contiguous FF load, not first element
    AccType_CNOTFIRST,
    /// MRS/MSR instruction used at EL1 and which is converted to a memory access that uses the
    /// EL2 translation regime
    AccType_NV2REGISTER,
    // Other operations
    /// Data cache maintenance
    AccType_DC,
    /// Instruction cache maintenance
    AccType_IC,
    /// DC ZVA instructions
    AccType_DCZVA,
    /// Address translation with PAN permission checks
    AccType_ATPAN,
    /// Address translation
    AccType_AT,
}

/// Library pseudocode for shared/functions/memory/AccessDescriptor

/// # AccessDescriptor
///
/// Memory access or translation invocation details that steer architectural behavior
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AccessDescriptor {
    pub acctype: AccType,
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

pub struct FullAddress {
    paspace: PASpace,
    /// bits(52}
    address: u64,
}

/// Library pseudocode for shared/functions/memory/MemAttrHints

pub struct MemAttrHints {
    /// bits(2}
    attrs: u8, // See MemAttr_*, Cacheability attributes
    /// bits(2}
    hints: u8, // See MemHint_*, Allocation hints
    transient: bool,
}

/// Library pseudocode for shared/functions/memory/MemType

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemType {
    MemType_Normal,
    MemType_Device,
}

/// Library pseudocode for shared/functions/memory/PASpace

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PASpace {
    PAS_NonSecure,
    PAS_Secure,
}

/// Library pseudocode for shared/functions/memory/Permissions

pub struct Permissions {
    /// Stage 1 hierarchical access permissions bits(2}
    ap_table: u8,
    /// Stage 1 hierarchical execute-never for single EL regimes bit
    xn_table: u8,
    /// Stage 1 hierarchical privileged execute-never bit
    pxn_table: u8,
    /// Stage 1 hierarchical unprivileged execute-never bit
    uxn_table: u8,
    /// Stage 1 access permissions bits(3}
    ap: u8,
    /// Stage 1 execute-never for single EL regimes bit
    xn: u8,
    /// Stage 1 unprivileged execute-never bit
    uxn: u8,
    /// Stage 1 privileged execute-never bit
    pxn: u8,
    /// Stage 2 access permissions bits(2}
    s2ap: u8,
    /// Stage 2 extended execute-never bit
    s2xnx: u8,
    /// Stage 2 execute-never bit
    s2xn: u8,
}

// Library pseudocode for shared/functions/memory/PhysMemRead

// // Returns the value read from memory, and a status.
// // Returned value is UNKNOWN if an external abort occurred while reading the
// // memory.
// // Otherwise the PhysMemRetStatus statuscode is Fault_None.
// (PhysMemRetStatus, bits(8*size)) PhysMemRead(AddressDescriptor desc, integer size,
//                                              AccessDescriptor accdesc);

/// Library pseudocode for shared/functions/memory/PhysMemRetStatus

pub struct PhysMemRetStatus {
    /// Fault Status
    statuscode: Fault,
    /// IMPLEMENTATION DEFINED syndrome for External aborts bit
    extflag: u8,
    /// optional error state returned on a physical memory access bits(2}
    errortype: u8,
    /// status of 64B store bits(64}
    store64bstatus: u64,
    /// Type of access that faulted
    acctype: AccType,
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

/// Library pseudocode for shared/functions/memory/VARange

/// VARange
/// =======
/// Virtual address ranges

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VARange {
    VARange_LOWER,
    VARange_UPPER,
}

/// Library pseudocode for shared/functions/memory/DeviceType

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviceType {
    DeviceType_GRE,
    DeviceType_nGRE,
    DeviceType_nGnRE,
    DeviceType_nGnRnE,
}

/// Library pseudocode for shared/functions/memory/MemAtomicOp

/// MemAtomicOp
/// ===========
/// Atomic data processing instruction types.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemAtomicOp {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CacheOp {
    CacheOp_Clean,
    CacheOp_Invalidate,
    CacheOp_CleanInvalidate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CacheOpScope {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CacheType {
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

// MemOp
// =====
// Memory access instruction types.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemOp {
    MemOp_LOAD,
    MemOp_STORE,
    MemOp_PREFETCH,
}

/// Library pseudocode for shared/functions/memory/Memory

// Memory Tag type
// ===============

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemTagType {
    MemTag_Untagged,
    MemTag_AllocationTagged,
    MemTag_CanonicallyTagged,
}

/// Library pseudocode for shared/functions/memory/MemoryAttributes

// MemoryAttributes
// ================
// Memory attributes descriptor

pub struct MemoryAttributes {
    memtype: MemType,
    // For Device memory types
    device: DeviceType,
    // Inner hints and attributes
    inner: MemAttrHints,
    // Outer hints and attributes
    outer: MemAttrHints,
    // Shareability attribute
    shareability: Shareability,
    // MTE tag type for this memory.
    tags: MemTagType,
    // Allocation Tag access permission
    notagaccess: bool,
    // XS attribute bit
    xs: bool,
}
