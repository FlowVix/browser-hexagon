<svelte:options runes />

<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import type { LevelData } from "./level_data";
    import p5 from "p5";
    import { sketch } from "./level_sketch";

    let { data }: { data: LevelData } = $props();

    let viewDiv: HTMLDivElement;
    let viewDivSize: [number, number] = $state([0, 0]);

    let onResize = () => {};
    $effect(() => {
        viewDivSize[0];
        viewDivSize[1];
        onResize();
    });

    let p: p5;
    onMount(() => {
        p = new p5((p: p5) => {
            onResize = () => p.resizeCanvas(viewDivSize[0], viewDivSize[1]);

            sketch(p, viewDivSize[0], viewDivSize[1], data);
        }, viewDiv);
    });
    onDestroy(() => {
        p.remove();
    });
</script>

<div
    class="w-full h-full overflow-hidden"
    bind:this={viewDiv}
    bind:clientWidth={viewDivSize[0]}
    bind:clientHeight={viewDivSize[1]}
></div>
