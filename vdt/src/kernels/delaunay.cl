typedef long PointId;

typedef struct VoronoiVertex {
    PointId v[4];
} VoronoiVertex;

typedef struct Triangle {
    PointId vertices[3];
} Triangle;


bool does_exist(int2 p, int dim) {
    return p.x >= 0 && p.y >= 0 && p.x < dim && p.y < dim;
}

bool is_undefined(int4 c) {
    return any(c <= (int4)-1);
}

int4 get_color(read_only image2d_t src, int2 p, int dim) {
    if (!does_exist(p, dim))
        return (int4)(INT_MIN, INT_MIN, -1, -1);

    sampler_t sampler_const =
        CLK_NORMALIZED_COORDS_FALSE |
        CLK_ADDRESS_NONE |
        CLK_FILTER_NEAREST;

    return read_imagei(src, sampler_const, p);
}

int count_triangles_in_vertex(VoronoiVertex *vertex) {
    int same_points_number = 0;
    PointId prev_point = vertex->v[3];
    for (int i = 0; i < 4; i++) {
        PointId current_point = vertex->v[i];
        if (current_point <= -1) return 0;

        if (current_point == prev_point) {
            same_points_number++;
        }

        prev_point = current_point;
    }

    if (same_points_number > 2 || vertex->v[0] == vertex->v[2] || vertex->v[1] == vertex->v[3]) {
        return 0;
    }

    return 2 - same_points_number;
}


__kernel void count_triangles(read_only image2d_t src, __global int *triangle_number) {
    int2 vertex_coords = (int2)(get_global_id(0), get_global_id(1));

    VoronoiVertex vertex = {
        .v = {
            get_color(src, vertex_coords               , get_global_size(0)).z, // bottom left pixel
            get_color(src, vertex_coords + (int2)(1, 0), get_global_size(0)).z, // bottom right pixel
            get_color(src, vertex_coords + (int2)(1, 1), get_global_size(0)).z, // upper right pixel
            get_color(src, vertex_coords + (int2)(0, 1), get_global_size(0)).z, // upper left pixel
        }
    };

    atomic_add(triangle_number, count_triangles_in_vertex(&vertex));
}

__kernel void build_triangles(
    read_only image2d_t src, __global Triangle *triangles, __global int *free_triangle)
{
    int2 vertex_coords = (int2)(get_global_id(0), get_global_id(1));

    VoronoiVertex vertex = {
        .v = {
            get_color(src, vertex_coords               , get_global_size(0)).z, // bottom left pixel
            get_color(src, vertex_coords + (int2)(1, 0), get_global_size(0)).z, // bottom right pixel
            get_color(src, vertex_coords + (int2)(1, 1), get_global_size(0)).z, // upper right pixel
            get_color(src, vertex_coords + (int2)(0, 1), get_global_size(0)).z, // upper left pixel
        }
    };

    int triangle_number = count_triangles_in_vertex(&vertex);
    int triangle_offset = atomic_add(free_triangle, triangle_number);

    if (triangle_number == 1) {
        __global PointId *current_triangle_vertex = triangles[triangle_offset].vertices;

        PointId prev_v = vertex.v[3];
        for (int i = 0; i < 4; i++) {
            PointId curr_v = vertex.v[i];
            if (curr_v != prev_v) {
                *current_triangle_vertex = curr_v;

                current_triangle_vertex++;
            }

            prev_v = curr_v;
        }
    } else if (triangle_number == 2) {
        for (int i = 0; i < 2; i++) {
            triangles[triangle_offset + i] = (Triangle) {
                .vertices = { vertex.v[i], vertex.v[i + 1], vertex.v[i + 2] },
            };
        }
    }
}