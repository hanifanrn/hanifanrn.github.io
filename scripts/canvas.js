let space;

function floatySpace() {
  let colors = ["#FF3F8E", "#04C2C9", "#2E55C1"];

  space = new CanvasSpace("canvas").display();
  let form = new Form(space);

  // Elements
  let pts = [];
  let center = space.size.$divide(13);
  let angle = (window.innerWidth / 2584);
  let count = window.innerWidth / 8;
  if (count > 144) count = 144;
  let line = new Line(0, angle).to(space.size.x, 0);
  let mouse = center.clone();

  let r = Math.min(space.size.x, space.size.y) * 1;
  for (let i = 0; i < count; i++) {
    let p = new Vector(
      Math.random() * r - Math.random() * r,
      Math.random() * r - Math.random() * r
    );
    p.moveBy(center).rotate2D((i * Math.PI) / count, center);
    p.brightness = 0.1;
    pts.push(p);
  }

  // Canvas
  space.add({
    animate: function (time, fps, context) {
      for (let i = 0; i < pts.length; i++) {
        // rotate the points slowly
        let pt = pts[i];

        pt.rotate2D(Const.one_degree / 34, center);
        form
          .stroke(false)
          .fill(colors[i % 3])
          .point(pt, 1);

        // get line from pt to the mouse line
        let ln = new Line(pt).to(line.getPerpendicularFromPoint(pt));

        // opacity of line derived from distance to the line
        let distFromMouse = Math.abs(ln.getDistanceFromPoint(mouse));

        if (distFromMouse < 55) {
          if (pts[i].brightness < 0.3) pts[i].brightness += 0.013;
        } else {
          if (pts[i].brightness > 0.1) pts[i].brightness -= 0.01;
        }

        let color = "rgba(255,255,255," + pts[i].brightness + ")";
        form.stroke(color).fill(true).line(ln);
      }
    },

    onMouseAction: function (type, x, y, evt) {
      if (type == "move") {
        mouse.set(x, y);
      }
    },

    onTouchAction: function (type, x, y, evt) {
      this.onMouseAction(type, x, y);
    },
  });

  space.bindMouse();
  space.play();
}

floatySpace();

$(window).resize(function () {
  space.removeAll();
  $("canvas").remove();
  floatySpace();
});
