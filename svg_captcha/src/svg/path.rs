use std::fmt::Write;

pub fn ctrl_points<F: Fn(usize) -> f32>(k: F, len: usize) -> (Vec<f32>, Vec<f32>) {
    let n = len - 1;
    let mut b = vec![4.0; n];
    b[0] = 2.0;
    b[n - 1] = 7.0;

    let mut r = vec![0.0; n];
    r[0] = k(0) + 2.0 * k(1);
    for (i, r_val) in r[1..n - 1].iter_mut().enumerate() {
        let idx = i + 1;
        *r_val = 4.0 * k(idx) + 2.0 * k(idx + 1);
    }
    r[n - 1] = 8.0 * k(n - 1) + k(n);

    for i in 1..n {
        let a_i = if i == n - 1 { 2.0 } else { 1.0 };
        let m = a_i / b[i - 1];
        b[i] -= m;
        r[i] -= m * r[i - 1];
    }

    let mut p1 = vec![0.0; n];
    let mut p2 = vec![0.0; n];

    p1[n - 1] = r[n - 1] / b[n - 1];
    for i in (0..n - 1).rev() {
        p1[i] = (r[i] - p1[i + 1]) / b[i];
    }

    for i in 0..n - 1 {
        p2[i] = 2.0 * k(i + 1) - p1[i + 1];
    }
    p2[n - 1] = 0.5 * (k(n) + p1[n - 1]);

    (p1, p2)
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn wave_path(points: &[Point], w: u32, h: u32) -> String {
    let len = points.len();
    let (p1_x, p2_x) = ctrl_points(|i| points[i].x, len);
    let (p1_y, p2_y) = ctrl_points(|i| points[i].y, len);

    let mut d = String::with_capacity(len * 64);
    let _ = write!(
        d,
        "M 0,{h} C 0,{h} {x0},{y0} {x0},{y0}",
        x0 = points[0].x,
        y0 = points[0].y
    );
    for i in 0..len - 1 {
        let _ = write!(
            d,
            " C {:.1},{:.1} {:.1},{:.1} {},{}",
            p1_x[i],
            p1_y[i],
            p2_x[i],
            p2_y[i],
            points[i + 1].x,
            points[i + 1].y
        );
    }
    // SAFETY: points always has at least 2 elements
    // 安全: points 至少有 2 个元素
    let last = unsafe { points.last().unwrap_unchecked() };
    let _ = write!(d, " C {},{} {w},{h} {w},{h} Z", last.x, last.y);
    d
}
