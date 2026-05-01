const STATE_LOADING = 0,
  STATE_READY = 1,
  STATE_VERIFYING = 2,
  STATE_SUCCESS = 3,
  STATE_FAIL = 4,
  ICON_N = 3,
  BADGE_R = 12,
  decode = (buf) => {
  const dv = new DataView(buf),
    decoder = new TextDecoder();
  let offset = 0;

  // Data format:
  // [8 bytes] id (u64, little endian)
  // [1 byte] tips_count
  // [tips_count times] [2 bytes length (u16 le)] [svg bytes]
  // [remaining bytes] webp image

  const id = dv.getBigUint64(offset, true).toString();
  offset += 8;

  const tip_count = dv.getUint8(offset++);
  const tips = [];
  for (let i = 0; i < tip_count; i++) {
    const tip_len = dv.getUint16(offset, true);
    offset += 2;
    tips.push(decoder.decode(new Uint8Array(buf, offset, tip_len)));
    offset += tip_len;
  }

  const bytes = new Uint8Array(buf, offset),
    blob = new Blob([bytes], { type: "image/webp" });

  return { id, tips, blob };
};

export default (root, onSuccess) => {
  const find = (s) => root.querySelector(s),
    findAll = (s) => root.querySelectorAll(s),
    icon = (i) =>
      '<b class="icon-item"><b class="badge">' +
      i +
      '</b><b class="icon-box"><b class="icon-shape"></b></b></b>';

  let captcha_id = null,
    xy_li = [],
    id_counter = 0,
    tip_state = STATE_LOADING,
    tip_uris = [],
    img_url = "",
    icons_html = "",
    fail_timer = null;

  for (let i = 1; i <= ICON_N; ++i) icons_html += icon(i);

  root.innerHTML =
    '<main class="captcha ing">' +
    "<header>" +
    '<b class="header-top">' +
    '<b class="title"></b>' +
    '<button class="refresh" title="刷新"></button>' +
    "</b>" +
    '<b class="icons-row">' +
    icons_html +
    "</b>" +
    "</header>" +
    '<b class="click-box"></b>' +
    "</main>";

  // cache stable DOM nodes
  // 缓存稳定 DOM 节点
  const main = find("main.captcha"),
    title = find(".title"),
    box = find(".click-box"),
    shapes = findAll(".icon-shape"),
    tip = (text, cls) => {
      title.innerText = text;
      title.className = "title" + (cls ? " " + cls : "");
    },
    showLoading = () => {
      tip("加载中...");
      box.style.display = "block";
      box.innerHTML = '<b class="state-overlay"><b class="spinner"></b></b>';
      for (let i = 0; i < shapes.length; ++i) {
        const s = shapes[i].style;
        s.webkitMaskImage = "";
        s.maskImage = "";
      }
    },
    showVerifying = () => {
      box.style.display = "block";
      if (!find(".state-overlay")) {
        box.innerHTML += '<b class="state-overlay"><b class="spinner"></b></b>';
      }
    },
    showResult = (text, text_cls, mark_cls) => {
      tip(text, text_cls);
      box.style.display = "block";
      box.innerHTML =
        '<b class="state-overlay" style="background:white;backdrop-filter:none"><b class="' +
        mark_cls +
        '"></b></b>';
    },
    showReady = () => {
      tip("请按序点击下图中的图标");
      box.style.display = "block";
      box.style.backgroundImage = 'url("' + img_url + '")';
      box.innerHTML = "";
      for (let i = 0; i < ICON_N; ++i) {
        if (tip_uris[i]) {
          const s = shapes[i].style;
          s.webkitMaskImage = tip_uris[i];
          s.maskImage = tip_uris[i];
        }
      }
    },
    update = () => {
      main.classList.toggle("ing", tip_state === STATE_LOADING || tip_state === STATE_VERIFYING);
      if (tip_state === STATE_LOADING) showLoading();
      else if (tip_state === STATE_VERIFYING) showVerifying();
      else if (tip_state === STATE_SUCCESS) showResult("验证成功", "success-text", "success-mark");
      else if (tip_state === STATE_FAIL) {
        showResult("验证失败，1秒后刷新", "error-text", "fail-mark");
        fail_timer = setTimeout(load, 1000);
      } else if (tip_state === STATE_READY) showReady();
    },
    reset = () => {
      id_counter = 0;
      xy_li = [];
      if (fail_timer) {
        clearTimeout(fail_timer);
        fail_timer = null;
      }
      box.innerHTML = "";
    },
    load = async () => {
      tip_state = STATE_LOADING;
      reset();
      update();

      try {
        const res = await fetch("/api/captcha"),
          buf = await res.arrayBuffer(),
          { id, tips, blob } = decode(buf);

        captcha_id = id;

        // precompute data URIs to avoid re-encoding on every update
        // 预计算 data URI，避免每次 update 重复编码
        tip_uris = tips.map((svg) => {
          if (!svg.includes("xmlns="))
            svg = svg.replace("<svg ", '<svg xmlns="http://www.w3.org/2000/svg" ');
          return "url('" + "data:image/svg+xml;charset=utf-8," + encodeURIComponent(svg) + "')";
        });

        if (img_url) URL.revokeObjectURL(img_url);
        img_url = URL.createObjectURL(blob);

        tip_state = STATE_READY;
      } catch (e) {
        console.error(e);
      }
      update();
    },
    verify = async () => {
      tip_state = STATE_VERIFYING;
      update();

      try {
        const res = await fetch("/api/verify", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
              id: captcha_id,
              clicks: xy_li,
            }),
          }),
          valid = await res.json();

        if (valid) {
          tip_state = STATE_SUCCESS;
          update();
          if (onSuccess) setTimeout(onSuccess, 1000);
        } else {
          tip_state = STATE_FAIL;
          update();
        }
      } catch (e) {
        console.error(e);
        tip_state = STATE_FAIL;
        update();
      }
    },
    click = (e) => {
      if (tip_state !== STATE_READY) return;

      const { offsetX: x, offsetY: y } = e;

      xy_li.push(x, y);
      ++id_counter;

      const badge = document.createElement("b");

      badge.className = "badge click-badge";
      badge.innerText = id_counter;
      badge.style.left = x - BADGE_R + "px";
      badge.style.top = y - BADGE_R + "px";
      badge.dataset.id = id_counter;

      badge.onclick = (ev) => {
        ev.stopPropagation();
        const clicked_id = Number(badge.dataset.id),
          badges = box.querySelectorAll(".click-badge");
        id_counter = clicked_id - 1;
        xy_li.splice(id_counter * 2);
        for (let i = 0; i < badges.length; ++i) {
          const b = badges[i];
          if (Number(b.dataset.id) >= clicked_id) b.remove();
        }
      };

      box.appendChild(badge);

      if (id_counter === ICON_N) {
        verify();
      }
    };

  find(".refresh").addEventListener("click", load);
  box.addEventListener("click", click);

  load();
};
