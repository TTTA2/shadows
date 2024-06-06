<script lang="ts">
    import { Input } from "$lib/components/ui/input";

    const pattern = ["A", "B"];

    type Log = {
        prefix: string,
        body: string,
    }

    export let log: Log[] = [
        {
            prefix: "A",
            body: "test",
        }
    ];

    let turn = 0;

    const onInput = (e: any) => {

        const index = Number(e.target.dataset.index);

        console.log(index);

        if (index == log.length - 1) {

            turn++;
            if (turn == pattern.length) turn = 0;
            
            log.push({ prefix: pattern[turn], body: "" });
            log = log;
        }
    }

</script>

<div class="h-full overflow-auto p-8">
    {#each log as l, i}
        <div>{l.prefix}</div>
        <div contenteditable data-index={i} on:input={onInput}></div>
    <!-- <Input bind:value={l.body} /> -->
    {/each}
</div>
