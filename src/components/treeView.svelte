<script lang="ts">

    import { TemplateDicitonary, getTemplatesList, hasChildren, type Template } from "../features/templates/templates";
    import TreeViewItem from "./treeViewItem.svelte";

    export let templates: TemplateDicitonary | undefined;
    export let nodeStates = new Map<string, boolean>();

    export let onSelectNode = (targetId: string) => {};

    const onExpand = (targetId: string, newState: boolean) => {
        nodeStates.set(targetId, newState);
    }

    const onSelect = (targetId: string) => {
        onSelectNode(targetId);
    }

</script>


<div class="w-full h-full p-2 overflow-auto">
    {#if templates} 
        {#each (templates.values()) as a }
            {#if a.parentId == ""}
                <TreeViewItem id={a.id} {...{templates, nodeStates, onExpand, onSelect}}></TreeViewItem>
            {/if}
        {/each}
    {/if}
</div>