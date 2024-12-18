<svelte:options runes />

<script lang="ts">
    import { onMount } from "svelte";
    import type { LevelState } from "./level_sketch.svelte";
    import { fmtTime } from "./util";
    import { fade } from "svelte/transition";

    let { state }: { state: LevelState } = $props();

    let [mins, secs] = $derived(fmtTime(state.runTime));

    let controlDiv: HTMLDivElement;
    onMount(() => {
        controlDiv.focus();
    });
</script>

<svelte:window
    onkeyup={e => {
        if (state != null) {
            state.keyReleased(e);
        }
    }}
/>

<div class="w-full h-full overflow-hidden absolute">
    <div
        class="w-full h-full flex flex-col justify-between absolute pointer-events-none"
    >
        <div
            class="w-[300px] h-[100px] bg-black/50 text-6xl text-white font-title font-medium flex items-center backdrop-blur-3xl rounded-br-3xl"
        >
            <span class="w-full text-right">{mins}</span>
            <span>:</span>
            <span class="w-full">{secs}</span>
        </div>
        {#if state.dead}
            <div
                class="w-[400px] h-[80px] bg-black/50 text-4xl text-white font-title font-medium flex justify-center items-center backdrop-blur-3xl rounded-tr-3xl"
            >
                Press R to restart!
            </div>
        {/if}
    </div>

    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div
        class="w-full h-full outline-none"
        tabindex={0}
        onkeydown={e => {
            // console.log("jn");
            if (state != null) {
                state.keyPressed(e);
            }
        }}
        bind:this={controlDiv}
    ></div>
</div>
