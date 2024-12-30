precision highp float;
uniform vec2 u_fractalPosition;
uniform vec2 u_cJulia;
uniform float u_fractalZoom;
uniform float u_brightness;
uniform float u_contrast;
uniform int u_highQuality;
uniform float u_r;
uniform float u_g;
uniform float u_b;
uniform float u_gamma;
uniform int u_fractal_type;

const int JULIA = 0;
const int MANDELBROT = 1;

float julia_inner(in int max_iter, in float cutoff, inout vec2 z) {
    vec2 z2;
    float tmp, value;
    int iterNumber;

    for(iterNumber = 0; iterNumber < max_iter; iterNumber++)
    {
        z2 = vec2(z.x * z.x, z.y * z.y); // z2x = zx * zx; z2y = zy * zy

        if( (z2.x + z2.y) > cutoff) {
            break;
        }

        tmp = z2.x - z2.y + u_cJulia.x;
        z.y = 2.0 * z.x * z.y + u_cJulia.y;
        z.x = tmp;
    }

    if(iterNumber == max_iter) {
        iterNumber = 1000000;
    }
    return float(iterNumber + 1);
}

float mandelbrot_inner(in int max_iter, in float cutoff, inout vec2 c) {
    vec2 z = vec2(0.0, 0.0);
    float tmp;
    int iterNumber;

    for(iterNumber=0; iterNumber < max_iter; iterNumber++)
    {
        if( (z.x * z.x + z.y * z.y) > cutoff) {
            break;
        }

        tmp = z.x * z.x - z.y * z.y + c.x;
        z.y = 2.0 * z.x * z.y + c.y;
        z.x = tmp;
    }

    // Used by the smoothing step
    c = z;

    if(iterNumber == max_iter) {
        iterNumber = 0;
    }
    return float(iterNumber + 1);
}

float computeHighQuality(in vec2 location)
{
    const int MAX_ITER = 4096;
    const float N = 16.0;
    float value;

    switch (u_fractal_type) {
        case JULIA:
            value = julia_inner(MAX_ITER, N, location); break;
        case MANDELBROT:
            value = mandelbrot_inner(MAX_ITER, N, location);
            // Smoothing the fractal: result = value - log2( log( abs(location) / log(N) ) )
            value = value - log2(log(sqrt(location.x * location.x + location.y * location.y)/log(4.0)) );
            break;
    }
    return log(value);
}

float computeLowQuality(in vec2 location)
{
    const int MAX_ITER = 1024;
    const float N = 4.0;
    float value;

    switch (u_fractal_type) {
        case JULIA:
            value = julia_inner(MAX_ITER, N, location); break;
        case MANDELBROT:
            value = mandelbrot_inner(MAX_ITER, N, location); break;
    }
    return log(value);
}

out vec4 out_color;

void main(void)
{
    vec2 location = vec2(gl_FragCoord.x, gl_FragCoord.y)/u_fractalZoom - u_fractalPosition;
    float value;

    if(u_highQuality != 0)
        value = computeHighQuality(location);
    else
        value = computeLowQuality(location);

    out_color =  vec4(
        pow(
            u_brightness + u_contrast * vec3(value, value, value) * vec3(u_r, u_g, u_b),
            vec3(1.0/u_gamma)
        ), 1.0);
}
