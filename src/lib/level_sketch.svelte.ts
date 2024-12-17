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
    for (let i = 0; i < 1000; i++) {
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
var fail = new Howl({
    src: ["fail.mp3"],
    html5: true,
});

export class LevelState {
    public song: Howl;

    public sides = 6;

    public viewRot = 0;
    public viewRotSpeed = 0;
    public playerRot = 0;
    public playerTilt = 0;
    public playerTiltTarget = 0;

    public bgDark: Color = [102, 1, 27];
    public bgBright: Color = [127, 1, 34];
    public wallDark: Color = [204, 2, 55];
    public wallBright: Color = [229, 2, 62];
    public depthColor: Color = [102 / 2, 1 / 2, 27 / 2];

    public colorSwap = false;

    public depth = 40;

    public walls: WallList = [];

    public progress = $state(0);

    public movingLeft = false;
    public movingRight = false;

    public nextTick = 0;

    public startTime = 0;
    public runTime = $state(0);

    public dead = $state(false);

    constructor(private p: p5, public data: LevelData) {
        this.song = new Howl({
            src: [data.song],
            html5: true,
        });
    }

    destroy() {
        this.p.remove();
    }

    miterWeight() {
        return CENTER_WEIGHT / Math.cos(Math.PI / this.sides);
    }

    angleFor(i: number) {
        if (i < this.sides) {
            return ((Math.PI * 2) / this.sides) * i;
        }
        return 0;
    }

    forVertex(
        radius: number,
        f: (x: number, y: number, angle: number, i: number) => void
    ) {
        for (let i = 0; i < this.sides; i++) {
            let angle = this.angleFor(i);
            f(Math.cos(angle) * radius, Math.sin(angle) * radius, angle, i);
        }
    }
    edgeColorTrans(i: number) {
        return this.sides == Math.floor(this.sides)
            ? this.sides % 2 == 1 && i == this.sides - 1
                ? 0.5
                : i % 2
            : i % 2;
    }

    forEdge(radius: number, f: (a: Vec2, b: Vec2, i: number) => void) {
        this.forVertex(radius, (x, y, angle, i) => {
            let nextAngle = this.angleFor(i + 1);
            let next: Vec2 = [
                Math.cos(nextAngle) * radius,
                Math.sin(nextAngle) * radius,
            ];

            f([x, y], next, i);
        });
    }

    wallVertices(wall: Wall): [Vec2, Vec2, Vec2, Vec2] {
        let angleA = this.angleFor(wall.column);
        let angleB = this.angleFor(wall.column + 1);

        let cosA = Math.cos(angleA);
        let sinA = Math.sin(angleA);
        let cosB = Math.cos(angleB);
        let sinB = Math.sin(angleB);

        let start = Math.max(wall.pos - this.progress, 5);
        let end = Math.max(wall.pos - this.progress + wall.size, 10);
        return [
            [cosA * start, sinA * start],
            [cosA * end, sinA * end],
            [cosB * end, sinB * end],
            [cosB * start, sinB * start],
        ];
    }
    forWallsSectioned(count: number, f: (w: Wall) => void) {
        for (let i = -1; i < count; i++) {
            let section =
                this.walls[Math.floor(this.progress / SECTION_SIZE) + i];
            if (section != undefined) {
                for (let wall of section) {
                    f(wall);
                }
            }
        }
    }

    restart() {
        this.viewRotSpeed =
            (Math.random() + 0.5) * Math.random() < 0.5 ? -1 : 1;
        this.dead = false;
        this.walls = generateLevel(this.data);
        this.progress = 0;
        this.song.stop();
        // sound.volume(0);
        this.startTime = performance.now() / 1000;
        this.song.seek(
            this.data.songStartTimes[
                Math.floor(Math.random() * this.data.songStartTimes.length)
            ]
        );
        // this.song.volume(0);
        this.song.play();
        this.nextTick = 0;
    }

    setup() {
        this.p.ellipseMode(this.p.RADIUS);
        this.p.frameRate(240);
        this.restart();
    }

    keyPressed(e: KeyboardEvent) {
        if (e.code == "KeyA" || e.code == "ArrowLeft") {
            this.movingLeft = true;
        } else if (e.code == "KeyD" || e.code == "ArrowRight") {
            this.movingRight = true;
        } else if (e.code == "KeyR") {
            this.restart();
        }
    }
    keyReleased(e: KeyboardEvent) {
        if (e.code == "KeyA" || e.code == "ArrowLeft") {
            this.movingLeft = false;
        } else if (e.code == "KeyD" || e.code == "ArrowRight") {
            this.movingRight = false;
        }
    }
    touchStarted() {
        if (this.p.touches.length == 0) {
            return;
        }
        if ((this.p.touches[0] as any).x >= this.p.width / 2) {
            this.movingRight = true;
        } else {
            this.movingLeft = true;
        }
    }
    touchEnded() {
        // console.log(p.touches);
        this.movingRight = false;
        this.movingLeft = false;
    }

    update() {
        let delta = this.p.deltaTime / 1000;

        if (!this.dead) {
            this.runTime = performance.now() / 1000 - this.startTime;

            if (this.runTime >= this.nextTick / (this.data.bpm / 60)) {
                if (this.nextTick % 2 == 0) {
                    this.colorSwap = !this.colorSwap;
                }
                if (this.nextTick % 8 == 0 && this.nextTick != 0) {
                    if (Math.random() < 0.75) {
                        let sign = Math.sign(this.viewRotSpeed);
                        this.viewRotSpeed = Math.random() + 0.5;
                        this.viewRotSpeed *= sign * -1;
                    }
                }
                this.nextTick += 1;
                // tick.play();
            }

            let canMoveLeft = true;
            let canMoveRight = true;
            this.forWallsSectioned(2, wall => {
                let [a, b, c, d] = this.wallVertices(wall);
                let playerPos: [number, number] = [
                    Math.cos(this.playerRot) * PLAYER_HEIGHT,
                    Math.sin(this.playerRot) * PLAYER_HEIGHT,
                ];
                if (
                    pointInTriangle(playerPos, a, b, c) ||
                    pointInTriangle(playerPos, a, c, d)
                ) {
                    this.dead = true;
                    return;
                }

                playerPos = [
                    Math.cos(this.playerRot + delta * 8) * PLAYER_HEIGHT,
                    Math.sin(this.playerRot + delta * 8) * PLAYER_HEIGHT,
                ];
                if (
                    pointInTriangle(playerPos, a, b, c) ||
                    pointInTriangle(playerPos, a, c, d)
                ) {
                    canMoveLeft = false;
                }
                playerPos = [
                    Math.cos(this.playerRot - delta * 8) * PLAYER_HEIGHT,
                    Math.sin(this.playerRot - delta * 8) * PLAYER_HEIGHT,
                ];
                if (
                    pointInTriangle(playerPos, a, b, c) ||
                    pointInTriangle(playerPos, a, c, d)
                ) {
                    canMoveRight = false;
                }
            });
            if (this.dead) {
                fail.play();
                this.song.stop();
                return;
            }

            this.playerTiltTarget = 0;
            if (this.movingLeft && canMoveLeft) {
                this.playerTiltTarget = 0.5;
                this.playerRot += delta * 8;
            }
            if (this.movingRight && canMoveRight) {
                this.playerTiltTarget = -0.5;
                this.playerRot -= delta * 8;
            }
            this.playerTilt = this.p.lerp(
                this.playerTiltTarget,
                this.playerTilt,
                0.5 ** (delta * 50)
            );

            this.progress +=
                (((delta * this.data.bpm) / 60) * this.data.beatSize) / 1;
        } else {
            this.viewRotSpeed = this.p.lerp(
                0,
                this.viewRotSpeed,
                0.5 ** (delta * 3)
            );
        }

        // for (let i = 0; i < walls.length; i++) {
        //     walls[i].pos -= ((delta * data.bpm) / 60) * data.beatSize;
        //     if (walls[i].pos < -50) {
        //         walls.splice(i, 1);
        //         i -= 1;
        //     }
        // }
    }

    draw() {
        let delta = this.p.deltaTime / 1000;

        this.update();

        this.p.background(0);

        this.p.translate(
            Math.floor(this.p.width / 2),
            Math.floor(this.p.height / 2)
        );
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
        this.p.scale(Math.max(this.p.width / 3840, this.p.height / 2160));
        this.p.scale(0.9);
        this.p.scale(1, -0.8);
        this.p.rotate(this.viewRot);

        let depthOffset = vec2(
            -Math.sin(this.viewRot) * this.depth,
            -Math.cos(-this.viewRot) * this.depth
        );

        // pinwheel
        this.p.push();
        this.p.translate(depthOffset[0], depthOffset[1]);
        this.forEdge(2, (a, b, i) => {
            let trans = this.edgeColorTrans(i);
            let color = [
                this.p.lerp(
                    this.bgDark[0],
                    this.bgBright[0],
                    this.colorSwap ? 1 - trans : trans
                ),
                this.p.lerp(
                    this.bgDark[1],
                    this.bgBright[1],
                    this.colorSwap ? 1 - trans : trans
                ),
                this.p.lerp(
                    this.bgDark[2],
                    this.bgBright[2],
                    this.colorSwap ? 1 - trans : trans
                ),
            ];

            this.p.stroke(color);
            this.p.strokeWeight(1);
            this.p.fill(color);

            this.p.beginShape();
            this.p.vertex(a[0], a[1]);
            this.p.vertex(a[0] * 2000, a[1] * 2000);
            this.p.vertex(b[0] * 2000, b[1] * 2000);
            this.p.vertex(b[0], b[1]);
            this.p.endShape();
        });
        this.p.pop();

        // p.noStroke();
        // p.fill(255, 50);
        // p.circle(0, 0, CENTER_RADIUS);

        const drawDepth = (verts: Vec2[]) => {
            for (let i = 0; i < verts.length; i++) {
                let a = verts[i];
                let b = verts[(i + 1) % verts.length];

                this.p.beginShape();
                this.p.vertex(a[0], a[1]);
                this.p.vertex(a[0] + depthOffset[0], a[1] + depthOffset[1]);
                this.p.vertex(b[0] + depthOffset[0], b[1] + depthOffset[1]);
                this.p.vertex(b[0], b[1]);
                this.p.endShape();
            }
        };

        let deg120 = (Math.PI * 2) / 3;

        let playerVerts = [0, 1, 2].map(i =>
            vec2plus(
                vec2polar(this.playerRot, PLAYER_HEIGHT),
                vec2polar(
                    i * deg120 + this.playerRot + this.playerTilt,
                    CENTER_WEIGHT * 1.2
                )
            )
        );

        if (this.depth > 0) {
            this.p.fill(this.depthColor);
            this.p.noStroke();

            // level depth
            this.forWallsSectioned(2, wall => {
                let verts = this.wallVertices(wall);
                drawDepth(verts);
            });

            // center depth

            let centerVerts: Vec2[] = [];
            this.forVertex(CENTER_RADIUS, (x, y) => {
                centerVerts.push(vec2(x, y));
            });
            drawDepth(centerVerts);

            drawDepth(playerVerts);
        }

        // level
        this.forWallsSectioned(2, wall => {
            let trans = this.edgeColorTrans(wall.column);
            let color = [
                this.p.lerp(
                    this.wallDark[0],
                    this.wallBright[0],
                    this.colorSwap ? 1 - trans : trans
                ),
                this.p.lerp(
                    this.wallDark[1],
                    this.wallBright[1],
                    this.colorSwap ? 1 - trans : trans
                ),
                this.p.lerp(
                    this.wallDark[2],
                    this.wallBright[2],
                    this.colorSwap ? 1 - trans : trans
                ),
            ];

            this.p.fill(color);
            this.p.stroke(color);
            this.p.strokeWeight(1);

            this.p.beginShape();
            for (let i of this.wallVertices(wall)) {
                this.p.vertex(i[0], i[1]);
            }
            this.p.endShape();
        });

        // center
        this.p.fill(this.bgDark);
        this.p.stroke(this.wallBright);
        this.p.strokeWeight(CENTER_WEIGHT);

        this.p.beginShape();
        this.forVertex(
            CENTER_RADIUS - this.miterWeight() / 2,
            (x, y, angle) => {
                this.p.vertex(x, y);
            }
        );
        this.p.endShape(this.p.CLOSE);

        // player
        this.p.noStroke();
        this.p.fill(this.wallBright);

        this.p.beginShape();
        for (let i of playerVerts) {
            this.p.vertex(i[0], i[1]);
        }
        this.p.endShape();

        // playerRot += p.deltaTime / 1000;
        this.viewRot += delta * this.viewRotSpeed;

        // p.fill(255);
        // p.noStroke();
        // p.textSize(20);
        // forEdge(CENTER_RADIUS, (a, b, i, trans) => {
        //     p.text(trans, (a[0] + b[0]) / 2, (a[1] + b[1]) / 2);
        // });
    }
}

// export const sketch = (
//     p: p5,
//     startWidth: number,
//     startHeight: number,
//     data: LevelData
// ) => {};
