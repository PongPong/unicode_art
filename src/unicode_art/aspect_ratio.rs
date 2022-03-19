pub trait AspectRatio {
    fn calculate(&self, img_width: u32, img_height: u32) -> (u32, u32);
}

#[derive(Debug)]
pub enum TermFit {
    Auto,
    // Fit,
    // Width,
    // Height,
}

impl Default for TermFit {
    fn default() -> Self {
        TermFit::Auto
    }
}

#[derive(Debug, Default)]
pub struct SimpleAspectRatio {
    expect_width: Option<u32>,
    expect_height: Option<u32>,
    termfit: TermFit,
    use_border: bool,
}

impl SimpleAspectRatio {
    // the 2.0f and 0.5f factors are used for text displays that (usually) have characters
    // that are taller than they are wide.
    #[inline]
    fn calc_width(&self, height: u32, img_width: u32, img_height: u32) -> u32 {
        (2.0 * height as f64 * img_width as f64 / img_height as f64).round() as u32
    }

    #[inline]
    fn calc_height(&self, width: u32, img_width: u32, img_height: u32) -> u32 {
        (0.5 * width as f64 * img_height as f64 / img_width as f64).round() as u32
    }

    #[inline]
    fn auto_height(&self, width: u32, img_width: u32, img_height: u32) -> (u32, u32) {
        let mut height = self.calc_height(width, img_width, img_height);
        let mut width = width;

        while height == 0 {
            width += 1;
            height = self.calc_height(width, img_width, img_height);
        }
        (width, height)
    }

    #[inline]
    fn auto_width(&self, height: u32, img_width: u32, img_height: u32) -> (u32, u32) {
        let mut width = self.calc_width(height, img_width, img_height);
        // adjust for too small dimensions
        let mut height = height;
        while width == 0 {
            height = height + 1;
            width = self.calc_height(height, img_width, img_height);
        }

        let term_width = 80;
        while matches!(self.termfit, TermFit::Auto)
            && (width + self.use_border as u32 * 2) > term_width
        {
            width = term_width - self.use_border as u32 * 2;
            (width, height) = self.auto_height(width, img_width, img_height);
        }
        (width, height)
    }

    pub fn new_auto_width(expect_height: u32, termfit: TermFit, use_border: bool) -> Self {
        Self {
            expect_width: None,
            expect_height: Some(expect_height),
            termfit,
            use_border,
        }
    }

    pub fn new_auto_height(expect_width: u32, termfit: TermFit, use_border: bool) -> Self {
        Self {
            expect_width: Some(expect_width),
            expect_height: None,
            termfit,
            use_border,
        }
    }

    // pub fn new(
    //     expect_width: Option<u32>,
    //     expect_height: Option<u32>,
    //     termfit: TermFit,
    //     use_border: bool,
    // ) -> Self {
    //     Self {
    //         expect_width,
    //         expect_height,
    //         termfit,
    //         use_border,
    //     }
    // }
}

impl AspectRatio for SimpleAspectRatio {
    fn calculate(&self, img_width: u32, img_height: u32) -> (u32, u32) {
        match (self.expect_width, self.expect_height) {
            // auto width
            (None, Some(height)) => self.auto_width(height, img_width, img_height),
            // auto height
            (Some(width), None) => self.auto_height(width, img_width, img_height),
            _ => (img_width, img_height),
        }
    }
}
