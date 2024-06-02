<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import * as  Dialog from "$lib/components/ui/dialog";
    import { Textarea } from "$lib/components/ui/textarea";

    export let text = "";
    
    export let onSave = (text: string) => {};

    const onChange = (e: any) => {
        text = e.target.value;
    }

    const onSaveClick = () => {
        onSave(text);
    }

</script>


<div class="grid grid-rows-[auto_minmax(0,1fr)] h-full ">
    <div>
        <Button variant="outline" size="sm" on:click={onSaveClick}>保存</Button>
    </div>
    <div class="p-4">
        <Textarea class="h-full rounded-none resize-none" value={text} on:change={onChange}></Textarea>
    </div>
</div>

<!-- <Dialog.Root>
    <Dialog.Trigger>
        <Button variant="outline">編集</Button>
    </Dialog.Trigger>
    <Dialog.Content>
        <Dialog.Header>
        <Dialog.Title>Are you sure absolutely sure?</Dialog.Title>
        <Dialog.Description>
            <Textarea />
        </Dialog.Description>
        </Dialog.Header>
    </Dialog.Content>
</Dialog.Root> -->