use crate::Uuid16;
use crate::rac_wire::encode_with_len_u8;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct ClusterIdRequest {
    pub cluster: Uuid16,
}

impl ClusterIdRequest {
    pub fn encoded_len(&self) -> usize {
        16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ClusterAuthRequest {
    pub cluster: Uuid16,
    pub user: String,
    pub pwd: String,
}

impl ClusterAuthRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.user.len() + 1 + self.pwd.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.user.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ClusterAdminRegisterRequest {
    pub cluster: Uuid16,
    pub name: String,
    pub descr: String,
    pub pwd: String,
    pub auth_flags: u8,
}

impl ClusterAdminRegisterRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.name.len() + 1 + self.descr.len() + 1 + self.pwd.len() + 1 + 2
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.descr.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        out.push(self.auth_flags);
        out.extend_from_slice(&[0, 0]);
        Ok(())
    }
}
