pub mod cinematic;
pub mod grain;
pub mod metaballs;
pub mod oil_painting;
pub mod ribbon;
pub mod warp;

use tiny_skia::Pixmap;

/// 高性能原地应用验证码抗 AI 滤镜
pub(crate) fn apply_all(pixmap: &mut Pixmap) {
    let w = pixmap.width() as usize;
    let h = pixmap.height() as usize;
    let data = pixmap.data_mut();

    let mut rng = fastrand::Rng::new();

    // 1. 底层纹理与光影 (Base Texture & Lighting)
    oil_painting::apply(data, w, h, &mut rng); // 印象派打底
    cinematic::apply(data, w, h, &mut rng); // 光影暗角

    // 3. 全局空间扭曲 (Global Spatial Distortion)
    warp::apply(data, w, h, &mut rng); // CRT 扭曲

    // 4. 遮挡与折射 (Occlusion & Refraction)
    ribbon::apply(data, w, h, &mut rng); // 棱镜折射带 (折射已经被扭曲的底层)
    metaballs::apply(data, w, h, &mut rng); // 流体玻璃

    // 5. 表面肌理与抗 AI 噪点 (Surface Texture & Anti-AI Grain)
    // 作为最后一步叠加，统一画面质感，不被拉伸变形，保持真实的物理纸张/胶片感
    grain::apply(data, w, h, &mut rng);
}
