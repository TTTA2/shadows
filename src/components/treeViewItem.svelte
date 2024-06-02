<script lang="ts">
    import { TemplateDicitonary, getChildren, hasChildren, type Template } from "../features/templates/templates";
    
    export let templates: TemplateDicitonary = new TemplateDicitonary();
    export let nodeStates: Map<string, boolean> | undefined;

    export let id: string = "";
    export let expanded = nodeStates?.get(id) || false;

    export let onExpand = (targetId: string, newState: boolean) => {};
    export let onSelect = (targetId: string) => {};

    const hasChild = hasChildren(id, templates);

    const onClick = () => onSelect(id);
        
    const toggle = () => {
        expanded = !expanded;
        onExpand(id, expanded);
    }
    
    const getName = () => templates.get(id)?.name;

</script>

<div class="item">

    <button class="ofwaw inline-flex items-center ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 rounded-md px-3 justify-start" on:dblclick={toggle} on:click={onClick}>
        {#if hasChild } <span>{ !expanded ? "+" : "-"}</span> {:else} <span>@</span> {/if}    
        <span>{getName()}</span>
    </button>

    {#if expanded && hasChild}
        <div class="chidlren">
            {#each (getChildren(id, templates)) as a }
                <svelte:self {...{id: a.id, templates, nodeStates, onExpand, onSelect }}></svelte:self>
            {/each}
        </div>
    {/if}

</div>

<style>

    .item {
        /* display: flex; */
        /* font-size: 0.875rem; */
        /* line-height: 1.25rem; */
        font-weight: 500;
    }

    button {
        display: flex;
        user-select: none;
        text-align: left;
    }
    
    .chidlren {
        margin-left: 32px;
    }

    .ofwaw {
        /* overflow-wrap: anywhere; */
    }

</style>