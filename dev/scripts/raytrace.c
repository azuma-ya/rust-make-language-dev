#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif


void draw(int width, int height, int pixelSize);
void printdensity(double d);
void solve(double a, double b, double c, double* result);
void mul(double s, double* v, double* result);
void add(double* v, double* w, double* result);
void sub(double* v, double* w, double* result);
double dot(double* v, double* w);
double norm(double* v);
void normalize(double* v, double* result);
double briFilm(double l, double m);
double briFloor(double x, double z);
double checker(double x, double z);
double constrain(double value, double minValue, double maxValue);
void timer(void (*f)(int, int, int), int width, int height, int pixelSize);

int main() {
    timer(draw, 250, 250, 1);
    return 0;
}

void draw(int width, int height, int pixelSize) {
    for (int y = 0; y < height; y += pixelSize) {
        for (int x = 0; x < width; x += pixelSize) {
            double l = x / (double)width - 0.5;
            double m = y / (double)height - 0.5;
            printdensity(255 * briFilm(l, m));
        }
        printf("\n");
    }
}

void printdensity(double d) {
    const char scale[] = "MWN$@%#&B89EGA6mK5HRkbYT43V0JL7gpaseyxznocv?jIftr1li*=-~^`':;,. ";
    int index = (int)((255 - d) / 4);
    if (index < 0) index = 0;
    if (index >= sizeof(scale) - 1) index = sizeof(scale) - 2;
    printf("%c ", scale[index]);
}

void solve(double a, double b, double c, double* result) {
    double discriminant = b * b - 4 * a * c;
    result[0] = (-b + sqrt(discriminant)) / (2 * a);
    result[1] = (-b - sqrt(discriminant)) / (2 * a);
}

void mul(double s, double* v, double* result) {
    result[0] = s * v[0];
    result[1] = s * v[1];
    result[2] = s * v[2];
}

void add(double* v, double* w, double* result) {
    result[0] = v[0] + w[0];
    result[1] = v[1] + w[1];
    result[2] = v[2] + w[2];
}

void sub(double* v, double* w, double* result) {
	double temp[3];
	mul(-1, w, temp);
    add(v, temp, result);
}

double dot(double* v, double* w) {
    return v[0] * w[0] + v[1] * w[1] + v[2] * w[2];
}

double norm(double* v) {
    return sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
}

void normalize(double* v, double* result) {
    return mul(1 / norm(v), v, result);
}

double min(double a, double b) {
    return (a < b) ? a : b;
}



double briFilm(double l, double m) {
    double w[3] = {l, m, 1};
    double w_normalized[3];
	normalize(w, w_normalized);

    double c[3] = {0, 1, 10};
    double r = 2;

    double answer[2];
	double neg_c[3];
	mul(-1, c, neg_c);
    solve(dot(w_normalized, w_normalized), 2 * dot(w_normalized, neg_c), dot(c, c) - r * r, answer);

	double s = min(answer[0], answer[1]);
    double sw[3];
	mul(s, w_normalized, sw);

    if (isnan(s)) {
        if (3 / m > -5 / m) {
            return briFloor((l * 3) / m, 3 / m);
        } else {
            return briFloor((l * 5) / m, -5 / m);
        }
    }
    
	double sub_sw_c[3];
    double n_normalized[3];
	sub(sw, c, sub_sw_c);
	normalize(sub_sw_c, n_normalized);

	double b_temp[3];
	mul(-2, sw, b_temp);
    double b[3] = {sw[0] + dot(b_temp, n_normalized) * n_normalized[0], 
                   sw[1] + dot(b_temp, n_normalized) * n_normalized[1], 
                   sw[2] + dot(b_temp, n_normalized) * n_normalized[2]};
    
    double u[2] = {
        (3 - dot((double[]){0, 1, 0}, sw)) / dot((double[]){0, 1, 0}, b),
        (5 - dot((double[]){0, -1, 0}, sw)) / dot((double[]){0, -1, 0}, b)
    };
    
    double k = (u[0] > u[1]) ? u[0] : u[1];
    double v[3];
    double v_temp[3];
	mul(k, b, v_temp);
	add(sw, v_temp, v);

    return briFloor(v[0], v[2]);
}

double briFloor(double x, double z) {
    if (fabs(z) > 60) return 0;
    return constrain((1000 * checker(x, z)) / (z * z), 0, 1);
}

double checker(double x, double z) {
    return constrain(6 * sin((x * M_PI) / 4) * cos((z * M_PI) / 4), 0, 1);
}

double constrain(double value, double minValue, double maxValue) {
    if (value < minValue) {
        return minValue;
    } else if (value > maxValue) {
        return maxValue;
    } else {
        return value;
    }
}

void timer(void (*f)(int, int, int), int width, int height, int pixelSize) {
    clock_t startTime = clock();
    f(width, height, pixelSize);
    clock_t endTime = clock();
    double elapsed = (double)(endTime - startTime) / CLOCKS_PER_SEC;
    printf("Execution time: %.4f seconds\n", elapsed);
}