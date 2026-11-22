/* OpenFlightDeck – Landingpage
   Ohne Abhängigkeiten. Die iPad-Karte ist eine Illustration, keine echten Daten. */
(function () {
  "use strict";
  document.documentElement.classList.remove("no-js");
  var reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  /* ---------- Kartengeometrie (viewBox 1000 x 750) ---------- */
  var WP = [ [140, 600], [300, 470], [560, 380], [780, 330] ];
  var WP_NAMES = ["EDAZ", "KBG", "MÜH", "EDDC"];
  var NM_PER_PX = 0.1;
  var D_POLY = "M690 230 L880 214 L930 330 L866 456 L700 446 L660 330 Z";
  var R_POLY = "M352 168 L500 152 L536 262 L452 322 L344 282 Z";
  var NEAREST = [520, 590];

  function routeD(pts) { return pts.map(function (p, i) { return (i ? "L" : "M") + p[0] + " " + p[1]; }).join(" "); }

  function deviceMarkup(id) {
    var wps = WP.slice(1, 3).map(function (p) { return '<circle class="m-wp" cx="' + p[0] + '" cy="' + p[1] + '" r="9"/>'; }).join("");
    return '' +
    '<div class="screen">' +
    '<svg class="map" viewBox="0 0 1000 750" aria-hidden="true">' +
      '<defs>' +
        '<pattern id="' + id + '-hatch" width="9" height="9" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><line class="hatch-line" x1="0" y1="0" x2="0" y2="9"/></pattern>' +
        '<pattern id="' + id + '-cross" width="9" height="9" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><line class="hatch-line" x1="0" y1="0" x2="0" y2="9"/><line class="hatch-line" x1="0" y1="0" x2="9" y2="0"/></pattern>' +
        '<clipPath id="' + id + '-cd"><path d="' + D_POLY + '"/></clipPath>' +
        '<clipPath id="' + id + '-cr"><path d="' + R_POLY + '"/></clipPath>' +
        '<radialGradient id="' + id + '-rg"><stop offset="0" stop-color="#e8c21c" stop-opacity=".75"/><stop offset=".35" stop-color="#46b85a" stop-opacity=".55"/><stop offset="1" stop-color="#46b85a" stop-opacity="0"/></radialGradient>' +
      '</defs>' +
      '<rect class="m-paper" width="1000" height="750"/>' +
      // Gelände
      '<path class="m-terrain" d="M-20 -20 H360 C340 60 280 120 180 150 C90 175 30 230 -20 260 Z"/>' +
      '<path class="m-contour" d="M-20 200 C40 170 110 130 160 110 C230 80 270 40 290 -20"/>' +
      '<path class="m-contour" d="M-20 130 C40 110 100 80 140 50 C170 30 190 0 200 -20"/>' +
      '<path class="m-terrain" d="M1020 520 C930 540 880 600 860 660 C850 700 860 740 870 770 H1020 Z"/>' +
      '<path class="m-contour" d="M1020 600 C960 610 930 650 925 700 C922 730 930 760 935 770"/>' +
      // Wald
      '<path class="m-forest" d="M420 520 C470 500 540 510 560 540 C580 570 540 610 480 615 C420 620 380 590 390 555 Z"/>' +
      '<path class="m-forest" d="M60 380 C110 350 190 360 200 400 C210 440 150 460 100 455 C50 450 30 410 60 380 Z"/>' +
      '<path class="m-forest" d="M820 80 C870 60 950 70 960 110 C970 150 900 160 860 150 C820 140 800 105 820 80 Z"/>' +
      // Straßen
      '<path class="m-road" d="M-10 300 C200 320 500 300 1010 380"/>' +
      '<path class="m-road" d="M420 -10 C440 200 470 500 540 760"/>' +
      // Wasser
      '<path class="m-water" d="M-20 520 C120 480 200 560 320 520 S520 430 600 470 S820 560 1020 500"/>' +
      '<ellipse class="m-lake" cx="760" cy="150" rx="46" ry="24"/>' +
      // Lufträume
      '<path class="m-e" d="M-10 118 C300 58 600 178 1010 88"/>' +
      '<g class="m-labelbox" transform="translate(150 70)"><rect width="56" height="44" rx="4"/><text x="28" y="19" text-anchor="middle">E</text><text x="28" y="37" text-anchor="middle" style="font-size:12px">2500</text></g>' +
      '<path class="m-fis" d="M610 -10 C590 200 640 520 590 760"/>' +
      '<path class="m-d-band" d="' + D_POLY + '" clip-path="url(#' + id + '-cd)"/>' +
      '<path class="m-d-line" d="' + D_POLY + '"/>' +
      '<path class="m-warnzone" d="' + D_POLY + '"/>' +
      '<g class="m-labelbox" transform="translate(842 380)"><rect width="66" height="44" rx="4"/><text x="33" y="19" text-anchor="middle" style="font-size:12px">FL100</text><text x="33" y="37" text-anchor="middle" style="font-size:12px">D 1500</text></g>' +
      '<path class="m-edr-band" stroke="url(#' + id + '-hatch)" d="' + R_POLY + '" clip-path="url(#' + id + '-cr)"/>' +
      '<path class="m-edr-line" d="' + R_POLY + '"/>' +
      '<g class="m-labelbox edr" transform="translate(396 210)"><rect width="84" height="44" rx="4"/><text x="42" y="19" text-anchor="middle" style="font-size:12px">ED-R</text><text x="42" y="37" text-anchor="middle" style="font-size:12px">GND–4500</text></g>' +
      // Radar
      '<g class="m-radar"><ellipse cx="830" cy="600" rx="170" ry="110" fill="url(#' + id + '-rg)"/><ellipse cx="240" cy="220" rx="120" ry="70" fill="url(#' + id + '-rg)" style="animation-delay:-4s"/></g>' +
      // Orte & Plätze
      '<text class="m-town" x="318" y="500">Kirchberg</text>' +
      '<text class="m-town" x="575" y="412">Mühlau</text>' +
      '<g class="m-field" transform="translate(140 600)"><circle r="13"/><line x1="-8" y1="8" x2="8" y2="-8"/><text class="code" x="22" y="30">EDAZ</text></g>' +
      '<g class="m-field" transform="translate(780 330)"><circle r="13"/><line x1="-9" y1="3" x2="9" y2="-3"/><text class="code" x="-20" y="-24">EDDC</text></g>' +
      '<g class="m-field" transform="translate(' + NEAREST[0] + ' ' + NEAREST[1] + ')"><circle r="11"/><line x1="0" y1="-8" x2="0" y2="8"/><text x="18" y="26">Seefeld</text></g>' +
      '<g class="m-field" transform="translate(240 300)"><circle r="11"/><line x1="-8" y1="0" x2="8" y2="0"/><text x="-30" y="-20">Hainau</text></g>' +
      // METAR
      '<g class="m-metar ov">' +
        metar(140, 600, "#2f9a5a", "VFR") + metar(780, 330, "#2f6fd6", "MVFR") + metar(NEAREST[0], NEAREST[1], "#2f9a5a", "VFR") + metar(240, 300, "#2f9a5a", "VFR") +
      '</g>' +
      // Route
      '<path class="m-route-halo" d="' + routeD(WP) + '"/>' +
      '<path class="m-route" d="' + routeD(WP) + '"/>' +
      wps +
      '<circle class="m-drag ov" r="16" cx="' + WP[2][0] + '" cy="' + WP[2][1] + '"/>' +
      '<path class="m-trail" d="' + routeD(WP) + '"/>' +
      // Direct-To / Nearest
      '<line class="m-direct ov" x1="0" y1="0" x2="' + NEAREST[0] + '" y2="' + NEAREST[1] + '"/>' +
      '<g class="m-nearest ov"><circle cx="' + NEAREST[0] + '" cy="' + NEAREST[1] + '" r="22"/></g>' +
      // Verkehr
      '<g class="m-traffic">' +
        '<g class="tgt" data-from="840,600" data-to="600,250" data-dur="22"><path d="M0 -15 L11 11 L0 5 L-11 11 Z"/><text x="18" y="6">+300 ↑</text></g>' +
        '<g class="tgt" data-from="160,240" data-to="520,170" data-dur="26"><path d="M0 -15 L11 11 L0 5 L-11 11 Z"/><text x="18" y="6">−500 ↓</text></g>' +
      '</g>' +
      // „Was ist hier“
      '<circle class="m-ripple ov" cx="560" cy="226" r="26"/>' +
      '<circle class="m-ripple r2 ov" cx="560" cy="226" r="26"/>' +
      '<circle class="m-tapdot ov" cx="560" cy="226" r="7"/>' +
      // Eigenes Flugzeug
      '<g class="m-plane"><path d="M0 -26 L4 -10 L22 0 L22 5 L4 1 L3 14 L9 19 L9 23 L0 20 L-9 23 L-9 19 L-3 14 L-4 1 L-22 5 L-22 0 L-4 -10 Z"/></g>' +
    '</svg>' +
    // Datenleiste
    '<div class="bar"><div><small>GS</small><b data-k="gs">105</b><b><i>kt</i></b></div><div><small>TRK</small><b data-k="trk">071°</b></div><div><small>ALT</small><b data-k="alt">3500</b><b><i>ft</i></b></div><div><small>WPT</small><b data-k="wpt">MÜH</b></div><div><small>ETE</small><b data-k="ete">4:10</b></div></div>' +
    '<div class="banner ov"><b>VORSICHT</b><span>Luftraum D in 2 Min · ab 1500 ft · Radar 119,055</span><span class="ack">Quittieren</span></div>' +
    // Vertikalprofil
    '<div class="profile"><svg viewBox="0 0 1000 165" aria-hidden="true">' +
      '<rect class="p-air" x="520" y="10" width="490" height="102"/>' +
      '<text class="p-label" x="540" y="34">D · 1500 ft – FL 100</text>' +
      '<path class="p-terrain" d="M0 165 V140 C80 130 140 112 220 124 C300 136 360 98 440 110 C520 122 600 132 700 120 C800 108 880 126 1000 118 V165 Z"/>' +
      '<line class="p-alt" x1="30" y1="54" x2="1000" y2="54"/>' +
      '<circle class="p-hit" cx="520" cy="54" r="9"/>' +
      '<path d="M18 54 L42 44 L42 64 Z" fill="var(--m-route)"/>' +
      '<text x="56" y="44">3500 ft</text>' +
      '<text x="250" y="160">5 NM</text><text x="500" y="160">10 NM</text><text x="750" y="160">15 NM</text>' +
    '</svg></div>' +
    // Was ist hier
    '<div class="sheet ov"><h4>Was ist hier?</h4>' +
      '<div class="row"><span><i class="chip">E</i>Luftraum</span><b>2500 ft – FL 100</b></div>' +
      '<div class="row"><span>Nächster Platz</span><b>Seefeld</b></div>' +
      '<div class="row"><span>Distanz</span><b>9,4 NM</b></div>' +
      '<div class="row"><span>Peilung</span><b>052°</b></div>' +
      '<div class="row"><span>Flugzeit</span><b>5 Min</b></div>' +
    '</div>' +
    // Navlog
    '<div class="navlog ov"><h4>Navlog <span>GS 105 kt · 20 l/h</span></h4><table>' +
      '<thead><tr><th>Strecke</th><th>Kurs</th><th>NM</th><th>Min</th></tr></thead>' +
      '<tbody><tr><td>EDAZ–KBG</td><td>051°</td><td>20,6</td><td>12</td></tr><tr><td>KBG–MÜH</td><td>071°</td><td data-k="nm2">27,5</td><td data-k="min2">16</td></tr><tr><td>MÜH–EDDC</td><td>077°</td><td>22,6</td><td>13</td></tr></tbody>' +
      '<tfoot><tr><td>Gesamt · 14 l</td><td></td><td data-k="nmT">70,7</td><td data-k="minT">41</td></tr></tfoot>' +
    '</table></div>' +
    '<div class="age">Daten · Stand 20.11. <i>●</i></div>' +
    '<div class="nearest-card ov"><b>Seefeld</b><span data-k="near">6,2 NM · 198° · 4 Min</span></div>' +
    '<div class="dto">D→<small>NEAREST</small></div>' +
    '</div>';
  }
  function metar(x, y, color, label) {
    var w = label.length * 9 + 14;
    return '<g transform="translate(' + x + ' ' + y + ')"><circle r="15" fill="' + color + '"/><rect x="' + (-w / 2) + '" y="-50" width="' + w + '" height="24" fill="' + color + '"/><text x="0" y="-33" text-anchor="middle">' + label + '</text></g>';
  }

  /* ---------- Gerät initialisieren ---------- */
  function Device(el) {
    var id = el.getAttribute("data-device");
    el.innerHTML = deviceMarkup(id);
    this.el = el;
    this.id = id;
    this.screen = el.querySelector(".screen");
    this.route = el.querySelector(".m-route");
    this.halo = el.querySelector(".m-route-halo");
    this.trail = el.querySelector(".m-trail");
    this.plane = el.querySelector(".m-plane");
    this.direct = el.querySelector(".m-direct");
    this.drag = el.querySelector(".m-drag");
    this.wps = el.querySelectorAll(".m-wp");
    this.edrBand = el.querySelector(".m-edr-band");
    this.k = {};
    var self = this;
    el.querySelectorAll("[data-k]").forEach(function (n) { self.k[n.getAttribute("data-k")] = n; });
    this.tgts = Array.prototype.map.call(el.querySelectorAll(".tgt"), function (g) {
      var f = g.getAttribute("data-from").split(",").map(Number), t = g.getAttribute("data-to").split(",").map(Number);
      return { g: g, f: f, t: t, dur: +g.getAttribute("data-dur"), ang: Math.atan2(t[0] - f[0], -(t[1] - f[1])) * 180 / Math.PI };
    });
    this.points = WP.map(function (p) { return p.slice(); });
    this.setRoute(this.points);
    this.visible = true;
    this.t0 = performance.now() - (id === "hero" ? 14000 : 9000);
    this.last = {};
  }
  Device.prototype.setRoute = function (pts) {
    var d = routeD(pts);
    this.route.setAttribute("d", d); this.halo.setAttribute("d", d); this.trail.setAttribute("d", d);
    this.len = this.route.getTotalLength();
    this.segLens = [];
    var acc = 0;
    for (var i = 1; i < pts.length; i++) { acc += Math.hypot(pts[i][0] - pts[i - 1][0], pts[i][1] - pts[i - 1][1]); this.segLens.push(acc); }
  };
  Device.prototype.set = function (key, val) {
    if (this.last[key] === val || !this.k[key]) return;
    this.last[key] = val; this.k[key].textContent = val;
  };
  Device.prototype.frame = function (now) {
    var t = (now - this.t0) / 1000;
    var state = this.el.getAttribute("data-state");
    var LOOP = 38;
    var p = reduce ? 0.62 : (t % LOOP) / LOOP;

    // Rubber-Band-Planung: Wegpunkt MÜH wandert hin und her
    if (state === "plan") {
      var s = reduce ? 0.6 : (Math.sin(t * 1.4) + 1) / 2;
      var pts = WP.map(function (q) { return q.slice(); });
      pts[2] = [WP[2][0] - 50 * s, WP[2][1] + 70 * s];
      this.setRoute(pts);
      this.drag.setAttribute("cx", pts[2][0]); this.drag.setAttribute("cy", pts[2][1]);
      this.wps[1].setAttribute("cx", pts[2][0]); this.wps[1].setAttribute("cy", pts[2][1]);
      var nm2 = Math.hypot(pts[2][0] - pts[1][0], pts[2][1] - pts[1][1]) * NM_PER_PX;
      var nm3 = Math.hypot(pts[3][0] - pts[2][0], pts[3][1] - pts[2][1]) * NM_PER_PX;
      var total = 20.6 + nm2 + nm3;
      this.set("nm2", nm2.toFixed(1).replace(".", ","));
      this.set("min2", String(Math.round(nm2 / 105 * 60)));
      this.set("nmT", total.toFixed(1).replace(".", ","));
      this.set("minT", String(Math.round(total / 105 * 60)));
      this.planned = true;
    } else if (this.planned) {
      this.planned = false;
      this.setRoute(WP);
      this.wps[1].setAttribute("cx", WP[2][0]); this.wps[1].setAttribute("cy", WP[2][1]);
    }

    // Flugzeug entlang der Route
    var L = this.len, cur = p * L;
    var a = this.route.getPointAtLength(cur);
    var dirA = cur + 4 <= L ? a : this.route.getPointAtLength(cur - 4);
    var dirB = cur + 4 <= L ? this.route.getPointAtLength(cur + 4) : a;
    var trk = (Math.atan2(dirB.x - dirA.x, -(dirB.y - dirA.y)) * 180 / Math.PI + 360) % 360;
    this.plane.setAttribute("transform", "translate(" + a.x.toFixed(1) + " " + a.y.toFixed(1) + ") rotate(" + trk.toFixed(1) + ")");
    this.trail.style.strokeDasharray = cur.toFixed(1) + " " + L.toFixed(1);

    // Datenleiste
    var gs = Math.round(105 + Math.sin(t * 0.7) * 2 + Math.sin(t * 2.3));
    var alt = Math.round((3500 + Math.sin(t * 0.5) * 30) / 10) * 10;
    var seg = 0; while (seg < this.segLens.length - 1 && cur > this.segLens[seg]) seg++;
    var remainNm = (this.segLens[seg] - cur) * NM_PER_PX;
    var eteSec = Math.max(0, Math.round(remainNm / gs * 3600));
    this.set("gs", String(gs));
    this.set("alt", String(alt));
    this.set("trk", ("00" + Math.round(trk)).slice(-3) + "°");
    this.set("wpt", WP_NAMES[seg + 1]);
    this.set("ete", Math.floor(eteSec / 60) + ":" + ("0" + (eteSec % 60)).slice(-2));

    // Vorwarnung im Hero-Loop kurz vor dem Luftraum D
    if (this.id === "hero") this.el.classList.toggle("flash", p > 0.66 && p < 0.83);

    // Direct-To-Linie
    if (state === "direct") {
      this.direct.setAttribute("x1", a.x.toFixed(1)); this.direct.setAttribute("y1", a.y.toFixed(1));
      var dx = NEAREST[0] - a.x, dy = NEAREST[1] - a.y;
      var dist = Math.hypot(dx, dy) * NM_PER_PX;
      var brg = (Math.atan2(dx, -dy) * 180 / Math.PI + 360) % 360;
      this.set("near", dist.toFixed(1).replace(".", ",") + " NM · " + ("00" + Math.round(brg)).slice(-3) + "° · " + Math.max(1, Math.round(dist / gs * 60)) + " Min");
    }

    // Verkehr
    for (var i = 0; i < this.tgts.length; i++) {
      var g = this.tgts[i], q = reduce ? 0.5 : ((t + i * 7) % g.dur) / g.dur;
      var x = g.f[0] + (g.t[0] - g.f[0]) * q, y = g.f[1] + (g.t[1] - g.f[1]) * q;
      g.g.setAttribute("transform", "translate(" + x.toFixed(1) + " " + y.toFixed(1) + ")");
      g.g.firstChild.setAttribute("transform", "rotate(" + g.ang.toFixed(1) + ")");
    }
  };
  Device.prototype.theme = function (name) {
    this.screen.classList.toggle("night", name === "night");
    this.screen.classList.toggle("cvd", name === "cvd");
    this.edrBand.setAttribute("stroke", "url(#" + this.id + (name === "cvd" ? "-cross" : "-hatch") + ")");
  };

  var devices = Array.prototype.map.call(document.querySelectorAll(".device[data-device]"), function (el) { return new Device(el); });

  if ("IntersectionObserver" in window) {
    var vio = new IntersectionObserver(function (entries) {
      entries.forEach(function (e) { devices.forEach(function (d) { if (d.el === e.target) d.visible = e.isIntersecting; }); });
    });
    devices.forEach(function (d) { vio.observe(d.el); });
  }

  function loop(now) {
    devices.forEach(function (d) { if (d.visible) d.frame(now); });
    if (!reduce) requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);
  if (reduce) { setInterval(function () { devices.forEach(function (d) { d.frame(performance.now()); }); }, 1000); }

  /* ---------- Scrollytelling ---------- */
  var story = devices.filter(function (d) { return d.id === "story"; })[0];
  var chips = document.querySelector(".theme-chips");
  var themeTimer = null;
  var THEMES = ["day", "night", "cvd"];
  function showTheme(i) {
    var name = THEMES[i % 3];
    story.theme(name);
    chips.querySelectorAll("span").forEach(function (s) { s.classList.toggle("on", s.getAttribute("data-t") === name); });
  }
  function setState(state) {
    if (!story || story.el.getAttribute("data-state") === state) return;
    story.el.setAttribute("data-state", state);
    clearInterval(themeTimer); themeTimer = null;
    if (state === "themes") {
      chips.classList.add("show");
      var i = 1; showTheme(i);
      themeTimer = setInterval(function () { i++; showTheme(i); }, 2400);
    } else {
      chips.classList.remove("show");
      story.theme("day");
    }
  }
  var steps = document.querySelectorAll(".step");
  if ("IntersectionObserver" in window && story) {
    var sio = new IntersectionObserver(function (entries) {
      entries.forEach(function (e) {
        if (!e.isIntersecting) return;
        steps.forEach(function (s) { s.classList.toggle("active", s === e.target); });
        setState(e.target.getAttribute("data-state"));
      });
    }, { rootMargin: window.matchMedia("(max-width: 900px)").matches ? "-66% 0px -30% 0px" : "-48% 0px -48% 0px" });
    steps.forEach(function (s) { sio.observe(s); });
    steps[0].classList.add("active");
  } else {
    steps.forEach(function (s) { s.classList.add("active"); });
  }

  /* ---------- Reveal ---------- */
  var reveals = document.querySelectorAll(".reveal");
  if ("IntersectionObserver" in window && !reduce) {
    var rio = new IntersectionObserver(function (entries) {
      entries.forEach(function (e) { if (e.isIntersecting) { e.target.classList.add("in"); rio.unobserve(e.target); } });
    }, { rootMargin: "0px 0px -8% 0px" });
    reveals.forEach(function (r) { rio.observe(r); });
  } else {
    reveals.forEach(function (r) { r.classList.add("in"); });
  }

  /* ---------- Scroll: Nav, Parallax, Roadmap ---------- */
  var nav = document.querySelector(".nav");
  var photo = document.querySelector(".sky-photo");
  var phases = document.getElementById("phases");
  var ticking = false;
  function onScroll() {
    ticking = false;
    var y = window.scrollY;
    nav.classList.toggle("scrolled", y > 30);
    if (!reduce && photo && y < window.innerHeight * 1.2) photo.style.setProperty("--py", (y * 0.25).toFixed(1));
    if (phases) {
      var r = phases.getBoundingClientRect();
      var vh = window.innerHeight;
      var nowNode = phases.querySelector(".now");
      var cap = nowNode ? (nowNode.offsetTop + 16) / r.height : 1;
      var prog = Math.min(cap, Math.max(0, (vh * 0.6 - r.top) / r.height));
      phases.style.setProperty("--prog", prog.toFixed(3));
    }
  }
  window.addEventListener("scroll", function () { if (!ticking) { ticking = true; requestAnimationFrame(onScroll); } }, { passive: true });
  onScroll();

  /* ---------- Turbulenz-Spiel ---------- */
  var field = document.getElementById("toyField");
  var toggle = document.getElementById("turbToggle");
  if (field && toggle) {
    var score = { small: 0, big: 0, miss: 0 };
    var out = { small: document.getElementById("hitSmall"), big: document.getElementById("hitBig"), miss: document.getElementById("miss") };
    function setShake(on) { toggle.setAttribute("aria-pressed", on ? "true" : "false"); field.classList.toggle("shake", on && !reduce); }
    setShake(true);
    toggle.addEventListener("click", function () { setShake(toggle.getAttribute("aria-pressed") !== "true"); });
    field.addEventListener("pointerdown", function (e) {
      var btn = e.target.closest(".target");
      if (btn) {
        var k = btn.getAttribute("data-target");
        score[k]++; out[k].textContent = score[k];
        btn.classList.remove("hit"); void btn.offsetWidth; btn.classList.add("hit");
      } else {
        score.miss++; out.miss.textContent = score.miss;
        var r = field.getBoundingClientRect();
        var m = document.createElement("span");
        m.className = "miss-mark";
        m.style.left = (e.clientX - r.left) + "px"; m.style.top = (e.clientY - r.top) + "px";
        field.appendChild(m);
        setTimeout(function () { m.remove(); }, 1000);
      }
    });
  }
})();
