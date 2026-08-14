//! Read-only macOS IOMedia object identity capture and replay.
//!
//! A BSD disk name is diagnostic metadata, not durable identity.  This module
//! binds each attached APFS node to the boot-scoped 64-bit IORegistry entry ID
//! of its IOMedia object and re-resolves that object through IOKit before any
//! future effect implementation may act.  The returned object deliberately
//! exposes no unmount/eject primitive and keeps both IOMedia references and
//! the DADiskRef alive through RAII.

#[cfg(target_os = "macos")]
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::AcceptanceError;

const IDENTITY_SCHEMA: &str = "hepta_mac_iomedia_registry_identity_v1";
const TOPOLOGY_SCHEMA: &str = "hepta_mac_attached_iomedia_topology_v1";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IOMediaRegistryIdentityV1 {
    pub authority_granted: bool,
    pub bsd_name: String,
    /// Canonical fixed-width lower-hex u64.  JSON numbers are forbidden so
    /// readers cannot silently round IDs above JavaScript's 53-bit range.
    pub registry_entry_id: String,
    pub schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttachedIOMediaTopologyV1 {
    pub apfs_container: IOMediaRegistryIdentityV1,
    pub apfs_volume: IOMediaRegistryIdentityV1,
    pub authority_granted: bool,
    pub boot_session_uuid: String,
    pub physical_store: IOMediaRegistryIdentityV1,
    pub physical_whole: IOMediaRegistryIdentityV1,
    pub schema: String,
}

#[derive(Clone, Copy, Debug)]
pub struct ExpectedIOMediaTopology<'a> {
    pub apfs_container: &'a str,
    pub apfs_volume: &'a str,
    pub physical_store: &'a str,
    pub physical_whole: &'a str,
}

impl<'a> ExpectedIOMediaTopology<'a> {
    fn ordered(self) -> [(&'static str, &'a str); 4] {
        [
            ("physical whole", self.physical_whole),
            ("physical store", self.physical_store),
            ("APFS container", self.apfs_container),
            ("APFS volume", self.apfs_volume),
        ]
    }
}

impl AttachedIOMediaTopologyV1 {
    fn ordered(&self) -> [(&'static str, &IOMediaRegistryIdentityV1); 4] {
        [
            ("physical whole", &self.physical_whole),
            ("physical store", &self.physical_store),
            ("APFS container", &self.apfs_container),
            ("APFS volume", &self.apfs_volume),
        ]
    }
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

fn valid_uuid(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 36
        && bytes
            .iter()
            .any(|byte| byte.is_ascii_hexdigit() && *byte != b'0')
        && bytes.iter().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => *byte == b'-',
            _ => byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase(),
        })
}

fn valid_bsd_name(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("disk") else {
        return false;
    };
    let (whole, slice) = match rest.split_once('s') {
        Some((whole, slice)) if !slice.contains('s') => (whole, Some(slice)),
        Some(_) => return false,
        None => (rest, None),
    };
    !whole.is_empty()
        && whole.bytes().all(|byte| byte.is_ascii_digit())
        && (whole == "0" || !whole.starts_with('0'))
        && slice.is_none_or(|slice| {
            !slice.is_empty()
                && !slice.starts_with('0')
                && slice.bytes().all(|byte| byte.is_ascii_digit())
        })
}

fn parse_registry_entry_id(value: &str) -> Result<u64, AcceptanceError> {
    if value.len() != 16
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(invalid(
            "IOMedia registry entry ID is not canonical 16-digit lower hex",
        ));
    }
    let parsed = u64::from_str_radix(value, 16)
        .map_err(|_| invalid("IOMedia registry entry ID does not fit u64"))?;
    if parsed == 0 {
        return Err(invalid("IOMedia registry entry ID is zero"));
    }
    Ok(parsed)
}

fn validate_registry_identity(
    identity: &IOMediaRegistryIdentityV1,
    expected_bsd_name: &str,
    label: &str,
) -> Result<(), AcceptanceError> {
    if identity.schema != IDENTITY_SCHEMA
        || identity.authority_granted
        || parse_registry_entry_id(&identity.registry_entry_id).is_err()
        || !valid_bsd_name(&identity.bsd_name)
        || identity.bsd_name != expected_bsd_name
    {
        return Err(invalid(format!(
            "{label} IOMedia identity is malformed, grants authority, or binds the wrong BSD name"
        )));
    }
    Ok(())
}

pub fn validate_iomedia_topology_identity_shape(
    identity: &AttachedIOMediaTopologyV1,
    expected: ExpectedIOMediaTopology<'_>,
) -> Result<(), AcceptanceError> {
    if identity.schema != TOPOLOGY_SCHEMA
        || identity.authority_granted
        || !valid_uuid(&identity.boot_session_uuid)
    {
        return Err(invalid(
            "IOMedia topology identity is malformed or grants authority",
        ));
    }
    let actual = identity.ordered();
    let expected = expected.ordered();
    let mut registry_entry_ids = BTreeSet::new();
    let mut bsd_names = BTreeSet::new();
    for (index, ((label, node), (expected_label, expected_bsd_name))) in
        actual.into_iter().zip(expected).enumerate()
    {
        if label != expected_label {
            return Err(invalid("IOMedia topology role order changed"));
        }
        validate_registry_identity(node, expected_bsd_name, label)?;
        if !registry_entry_ids.insert(node.registry_entry_id.as_str())
            || !bsd_names.insert(node.bsd_name.as_str())
        {
            return Err(invalid(format!(
                "IOMedia topology node {index} aliases another BSD name or registry entry ID"
            )));
        }
    }
    Ok(())
}

fn validate_iomedia_topology_identity_against_boot(
    identity: &AttachedIOMediaTopologyV1,
    expected: ExpectedIOMediaTopology<'_>,
    current_boot_session_uuid: &str,
) -> Result<(), AcceptanceError> {
    validate_iomedia_topology_identity_shape(identity, expected)?;
    if !valid_uuid(current_boot_session_uuid)
        || identity.boot_session_uuid != current_boot_session_uuid
    {
        return Err(invalid(
            "IOMedia topology identity belongs to another boot session",
        ));
    }
    Ok(())
}

pub fn validate_iomedia_topology_identity_current_boot(
    identity: &AttachedIOMediaTopologyV1,
    expected: ExpectedIOMediaTopology<'_>,
) -> Result<(), AcceptanceError> {
    validate_iomedia_topology_identity_against_boot(
        identity,
        expected,
        &current_boot_session_uuid()?,
    )
}

fn registry_identity(bsd_name: String, registry_entry_id: u64) -> IOMediaRegistryIdentityV1 {
    IOMediaRegistryIdentityV1 {
        authority_granted: false,
        bsd_name,
        registry_entry_id: format!("{registry_entry_id:016x}"),
        schema: IDENTITY_SCHEMA.to_string(),
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::marker::PhantomData;
    use std::os::raw::c_char;
    use std::os::raw::c_int;
    use std::os::raw::c_uint;
    use std::os::raw::c_void;
    use std::rc::Rc;

    use super::*;

    type CFAllocatorRef = *const c_void;
    type CFDictionaryRef = *const c_void;
    type CFMutableDictionaryRef = *mut c_void;
    type CFTypeRef = *const c_void;
    type DADiskRef = *const c_void;
    type DASessionRef = *const c_void;
    type IoIterator = c_uint;
    type IoObject = c_uint;
    type IoRegistryEntry = c_uint;
    type IoService = c_uint;
    type KernReturn = c_int;
    type MachPort = c_uint;

    const IO_OBJECT_NULL: IoObject = 0;
    const KERN_SUCCESS: KernReturn = 0;
    const K_IOMAIN_PORT_DEFAULT: MachPort = 0;

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        fn CFRelease(value: CFTypeRef);
        fn CFRetain(value: CFTypeRef) -> CFTypeRef;
    }

    #[link(name = "DiskArbitration", kind = "framework")]
    unsafe extern "C" {
        fn DASessionCreate(allocator: CFAllocatorRef) -> DASessionRef;
        fn DADiskCreateFromIOMedia(
            allocator: CFAllocatorRef,
            session: DASessionRef,
            media: IoService,
        ) -> DADiskRef;
        fn DADiskCopyIOMedia(disk: DADiskRef) -> IoService;
        fn DADiskGetBSDName(disk: DADiskRef) -> *const c_char;
    }

    #[link(name = "IOKit", kind = "framework")]
    unsafe extern "C" {
        fn IOIteratorNext(iterator: IoIterator) -> IoObject;
        fn IOObjectConformsTo(object: IoObject, class_name: *const c_char) -> libc::boolean_t;
        fn IOObjectRelease(object: IoObject) -> KernReturn;
        fn IORegistryEntryGetRegistryEntryID(
            entry: IoRegistryEntry,
            entry_id: *mut u64,
        ) -> KernReturn;
        fn IORegistryEntryIDMatching(entry_id: u64) -> CFMutableDictionaryRef;
        fn IOServiceGetMatchingService(main_port: MachPort, matching: CFDictionaryRef)
        -> IoService;
        fn IOServiceGetMatchingServices(
            main_port: MachPort,
            matching: CFDictionaryRef,
            existing: *mut IoIterator,
        ) -> KernReturn;
        fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;
    }

    pub(super) fn current_boot_session_uuid_impl() -> Result<String, AcceptanceError> {
        let name = CString::new("kern.bootsessionuuid").expect("fixed sysctl name");
        let mut length = 0_usize;
        if unsafe {
            libc::sysctlbyname(
                name.as_ptr(),
                std::ptr::null_mut(),
                &mut length,
                std::ptr::null_mut(),
                0,
            )
        } != 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
        if !(2..=128).contains(&length) {
            return Err(invalid("boot session UUID sysctl length is invalid"));
        }
        let mut bytes = vec![0_u8; length];
        if unsafe {
            libc::sysctlbyname(
                name.as_ptr(),
                bytes.as_mut_ptr().cast(),
                &mut length,
                std::ptr::null_mut(),
                0,
            )
        } != 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
        bytes.truncate(length);
        if bytes.last() == Some(&0) {
            bytes.pop();
        }
        let value = String::from_utf8(bytes)
            .map_err(|_| invalid("boot session UUID is not UTF-8"))?
            .to_ascii_lowercase();
        if !valid_uuid(&value) {
            return Err(invalid("boot session UUID is malformed"));
        }
        Ok(value)
    }

    struct Session(DASessionRef);

    impl Session {
        fn create() -> Result<Self, AcceptanceError> {
            let session = unsafe { DASessionCreate(std::ptr::null()) };
            if session.is_null() {
                return Err(invalid("DiskArbitration session creation failed"));
            }
            Ok(Self(session))
        }
    }

    impl Drop for Session {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) };
            }
        }
    }

    impl Clone for Session {
        fn clone(&self) -> Self {
            unsafe { CFRetain(self.0) };
            Self(self.0)
        }
    }

    struct IoObjectGuard(IoObject);

    impl IoObjectGuard {
        fn new(object: IoObject, label: &str) -> Result<Self, AcceptanceError> {
            if object == IO_OBJECT_NULL {
                return Err(invalid(format!("{label} returned no IOKit object")));
            }
            Ok(Self(object))
        }
    }

    impl Drop for IoObjectGuard {
        fn drop(&mut self) {
            if self.0 != IO_OBJECT_NULL {
                let _ = unsafe { IOObjectRelease(self.0) };
            }
        }
    }

    struct DADiskGuard(DADiskRef);

    impl DADiskGuard {
        fn new(disk: DADiskRef) -> Result<Self, AcceptanceError> {
            if disk.is_null() {
                return Err(invalid("DADiskCreateFromIOMedia returned no disk"));
            }
            Ok(Self(disk))
        }
    }

    impl Drop for DADiskGuard {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) };
            }
        }
    }

    pub struct ResolvedIOMediaObject {
        report: IOMediaRegistryIdentityV1,
        // Rust drops fields in declaration order. Keep the dependent DADisk
        // media, DADiskRef, and matched IOMedia ahead of the DASession so the
        // session is always the final CoreFoundation object released.
        _disk_media: IoObjectGuard,
        _disk: DADiskGuard,
        _matched_media: IoObjectGuard,
        _session: Session,
        _not_send_or_sync: PhantomData<Rc<()>>,
    }

    impl ResolvedIOMediaObject {
        pub fn identity(&self) -> &IOMediaRegistryIdentityV1 {
            &self.report
        }
    }

    fn registry_id(object: IoObject, label: &str) -> Result<u64, AcceptanceError> {
        let mut registry_entry_id = 0_u64;
        let rc = unsafe { IORegistryEntryGetRegistryEntryID(object, &mut registry_entry_id) };
        if rc != KERN_SUCCESS || registry_entry_id == 0 {
            return Err(invalid(format!(
                "{label} registry entry ID lookup failed with IOKit status 0x{rc:x}"
            )));
        }
        Ok(registry_entry_id)
    }

    fn require_iomedia(object: IoObject, label: &str) -> Result<(), AcceptanceError> {
        let class = CString::new("IOMedia").expect("fixed IOKit class");
        if unsafe { IOObjectConformsTo(object, class.as_ptr()) } == 0 {
            return Err(invalid(format!(
                "{label} registry object does not conform to IOMedia"
            )));
        }
        Ok(())
    }

    fn disk_bsd_name(disk: DADiskRef) -> Result<String, AcceptanceError> {
        let pointer = unsafe { DADiskGetBSDName(disk) };
        if pointer.is_null() {
            return Err(invalid("DADiskRef has no BSD name"));
        }
        let name = unsafe { CStr::from_ptr(pointer) }
            .to_str()
            .map_err(|_| invalid("DADiskRef BSD name is not UTF-8"))?
            .to_string();
        Ok(name)
    }

    fn describe_media(
        session: &Session,
        media: IoObjectGuard,
        expected_registry_entry_id: u64,
        expected_bsd_name: &str,
    ) -> Result<ResolvedIOMediaObject, AcceptanceError> {
        require_iomedia(media.0, "matched")?;
        let matched_registry_entry_id = registry_id(media.0, "matched IOMedia")?;
        if matched_registry_entry_id != expected_registry_entry_id {
            return Err(invalid(
                "IORegistryEntryIDMatching resolved a different registry entry ID",
            ));
        }
        let disk = DADiskGuard::new(unsafe {
            DADiskCreateFromIOMedia(std::ptr::null(), session.0, media.0)
        })?;
        let bsd_name = disk_bsd_name(disk.0)?;
        if bsd_name != expected_bsd_name {
            return Err(invalid(
                "resolved IOMedia registry ID now has a different BSD name",
            ));
        }
        let disk_media =
            IoObjectGuard::new(unsafe { DADiskCopyIOMedia(disk.0) }, "DADiskCopyIOMedia")?;
        require_iomedia(disk_media.0, "DADisk replay")?;
        let replayed_registry_entry_id = registry_id(disk_media.0, "DADisk IOMedia replay")?;
        if replayed_registry_entry_id != expected_registry_entry_id {
            return Err(invalid(
                "DADiskCopyIOMedia replay changed the registry entry ID",
            ));
        }
        Ok(ResolvedIOMediaObject {
            report: registry_identity(bsd_name, replayed_registry_entry_id),
            _disk_media: disk_media,
            _disk: disk,
            _matched_media: media,
            _session: session.clone(),
            _not_send_or_sync: PhantomData,
        })
    }

    pub fn resolve(
        registry_entry_id: &str,
        expected_boot_session_uuid: &str,
        expected_bsd_name: &str,
    ) -> Result<ResolvedIOMediaObject, AcceptanceError> {
        let registry_entry_id = parse_registry_entry_id(registry_entry_id)?;
        if !valid_uuid(expected_boot_session_uuid) {
            return Err(invalid("IOMedia resolution input is malformed"));
        }
        if !valid_bsd_name(expected_bsd_name) {
            return Err(invalid("IOMedia resolution expected BSD name is malformed"));
        }
        let current_boot = current_boot_session_uuid_impl()?;
        if current_boot != expected_boot_session_uuid {
            return Err(invalid(
                "IOMedia registry entry ID belongs to a different boot session",
            ));
        }
        let matching = unsafe { IORegistryEntryIDMatching(registry_entry_id) };
        if matching.is_null() {
            return Err(invalid(
                "IORegistryEntryIDMatching could not construct a matching dictionary",
            ));
        }
        let media = IoObjectGuard::new(
            unsafe { IOServiceGetMatchingService(K_IOMAIN_PORT_DEFAULT, matching) },
            "IOServiceGetMatchingService",
        )?;
        let session = Session::create()?;
        describe_media(&session, media, registry_entry_id, expected_bsd_name)
    }

    pub fn enumerate() -> Result<Vec<IOMediaRegistryIdentityV1>, AcceptanceError> {
        let class = CString::new("IOMedia").expect("fixed IOKit class");
        let matching = unsafe { IOServiceMatching(class.as_ptr()) };
        if matching.is_null() {
            return Err(invalid("IOServiceMatching could not match IOMedia"));
        }
        let mut iterator = IO_OBJECT_NULL;
        let rc =
            unsafe { IOServiceGetMatchingServices(K_IOMAIN_PORT_DEFAULT, matching, &mut iterator) };
        if rc != KERN_SUCCESS || iterator == IO_OBJECT_NULL {
            return Err(invalid(format!(
                "IOServiceGetMatchingServices failed with IOKit status 0x{rc:x}"
            )));
        }
        let iterator = IoObjectGuard(iterator);
        let session = Session::create()?;
        let boot = current_boot_session_uuid_impl()?;
        let mut identities = BTreeMap::new();
        let mut registry_entry_ids = BTreeSet::new();
        loop {
            let object = unsafe { IOIteratorNext(iterator.0) };
            if object == IO_OBJECT_NULL {
                break;
            }
            let media = IoObjectGuard(object);
            require_iomedia(media.0, "enumerated")?;
            let registry_entry_id = registry_id(media.0, "enumerated IOMedia")?;
            if !registry_entry_ids.insert(registry_entry_id) {
                return Err(invalid(
                    "IOMedia enumeration returned a duplicate registry entry ID",
                ));
            }
            let disk = DADiskGuard::new(unsafe {
                DADiskCreateFromIOMedia(std::ptr::null(), session.0, media.0)
            })?;
            let bsd_name = disk_bsd_name(disk.0)?;
            // APFS snapshot/synthesized media can use nested names such as
            // disk3s1s1. They are real IOMedia objects but can never be one of
            // this fixture's four exact diskN/diskNsN topology roles.
            if !valid_bsd_name(&bsd_name) {
                continue;
            }
            let disk_media = IoObjectGuard::new(
                unsafe { DADiskCopyIOMedia(disk.0) },
                "enumerated DADiskCopyIOMedia",
            )?;
            require_iomedia(disk_media.0, "enumerated DADisk replay")?;
            if registry_id(disk_media.0, "enumerated DADisk IOMedia replay")? != registry_entry_id {
                return Err(invalid(
                    "enumerated DADiskCopyIOMedia changed the registry entry ID",
                ));
            }
            if identities
                .insert(
                    bsd_name.clone(),
                    registry_identity(bsd_name, registry_entry_id),
                )
                .is_some()
            {
                return Err(invalid("IOMedia enumeration returned a duplicate BSD name"));
            }
        }
        if identities.is_empty() {
            return Err(invalid(
                "IOMedia enumeration returned no objects; capability is unavailable",
            ));
        }
        for identity in identities.values() {
            let replay = resolve(
                identity.registry_entry_id.as_str(),
                // Stored IDs are canonical strings; the resolver parses to
                // u64 only immediately before the IOKit call.
                &boot,
                identity.bsd_name.as_str(),
            )?;
            if replay.identity() != identity {
                return Err(invalid("IOMedia enumeration replay changed identity"));
            }
        }
        Ok(identities.into_values().collect())
    }

    pub fn capture(
        expected: ExpectedIOMediaTopology<'_>,
    ) -> Result<AttachedIOMediaTopologyV1, AcceptanceError> {
        let boot = current_boot_session_uuid_impl()?;
        let identities = enumerate()?
            .into_iter()
            .map(|identity| (identity.bsd_name.clone(), identity))
            .collect::<BTreeMap<_, _>>();
        let find = |label: &str, bsd_name: &str| {
            identities.get(bsd_name).cloned().ok_or_else(|| {
                invalid(format!(
                    "fresh IOMedia graph has no exact {label} node {bsd_name}"
                ))
            })
        };
        let topology = AttachedIOMediaTopologyV1 {
            apfs_container: find("APFS container", expected.apfs_container)?,
            apfs_volume: find("APFS volume", expected.apfs_volume)?,
            authority_granted: false,
            boot_session_uuid: boot.clone(),
            physical_store: find("physical store", expected.physical_store)?,
            physical_whole: find("physical whole", expected.physical_whole)?,
            schema: TOPOLOGY_SCHEMA.to_string(),
        };
        validate_iomedia_topology_identity_current_boot(&topology, expected)?;
        let mut retained = Vec::new();
        for (_, node) in topology.ordered() {
            retained.push(resolve(
                node.registry_entry_id.as_str(),
                &boot,
                node.bsd_name.as_str(),
            )?);
        }
        if retained
            .iter()
            .any(|object| object.identity().authority_granted)
        {
            return Err(invalid(
                "read-only IOMedia capture unexpectedly granted authority",
            ));
        }
        // These handles intentionally live only through the capture
        // consistency check. This schema grants no effect authority. A future
        // isolated effect runner must fresh-resolve all four IDs and retain
        // every handle through its callback and terminal observation.
        Ok(topology)
    }
}

#[cfg(target_os = "macos")]
pub use platform::ResolvedIOMediaObject;

#[cfg(not(target_os = "macos"))]
pub struct ResolvedIOMediaObject {
    _unsupported: (),
}

#[cfg(not(target_os = "macos"))]
impl ResolvedIOMediaObject {
    pub fn identity(&self) -> &IOMediaRegistryIdentityV1 {
        unreachable!("non-macOS resolver never constructs an IOMedia object")
    }
}

pub fn current_boot_session_uuid() -> Result<String, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::current_boot_session_uuid_impl()
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(invalid(
            "IOMedia boot-session identity is unsupported outside macOS",
        ))
    }
}

pub fn enumerate_iomedia_registry_identities()
-> Result<Vec<IOMediaRegistryIdentityV1>, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::enumerate()
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(invalid("IOMedia enumeration is unsupported outside macOS"))
    }
}

pub fn resolve_iomedia_registry_identity(
    registry_entry_id: &str,
    expected_boot_session_uuid: &str,
    expected_bsd_name: &str,
) -> Result<ResolvedIOMediaObject, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::resolve(
            registry_entry_id,
            expected_boot_session_uuid,
            expected_bsd_name,
        )
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (
            registry_entry_id,
            expected_boot_session_uuid,
            expected_bsd_name,
        );
        Err(invalid(
            "IOMedia registry-entry resolution is unsupported outside macOS",
        ))
    }
}

pub fn capture_attached_iomedia_topology(
    expected: ExpectedIOMediaTopology<'_>,
) -> Result<AttachedIOMediaTopologyV1, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::capture(expected)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = expected;
        Err(invalid(
            "attached IOMedia topology capture is unsupported outside macOS",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_identity(name: &str, id: u64) -> IOMediaRegistryIdentityV1 {
        registry_identity(name.to_string(), id)
    }

    fn test_topology(boot: &str) -> AttachedIOMediaTopologyV1 {
        AttachedIOMediaTopologyV1 {
            apfs_container: test_identity("disk10", 103),
            apfs_volume: test_identity("disk10s1", 104),
            authority_granted: false,
            boot_session_uuid: boot.to_string(),
            physical_store: test_identity("disk9s1", 102),
            physical_whole: test_identity("disk9", 101),
            schema: TOPOLOGY_SCHEMA.to_string(),
        }
    }

    fn expected() -> ExpectedIOMediaTopology<'static> {
        ExpectedIOMediaTopology {
            apfs_container: "disk10",
            apfs_volume: "disk10s1",
            physical_store: "disk9s1",
            physical_whole: "disk9",
        }
    }

    #[test]
    fn topology_rejects_zero_duplicate_wrong_boot_and_wrong_node_bindings() {
        let boot = "11111111-1111-4111-8111-111111111111";
        assert!(
            validate_iomedia_topology_identity_against_boot(&test_topology(boot), expected(), boot)
                .is_ok()
        );

        let mut zero = test_topology(boot);
        zero.physical_whole.registry_entry_id = "0000000000000000".to_string();
        assert!(validate_iomedia_topology_identity_against_boot(&zero, expected(), boot).is_err());

        let mut duplicate = test_topology(boot);
        duplicate.apfs_volume.registry_entry_id =
            duplicate.apfs_container.registry_entry_id.clone();
        assert!(
            validate_iomedia_topology_identity_against_boot(&duplicate, expected(), boot).is_err()
        );

        let wrong_boot = "22222222-2222-4222-8222-222222222222";
        assert!(
            validate_iomedia_topology_identity_against_boot(
                &test_topology(boot),
                expected(),
                wrong_boot,
            )
            .is_err()
        );

        let mut wrong_name = test_topology(boot);
        wrong_name.physical_store.bsd_name = "disk11s1".to_string();
        assert!(
            validate_iomedia_topology_identity_against_boot(&wrong_name, expected(), boot).is_err()
        );

        let mut wrong_role = test_topology(boot);
        std::mem::swap(&mut wrong_role.physical_store, &mut wrong_role.apfs_volume);
        assert!(
            validate_iomedia_topology_identity_against_boot(&wrong_role, expected(), boot).is_err()
        );
    }

    #[test]
    fn boot_uuid_and_bsd_name_shapes_are_closed_world() {
        assert!(!valid_uuid("00000000-0000-0000-0000-000000000000"));
        for valid in ["disk0", "disk1", "disk1s1", "disk12s34"] {
            assert!(valid_bsd_name(valid), "{valid}");
        }
        for invalid in [
            "disk1s",
            "disk01",
            "disk01s1",
            "disk1s00",
            "disk1s0",
            "disk1s1s2",
        ] {
            assert!(!valid_bsd_name(invalid), "{invalid}");
        }
    }

    #[test]
    fn registry_entry_ids_are_canonical_strings_not_json_numbers() {
        let boot = "11111111-1111-4111-8111-111111111111";
        let topology = test_topology(boot);
        let encoded = serde_json::to_string(&topology).expect("serialize topology identity");
        assert!(encoded.contains(r#""registry_entry_id":"0000000000000065""#));
        assert!(!encoded.contains(r#""registry_entry_id":101"#));

        let mut short = topology.clone();
        short.physical_whole.registry_entry_id = "65".to_string();
        assert!(validate_iomedia_topology_identity_shape(&short, expected()).is_err());
        let mut uppercase = topology;
        uppercase.physical_whole.registry_entry_id = "00000000000000AF".to_string();
        assert!(validate_iomedia_topology_identity_shape(&uppercase, expected()).is_err());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn rootless_live_iomedia_enumeration_and_registry_id_replay_is_authority_free() {
        let boot = current_boot_session_uuid().expect("current boot session UUID");
        let identities = enumerate_iomedia_registry_identities()
            .expect("read-only IOMedia enumeration must be available");
        assert!(identities.len() >= 4);
        let mut ids = BTreeSet::new();
        for identity in identities {
            assert!(!identity.authority_granted);
            assert!(ids.insert(identity.registry_entry_id.clone()));
            let object = resolve_iomedia_registry_identity(
                identity.registry_entry_id.as_str(),
                &boot,
                &identity.bsd_name,
            )
            .expect("registry ID must replay through DADiskRef");
            assert_eq!(object.identity(), &identity);
            assert!(!object.identity().authority_granted);
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn live_resolution_rejects_not_found_wrong_boot_and_reused_name_mismatch() {
        let boot = current_boot_session_uuid().expect("current boot session UUID");
        let identities = enumerate_iomedia_registry_identities()
            .expect("read-only IOMedia enumeration must be available");
        let first = identities.first().expect("at least one IOMedia object");

        assert!(
            resolve_iomedia_registry_identity("ffffffffffffffff", &boot, &first.bsd_name).is_err()
        );
        assert!(
            resolve_iomedia_registry_identity(
                first.registry_entry_id.as_str(),
                "22222222-2222-4222-8222-222222222222",
                &first.bsd_name,
            )
            .is_err()
        );
        let forged_name = if first.bsd_name == "disk999" {
            "disk998"
        } else {
            "disk999"
        };
        assert!(
            resolve_iomedia_registry_identity(
                first.registry_entry_id.as_str(),
                &boot,
                forged_name,
            )
            .is_err()
        );
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn non_macos_identity_operations_fail_closed_as_unsupported() {
        assert!(current_boot_session_uuid().is_err());
        assert!(enumerate_iomedia_registry_identities().is_err());
        assert!(resolve_iomedia_registry_identity("0000000000000001", "x", "disk1").is_err());
        assert!(capture_attached_iomedia_topology(expected()).is_err());
    }
}
