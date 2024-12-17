export type Vec2 = [number, number];

export const vec2 = (x: number, y: number): Vec2 => [x, y];
export const vec2polar = (angle: number, length: number): Vec2 =>
    vec2(Math.cos(angle) * length, Math.sin(angle) * length);
export const vec2plus = (a: Vec2, b: Vec2) => vec2(a[0] + b[0], a[1] + b[1]);

const vsign = (p1: Vec2, p2: Vec2, p3: Vec2): number =>
    (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1]);

export const pointInTriangle = (
    pt: Vec2,
    v1: Vec2,
    v2: Vec2,
    v3: Vec2
): boolean => {
    let [d1, d2, d3] = [
        vsign(pt, v1, v2),
        vsign(pt, v2, v3),
        vsign(pt, v3, v1),
    ];

    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;

    return !(has_neg && has_pos);
};

const pad = (s: string, c: string, len: number) =>
    c.repeat(Math.max(len - s.length, 0)) + s;
export const fmtTime = (time: number) => [
    pad(`${Math.floor(time / 60)}`, "0", 2),
    pad(`${Math.floor(time % 60)}`, "0", 2),
];
