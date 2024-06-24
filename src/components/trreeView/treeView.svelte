<script lang="ts">

    import { TemplateDicitonary, getTemplatesList, hasChildren, type Template } from "../../features/templates/templates";
    import TreeViewItem from "./treeViewItem.svelte";

    export let selectedId: string | undefined = undefined;

    export let templates: TemplateDicitonary | undefined;
    export let nodeStates = new Map<string, boolean>();

    export let onSelectNode = (targetId: string | undefined) => {};

    let clickId = { id1: "", id2: "" }

    const onExpand = (targetId: string, newState: boolean) => {
        nodeStates.set(targetId, newState);
    }

    const onSelect = (targetId: string) => {
        onSelectNode(targetId);

        selectedId = targetId;

        clickId.id1 = crypto.randomUUID();

        console.log("A");
    }

    const onClick = () => {

        if (clickId.id1 == "") {
            onSelectNode(undefined);
            selectedId = undefined
        }

        clickId.id1 = "";

        // selectedId = undefined;
        console.log(templates);
    }

</script>

<div class="w-full h-full p-2 overflow-auto" on:click={onClick}>
    {#if templates} 
        {#each (templates.values()) as a }
            {#if a.parentId == ""}
                <TreeViewItem id={a.id} bind:templates={templates} {...{ selectedId, nodeStates, onExpand, onSelect}}></TreeViewItem>
            {/if}
        {/each}
    {/if}
</div>