<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { invoke } from "@tauri-apps/api/tauri";
    import * as Tabs from "$lib/components/ui/tabs/index";
    import * as Card from "$lib/components/ui/card/index";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import TreeView from "../components/treeView.svelte";
    import * as Menubar from "$lib/components/ui/menubar";
    import * as Resizable from "$lib/components/ui/resizable";

    import {
        TemplateDicitonary,
        getTemplatesList,
        saveTemplate,
        type Template,
    } from "../features/templates/templates";
    
    import PlainTextView from "../components/plainTextView.svelte";
    import TemplateEditor from "../components/dialog/templateEditor.svelte";
    import { Switch } from "$lib/components/ui/switch";
    import { checkUpdate } from "@tauri-apps/api/updater";

    let templates = new TemplateDicitonary();
    let selectedTemplateId: string | undefined = undefined;

    let isEditMode = true;

    $: selectedTemplate = selectedTemplateId ? templates?.get(selectedTemplateId) : undefined;

    const onSelectNode = (targetId: string) => {
        selectedTemplateId = targetId;
        console.log(targetId);
    };

    const onSave = (text: string) => {
        
        if (!selectedTemplateId) return;
        
        const template = templates.get(selectedTemplateId);

        if (template) {
            template.body = text;
            saveTemplate(template, templates);
        }
    }

    const onNewTemplate = () => {

        const temp: Template =  { id: crypto.randomUUID(), body: "", name: "新規テンプレート", parentId: "" };
        saveTemplate(temp, templates);

        templates.set(temp.id, temp);
        templates = templates;
        // templates = new TemplateDicitonary([...Array.from(templates), [temp.id, temp]]);
    }

    const onNewChildTemplate = () => {

        console.log(selectedTemplateId);

        if (!selectedTemplateId) return;

        const temp: Template =  { id: crypto.randomUUID(), body: "", name: "新規子テンプレート", parentId: selectedTemplateId };
        saveTemplate(temp, templates);

        templates.set(temp.id, temp);
        // templates = templates;
        templates = new TemplateDicitonary([...Array.from(templates), [temp.id, temp]]);
        

        // templates.set(temp.id, temp);
        // templates = new TemplateDicitonary(templates);
    }

    const onToggleEdit = () => {
        isEditMode = !isEditMode;
    }

    //こっちじゃなくてもいい？
    // getTemplatesList().then(t => templates = new TemplateDicitonary(t));
    getTemplatesList().then((t) => (templates = t));

</script>

<div class="app_struct">
    <div class="border-b">
        <nav class="flex h-16 items-center px-4 gap-10 justify-between">
            <a
                href="#"
                class="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
                >Shadow</a
            >
                <div class="flex w-120 gap-[32px]">
                <!-- <Button class="pl-[16px] pr-[128px] text-left" variant="outline">{}</Button> -->
                <Button class="pl-[16px] pr-[128px] text-left" variant="outline">キーワード検索...</Button>

                <div>
                    <div class="flex items-center space-x-2">
                        <Switch id="airplane-mode" onCheckedChange={onToggleEdit}/>
                        <Label for="airplane-mode">編集モード</Label>
                    </div>
        
                    {#if !isEditMode} 
                        <Button variant="outline" on:click={onNewTemplate}>新規</Button>
                        <Button variant="outline" on:click={onNewChildTemplate}>新規子</Button>
                    {/if}
                </div>

            </div>
        </nav>
    </div>

    <div class="container">

        <Resizable.PaneGroup direction="horizontal">
            <Resizable.Pane defaultSize={20}>
                <TreeView bind:templates={templates} {...{ onSelectNode }}></TreeView>
                <!-- <TreeView bind:templates={templates} {...{ templates, onSelectNode }}></TreeView> -->
            </Resizable.Pane>
            <Resizable.Handle withHandle  />
            <Resizable.Pane defaultSize={20}>
                <!-- <div class="border-r"></div> -->
            </Resizable.Pane>
            <Resizable.Handle withHandle  />
            <Resizable.Pane>

                {#if isEditMode} 
                    <PlainTextView source={selectedTemplate?.body} />
                {:else}    
                    <TemplateEditor text={selectedTemplate?.body} {onSave }></TemplateEditor>
                {/if}

            </Resizable.Pane>
          </Resizable.PaneGroup>

    </div>


</div>

<!-- <form class="row" on:submit|preventDefault={greet}>
        <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
        <button type="submit">Greet</button>
      </form> -->

<style>
    .app_struct {
        overflow: hidden;
        display: grid;
        grid-template-rows: auto minmax(0, 1fr);
        width: 100vw;
        height: 100vh;
        padding: 0;
        margin: 0;
    }

    .container {
        /* display: grid; */
        /* grid-template-columns: 20% 20% 1fr; */
        width: 100%;
        height: 100%;
        padding: 0;
        margin: 0;
    }

    .top_pane {
    }

    .main_pane {
    }

    .paine_pane {
    }

    .paine_pane {
    }
</style>
