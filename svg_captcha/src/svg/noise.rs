use std::f32::consts::TAU;

/// A simple 2D noise implementation using a grid of random gradients.
///
/// 一个基于随机梯度网格的简单 2D 噪声实现。
pub(crate) struct Noise2d {
    grid: Vec<f32>,
    w: usize,
    h: usize,
}

impl Noise2d {
    pub fn new(w: usize, h: usize) -> Self {
        let mut grid = Vec::with_capacity(w * h);
        for _ in 0..w * h {
            grid.push(fastrand::f32() * TAU);
        }
        Self { grid, w, h }
    }

    /// Gets the interpolated angle at (x, y) where x, y are in [0, 1].
    pub fn get(&self, x: f32, y: f32) -> f32 {
        let gx = x * (self.w - 1) as f32;
        let gy = y * (self.h - 1) as f32;

        let x0 = gx as usize;
        let y0 = gy as usize;
        let x1 = (x0 + 1).min(self.w - 1);
        let y1 = (y0 + 1).min(self.h - 1);

        let tx = gx - x0 as f32;
        let ty = gy - y0 as f32;

        let v00 = self.grid[y0 * self.w + x0];
        let v10 = self.grid[y0 * self.w + x1];
        let v01 = self.grid[y1 * self.w + x0];
        let v11 = self.grid[y1 * self.w + x1];

        // Bi-linear interpolation of angles (simplified)
        let top = v00 + tx * (v10 - v00);
        let bottom = v01 + tx * (v11 - v01);
        top + ty * (bottom - top)
    }
}
