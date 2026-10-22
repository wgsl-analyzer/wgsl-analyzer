/// A point in the complex plane
alias Complex = vec2f;

/// The function z -> z^2 + c
fn quadraticMap(z: Complex, c: Complex) -> Complex {
    return vec2f(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
}

/// Evaluates the mandelbrot set, and returns whether it escapes or not
fn mandelbrot(position: Complex) -> bool {
    const maxIterations: u32 = 100;

    var current: Complex = vec2f(0.0);
    for (var i: u32 = 0; i < maxIterations; i++) {
        current = quadraticMap(current, position);
        if dot(current, current) > 16.0 {
            return true;
        }
    }
    return false;
}
