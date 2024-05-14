use image::GenericImageView;

/// Returns `true` if the given coordinates are within the bounds of the image.
#[inline]
pub fn in_bounds<I: GenericImageView>(image: &I, x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < image.width() as i32 && y < image.height() as i32
}

/// Returns the pixel at the given coordinates if it is within the bounds of the image.
#[inline]
pub fn get_pixel<I: GenericImageView>(image: &I, x: i32, y: i32) -> Option<I::Pixel> {
    in_bounds(image, x, y).then(|| unsafe { image.unsafe_get_pixel(x as u32, y as u32) })
}

/// Returns the pixel at the given coordinates, clamping the coordinates to the image bounds.
#[inline]
pub fn clamp_pixel<I: GenericImageView>(image: &I, x: i32, y: i32) -> I::Pixel {
    unsafe {
        image.unsafe_get_pixel(
            x.clamp(0, image.width() as i32 - 1) as u32,
            y.clamp(0, image.height() as i32 - 1) as u32,
        )
    }
}

/// Returns the pixel at the given coordinates, without checking for empty image.
#[inline]
pub unsafe fn clamp_pixel_unchecked<I: GenericImageView>(image: &I, x: i32, y: i32) -> I::Pixel {
    image.unsafe_get_pixel(
        (x.max(0) as u32).min(image.width() - 1),
        (y.max(0) as u32).min(image.height() - 1),
    )
}

#[cfg(test)]
mod tests {
    use image::GrayImage;

    use super::*;

    #[test]
    fn in_bounds_for_empty_image() {
        let image = GrayImage::new(0, 0);
        for (x, y) in [(0, 0), (-1, -1), (1, 1), (1, 0), (0, 1), (-1, 0), (0, -1)] {
            assert!(!in_bounds(&image, x, y));
        }
    }

    #[test]
    fn in_bounds_for_non_empty_image() {
        let image = GrayImage::new(1, 1);

        assert!(in_bounds(&image, 0, 0));
        for (x, y) in [(-1, -1), (1, 1), (1, 0), (0, 1), (-1, 0), (0, -1)] {
            assert!(!in_bounds(&image, x, y));
        }
    }

    #[test]
    fn lookup_pixel_for_empty_image() {
        let image = GrayImage::new(0, 0);
        for (x, y) in [(0, 0), (-1, -1), (1, 1), (1, 0), (0, 1), (-1, 0), (0, -1)] {
            assert!(get_pixel(&image, x, y).is_none());
        }
    }

    #[test]
    fn lookup_pixel_for_non_empty_image() {
        let image = GrayImage::from_pixel(1, 1, [255].into());

        assert!(get_pixel(&image, -1, -1).is_none());
        assert!(get_pixel(&image, 1, 1).is_none());
        assert!(get_pixel(&image, 0, 0).is_some());
        assert_eq!(
            get_pixel(&image, 0, 0),
            image.get_pixel_checked(0, 0).copied()
        );
    }

    #[test]
    #[should_panic]
    fn clamp_pixel_for_empty_image() {
        let image = GrayImage::new(0, 0);
        clamp_pixel(&image, 0, 0);
    }

    #[test]
    fn clamp_pixel_for_non_empty_image() {
        let image = GrayImage::from_vec(2, 2, vec![32, 64, 128, 255]).unwrap();
        let (w, h) = (image.width() as i32, image.height() as i32);
        let (b, r) = (h - 1, w - 1);

        // near top-left corner
        assert_eq!(&clamp_pixel(&image, -1, -1), image.get_pixel(0, 0));
        assert_eq!(&clamp_pixel(&image, 0, -1), image.get_pixel(0, 0));
        assert_eq!(&clamp_pixel(&image, -1, 0), image.get_pixel(0, 0));

        // near bottom-right corner
        assert_eq!(&clamp_pixel(&image, w, b), image.get_pixel(1, 1));
        assert_eq!(&clamp_pixel(&image, w, b), image.get_pixel(1, 1));
        assert_eq!(&clamp_pixel(&image, r, h), image.get_pixel(1, 1));

        // near top-right corner
        assert_eq!(&clamp_pixel(&image, w, 0), image.get_pixel(1, 0));
        assert_eq!(&clamp_pixel(&image, r, -1), image.get_pixel(1, 0));
        assert_eq!(&clamp_pixel(&image, w, -1), image.get_pixel(1, 0));

        // near bottom-left corner
        assert_eq!(&clamp_pixel(&image, -1, b), image.get_pixel(0, 1));
        assert_eq!(&clamp_pixel(&image, -1, h), image.get_pixel(0, 1));
        assert_eq!(&clamp_pixel(&image, 0, h), image.get_pixel(0, 1));

        // corners of the image
        assert_eq!(&clamp_pixel(&image, 0, 0), image.get_pixel(0, 0));
        assert_eq!(&clamp_pixel(&image, r, 0), image.get_pixel(1, 0));
        assert_eq!(&clamp_pixel(&image, 0, b), image.get_pixel(0, 1));
        assert_eq!(&clamp_pixel(&image, r, b), image.get_pixel(1, 1));
    }

    #[test]
    fn clamp_pixel_for_non_empty_image_unsafe() {
        let image = GrayImage::from_vec(2, 2, vec![32, 64, 128, 255]).unwrap();
        let (w, h) = (image.width() as i32, image.height() as i32);
        let (b, r) = (h - 1, w - 1);

        unsafe {
            // near top-left corner
            assert_eq!(
                &clamp_pixel_unchecked(&image, -1, -1),
                image.get_pixel(0, 0)
            );
            assert_eq!(&clamp_pixel_unchecked(&image, 0, -1), image.get_pixel(0, 0));
            assert_eq!(&clamp_pixel_unchecked(&image, -1, 0), image.get_pixel(0, 0));

            // near bottom-right corner
            assert_eq!(&clamp_pixel_unchecked(&image, w, b), image.get_pixel(1, 1));
            assert_eq!(&clamp_pixel_unchecked(&image, w, b), image.get_pixel(1, 1));
            assert_eq!(&clamp_pixel_unchecked(&image, r, h), image.get_pixel(1, 1));

            // near top-right corner
            assert_eq!(&clamp_pixel_unchecked(&image, w, 0), image.get_pixel(1, 0));
            assert_eq!(&clamp_pixel_unchecked(&image, r, -1), image.get_pixel(1, 0));
            assert_eq!(&clamp_pixel_unchecked(&image, w, -1), image.get_pixel(1, 0));

            // near bottom-left corner
            assert_eq!(&clamp_pixel_unchecked(&image, -1, b), image.get_pixel(0, 1));
            assert_eq!(&clamp_pixel_unchecked(&image, -1, h), image.get_pixel(0, 1));
            assert_eq!(&clamp_pixel_unchecked(&image, 0, h), image.get_pixel(0, 1));

            // corners of the image
            assert_eq!(&clamp_pixel_unchecked(&image, 0, 0), image.get_pixel(0, 0));
            assert_eq!(&clamp_pixel_unchecked(&image, r, 0), image.get_pixel(1, 0));
            assert_eq!(&clamp_pixel_unchecked(&image, 0, b), image.get_pixel(0, 1));
            assert_eq!(&clamp_pixel_unchecked(&image, r, b), image.get_pixel(1, 1));
        }
    }
}
