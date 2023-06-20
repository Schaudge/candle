use crate::{
    storage::{CpuStorage, Storage},
    DType, Result, Shape,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Device {
    Cpu,
}

// TODO: Should we back the cpu implementation using the NdArray crate or similar?
pub trait NdArray {
    fn shape(&self) -> Result<Shape>;

    fn to_cpu_storage(&self) -> CpuStorage;
}

impl<S: crate::WithDType> NdArray for S {
    fn shape(&self) -> Result<Shape> {
        Ok(Shape::from(()))
    }

    fn to_cpu_storage(&self) -> CpuStorage {
        S::to_cpu_storage(&[*self])
    }
}

impl<S: crate::WithDType> NdArray for &[S] {
    fn shape(&self) -> Result<Shape> {
        Ok(Shape::from(self.len()))
    }

    fn to_cpu_storage(&self) -> CpuStorage {
        S::to_cpu_storage(self)
    }
}

impl Device {
    pub(crate) fn zeros(&self, shape: &Shape, dtype: DType) -> Storage {
        match self {
            Device::Cpu => {
                let elem_count = shape.elem_count();
                let storage = match dtype {
                    DType::F32 => {
                        let data = vec![0f32; elem_count];
                        CpuStorage::F32(data)
                    }
                    DType::F64 => {
                        let data = vec![0f64; elem_count];
                        CpuStorage::F64(data)
                    }
                };
                Storage::Cpu(storage)
            }
        }
    }

    pub(crate) fn tensor<A: NdArray>(&self, array: A) -> Storage {
        match self {
            Device::Cpu => Storage::Cpu(array.to_cpu_storage()),
        }
    }
}