function draw(width, height, pixelSize) {
  for (let y = 0; y < height; y += pixelSize) {
    for (let x = 0; x < width; x += pixelSize) {
      let l = x / width - 1 / 2;
      let m = y / height - 1 / 2;
      printdensity(255 * briFilm(l, m));
    }
    process.stdout.write("\n");
  }
}

function solve(a, b, c) {
  return [
    (-b + Math.sqrt(b ** 2 - 4 * a * c)) / (2 * a),
    (-b - Math.sqrt(b ** 2 - 4 * a * c)) / (2 * a),
  ];
}

function mul(s, v) {
  return [s * v[0], s * v[1], s * v[2]];
}

function add(v, w) {
  return [v[0] + w[0], v[1] + w[1], v[2] + w[2]];
}

function sub(v, w) {
  return add(v, mul(-1, w));
}

function dot(v, w) {
  return v[0] * w[0] + v[1] * w[1] + v[2] * w[2];
}

function norm(v) {
  return Math.sqrt(v[0] ** 2 + v[1] ** 2 + v[2] ** 2);
}

function normalize(v) {
  return mul(1 / norm(v), v);
}

function briFilm(l, m) {
  let w = normalize([l, m, 1]);
  let c = [0, 1, 10];
  let r = 2;
  let s = Math.min(
    ...solve(dot(w, w), 2 * dot(w, mul(-1, c)), dot(c, c) - r ** 2)
  );
  let sw = mul(s, w);

  if (isNaN(s)) {
    if (3 / m > -5 / m) {
      return briFloor((l * 3) / m, 3 / m);
    } else {
      return briFloor((l * 5) / m, -5 / m);
    }
  }

  let n = normalize(sub(sw, c));
  let b = add(sw, mul(dot(mul(-2, sw), n), n));

  let u = [
    (3 - dot([0, 1, 0], sw)) / dot([0, 1, 0], b),
    (5 - dot([0, -1, 0], sw)) / dot([0, -1, 0], b),
  ];

  let k = u[0] > u[1] ? u[0] : u[1];
  let v = add(sw, mul(k, b));
  return briFloor(v[0], v[2]);
}

function briFloor(x, z) {
  if (Math.abs(z) > 60) return 0;
  return constrain((1000 * checker(x, z)) / z ** 2, 0, 1);
}

function checker(x, z) {
  return constrain(
    6 * Math.sin((x * Math.PI) / 4) * Math.cos((z * Math.PI) / 4),
    0,
    1
  );
}

function constrain(value, minValue, maxValue) {
  if (value < minValue) {
    return minValue;
  } else if (value > maxValue) {
    return maxValue;
  } else {
    return value;
  }
}

function printdensity(d) {
  let scale =
    "MWN$@%#&B89EGA6mK5HRkbYT43V0JL7gpaseyxznocv?jIftr1li*=-~^`':;,. ";
  process.stdout.write(`${scale[Math.floor((255 - d) / 4)]} `);
}

function timer(f, ...args) {
  let startTime = process.hrtime.bigint();
  f(...args);
  const endTime = process.hrtime.bigint();
  const elapsed = Number(endTime - startTime) / 1e9;
  console.log(`Execution time: ${elapsed.toFixed(4)} seconds`);
}

timer(draw, 250, 250, 1);
