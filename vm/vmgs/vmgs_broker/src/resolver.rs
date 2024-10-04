// Copyright (C) Microsoft Corporation. All rights reserved.

//! Resource resolver for VMGS files.

use crate::non_volatile_store::EncryptionNotSupported;
use crate::non_volatile_store::VmgsNonVolatileStore;
use crate::VmgsClient;
use vm_resource::kind::NonVolatileStoreKind;
use vm_resource::ResolveResource;
use vmcore::non_volatile_store::resources::ResolvedNonVolatileStore;
use vmgs_resources::VmgsFileHandle;

/// A resource resolver for VMGS files.
pub struct VmgsFileResolver {
    client: VmgsClient,
}

impl VmgsFileResolver {
    /// Create a new resolver.
    pub fn new(client: VmgsClient) -> Self {
        Self { client }
    }
}

impl ResolveResource<NonVolatileStoreKind, VmgsFileHandle> for VmgsFileResolver {
    type Output = ResolvedNonVolatileStore;
    type Error = EncryptionNotSupported;

    fn resolve(&self, resource: VmgsFileHandle, (): ()) -> Result<Self::Output, Self::Error> {
        Ok(VmgsNonVolatileStore::new(
            self.client.clone(),
            vmgs_format::FileId(resource.file_id),
            resource.encrypted,
        )?
        .into())
    }
}