<script lang="ts">
    
    import type { Template } from "../features/templates/templates";

    export let templates: Template[] = [];
    export let selectedId: string | undefined = undefined;


</script>


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

