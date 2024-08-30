import math
import time

def solve(a, b, c):
    discriminant = b ** 2 - 4 * a * c
    if(discriminant > 0):
        return [
            (-b + math.sqrt(discriminant)) / (2 * a),
            (-b - math.sqrt(discriminant)) / (2 * a),
        ]
    
    return [
        math.nan,
        math.nan,
    ]

def mul(s, v):
    return [s * v[0], s * v[1], s * v[2]]

def add(v, w):
    return [v[0] + w[0], v[1] + w[1], v[2] + w[2]]

def sub(v, w):
    return add(v, mul(-1, w))

def dot(v, w):
    return v[0] * w[0] + v[1] * w[1] + v[2] * w[2]

def norm(v):
    return math.sqrt(v[0] ** 2 + v[1] ** 2 + v[2] ** 2)

def normalize(v):
    return mul(1 / norm(v), v)

def bri_floor(x, z):
    if abs(z) > 60:
        return 0
    return constrain((1000 * checker(x, z)) / z ** 2, 0, 1)

def checker(x, z):
    return constrain(6 * math.sin(x * math.pi / 4) * math.cos(z * math.pi / 4), 0, 1)

def constrain(value, min_value, max_value):
    return max(min_value, min(value, max_value))

def bri_film(l, m):
    w = normalize([l, m, 1])
    c = [0, 1, 10]
    r = 2
    roots = solve(dot(w, w), 2 * dot(w, mul(-1, c)), dot(c, c) - r ** 2)
    s = min(roots)
    sw = mul(s, w)

    if math.isnan(s):
        if m == 0:
          return  bri_floor(math.inf, math.inf)
        elif 3 / m > -5 / m:
            return bri_floor(l * 3 / m, 3 / m)
        else:
            return bri_floor(l * 5 / m, -5 / m)

    n = normalize(sub(sw, c))
    b = add(sw, mul(dot(mul(-2, sw), n), n))

    u = [
        (3 - dot([0, 1, 0], sw)) / dot([0, 1, 0], b),
        (5 - dot([0, -1, 0], sw)) / dot([0, -1, 0], b),
    ]

    k = max(u)
    v = add(sw, mul(k, b))
    return bri_floor(v[0], v[2])

def print_density(d):
    scale = "MWN$@%#&B89EGA6mK5HRkbYT43V0JL7gpaseyxznocv?jIftr1li*=-~^`':;,. "
    print(scale[math.floor((255 - d) / 4)], end=" ")

def draw(width, height, pixel_size):
    for y in range(0, height, pixel_size):
        for x in range(0, width, pixel_size):
            l = x / width - 1 / 2
            m = y / height - 1 / 2
            print_density(255 * bri_film(l, m))
        print()

def timer(f, *args):
    start_time = time.time()
    f(*args)
    end_time = time.time()
    elapsed_time = end_time - start_time
    print(f"Execution time: {elapsed_time:.4f} seconds")
    

timer(draw, 250, 250, 1);