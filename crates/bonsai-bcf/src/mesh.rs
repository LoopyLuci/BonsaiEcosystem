use crate::Result;

pub struct ServiceMesh;

impl ServiceMesh {
    pub fn new() -> Self {
        Self
    }

    pub async fn register_container(
        &self,
        _name: &str,
        _vault: &crate::vault::ContainerVault,
    ) -> Result<()> {
        Ok(())
    }
}
