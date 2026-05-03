import { readFileSync, writeFileSync } from "node:fs";
import { optimize } from "svgo";

const arc = (args, x, y, pts, is_rel) => {
  for (let i = 0, len = args.length; i < len; i += 7) {
    let rx = Math.abs(args[i]),
      ry = Math.abs(args[i + 1]),
      phi_deg = args[i + 2],
      f_a = args[i + 3],
      f_s = args[i + 4],
      next_x = is_rel ? x + args[i + 5] : args[i + 5],
      next_y = is_rel ? y + args[i + 6] : args[i + 6];

    if (rx === 0 || ry === 0) {
      x = next_x;
      y = next_y;
      pts.push([x, y]);
      continue;
    }

    let phi = (phi_deg * Math.PI) / 180,
      cos_phi = Math.cos(phi),
      sin_phi = Math.sin(phi),
      dx_2 = (x - next_x) / 2,
      dy_2 = (y - next_y) / 2,
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
      cx = cos_phi * cx_p - sin_phi * cy_p + (x + next_x) / 2,
      cy = sin_phi * cx_p + cos_phi * cy_p + (y + next_y) / 2,
      delta_x = Math.sqrt((rx * cos_phi) ** 2 + (ry * sin_phi) ** 2),
      delta_y = Math.sqrt((rx * sin_phi) ** 2 + (ry * cos_phi) ** 2);

    pts.push(
      [cx - delta_x, cy - delta_y],
      [cx + delta_x, cy + delta_y],
      [next_x, next_y],
    );

    x = next_x;
    y = next_y;
  }
  return [x, y];
};

const CMD_HANDLERS = {
  M: (args, x, y, sx, sy, pts) => {
    x = args[0];
    y = args[1];
    sx = x;
    sy = y;
    pts.push([x, y]);
    for (let i = 2, len = args.length; i < len; i += 2) {
      x = args[i];
      y = args[i + 1];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  m: (args, x, y, sx, sy, pts) => {
    if (pts.length === 0) {
      x = args[0];
      y = args[1];
    } else {
      x += args[0];
      y += args[1];
    }
    sx = x;
    sy = y;
    pts.push([x, y]);
    for (let i = 2, len = args.length; i < len; i += 2) {
      x += args[i];
      y += args[i + 1];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  L: (args, x, y, sx, sy, pts) => {
    for (let i = 0, len = args.length; i < len; i += 2) {
      x = args[i];
      y = args[i + 1];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  l: (args, x, y, sx, sy, pts) => {
    for (let i = 0, len = args.length; i < len; i += 2) {
      x += args[i];
      y += args[i + 1];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  H: (args, x, y, sx, sy, pts) => {
    for (let i = 0, len = args.length; i < len; ++i) {
      x = args[i];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  h: (args, x, y, sx, sy, pts) => {
    for (let i = 0, len = args.length; i < len; ++i) {
      x += args[i];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  V: (args, x, y, sx, sy, pts) => {
    for (let i = 0, len = args.length; i < len; ++i) {
      y = args[i];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  v: (args, x, y, sx, sy, pts) => {
    for (let i = 0, len = args.length; i < len; ++i) {
      y += args[i];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  C: (args, x, y, sx, sy, pts) => {
    for (let i = 0, len = args.length; i < len; i += 6) {
      pts.push([args[i], args[i + 1]], [args[i + 2], args[i + 3]]);
      x = args[i + 4];
      y = args[i + 5];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  c: (args, x, y, sx, sy, pts) => {
    for (let i = 0, len = args.length; i < len; i += 6) {
      pts.push(
        [x + args[i], y + args[i + 1]],
        [x + args[i + 2], y + args[i + 3]],
      );
      x += args[i + 4];
      y += args[i + 5];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  S: (args, x, y, sx, sy, pts) => {
    for (let i = 0, len = args.length; i < len; i += 4) {
      pts.push([args[i], args[i + 1]]);
      x = args[i + 2];
      y = args[i + 3];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  s: (args, x, y, sx, sy, pts) => {
    for (let i = 0, len = args.length; i < len; i += 4) {
      pts.push([x + args[i], y + args[i + 1]]);
      x += args[i + 2];
      y += args[i + 3];
      pts.push([x, y]);
    }
    return [x, y, sx, sy];
  },
  A: (args, x, y, sx, sy, pts) => {
    const [nx, ny] = arc(args, x, y, pts, false);
    return [nx, ny, sx, sy];
  },
  a: (args, x, y, sx, sy, pts) => {
    const [nx, ny] = arc(args, x, y, pts, true);
    return [nx, ny, sx, sy];
  },
  Z: (args, x, y, sx, sy) => [sx, sy, sx, sy],
};

CMD_HANDLERS.Q = CMD_HANDLERS.S;
CMD_HANDLERS.q = CMD_HANDLERS.s;
CMD_HANDLERS.z = CMD_HANDLERS.Z;

const parse = (d) => {
  const commands = d.match(/[a-df-z][^a-df-z]*/gi) || [],
    pts = [];
  let x = 0,
    y = 0,
    sx = 0,
    sy = 0;

  for (let cmd_idx = 0, cmd_len = commands.length; cmd_idx < cmd_len; ++cmd_idx) {
    const cmd = commands[cmd_idx],
      type = cmd[0],
      args =
        cmd
          .slice(1)
          .match(/-?\d*\.?\d+(?:e[+-]?\d+)?/gi)
          ?.map(Number) || [],
      handler = CMD_HANDLERS[type];

    if (handler) {
      [x, y, sx, sy] = handler(args, x, y, sx, sy, pts);
    }
  }
  return pts;
};

const PADDING = 16;

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
