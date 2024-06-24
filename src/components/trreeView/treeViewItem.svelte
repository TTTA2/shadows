<script lang="ts">
    
    import { TemplateDicitonary, getChildren, hasChildren, type Template } from "../../features/templates/templates";
    
    export let templates: TemplateDicitonary = new TemplateDicitonary();
    export let nodeStates: Map<string, boolean> | undefined;

    export let id: string = "";
    export let expanded = nodeStates?.get(id) || false;

    export let selectedId: string | undefined = undefined;

    export let onExpand = (targetId: string, newState: boolean) => {};
    export let onSelect = (targetId: string) => {};

    $: hasChild = hasChildren(id, templates);
    $: children = getChildren(id, templates).sort((a, b) => a.name.localeCompare(b.name));

    const onClick = () => onSelect(id);
        
    const toggle = () => {
        expanded = !expanded;
        onExpand(id, expanded);
    }
    
    const getName = () => templates.get(id)?.name;

</script>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />

<div class="item">

    {#if selectedId == id}
        <button class="ofwaw inline-flex items-center ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-accent text-accent-foreground h-9 rounded-md px-3 justify-start" on:dblclick={toggle} on:click={onClick}>
            {#if hasChild } 
                {#if !expanded} 
                    <span class="material-symbols-outlined">keyboard_arrow_down</span>
                {:else}
                    <span class="material-symbols-outlined">keyboard_arrow_right</span>
                {/if}
            {:else} 
                <span class="material-symbols-outlined">draft</span>
            {/if}    
            <span>{getName()}</span>
        </button>
    {:else}

        <button class="ofwaw inline-flex items-center ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 rounded-md px-3 justify-start" on:dblclick={toggle} on:click={onClick}>
            {#if hasChild } 
                {#if !expanded} 
                    <span class="material-symbols-outlined">keyboard_arrow_down</span>
                {:else}
                    <span class="material-symbols-outlined">keyboard_arrow_right</span>
                {/if}
            {:else} 
                <span class="material-symbols-outlined">draft</span>
            {/if}    
            <span>{getName()}</span>
        </button>

    {/if}


    {#if expanded && hasChild}
        <div class="chidlren">
            {#each (children) as a }
                <svelte:self {...{id: a.id, selectedId, templates, nodeStates, onExpand, onSelect }}></svelte:self>
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

    .material-symbols-outlined {
    font-variation-settings:
    'FILL' 0,
    'wght' 400,
    'GRAD' 0,
    'opsz' 24
    }

</style>