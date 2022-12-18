//!
//! Contain geometry asset definitions.
//!

mod point_cloud;
pub use point_cloud::*;

mod tri_mesh;
pub use tri_mesh::*;

pub use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Geometry {
    Points(PointCloud),
    Triangles(TriMesh),
}

///
/// An array of indices. Supports different data types.
///
#[derive(Clone, Debug)]
pub enum Indices {
    /// Do not use indices, ie. the faces are all unconnected.
    None,
    /// Uses unsigned 8 bit integer for each index.
    U8(Vec<u8>),
    /// Uses unsigned 16 bit integer for each index.
    U16(Vec<u16>),
    /// Uses unsigned 32 bit integer for each index.
    U32(Vec<u32>),
}

impl Indices {
    ///
    /// Converts all the indices as `u32` data type.
    ///
    pub fn into_u32(self) -> Option<Vec<u32>> {
        match self {
            Self::None => None,
            Self::U8(mut values) => Some(values.drain(..).map(|i| i as u32).collect::<Vec<_>>()),
            Self::U16(mut values) => Some(values.drain(..).map(|i| i as u32).collect::<Vec<_>>()),
            Self::U32(values) => Some(values),
        }
    }

    ///
    /// Clones and converts all the indices as `u32` data type.
    ///
    pub fn to_u32(&self) -> Option<Vec<u32>> {
        match self {
            Self::None => None,
            Self::U8(values) => Some(values.iter().map(|i| *i as u32).collect::<Vec<_>>()),
            Self::U16(values) => Some(values.iter().map(|i| *i as u32).collect::<Vec<_>>()),
            Self::U32(values) => Some(values.clone()),
        }
    }

    ///
    /// Returns the number of indices.
    ///
    pub fn len(&self) -> Option<usize> {
        match self {
            Self::None => None,
            Self::U8(values) => Some(values.len()),
            Self::U16(values) => Some(values.len()),
            Self::U32(values) => Some(values.len()),
        }
    }
}

impl std::default::Default for Indices {
    fn default() -> Self {
        Self::None
    }
}

///
/// An array of positions. Supports f32 and f64 data types.
///
#[derive(Clone)]
pub enum Positions {
    /// Uses 32 bit float for the vertex positions.
    F32(Vec<Vec3>),
    /// Uses 64 bit float for the vertex positions.
    F64(Vec<Vector3<f64>>),
}

impl Positions {
    ///
    /// Converts and returns all the positions as `f32` data type.
    ///
    pub fn into_f32(self) -> Vec<Vec3> {
        match self {
            Self::F32(values) => values,
            Self::F64(mut values) => values
                .drain(..)
                .map(|v| Vec3::new(v.x as f32, v.y as f32, v.z as f32))
                .collect::<Vec<_>>(),
        }
    }

    ///
    /// Clones and converts all the positions as `f32` data type.
    ///
    pub fn to_f32(&self) -> Vec<Vec3> {
        match self {
            Self::F32(values) => values.clone(),
            Self::F64(values) => values
                .iter()
                .map(|v| Vec3::new(v.x as f32, v.y as f32, v.z as f32))
                .collect::<Vec<_>>(),
        }
    }
    ///
    /// Converts and returns all the positions as `f64` data type.
    ///
    pub fn into_f64(self) -> Vec<Vector3<f64>> {
        match self {
            Self::F32(mut values) => values
                .drain(..)
                .map(|v| Vector3::new(v.x as f64, v.y as f64, v.z as f64))
                .collect::<Vec<_>>(),
            Self::F64(values) => values,
        }
    }

    ///
    /// Clones and converts all the positions as `f64` data type.
    ///
    pub fn to_f64(&self) -> Vec<Vector3<f64>> {
        match self {
            Self::F32(values) => values
                .iter()
                .map(|v| Vector3::new(v.x as f64, v.y as f64, v.z as f64))
                .collect::<Vec<_>>(),
            Self::F64(values) => values.clone(),
        }
    }

    ///
    /// Returns the number of positions.
    ///
    pub fn len(&self) -> usize {
        match self {
            Self::F32(values) => values.len(),
            Self::F64(values) => values.len(),
        }
    }
}

impl std::default::Default for Positions {
    fn default() -> Self {
        Self::F32(Vec::new())
    }
}

impl std::fmt::Debug for Positions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Positions");
        match self {
            Self::F32(ind) => d.field("f32", &ind.len()),
            Self::F64(ind) => d.field("f64", &ind.len()),
        };
        d.finish()
    }
}
