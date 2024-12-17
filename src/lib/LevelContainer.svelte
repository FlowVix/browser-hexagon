<svelte:options runes />

<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import type { LevelData } from "./level_data";
    import p5 from "p5";
    import { LevelState } from "./level_sketch.svelte";
    import Level from "./Level.svelte";

    let { data }: { data: LevelData } = $props();

    let viewDiv: HTMLDivElement;
    let viewDivSize: [number, number] = $state([0, 0]);

    let onResize = () => {};
    $effect(() => {
        viewDivSize[0];
        viewDivSize[1];
        onResize();
    });

    let levelState: LevelState | null = $state(null);
    onMount(() => {
        new p5((p: p5) => {
            onResize = () => p.resizeCanvas(viewDivSize[0], viewDivSize[1]);

            levelState = new LevelState(p, data);

            p.setup = () => {
                onResize();
                levelState!.setup();
            };
            // p.keyPressed = (e: KeyboardEvent) => {
            //     levelState!.keyPressed(e);
            // };
            // p.keyReleased = (e: KeyboardEvent) => {
            //     levelState!.keyReleased(e);
            // };
            p.touchStarted = () => {
                levelState!.touchStarted();
            };
            p.touchEnded = () => {
                levelState!.touchEnded();
            };
            p.draw = () => {
                levelState!.draw();
            };
        }, viewDiv);
    });
    onDestroy(() => {
        if (levelState != null) {
            levelState.destroy();
        }
    });
</script>

<div class="w-full h-full overflow-hidden">
    {#if levelState != null}
        <Level state={levelState} />
    {/if}
    <div
        class="w-full h-full overflow-hidden"
        bind:this={viewDiv}
        bind:clientWidth={viewDivSize[0]}
        bind:clientHeight={viewDivSize[1]}
    ></div>
</div>
