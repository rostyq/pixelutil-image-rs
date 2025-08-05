use crate::index::ImageAxisIndex;

/// Trait for types that can represent image coordinates
pub trait ImageCoordinate {
    /// Return the `(x, y)` pixel indices, or [`None`] if the coordinate is invalid.
    fn image_coordinate(&self) -> Option<(u32, u32)>;

    /// Return clamped `(x, y)` pixel indices within the given bounds.
    /// Bounds are `(0, 0)` and `(right, bottom)`.
    fn image_coordinate_clamped(&self, right: u32, bottom: u32) -> (u32, u32);
}

impl<T: ImageAxisIndex + Copy> ImageCoordinate for (T, T) {
    #[inline]
    fn image_coordinate(&self) -> Option<(u32, u32)> {
        self.0
            .to_image_axis_index()
            .zip(self.1.to_image_axis_index())
    }

    #[inline]
    fn image_coordinate_clamped(&self, right: u32, bottom: u32) -> (u32, u32) {
        (
            self.0.clamp_image_axis_index(right),
            self.1.clamp_image_axis_index(bottom),
        )
    }
}

impl<T: ImageAxisIndex + Copy> ImageCoordinate for [T; 2] {
    #[inline]
    fn image_coordinate(&self) -> Option<(u32, u32)> {
        unsafe { self.get_unchecked(0) }
            .to_image_axis_index()
            .zip(unsafe { self.get_unchecked(1) }.to_image_axis_index())
    }

    #[inline]
    fn image_coordinate_clamped(&self, right: u32, bottom: u32) -> (u32, u32) {
        (
            unsafe { self.get_unchecked(0) }.clamp_image_axis_index(right),
            unsafe { self.get_unchecked(1) }.clamp_image_axis_index(bottom),
        )
    }
}

impl<T: ImageAxisIndex + Clone> ImageCoordinate for &[T; 2] {
    #[inline]
    fn image_coordinate(&self) -> Option<(u32, u32)> {
        unsafe { self.get_unchecked(0) }
            .clone()
            .to_image_axis_index()
            .zip(
                unsafe { self.get_unchecked(1) }
                    .clone()
                    .to_image_axis_index(),
            )
    }

    #[inline]
    fn image_coordinate_clamped(&self, right: u32, bottom: u32) -> (u32, u32) {
        (
            unsafe { self.get_unchecked(0) }
                .clone()
                .clamp_image_axis_index(right),
            unsafe { self.get_unchecked(1) }
                .clone()
                .clamp_image_axis_index(bottom),
        )
    }
}

#[cfg(feature = "nalgebra")]
impl<T: ImageAxisIndex + nalgebra::Scalar> ImageCoordinate for &nalgebra::Point2<T> {
    #[inline]
    fn image_coordinate(&self) -> Option<(u32, u32)> {
        self.x
            .clone()
            .to_image_axis_index()
            .zip(self.y.clone().to_image_axis_index())
    }

    #[inline]
    fn image_coordinate_clamped(&self, right: u32, bottom: u32) -> (u32, u32) {
        (
            self.x.clone().clamp_image_axis_index(right),
            self.y.clone().clamp_image_axis_index(bottom),
        )
    }
}

#[cfg(feature = "nalgebra")]
impl<T: ImageAxisIndex + nalgebra::Scalar + Copy> ImageCoordinate for nalgebra::Point2<T> {
    #[inline]
    fn image_coordinate(&self) -> Option<(u32, u32)> {
        self.x
            .to_image_axis_index()
            .zip(self.y.clone().to_image_axis_index())
    }

    #[inline]
    fn image_coordinate_clamped(&self, right: u32, bottom: u32) -> (u32, u32) {
        (
            self.x.clamp_image_axis_index(right),
            self.y.clamp_image_axis_index(bottom),
        )
    }
}
