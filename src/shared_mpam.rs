// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later

// Library pseudocode for shared/functions/mpam/AltPARTIDSpace

// // AltPARTIDSpace()
// // ================
// // From the Security state, EL and ALTSP configuration, determine
// // whether to primary space or the alt space is selected and which
// // PARTID space is the alternative space. Return that alternative
// // PARTID space if selected or the primary space if not.

// PARTIDSpaceType AltPARTIDSpace(bits(2) el, SecurityState security,
//                               PARTIDSpaceType primaryPIDSpace)
//    case security of
//        when SS_NonSecure
//            assert el != EL3;
//            return primaryPIDSpace;
//        when SS_Secure
//            assert el != EL3;
//            if primaryPIDSpace == PIDSpace_NonSecure then
//                return primaryPIDSpace;
//            return AltPIDSecure(el, primaryPIDSpace);
//        when SS_Root
//            assert el == EL3;
//            if MPAM3_EL3.ALTSP_EL3 == '1' then
//                if MPAM3_EL3.RT_ALTSP_NS == '1' then
//                    return PIDSpace_NonSecure;
//                else
//                    return PIDSpace_Secure;
//            else
//                return primaryPIDSpace;
//        when SS_Realm
//            assert el != EL3;
//            return AltPIDRealm(el, primaryPIDSpace);
//        otherwise
//            Unreachable();

// Library pseudocode for shared/functions/mpam/AltPIDRealm

// // AltPIDRealm()
// // =============
// // Compute PARTID space as either the primary PARTID space or
// // alternative PARTID space in the Realm Security state.
// // Helper for AltPARTIDSpace.

// PARTIDSpaceType AltPIDRealm(bits(2) el, PARTIDSpaceType primaryPIDSpace)
//    PARTIDSpaceType PIDSpace = primaryPIDSpace;
//    case el of
//        when EL0
//            if ELIsInHost(EL0) then
//                if !UsePrimarySpaceEL2() then
//                    PIDSpace = PIDSpace_NonSecure;
//            elsif !UsePrimarySpaceEL10() then
//                PIDSpace = PIDSpace_NonSecure;
//        when EL1
//            if !UsePrimarySpaceEL10() then
//                PIDSpace = PIDSpace_NonSecure;
//        when EL2
//            if !UsePrimarySpaceEL2() then
//                PIDSpace = PIDSpace_NonSecure;
//        otherwise
//            Unreachable();
//    return PIDSpace;

// Library pseudocode for shared/functions/mpam/AltPIDSecure

// // AltPIDSecure()
// // ==============
// // Compute PARTID space as either the primary PARTID space or
// // alternative PARTID space in the Secure Security state.
// // Helper for AltPARTIDSpace.

// PARTIDSpaceType AltPIDSecure(bits(2) el, PARTIDSpaceType primaryPIDSpace)
//    PARTIDSpaceType PIDSpace = primaryPIDSpace;
//    case el of
//        when EL0
//            if EL2Enabled() then
//                if ELIsInHost(EL0) then
//                    if !UsePrimarySpaceEL2() then
//                        PIDSpace = PIDSpace_NonSecure;
//                elsif !UsePrimarySpaceEL10() then
//                    PIDSpace = PIDSpace_NonSecure;
//            elsif MPAM3_EL3.ALTSP_HEN == '0' && MPAM3_EL3.ALTSP_HFC == '1' then
//                PIDSpace = PIDSpace_NonSecure;
//        when EL1
//            if EL2Enabled() then
//                if !UsePrimarySpaceEL10() then
//                    PIDSpace = PIDSpace_NonSecure;
//            elsif MPAM3_EL3.ALTSP_HEN == '0' && MPAM3_EL3.ALTSP_HFC == '1' then
//                PIDSpace = PIDSpace_NonSecure;
//        when EL2
//            if !UsePrimarySpaceEL2() then
//                PIDSpace = PIDSpace_NonSecure;
//        otherwise
//            Unreachable();
//    return PIDSpace;

// Library pseudocode for shared/functions/mpam/DefaultMPAMInfo

// // DefaultMPAMInfo()
// // =================
// // Returns default MPAM info.  The partidspace argument sets
// // the PARTID space of the default MPAM information returned.

// MPAMinfo DefaultMPAMInfo(PARTIDSpaceType partidspace)
//    MPAMinfo defaultinfo;
//    defaultinfo.mpam_sp = partidspace;
//    defaultinfo.partid  = DEFAULT_PARTID;
//    defaultinfo.pmg     = DEFAULT_PMG;
//    return defaultinfo;

// Library pseudocode for shared/functions/mpam/GenMPAM

// // GenMPAM()
// // =========
// // Returns MPAMinfo for exception level el.
// // If in_d is TRUE returns MPAM information using PARTID_I and PMG_I fields
// // of MPAMel_ELx register and otherwise using PARTID_D and PMG_D fields.
// // If in_sm is TRUE returns MPAM information using PARTID_D and PMG_D fields
// // of MPAMSM_EL1 register.
// // Produces a PARTID in PARTID space pspace.

// MPAMinfo GenMPAM(bits(2) el, boolean in_d, boolean in_sm, PARTIDSpaceType pspace)
//    MPAMinfo returninfo;
//    PARTIDType partidel;
//    boolean perr;
//    // gstplk is guest OS application locked by the EL2 hypervisor to
//    // only use EL1 the virtual machine's PARTIDs.
//    boolean gstplk = (el == EL0 && EL2Enabled() &&
//                      MPAMHCR_EL2.GSTAPP_PLK == '1' &&
//                      HCR_EL2.TGE == '0');
//    bits(2) eff_el = if gstplk then EL1 else el;
//    (partidel, perr) = GenPARTID(eff_el, in_d, in_sm);
//    PMGType groupel  = GenPMG(eff_el, in_d, in_sm, perr);
//    returninfo.mpam_sp = pspace;
//    returninfo.partid  = partidel;
//    returninfo.pmg     = groupel;
//    return returninfo;

// Library pseudocode for shared/functions/mpam/GenMPAMAtEL

// // GenMPAMAtEL()
// // =============
// // Returns MPAMinfo for the specified EL.
// // May be called if MPAM is not implemented (but in an version that supports
// // MPAM), MPAM is disabled, or in AArch32.  In AArch32, convert the mode to
// // EL if can and use that to drive MPAM information generation.  If mode
// // cannot be converted, MPAM is not implemented, or MPAM is disabled return
// // default MPAM information for the current security state.

// MPAMinfo GenMPAMAtEL(AccessType acctype, bits(2) el)
//    bits(2) mpamEL;
//    boolean validEL = FALSE;
//    SecurityState security = SecurityStateAtEL(el);
//    boolean in_d = FALSE;
//    boolean in_sm = FALSE;
//    PARTIDSpaceType pspace = PARTIDSpaceFromSS(security);
//    if pspace == PIDSpace_NonSecure && !MPAMIsEnabled() then
//        return DefaultMPAMInfo(pspace);
//    if UsingAArch32() then
//        (validEL, mpamEL) = ELFromM32(PSTATE.M);
//    else
//        mpamEL = if acctype == AccessType_NV2 then EL2 else el;
//        validEL = TRUE;
//    case acctype of
//        when AccessType_IFETCH, AccessType_IC
//            in_d = TRUE;
//        when AccessType_SME
//            in_sm = (boolean IMPLEMENTATION_DEFINED "Shared SMCU" ||
//                     boolean IMPLEMENTATION_DEFINED "MPAMSM_EL1 label precedence");
//        when AccessType_ASIMD
//            in_sm = (IsFeatureImplemented(FEAT_SME) && PSTATE.SM == '1' &&
//                     (boolean IMPLEMENTATION_DEFINED "Shared SMCU" ||
//                      boolean IMPLEMENTATION_DEFINED "MPAMSM_EL1 label precedence"));
//        when AccessType_SVE
//            in_sm = (IsFeatureImplemented(FEAT_SME) && PSTATE.SM == '1' &&
//                     (boolean IMPLEMENTATION_DEFINED "Shared SMCU" ||
//                      boolean IMPLEMENTATION_DEFINED "MPAMSM_EL1 label precedence"));
//        otherwise
//            // Other access types are DATA accesses
//            in_d = FALSE;
//    if !validEL then
//        return DefaultMPAMInfo(pspace);
//    elsif IsFeatureImplemented(FEAT_RME) && MPAMIDR_EL1.HAS_ALTSP == '1' then
//        // Substitute alternative PARTID space if selected
//        pspace = AltPARTIDSpace(mpamEL, security, pspace);
//    if IsFeatureImplemented(FEAT_MPAMv0p1) && MPAMIDR_EL1.HAS_FORCE_NS == '1' then
//        if MPAM3_EL3.FORCE_NS == '1' && security == SS_Secure then
//            pspace = PIDSpace_NonSecure;
//    if ((IsFeatureImplemented(FEAT_MPAMv0p1) || IsFeatureImplemented(FEAT_MPAMv1p1)) &&
//          MPAMIDR_EL1.HAS_SDEFLT == '1') then
//        if MPAM3_EL3.SDEFLT == '1' && security == SS_Secure then
//            return DefaultMPAMInfo(pspace);
//    if !MPAMIsEnabled() then
//        return DefaultMPAMInfo(pspace);
//    else
//        return GenMPAM(mpamEL, in_d, in_sm, pspace);

// Library pseudocode for shared/functions/mpam/GenMPAMCurEL

// // GenMPAMCurEL()
// // ==============
// // Returns MPAMinfo for the current EL and security state.
// // May be called if MPAM is not implemented (but in an version that supports
// // MPAM), MPAM is disabled, or in AArch32.  In AArch32, convert the mode to
// // EL if can and use that to drive MPAM information generation.  If mode
// // cannot be converted, MPAM is not implemented, or MPAM is disabled return
// // default MPAM information for the current security state.

// MPAMinfo GenMPAMCurEL(AccessType acctype)
//    return GenMPAMAtEL(acctype, PSTATE.EL);

// Library pseudocode for shared/functions/mpam/GenPARTID

// // GenPARTID()
// // ===========
// // Returns physical PARTID and error boolean for exception level el.
// // If in_d is TRUE then PARTID is from MPAMel_ELx.PARTID_I and
// // otherwise from MPAMel_ELx.PARTID_D.
// // If in_sm is TRUE then PARTID is from MPAMSM_EL1.PARTID_D.

// (PARTIDType, boolean) GenPARTID(bits(2) el, boolean in_d, boolean in_sm)
//    PARTIDType partidel = GetMPAM_PARTID(el, in_d, in_sm);
//    PARTIDType partid_max = MPAMIDR_EL1.PARTID_MAX;
//    if UInt(partidel) > UInt(partid_max) then
//        return (DEFAULT_PARTID, TRUE);
//    if MPAMIsVirtual(el) then
//        return MAP_vPARTID(partidel);
//    else
//        return (partidel, FALSE);

// Library pseudocode for shared/functions/mpam/GenPMG

// // GenPMG()
// // ========
// // Returns PMG for exception level el and I- or D-side (in_d).
// // If PARTID generation (GenPARTID) encountered an error, GenPMG() should be
// // called with partid_err as TRUE.

// PMGType GenPMG(bits(2) el, boolean in_d, boolean in_sm, boolean partid_err)
//    integer pmg_max = UInt(MPAMIDR_EL1.PMG_MAX);
//    // It is CONSTRAINED UNPREDICTABLE whether partid_err forces PMG to
//    // use the default or if it uses the PMG from getMPAM_PMG.
//    if partid_err then
//        return DEFAULT_PMG;
//    PMGType groupel = GetMPAM_PMG(el, in_d, in_sm);
//    if UInt(groupel) <= pmg_max then
//        return groupel;
//    return DEFAULT_PMG;

// Library pseudocode for shared/functions/mpam/GetMPAM_PARTID

// // GetMPAM_PARTID()
// // ================
// // Returns a PARTID from one of the MPAMn_ELx or MPAMSM_EL1 registers.
// // If in_sm is TRUE, the MPAMSM_EL1 register is used. Otherwise,
// // MPAMn selects the MPAMn_ELx register used.
// // If in_d is TRUE, selects the PARTID_I field of that
// // register.  Otherwise, selects the PARTID_D field.

// PARTIDType GetMPAM_PARTID(bits(2) MPAMn, boolean in_d, boolean in_sm)
//    PARTIDType partid;

//    if in_sm then
//        partid = MPAMSM_EL1.PARTID_D;
//        return partid;

//    if in_d then
//        case MPAMn of
//            when '11' partid = MPAM3_EL3.PARTID_I;
//            when '10' partid = if EL2Enabled() then MPAM2_EL2.PARTID_I else DEFAULT_PARTID;
//            when '01' partid = MPAM1_EL1.PARTID_I;
//            when '00' partid = MPAM0_EL1.PARTID_I;
//            otherwise partid = PARTIDType UNKNOWN;
//    else
//        case MPAMn of
//            when '11' partid = MPAM3_EL3.PARTID_D;
//            when '10' partid = if EL2Enabled() then MPAM2_EL2.PARTID_D else DEFAULT_PARTID;
//            when '01' partid = MPAM1_EL1.PARTID_D;
//            when '00' partid = MPAM0_EL1.PARTID_D;
//            otherwise partid = PARTIDType UNKNOWN;
//    return partid;

// Library pseudocode for shared/functions/mpam/GetMPAM_PMG

// // GetMPAM_PMG()
// // =============
// // Returns a PMG from one of the MPAMn_ELx or MPAMSM_EL1 registers.
// // If in_sm is TRUE, the MPAMSM_EL1 register is used. Otherwise,
// // MPAMn selects the MPAMn_ELx register used.
// // If in_d is TRUE, selects the PMG_I field of that
// // register.  Otherwise, selects the PMG_D field.

// PMGType GetMPAM_PMG(bits(2) MPAMn, boolean in_d, boolean in_sm)
//    PMGType pmg;

//    if in_sm then
//        pmg = MPAMSM_EL1.PMG_D;
//        return pmg;

//    if in_d then
//        case MPAMn of
//            when '11' pmg = MPAM3_EL3.PMG_I;
//            when '10' pmg = if EL2Enabled() then MPAM2_EL2.PMG_I else DEFAULT_PMG;
//            when '01' pmg = MPAM1_EL1.PMG_I;
//            when '00' pmg = MPAM0_EL1.PMG_I;
//            otherwise pmg = PMGType UNKNOWN;
//    else
//        case MPAMn of
//            when '11' pmg = MPAM3_EL3.PMG_D;
//            when '10' pmg = if EL2Enabled() then MPAM2_EL2.PMG_D else DEFAULT_PMG;
//            when '01' pmg = MPAM1_EL1.PMG_D;
//            when '00' pmg = MPAM0_EL1.PMG_D;
//            otherwise pmg = PMGType UNKNOWN;
//    return pmg;

// Library pseudocode for shared/functions/mpam/MAP_vPARTID

// // MAP_vPARTID()
// // =============
// // Performs conversion of virtual PARTID into physical PARTID
// // Contains all of the error checking and implementation
// // choices for the conversion.

// (PARTIDType, boolean) MAP_vPARTID(PARTIDType vpartid)
//    // should not ever be called if EL2 is not implemented
//    // or is implemented but not enabled in the current
//    // security state.
//    PARTIDType ret;
//    boolean err;
//    integer virt    = UInt(vpartid);
//    integer vpmrmax = UInt(MPAMIDR_EL1.VPMR_MAX);

//    // vpartid_max is largest vpartid supported
//    integer vpartid_max = (vpmrmax << 2) + 3;

//    // One of many ways to reduce vpartid to value less than vpartid_max.
//    if UInt(vpartid) > vpartid_max then
//        virt = virt MOD (vpartid_max+1);

//    // Check for valid mapping entry.
//    if MPAMVPMV_EL2<virt> == '1' then
//        // vpartid has a valid mapping so access the map.
//        ret = mapvpmw(virt);
//        err = FALSE;

//    // Is the default virtual PARTID valid?
//    elsif MPAMVPMV_EL2<0> == '1' then
//        // Yes, so use default mapping for vpartid == 0.
//        ret = MPAMVPM0_EL2<0 +: 16>;
//        err = FALSE;

//    // Neither is valid so use default physical PARTID.
//    else
//        ret = DEFAULT_PARTID;
//        err = TRUE;

//    // Check that the physical PARTID is in-range.
//    // This physical PARTID came from a virtual mapping entry.
//    integer partid_max = UInt(MPAMIDR_EL1.PARTID_MAX);
//    if UInt(ret) > partid_max then
//        // Out of range, so return default physical PARTID
//        ret = DEFAULT_PARTID;
//        err = TRUE;
//    return (ret, err);

// Library pseudocode for shared/functions/mpam/MPAM

// constant PARTIDType DEFAULT_PARTID = 0<15:0>;
// constant PMGType    DEFAULT_PMG    = 0<7:0>;

// // Defines the MPAM _engine_. The _engine_ produces the MPAM labels for memory
// // accesses from the state information stored in the MPAM System registers.

// // The MPAM _engine_ runs in all states and with the MPAM AArch64 system
// // registers and PE execution state controlling its behavior.

// // MPAM Types
// // ==========

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct PARTIDType(pub u16);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct PMGType(pub u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PARTIDSpaceType {
    PIDSpace_Secure,
    PIDSpace_Root,
    PIDSpace_Realm,
    PIDSpace_NonSecure,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MPAMinfo {
    pub mpam_sp: PARTIDSpaceType,
    pub partid: PARTIDType,
    pub pmg: PMGType,
}

// Library pseudocode for shared/functions/mpam/MPAMIsEnabled

// // MPAMIsEnabled()
// // ===============
// // Returns TRUE if MPAMisEnabled.

// boolean MPAMIsEnabled()
//    el = HighestEL();
//    case el of
//        when EL3 return MPAM3_EL3.MPAMEN == '1';
//        when EL2 return MPAM2_EL2.MPAMEN == '1';
//        when EL1 return MPAM1_EL1.MPAMEN == '1';

// Library pseudocode for shared/functions/mpam/MPAMIsVirtual

// // MPAMIsVirtual()
// // ===============
// // Returns TRUE if MPAM is configured to be virtual at EL.

// boolean MPAMIsVirtual(bits(2) el)
//    return (MPAMIDR_EL1.HAS_HCR == '1' && EL2Enabled() &&
//            ((el == EL0 && MPAMHCR_EL2.EL0_VPMEN == '1' && !ELIsInHost(EL0)) ||
//             (el == EL1 && MPAMHCR_EL2.EL1_VPMEN == '1')));

// Library pseudocode for shared/functions/mpam/PARTIDSpaceFromSS

// // PARTIDSpaceFromSS()
// // ===================
// // Returns the primary PARTID space from the Security State.

// PARTIDSpaceType PARTIDSpaceFromSS(SecurityState security)
//    case security of
//        when SS_NonSecure
//            return PIDSpace_NonSecure;
//        when SS_Root
//            return PIDSpace_Root;
//        when SS_Realm
//            return PIDSpace_Realm;
//        when SS_Secure
//            return PIDSpace_Secure;
//        otherwise
//            Unreachable();

// Library pseudocode for shared/functions/mpam/UsePrimarySpaceEL10

// // UsePrimarySpaceEL10()
// // =====================
// // Checks whether Primary space is configured in the
// // MPAM3_EL3 and MPAM2_EL2 ALTSP control bits that affect
// // MPAM ALTSP use at EL1 and EL0.

// boolean UsePrimarySpaceEL10()
//    if MPAM3_EL3.ALTSP_HEN == '0' then
//        return MPAM3_EL3.ALTSP_HFC == '0';
//    return !MPAMIsEnabled() || !EL2Enabled() || MPAM2_EL2.ALTSP_HFC == '0';

// Library pseudocode for shared/functions/mpam/UsePrimarySpaceEL2

// // UsePrimarySpaceEL2()
// // ====================
// // Checks whether Primary space is configured in the
// // MPAM3_EL3 and MPAM2_EL2 ALTSP control bits that affect
// // MPAM ALTSP use at EL2.

// boolean UsePrimarySpaceEL2()
//    if MPAM3_EL3.ALTSP_HEN == '0' then
//        return MPAM3_EL3.ALTSP_HFC == '0';
//    return !MPAMIsEnabled() || MPAM2_EL2.ALTSP_EL2 == '0';

// Library pseudocode for shared/functions/mpam/mapvpmw

// // mapvpmw()
// // =========
// // Map a virtual PARTID into a physical PARTID using
// // the MPAMVPMn_EL2 registers.
// // vpartid is now assumed in-range and valid (checked by caller)
// // returns physical PARTID from mapping entry.

// PARTIDType mapvpmw(integer vpartid)
//    bits(64) vpmw;
//    integer  wd = vpartid DIV 4;
//    case wd of
//        when 0 vpmw = MPAMVPM0_EL2;
//        when 1 vpmw = MPAMVPM1_EL2;
//        when 2 vpmw = MPAMVPM2_EL2;
//        when 3 vpmw = MPAMVPM3_EL2;
//        when 4 vpmw = MPAMVPM4_EL2;
//        when 5 vpmw = MPAMVPM5_EL2;
//        when 6 vpmw = MPAMVPM6_EL2;
//        when 7 vpmw = MPAMVPM7_EL2;
//        otherwise vpmw = Zeros(64);
//    // vpme_lsb selects LSB of field within register
//    integer vpme_lsb = (vpartid MOD 4) * 16;
//    return vpmw<vpme_lsb +: 16>;
