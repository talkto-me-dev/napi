const STATE_ING = 0,
  STATE_READY = 1,
  STATE_SUCCESS = 2,
  STATE_FAIL = 3,
  ICON_N = 3,
  BADGE_R = 12,
  UUID_LEN = 16,
  vbD = (b, i, max) => {
    let v = 0,
      s = 0;
    for (; i < max; ++i) {
      const x = b[i];
      v |= (x & 0x7f) << s;
      s += 7;
      if (x < 128) return [v >>> 0, i + 1];
    }
  },
  decode = (buf) => {
    const bytes = new Uint8Array(buf),
      { length } = bytes,
      id = bytes.slice(0, UUID_LEN),
      decoder = new TextDecoder(),
      tips = [],
      lens = [];
    let offset = UUID_LEN;
    for (let i = 0; i < ICON_N; ++i) {
      const [len, next] = vbD(bytes, offset, length);
      lens.push(len);
      offset = next;
    }
    for (const len of lens) {
      tips.push(decoder.decode(bytes.subarray(offset, offset + len)));
      offset += len;
    }
    return [id, tips, new Blob([bytes.subarray(offset)], { type: "image/webp" })];
  };

export default (root, onEnd) => {
  const find = (s) => root.querySelector(s),
    findAll = (s) => root.querySelectorAll(s);

  let state = STATE_ING,
    tip_li = [],
    img_url = "",
    fail_timer = null,
    captcha_id = null,
    xy_li = [],
    id_n = 0,
    title = "";

  const renderHeader = () => {
    let icons_html = "";
    for (let i = 1; i <= ICON_N; ++i) {
      icons_html +=
        '<b class="icon-item">' +
        '<b class="badge">' +
        i +
        "</b>" +
        '<b class="icon-box"><b class="icon-shape"></b></b>' +
        "</b>";
    }
    return (
      "<header>" +
      "<b>" +
      '<b class="title"></b>' +
      '<button class="refresh" title="刷新"></button>' +
      "</b>" +
      "<b>" +
      icons_html +
      "</b>" +
      "</header>"
    );
  };

  root.innerHTML =
    '<main class="captcha ing">' + renderHeader() + '<b class="click-box"></b>' + "</main>";

  const main = find("main.captcha"),
    title_el = find(".title"),
    box = find(".click-box"),
    shapes = findAll(".icon-shape"),
    refresh_btn = find(".refresh"),
    updateUi = () => {
      main.className = "captcha " + ["ing", "ready", "suc", "fail"][state];
      title_el.innerText = title;

      if (img_url && (state != STATE_ING || id_n > 0)) {
        box.style.backgroundImage = "url(" + img_url + ")";
      } else {
        box.style.backgroundImage = "";
      }

      if (state == STATE_READY) {
        tip_li.forEach((uri, i) => {
          shapes[i].style.maskImage = uri;
          shapes[i].style.webkitMaskImage = uri;
        });
      }

      if (state != STATE_READY) {
        let overlay_html = "";
        if (state == STATE_ING && id_n == 0) {
          overlay_html = '<b class="spinner"></b>';
        } else if (state == STATE_SUCCESS) {
          overlay_html = '<b class="result-mark success-mark"></b>';
        } else if (state == STATE_FAIL) {
          overlay_html = '<b class="result-mark fail-mark"></b>';
        }

        if (overlay_html) {
          box.innerHTML = '<b class="state-overlay">' + overlay_html + "</b>";
        }
      } else {
        box.innerHTML = "";
        for (let i = 0; i < id_n; ++i) {
          const badge = document.createElement("b");
          badge.className = "badge click-badge";
          badge.innerText = i + 1;
          badge.style.left = xy_li[i * 2] - BADGE_R + "px";
          badge.style.top = xy_li[i * 2 + 1] - BADGE_R + "px";
          badge.onclick = (e) => badgeClick(e, i + 1);
          box.appendChild(badge);
        }
      }
    },
    load = async () => {
      state = STATE_ING;
      title = "加载中…";
      id_n = 0;
      xy_li = [];
      if (fail_timer) {
        clearTimeout(fail_timer);
        fail_timer = null;
      }
      updateUi();

      try {
        const res = await fetch("/api/captcha"),
          buf = await res.arrayBuffer(),
          [id, tips, blob] = decode(buf);

        captcha_id = id;
        tip_li = tips.map((svg) => {
          if (!svg.includes("xmlns="))
            svg = svg.replace("<svg ", '<svg xmlns="http://www.w3.org/2000/svg" ');
          return "url('data:image/svg+xml;charset=utf-8," + encodeURIComponent(svg) + "')";
        });

        if (img_url) URL.revokeObjectURL(img_url);
        img_url = URL.createObjectURL(blob);

        state = STATE_READY;
        title = "请按序点击下图中的图标";
      } catch (e) {
        console.error(e);
        fail();
      }
      updateUi();
    },
    fail = () => {
      state = STATE_FAIL;
      title = "验证失败，一秒后刷新";
      updateUi();
      fail_timer = setTimeout(load, 1000);
    },
    verify = async () => {
      state = STATE_ING;
      title = "验证中…";
      updateUi();

      try {
        const payload = new Uint8Array(UUID_LEN + xy_li.length * 2),
          xy_16 = new Uint16Array(payload.buffer, UUID_LEN);

        payload.set(captcha_id, 0);
        for (let i = 0; i < xy_li.length; ++i) {
          xy_16[i] = xy_li[i];
        }

        const res = await fetch("/api/captcha", {
          method: "POST",
          body: payload,
        });

        if ((await res.text()) == "1") {
          state = STATE_SUCCESS;
          title = "验证成功";
          updateUi();
          if (onEnd) setTimeout(() => onEnd(true), 1000);
        } else {
          fail();
        }
      } catch (e) {
        console.error(e);
        fail();
      }
    },
    click = (e) => {
      if (state != STATE_READY) return;
      if (e.target != box && !e.target.classList.contains("click-badge")) return;

      const rect = box.getBoundingClientRect(),
        offsetX = e.clientX - rect.left,
        offsetY = e.clientY - rect.top;

      xy_li.push(offsetX, offsetY);
      ++id_n;

      updateUi();

      if (id_n == ICON_N) {
        verify();
      }
    },
    badgeClick = (e, clicked_id) => {
      e.stopPropagation();
      id_n = clicked_id - 1;
      xy_li = xy_li.slice(0, id_n * 2);
      updateUi();
    };

  refresh_btn.onclick = load;
  box.onclick = click;

  load();
};
