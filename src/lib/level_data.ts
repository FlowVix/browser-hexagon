export type PatternWall = {
    column: number;
    pos: number;
    size: number;
};
export type Pattern = () => PatternWall[];
export type LevelPatterns = Record<
    number,
    { pattern: Pattern; weight: number }[]
>;
export type LevelData = {
    song: string;
    songStartTimes: number[];

    patterns: LevelPatterns;
    bpm: number;
    beatSize: number;
};

const wall = (column: number, pos: number, size: number): PatternWall => ({
    column,
    pos,
    size,
});
const rotWall = (
    w: PatternWall,
    amount: number,
    sides: number
): PatternWall => ({
    ...w,
    column: (w.column + amount) % sides,
});
const bumpWall = (w: PatternWall, beats: number): PatternWall => ({
    ...w,
    pos: w.pos + beats,
});

export const HEX_0: Pattern = () => [
    wall(0, 0, 1 / 4),
    wall(1, 0, 1 / 4),
    wall(2, 0, 1 / 4),
    wall(3, 0, 1 / 4),
    wall(4, 0, 1 / 4),
];
export const HEX_010: Pattern = () => [
    wall(0, 0, 1 / 4),
    wall(1, 0, 1 / 4),
    wall(2, 0, 1 / 4),
    wall(4, 0, 1 / 4),
];
export const TRI_GAP: Pattern = () => [
    wall(0, 0, 1 / 4),
    wall(2, 0, 1 / 4),
    wall(4, 0, 1 / 4),
];
export const TRI_GAP_DOUBLE: Pattern = () => [
    wall(0, 0, 1 / 4),
    wall(2, 0, 1 / 4),
    wall(4, 0, 1 / 4),
    wall(0, 1 / 2, 1 / 4),
    wall(2, 1 / 2, 1 / 4),
    wall(4, 1 / 2, 1 / 4),
];
export const TRI_GAP_TRIPLE_SWAP: Pattern = () =>
    [0, 1, 2].flatMap(i => TRI_GAP().map(w => bumpWall(rotWall(w, i, 6), i)));
export const HEX_0_FLIP3: Pattern = () => [
    wall(0, 0, 1 / 4),
    wall(1, 0, 1 / 4),
    wall(2, 0, 1 / 4),
    wall(3, 0, 1 / 4),
    wall(4, 0, 1 / 4),

    wall(0, 2, 1 / 4),
    wall(1, 2, 1 / 4),
    wall(3, 2, 1 / 4),
    wall(4, 2, 1 / 4),
    wall(5, 2, 1 / 4),

    wall(0, 4, 1 / 4),
    wall(1, 4, 1 / 4),
    wall(2, 4, 1 / 4),
    wall(3, 4, 1 / 4),
    wall(4, 4, 1 / 4),
];
export const DOUBLE_CLAW: Pattern = () => [
    wall(0, 0, 0.5),
    wall(1, 0, 1),
    wall(2, 0, 1.5),
    wall(3, 0, 1),
    wall(4, 0, 0.5),

    wall(3, 2.5, 0.5),
    wall(4, 2.5, 1),
    wall(5, 2.5, 1.5),
    wall(0, 2.5, 1),
    wall(1, 2.5, 0.5),

    // wall(0, 2, 1 / 4),
    // wall(1, 2, 1 / 4),
    // wall(3, 2, 1 / 4),
    // wall(4, 2, 1 / 4),
    // wall(5, 2, 1 / 4),

    // wall(0, 4, 1 / 4),
    // wall(1, 4, 1 / 4),
    // wall(2, 4, 1 / 4),
    // wall(3, 4, 1 / 4),
    // wall(4, 4, 1 / 4),
];

export const HEX_0_SPIRAL: Pattern = () => {
    let dir = Math.random() < 0.5 ? -1 : 1;

    return [0, 1, 2, 3, 4]
        .map(i => i * dir)
        .flatMap(i =>
            HEX_0().map(w => bumpWall(rotWall(w, i, 6), Math.abs(i)))
        );
};

export const FLIP_FLOP: Pattern = () => {
    let side = [
        wall(0, 0, 3.25),
        wall(1, 0, 0.25),
        wall(2, 1, 0.25),
        wall(1, 2, 0.25),
        wall(2, 3, 0.25),
    ];

    return [...side, ...side.map(w => rotWall(w, 3, 6))];
};
