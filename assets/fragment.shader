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


float julia_inner(in int max_iter, in float cutoff, inout vec2 z) {
    vec2 z2;
    float tmp, value;
    int iterNumber = max_iter;

    // start at 1, since Log(0) = NaN
    for(int i = 1; i < max_iter; i++)
    {
        z2 = vec2(z.x * z.x, z.y * z.y); // z2x = zx * zx; z2y = zy * zy

        if( (z2.x + z2.y) > cutoff)
        {
                iterNumber = i;
                break;
        }

        tmp = z2.x - z2.y + u_cJulia.x;
        z.y = 2.0 * z.x * z.y + u_cJulia.y;
        z.x = tmp;
    }
    return float(iterNumber + 1);
}

float computeHighQuality(in vec2 z)
{
    const int MAX_ITER = 4096;
    const float N = 4.0;

    // Smoothing the fractal: result = IterNumber - log2( log( abs(z) / log(N) ) )
    float value = julia_inner(MAX_ITER, N * N, z) - log2( log( sqrt( z.x * z.x + z.y * z.y )/log(N)) );
    return log(value);
}

float computeLowQuality(in vec2 z)
{
    const int MAX_ITER = 1024;
    const float N = 2.0;

    return log(julia_inner(MAX_ITER, N * N, z));
}

out vec4 out_color;

void main(void)
{
    vec2 z = vec2( (gl_FragCoord.x/u_fractalZoom - u_fractalPosition.x), (gl_FragCoord.y/u_fractalZoom - u_fractalPosition.y) );
    float value;

    if(u_highQuality != 0)
        value = computeHighQuality(z);
    else
        value = computeLowQuality(z);

    out_color =  vec4(
        pow(
            u_brightness + u_contrast * vec3(value, value, value) * vec3(u_r, u_g, u_b),
            vec3(1.0/u_gamma)
        ), 1.0);
}
