typedef long PointId;
typedef long TriangleId;

typedef struct VoronoiVertex {
    PointId v[4];
} VoronoiVertex;

typedef struct Triangle {
    PointId vertices[3];
    TriangleId neighbours[3];
    int neighbours_number;
    bool is_visible;
    bool is_finalized;
} Triangle;

typedef struct TriangleFan {
    PointId center;
    int triangle_offset;
    int triangle_number;
    int last_triangle_index;
} TriangleFan;


bool exists(int2 p, int dim) {
    return p.x >= 0 && p.y >= 0 && p.x < dim && p.y < dim;
}

bool is_undefined(int4 c) {
    return any(c <= (int4)-1);
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

int count_shared_points(Triangle triangle0, Triangle triangle1) {
    int shared_points_number = 0;
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            if (triangle0.vertices[i] == triangle1.vertices[j]) {
                shared_points_number++;
            }
        }
    }

    return shared_points_number;
}

bool triangle_contains_point(Triangle triangle, PointId point) {
    for (int i = 0; i < 3; i++) {
        if (triangle.vertices[i] == point) {
            return true;
        }
    }

    return false;
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
            __global Triangle *triangle = &triangles[triangle_offset + i];
            for (int j = 0; j < 3; j++) {
                triangle->vertices[j] = vertex.v[(j + i * 2) % 4];
            }
        }
    }
}

__kernel void calculate_triangle_neighbours(__global Triangle *triangles) {
    __global Triangle *triangle = &triangles[get_global_id(0)];
    Triangle supposed_neighbour = triangles[get_global_id(1)];

    int shared_points_number = count_shared_points(*triangle, supposed_neighbour);

    if (shared_points_number == 2) {
        int neighbour_idx = atomic_inc(&triangle->neighbours_number);
        if (neighbour_idx < 3) {
            triangle->neighbours[neighbour_idx] = get_global_id(1);
        }
    }
}

__kernel void count_triangles_in_fans(
    __global Triangle *triangles,
    __global TriangleFan *triangle_fans
) {
    Triangle triangle = triangles[get_global_id(0)];
    __global TriangleFan *triangle_fan = &triangle_fans[get_global_id(1)];

    if (triangle_contains_point(triangle, triangle_fan->center)) {
        atomic_inc(&triangle_fan->triangle_number);
    }
}

__kernel void calculate_triangle_offset_in_fans(
    __global TriangleFan *triangle_fans,
    __global int *free_triangle_index
) {
    __global TriangleFan *triangle_fan = &triangle_fans[get_global_id(0)];
    triangle_fan->triangle_offset = atomic_add(free_triangle_index, triangle_fan->triangle_number);
}

__kernel void find_triangles_in_fans(
    __global Triangle *triangles,
    __global TriangleFan *triangle_fans,
    __global TriangleId *flatten_triangle_fans
) {
    Triangle triangle = triangles[get_global_id(0)];
    __global TriangleFan *triangle_fan = &triangle_fans[get_global_id(1)];

    if (triangle_contains_point(triangle, triangle_fan->center)) {
        int last_triangle_index = atomic_inc(&triangle_fan->last_triangle_index);
        flatten_triangle_fans[triangle_fan->triangle_offset + last_triangle_index]
            = get_global_id(0);
    }
}