use crate::PreallocatedArray;
use core::ops::{Deref, DerefMut};
use generic_array;

pub struct GenericArrayAdapter<T, S>
where
    T: Default + Copy,
    S: generic_array::ArrayLength<T>,
{
    data: generic_array::GenericArray<T, S>,
    pd: core::marker::PhantomData<S>,
}

impl<T, S> PreallocatedArray<T> for GenericArrayAdapter<T, S>
where
    T: Default + Copy,
    S: generic_array::ArrayLength<T>,
{
}

impl<T, S> Default for GenericArrayAdapter<T, S>
where
    T: Default + Copy,
    S: generic_array::ArrayLength<T>,
{
    fn default() -> Self {
        Self {
            data: generic_array::GenericArray::<T, S>::default(),
            pd: core::marker::PhantomData::<S> {},
        }
    }
}

impl<T, S> Deref for GenericArrayAdapter<T, S>
where
    T: Default + Copy,
    S: generic_array::ArrayLength<T>,
{
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, S> DerefMut for GenericArrayAdapter<T, S>
where
    T: Default + Copy,
    S: generic_array::ArrayLength<T>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
