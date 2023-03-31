// int4 color
// color.x - x coordinate of the closest site
// color.y - y coordinate of the closest site
// color.z - index of the closest site
// color.w - is color undefined (-1)

bool exists(int2 p, int dim) {
    return p.x >= 0 && p.y >= 0 && p.x < dim && p.y < dim;
}

bool is_undefined(int4 c) {
    return any(c <= (int4)-1);
}

float dist(int2 p, int2 s) {
    int x = s.x - p.x;
    int y = s.y - p.y;
    return sqrt((float)(x * x + y * y));
}

int2 nearest_site(int2 p, int2 s0, int2 s1) {
    return (dist(p, s0) < dist(p, s1)) ? s0 : s1;
}

int4 get_color(read_only image2d_t src, int2 p, int dim) {
    if (!exists(p, dim))
        return (int4)(-dim * 10, -dim * 10, -1, -1);

    sampler_t sampler_const =
        CLK_NORMALIZED_COORDS_FALSE |
        CLK_ADDRESS_NONE |
        CLK_FILTER_NEAREST;

    return read_imagei(src, sampler_const, p);
}


__kernel void plot_sites(__global float2 *sites, write_only image2d_t dst) {
    int2 site = convert_int2_rtz(sites[get_global_id(0)]);
    int4 color = (int4)(site.xy, get_global_id(0), 1);

    write_imagei(dst, site, color);
}

__kernel void fill_voronoi(read_only image2d_t src, write_only image2d_t dst, int k) {
    int2 coords = (int2)(get_global_id(0), get_global_id(1));
    int dim = get_global_size(0);

    int4 p = get_color(src, coords, dim);

    for (int i = -k; i <= k; i += k) {
        for (int j = -k; j <= k; j += k) {
            int4 q = get_color(src, coords + (int2)(i, j), dim);
            if (!is_undefined(q) && (is_undefined(p) || dist(coords, p.xy) > dist(coords, q.xy))) {
                p = q;
            }
        }
    }

    write_imagei(dst, coords, p);
}

__kernel void conquer_islands(
    read_only image2d_t src, write_only image2d_t dst, __global int *changed_number)
{
    int2 directions[4][3] = {
        { (int2)( 1,  0), (int2)( 1,  1), (int2)( 0,  1) },
        { (int2)( 0,  1), (int2)(-1,  1), (int2)(-1,  0) },
        { (int2)(-1,  0), (int2)(-1, -1), (int2)( 0, -1) },
        { (int2)( 0, -1), (int2)( 1, -1), (int2)( 1,  0) },
    };
    int dim = get_global_size(0);

    int2 p = (int2)(get_global_id(0), get_global_id(1));

    int4 p_color = get_color(src, p, dim);
    int2 s = p_color.xy;

    if (all(p == s)) return;

    int quadrant = -1;
    if (s.x >= p.x && s.y >= p.y) {
        quadrant = 0;
    } else if (s.x < p.x && s.y >= p.y) {
        quadrant = 1;
    } else if (s.x < p.x && s.y < p.y) {
        quadrant = 2;
    } else if (s.x >= p.x && s.y < p.y) {
        quadrant = 3;
    }

    int2 nearest_s = (int2)(-dim * 10, -dim * 10);
    for (int i = 0; i < 3; i++) {
        int2 q = p + directions[quadrant][i];
        int4 q_color = get_color(src, q, dim);

        if (all(p_color == q_color)) {
            write_imagei(dst, p, p_color);

            return;
        }

        nearest_s = nearest_site(p, nearest_s, q_color.xy);
    }

    write_imagei(dst, p, get_color(src, nearest_s, dim));

    atomic_inc(changed_number);
}