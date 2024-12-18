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
    import LevelContainer from "$lib/LevelContainer.svelte";
    import * as wasm from "$lib/pkg/wasm_lib";
    import * as monaco from "monaco-editor";
    import { onMount } from "svelte";
    let playing = $state(false);

    let editorDiv: HTMLDivElement;
    let editor: monaco.editor.IStandaloneCodeEditor | null = $state(null);

    import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
    import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
    import cssWorker from "monaco-editor/esm/vs/language/css/css.worker?worker";
    import htmlWorker from "monaco-editor/esm/vs/language/html/html.worker?worker";
    import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";

    self.MonacoEnvironment = {
        getWorker(_, label) {
            if (label === "json") {
                return new jsonWorker();
            }
            if (label === "css" || label === "scss" || label === "less") {
                return new cssWorker();
            }
            if (
                label === "html" ||
                label === "handlebars" ||
                label === "razor"
            ) {
                return new htmlWorker();
            }
            if (label === "typescript" || label === "javascript") {
                return new tsWorker();
            }
            return new editorWorker();
        },
    };

    onMount(() => {
        editor = monaco.editor.create(editorDiv, {
            value: "",
            language: "null",
            automaticLayout: true,
            theme: "vs-dark",
            detectIndentation: false,
            mouseWheelZoom: true,
            // fontSize: 18,
            fontFamily: "JetBrains Mono",
            // lineHeight: 1.5,
        });
        MonacoEnvironment;
    });
</script>

<div class="editor-container w-[800px] h-[800px]" bind:this={editorDiv}></div>

<button
    class="bg-green-500 text-3xl p-8 m-8"
    onclick={() => {
        if (editor == null) {
            return;
        }
        console.clear();
        wasm.bluh(editor.getValue());
    }}>Run</button
>

<!-- <button
    onclick={() => {
        let gog = performance.now();
        wasm.bluh(`
var a = 0;
while a < 1000000 {
    
    a += 1;
}
    `);
        console.log("glig ", (performance.now() - gog) / 1000);
    }}>Lol</button
>

{#if playing}
    <div class="w-screen h-screen">
        <LevelContainer
            data={{
                song: "courtesy.mp3",
                songStartTimes: [0.06],
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
    <button
        class="text-5xl font-title font-medium m-8 bg-black/50 border-2 border-white p-8 rounded-xl text-white"
        onclick={() => (playing = true)}>Play</button
    >
{/if} -->
