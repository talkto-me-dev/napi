import { readFileSync, writeFileSync } from "node:fs";
import { optimize } from "svgo";

const parse = (d) => {
    const commands = d.match(/[a-df-z][^a-df-z]*/gi) || [],
      pts = [];
    let curr_x = 0,
      curr_y = 0,
      start_x = 0,
      start_y = 0;

    commands.forEach((cmd) => {
      const type = cmd[0],
        args =
          cmd
            .slice(1)
            .match(/-?\d*\.?\d+(?:e[+-]?\d+)?/gi)
            ?.map(Number) || [];

      switch (type) {
        case "M":
          curr_x = args[0];
          curr_y = args[1];
          start_x = curr_x;
          start_y = curr_y;
          pts.push([curr_x, curr_y]);
          for (let i = 2; i < args.length; i += 2) {
            curr_x = args[i];
            curr_y = args[i + 1];
            pts.push([curr_x, curr_y]);
          }
          break;
        case "m":
          if (pts.length === 0) {
            curr_x = args[0];
            curr_y = args[1];
          } else {
            curr_x += args[0];
            curr_y += args[1];
          }
          start_x = curr_x;
          start_y = curr_y;
          pts.push([curr_x, curr_y]);
          for (let i = 2; i < args.length; i += 2) {
            curr_x += args[i];
            curr_y += args[i + 1];
            pts.push([curr_x, curr_y]);
          }
          break;
        case "L":
          for (let i = 0; i < args.length; i += 2) {
            curr_x = args[i];
            curr_y = args[i + 1];
            pts.push([curr_x, curr_y]);
          }
          break;
        case "l":
          for (let i = 0; i < args.length; i += 2) {
            curr_x += args[i];
            curr_y += args[i + 1];
            pts.push([curr_x, curr_y]);
          }
          break;
        case "H":
          args.forEach((x) => {
            curr_x = x;
            pts.push([curr_x, curr_y]);
          });
          break;
        case "h":
          args.forEach((dx) => {
            curr_x += dx;
            pts.push([curr_x, curr_y]);
          });
          break;
        case "V":
          args.forEach((y) => {
            curr_y = y;
            pts.push([curr_x, curr_y]);
          });
          break;
        case "v":
          args.forEach((dy) => {
            curr_y += dy;
            pts.push([curr_x, curr_y]);
          });
          break;
        case "C":
          for (let i = 0; i < args.length; i += 6) {
            pts.push([args[i], args[i + 1]], [args[i + 2], args[i + 3]]);
            curr_x = args[i + 4];
            curr_y = args[i + 5];
            pts.push([curr_x, curr_y]);
          }
          break;
        case "c":
          for (let i = 0; i < args.length; i += 6) {
            pts.push(
              [curr_x + args[i], curr_y + args[i + 1]],
              [curr_x + args[i + 2], curr_y + args[i + 3]],
            );
            curr_x += args[i + 4];
            curr_y += args[i + 5];
            pts.push([curr_x, curr_y]);
          }
          break;
        case "S":
        case "Q":
          for (let i = 0; i < args.length; i += 4) {
            pts.push([args[i], args[i + 1]]);
            curr_x = args[i + 2];
            curr_y = args[i + 3];
            pts.push([curr_x, curr_y]);
          }
          break;
        case "s":
        case "q":
          for (let i = 0; i < args.length; i += 4) {
            pts.push([curr_x + args[i], curr_y + args[i + 1]]);
            curr_x += args[i + 2];
            curr_y += args[i + 3];
            pts.push([curr_x, curr_y]);
          }
          break;
        case "A":
        case "a":
          for (let i = 0; i < args.length; i += 7) {
            let rx = Math.abs(args[i]),
              ry = Math.abs(args[i + 1]),
              phi_deg = args[i + 2],
              f_a = args[i + 3],
              f_s = args[i + 4],
              next_x = type === "A" ? args[i + 5] : curr_x + args[i + 5],
              next_y = type === "A" ? args[i + 6] : curr_y + args[i + 6];

            if (rx === 0 || ry === 0) {
              curr_x = next_x;
              curr_y = next_y;
              pts.push([curr_x, curr_y]);
              continue;
            }

            let phi = (phi_deg * Math.PI) / 180,
              cos_phi = Math.cos(phi),
              sin_phi = Math.sin(phi),
              dx_2 = (curr_x - next_x) / 2,
              dy_2 = (curr_y - next_y) / 2,
              x_1_p = cos_phi * dx_2 + sin_phi * dy_2,
              y_1_p = -sin_phi * dx_2 + cos_phi * dy_2,
              lambda = (x_1_p * x_1_p) / (rx * rx) + (y_1_p * y_1_p) / (ry * ry);

            if (lambda > 1) {
              let factor = Math.sqrt(lambda);
              rx *= factor;
              ry *= factor;
            }

            let num = rx * rx * ry * ry - rx * rx * y_1_p * y_1_p - ry * ry * x_1_p * x_1_p,
              den = rx * rx * y_1_p * y_1_p + ry * ry * x_1_p * x_1_p,
              c = Math.sqrt(Math.max(0, num / den));
            if (f_a === f_s) c = -c;

            let cx_p = c * ((rx * y_1_p) / ry),
              cy_p = c * (-(ry * x_1_p) / rx),
              cx = cos_phi * cx_p - sin_phi * cy_p + (curr_x + next_x) / 2,
              cy = sin_phi * cx_p + cos_phi * cy_p + (curr_y + next_y) / 2,
              delta_x = Math.sqrt((rx * cos_phi) ** 2 + (ry * sin_phi) ** 2),
              delta_y = Math.sqrt((rx * sin_phi) ** 2 + (ry * cos_phi) ** 2);

            pts.push(
              [cx - delta_x, cy - delta_y],
              [cx + delta_x, cy + delta_y],
              [next_x, next_y],
            );

            curr_x = next_x;
            curr_y = next_y;
          }
          break;
        case "Z":
        case "z":
          curr_x = start_x;
          curr_y = start_y;
          break;
      }
    });
    return pts;
  },
  PADDING = 16;

export const resize = (path) => {
  let data = readFileSync(path, "utf8"),
    clean_data = data;

  while (clean_data.includes('<g transform="translate')) {
    const next = clean_data.replace(
      /<g transform="translate\([^)]+\)\s*scale\([^)]+\)">\s*([\s\S]*?)\s*<\/g>/g,
      "$1",
    );
    if (next === clean_data) break;
    clean_data = next;
  }

  const { data: optimized } = optimize(clean_data, {
    path,
    multipass: true,
    plugins: [
      {
        name: "preset-default",
        params: {
          overrides: {
            convertPathData: { applyTransforms: true, makeAbsolute: true },
            convertTransform: true,
            moveGroupAttrsToElems: true,
          },
        },
      },
      "convertShapeToPath",
      "removeDimensions",
    ],
  });

  let all_pts = [],
    max_stroke = 0;

  const stroke_widths = optimized.match(/stroke-width="([^"]+)"/g);
  if (stroke_widths) {
    for (let i = 0, len = stroke_widths.length; i < len; ++i) {
      const val = Number(stroke_widths[i].match(/stroke-width="([^"]+)"/)[1]);
      if (!isNaN(val) && val > max_stroke) max_stroke = val;
    }
  }

  const d_matches = optimized.match(/d="([^"]+)"/g);
  if (d_matches) {
    for (let i = 0, len = d_matches.length; i < len; ++i) {
      const d = d_matches[i].match(/d="([^"]+)"/)[1];
      all_pts.push(...parse(d));
    }
  }

  const circle_matches = optimized.match(/<circle[^>]+>/g);
  if (circle_matches) {
    for (let i = 0, len = circle_matches.length; i < len; ++i) {
      const m = circle_matches[i],
        cx = Number(m.match(/cx="([^"]+)"/)?.[1] || 0),
        cy = Number(m.match(/cy="([^"]+)"/)?.[1] || 0),
        r = Number(m.match(/r="([^"]+)"/)?.[1] || 0);
      all_pts.push([cx - r, cy - r], [cx + r, cy + r]);
    }
  }

  if (all_pts.length > 0) {
    let min_x = Infinity,
      max_x = -Infinity,
      min_y = Infinity,
      max_y = -Infinity;

    for (let i = 0, len = all_pts.length; i < len; ++i) {
      const p = all_pts[i],
        x = p[0],
        y = p[1];
      if (x < min_x) min_x = x;
      if (x > max_x) max_x = x;
      if (y < min_y) min_y = y;
      if (y > max_y) max_y = y;
    }

    min_x -= max_stroke / 2;
    max_x += max_stroke / 2;
    min_y -= max_stroke / 2;
    max_y += max_stroke / 2;

    const width = max_x - min_x,
      height = max_y - min_y;

    if (width > 0 && height > 0) {
      const target_size = 1024 - PADDING * 2,
        scale = target_size / Math.max(width, height),
        tx = (1024 - width * scale) / 2 - min_x * scale,
        ty = (1024 - height * scale) / 2 - min_y * scale,
        inner_content = optimized.replace(/<svg[^>]*>([\s\S]*?)<\/svg>/, "$1").trim(),
        new_svg =
          '<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024">\n' +
          '  <g transform="translate(' +
          tx.toFixed(3) +
          ", " +
          ty.toFixed(3) +
          ") scale(" +
          scale.toFixed(3) +
          ')">\n' +
          "    " +
          inner_content +
          "\n" +
          "  </g>\n" +
          "</svg>";
      writeFileSync(path, new_svg);
      return true;
    }
  } else {
    const new_svg = optimized.replace(
      /<svg([^>]*)>/,
      '<svg$1 width="1024" height="1024" viewBox="0 0 1024 1024">',
    );
    writeFileSync(path, new_svg);
    return false;
  }
};
