precision highp float;
uniform vec2 u_fractalPosition;
uniform vec2 u_cJulia;
uniform float u_fractalZoom;
uniform float u_brightness;
uniform float u_contrast;
uniform int u_highQuality;

float computeLowQuality(in vec2 z)
{
    const int MAX_ITER = 1024;
    const float N = 2.0;

    vec2 z2;
    float tmp;
    int iterNumber = MAX_ITER;

    // start at 1, since Log(0) = NaN
    for(int i = 1; i < MAX_ITER; i++)
    {
        z2 = vec2(z.x * z.x, z.y * z.y); // z2x = zx * zx; z2y = zy * zy

        if( (z2.x + z2.y) > N*N)
        {
                iterNumber = i;
                break;
        }

        tmp = z2.x - z2.y + u_cJulia.x;
        z.y = 2.0 * z.x * z.y + u_cJulia.y;
        z.x = tmp;
    }

    return log(float(iterNumber + 1));
}

float computeHighQuality(in vec2 z)
{
    const int MAX_ITER = 4096;
    const float N = 4.0;

    vec2 z2;
    float tmp, value;
    int iterNumber = MAX_ITER;

    // start at 1, since Log(0) = NaN
    for(int i = 1; i < MAX_ITER; i++)
    {
        z2 = vec2(z.x * z.x, z.y * z.y); // z2x = zx * zx; z2y = zy * zy

        if( (z2.x + z2.y) > N * N)
        {
                iterNumber = i;
                break;
        }

        tmp = z2.x - z2.y + u_cJulia.x;
        z.y = 2.0 * z.x * z.y + u_cJulia.y;
        z.x = tmp;

    }

    // Smoothing the fractal: result = IterNumber - log2( log( abs(z) / log(N) ) )
    value = float(iterNumber + 1) - log2( log( sqrt( z.x * z.x + z.y * z.y )/log(N)) );
    return log(value);

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

    out_color =  vec4( u_brightness + u_contrast * vec3(value, value, value) * vec3(0.8, 0.75, 1.0), 1.0);
}
