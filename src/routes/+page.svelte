<script lang="ts">
    import {
        DOUBLE_CLAW,
        FLIP_FLOP,
        HEX_0,
        HEX_010,
        HEX_0_FLIP3,
        HEX_0_SPIRAL,
        TRI_GAP,
        TRI_GAP_DOUBLE,
        TRI_GAP_TRIPLE_SWAP,
    } from "$lib/level_data";
    import LevelPlay from "$lib/LevelPlay.svelte";
    let playing = $state(false);
</script>

{#if playing}
    <div class="w-screen h-screen">
        <LevelPlay
            data={{
                song: "courtesy.mp3",
                songStartTimes: [0.06, 51.76, 1 * 60 + 39.7],
                patterns: {
                    6: [
                        { pattern: HEX_0, weight: 2 },
                        { pattern: HEX_010, weight: 2 },
                        { pattern: TRI_GAP, weight: 2 },
                        { pattern: HEX_0_FLIP3, weight: 1 },
                        { pattern: TRI_GAP_DOUBLE, weight: 2 },
                        { pattern: TRI_GAP_TRIPLE_SWAP, weight: 2 },
                        { pattern: DOUBLE_CLAW, weight: 1 },
                        { pattern: HEX_0_SPIRAL, weight: 1 },
                        { pattern: FLIP_FLOP, weight: 1 },
                    ],
                },
                bpm: 130,
                beatSize: 600,
            }}
        />
    </div>
{:else}
    <button class="text-9xl" onclick={() => (playing = true)}>Play</button>
{/if}
