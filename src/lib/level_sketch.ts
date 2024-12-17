import type p5 from "p5";
import type { LevelData, Pattern, PatternWall } from "./level_data";
import { pointInTriangle, vec2, vec2plus, vec2polar, type Vec2 } from "./util";
import { Howl } from "howler";

type Color = [number, number, number];

const CENTER_RADIUS = 200;
const CENTER_WEIGHT = CENTER_RADIUS / 8;
const PLAYER_HEIGHT = 240;

const SECTION_SIZE = 4000;

type Wall = {
    column: number;
    pos: number;
    size: number;
};
type WallList = Record<number, Wall[]>;

const generateLevel = (data: LevelData): WallList => {
    let transformedPatterns: Record<
        number,
        { total: number; patterns: { pattern: Pattern; threshold: number }[] }
    > = {};
    for (let [k, v] of Object.entries(data.patterns)) {
        let transformed: (typeof transformedPatterns)[keyof typeof transformedPatterns] =
            { total: 0, patterns: [] };
        let threshold = 0;
        for (let i of v) {
            transformed.total += i.weight;
            threshold += i.weight;
            transformed.patterns.push({
                pattern: i.pattern,
                threshold,
            });
        }
        transformedPatterns[parseInt(k)] = transformed;
    }

    let outWalls: WallList = {};
    const addWall = (w: Wall) => {
        let section = Math.floor(w.pos / SECTION_SIZE);
        if (outWalls[section] == undefined) {
            outWalls[section] = [];
        }
        outWalls[section].push(w);
    };

    let beat = 4;
    let sides = 6;
    const transformWall = (wall: PatternWall): Wall => ({
        column: wall.column,
        pos: wall.pos * data.beatSize,
        size: wall.size * data.beatSize,
    });
    for (let i = 0; i < 100; i++) {
        let patterns = transformedPatterns[sides];
        let rand = Math.random() * patterns.total;
        let randRot = Math.floor(Math.random() * sides);

        let beatNext = beat;

        for (let p of patterns.patterns) {
            if (rand <= p.threshold) {
                let walls = p.pattern();
                for (let w of walls) {
                    w.pos += beat;

                    let jump = Math.ceil(w.pos + w.size + 1.1);
                    if (beatNext < jump) {
                        beatNext = jump;
                    }

                    // console.log(w.pos + w.size);
                    let tw = transformWall(w);
                    tw.column = (tw.column + randRot) % sides;
                    addWall(tw);
                }
                break;
            }
        }
        beat = beatNext;
        // let totalWeight =
    }
    // console.log(outWalls);
    return outWalls;
};

var tick = new Howl({
    src: ["tick.mp3"],
    html5: true,
});

export const sketch = (
    p: p5,
    startWidth: number,
    startHeight: number,
    data: LevelData
) => {
    let song = new Howl({
        src: [data.song],
        html5: true,
    });

    // const TICK_TIME = 1 / (data.bpm / 60);

    let sides = 6;

    let viewRot = 0;
    let viewRotSpeed = Math.random() + 0.5;
    if (Math.random() < 0.5) {
        viewRotSpeed *= -1;
    }

    let playerRot = 0;

    let bgDark: Color = [102, 1, 27];
    let bgBright: Color = [127, 1, 34];
    let wallDark: Color = [204, 2, 55];
    let wallBright: Color = [229, 2, 62];
    let depthColor: Color = [102 / 2, 1 / 2, 27 / 2];

    let colorSwap = false;

    let depth = 40;

    let walls: WallList = [];

    let progress = 0;

    let movingLeft = false;
    let movingRight = false;

    let nextTick = 0;

    let startSeek = 0;

    // let pulse = 0.9;

    const miterWeight = () => CENTER_WEIGHT / Math.cos(Math.PI / sides);

    const angleFor = (i: number) => {
        if (i < sides) {
            return ((Math.PI * 2) / sides) * i;
        }
        return 0;
    };

    const forVertex = (
        radius: number,
        f: (x: number, y: number, angle: number, i: number) => void
    ) => {
        for (let i = 0; i < sides; i++) {
            let angle = angleFor(i);
            f(Math.cos(angle) * radius, Math.sin(angle) * radius, angle, i);
        }
    };
    const edgeColorTrans = (i: number) =>
        sides == Math.floor(sides)
            ? sides % 2 == 1 && i == sides - 1
                ? 0.5
                : i % 2
            : i % 2;
    const forEdge = (
        radius: number,
        f: (a: Vec2, b: Vec2, i: number) => void
    ) => {
        forVertex(radius, (x, y, angle, i) => {
            let nextAngle = angleFor(i + 1);
            let next: Vec2 = [
                Math.cos(nextAngle) * radius,
                Math.sin(nextAngle) * radius,
            ];

            f([x, y], next, i);
        });
    };

    const wallVertices = (wall: Wall): [Vec2, Vec2, Vec2, Vec2] => {
        let angleA = angleFor(wall.column);
        let angleB = angleFor(wall.column + 1);

        let cosA = Math.cos(angleA);
        let sinA = Math.sin(angleA);
        let cosB = Math.cos(angleB);
        let sinB = Math.sin(angleB);

        let start = Math.max(wall.pos - progress, 5);
        let end = Math.max(wall.pos - progress + wall.size, 10);
        return [
            [cosA * start, sinA * start],
            [cosA * end, sinA * end],
            [cosB * end, sinB * end],
            [cosB * start, sinB * start],
        ];
    };
    const forWallsSectioned = (count: number, f: (w: Wall) => void) => {
        for (let i = -1; i < count; i++) {
            let section = walls[Math.floor(progress / SECTION_SIZE) + i];
            if (section != undefined) {
                for (let wall of section) {
                    f(wall);
                }
            }
        }
    };

    const restart = () => {
        walls = generateLevel(data);
        progress = 0;
        song.stop();
        // sound.volume(0);
        startSeek =
            data.songStartTimes[
                Math.floor(Math.random() * data.songStartTimes.length)
            ];
        song.seek(startSeek);
        song.play();
        nextTick = 0;
    };

    p.setup = () => {
        p.resizeCanvas(startWidth, startHeight);
        p.ellipseMode(p.RADIUS);
        p.frameRate(240);
        restart();
    };

    p.keyPressed = (e: KeyboardEvent) => {
        if (e.key == "a" || e.key == "ArrowLeft") {
            movingLeft = true;
        } else if (e.key == "d" || e.key == "ArrowRight") {
            movingRight = true;
        }
    };
    p.keyReleased = (e: KeyboardEvent) => {
        if (e.key == "a" || e.key == "ArrowLeft") {
            movingLeft = false;
        } else if (e.key == "d" || e.key == "ArrowRight") {
            movingRight = false;
        }
    };

    const update = () => {
        let delta = p.deltaTime / 1000;

        if (song.seek() - startSeek >= nextTick / (data.bpm / 60)) {
            console.log(nextTick);
            if (nextTick % 2 == 0) {
                colorSwap = !colorSwap;
            }
            if (nextTick % 8 == 0 && nextTick != 0) {
                console.log("gagaga");
                if (Math.random() < 0.75) {
                    let sign = Math.sign(viewRotSpeed);
                    viewRotSpeed = Math.random() + 0.5;
                    viewRotSpeed *= sign * -1;
                }
            }
            nextTick += 1;
            // tick.play();
        }

        let died = false;
        let canMoveLeft = true;
        let canMoveRight = true;
        forWallsSectioned(2, wall => {
            let [a, b, c, d] = wallVertices(wall);
            let playerPos: [number, number] = [
                Math.cos(playerRot) * PLAYER_HEIGHT,
                Math.sin(playerRot) * PLAYER_HEIGHT,
            ];
            if (
                pointInTriangle(playerPos, a, b, c) ||
                pointInTriangle(playerPos, a, c, d)
            ) {
                died = true;
                return;
            }

            playerPos = [
                Math.cos(playerRot + delta * 8) * PLAYER_HEIGHT,
                Math.sin(playerRot + delta * 8) * PLAYER_HEIGHT,
            ];
            if (
                pointInTriangle(playerPos, a, b, c) ||
                pointInTriangle(playerPos, a, c, d)
            ) {
                canMoveLeft = false;
            }
            playerPos = [
                Math.cos(playerRot - delta * 8) * PLAYER_HEIGHT,
                Math.sin(playerRot - delta * 8) * PLAYER_HEIGHT,
            ];
            if (
                pointInTriangle(playerPos, a, b, c) ||
                pointInTriangle(playerPos, a, c, d)
            ) {
                canMoveRight = false;
            }
        });
        if (died) {
            restart();
            return;
        }

        if (movingLeft && canMoveLeft) {
            playerRot += delta * 8;
        }
        if (movingRight && canMoveRight) {
            playerRot -= delta * 8;
        }

        progress += (((delta * data.bpm) / 60) * data.beatSize) / 1;

        // for (let i = 0; i < walls.length; i++) {
        //     walls[i].pos -= ((delta * data.bpm) / 60) * data.beatSize;
        //     if (walls[i].pos < -50) {
        //         walls.splice(i, 1);
        //         i -= 1;
        //     }
        // }
    };

    p.draw = () => {
        let delta = p.deltaTime / 1000;

        update();

        p.background(0);

        p.translate(Math.floor(p.width / 2), Math.floor(p.height / 2));
        // p.scale(
        //     p.lerp(
        //         0.9,
        //         0.92,
        //         1 -
        //             (Math.cos(
        //                 ((sound.seek() - data.tickDelay - 0.15) / TICK_TIME) *
        //                     2 *
        //                     Math.PI
        //             ) /
        //                 2 +
        //                 0.5) **
        //                 0.5
        //     )
        // );
        p.scale(Math.max(p.width / 3840, p.height / 2160));
        p.scale(0.9);
        p.scale(1, -0.8);
        p.rotate(viewRot);

        let depthOffset = vec2(
            -Math.sin(viewRot) * depth,
            -Math.cos(-viewRot) * depth
        );

        // pinwheel
        p.push();
        p.translate(depthOffset[0], depthOffset[1]);
        forEdge(2, (a, b, i) => {
            let trans = edgeColorTrans(i);
            let color = [
                p.lerp(bgDark[0], bgBright[0], colorSwap ? 1 - trans : trans),
                p.lerp(bgDark[1], bgBright[1], colorSwap ? 1 - trans : trans),
                p.lerp(bgDark[2], bgBright[2], colorSwap ? 1 - trans : trans),
            ];

            p.stroke(color);
            p.strokeWeight(1);
            p.fill(color);

            p.beginShape();
            p.vertex(a[0], a[1]);
            p.vertex(a[0] * 2000, a[1] * 2000);
            p.vertex(b[0] * 2000, b[1] * 2000);
            p.vertex(b[0], b[1]);
            p.endShape();
        });
        p.pop();

        // p.noStroke();
        // p.fill(255, 50);
        // p.circle(0, 0, CENTER_RADIUS);

        const drawDepth = (verts: Vec2[]) => {
            for (let i = 0; i < verts.length; i++) {
                let a = verts[i];
                let b = verts[(i + 1) % verts.length];

                p.beginShape();
                p.vertex(a[0], a[1]);
                p.vertex(a[0] + depthOffset[0], a[1] + depthOffset[1]);
                p.vertex(b[0] + depthOffset[0], b[1] + depthOffset[1]);
                p.vertex(b[0], b[1]);
                p.endShape();
            }
        };

        let deg120 = (Math.PI * 2) / 3;
        let playerVerts = [0, 1, 2].map(i =>
            vec2plus(
                vec2polar(playerRot, PLAYER_HEIGHT),
                vec2polar(i * deg120 + playerRot, CENTER_WEIGHT * 1.2)
            )
        );

        if (depth > 0) {
            p.fill(depthColor);
            p.noStroke();

            // level depth
            forWallsSectioned(2, wall => {
                let verts = wallVertices(wall);
                drawDepth(verts);
            });

            // center depth

            let centerVerts: Vec2[] = [];
            forVertex(CENTER_RADIUS, (x, y) => {
                centerVerts.push(vec2(x, y));
            });
            drawDepth(centerVerts);

            drawDepth(playerVerts);
        }

        // level
        forWallsSectioned(2, wall => {
            let trans = edgeColorTrans(wall.column);
            let color = [
                p.lerp(
                    wallDark[0],
                    wallBright[0],
                    colorSwap ? 1 - trans : trans
                ),
                p.lerp(
                    wallDark[1],
                    wallBright[1],
                    colorSwap ? 1 - trans : trans
                ),
                p.lerp(
                    wallDark[2],
                    wallBright[2],
                    colorSwap ? 1 - trans : trans
                ),
            ];

            p.fill(color);
            p.stroke(color);
            p.strokeWeight(1);

            p.beginShape();
            for (let i of wallVertices(wall)) {
                p.vertex(i[0], i[1]);
            }
            p.endShape();
        });

        // center
        p.fill(bgDark);
        p.stroke(wallBright);
        p.strokeWeight(CENTER_WEIGHT);

        p.beginShape();
        forVertex(CENTER_RADIUS - miterWeight() / 2, (x, y, angle) => {
            p.vertex(x, y);
        });
        p.endShape(p.CLOSE);

        // player
        p.noStroke();
        p.fill(wallBright);

        p.beginShape();
        for (let i of playerVerts) {
            p.vertex(i[0], i[1]);
        }
        p.endShape();

        // playerRot += p.deltaTime / 1000;
        viewRot += delta * viewRotSpeed;

        // p.fill(255);
        // p.noStroke();
        // p.textSize(20);
        // forEdge(CENTER_RADIUS, (a, b, i, trans) => {
        //     p.text(trans, (a[0] + b[0]) / 2, (a[1] + b[1]) / 2);
        // });
    };
};
