<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { clipboard } from "@tauri-apps/api";

    export let text = "";

    const copy = () => {
        navigator.clipboard.writeText(text);
        console.log("test");
    }

</script>

<div class="b border relative">
    <!-- <div class="flex absolute right-8"> -->
        <Button class="absolute right-8 h-8" size="sm" variant="ghost" on:click={copy}>Copy</Button>
    <!-- </div> -->
    <div>{text}</div>
</div>

<style>

    .b {
        display: flex;
        border-radius: 4px;
        padding: 16px 8px 16px 8px;
        white-space: pre-wrap;
    }

</style>