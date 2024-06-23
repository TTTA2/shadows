import { invoke } from "@tauri-apps/api";

export type Template = {
    id: string,
    name: string,
    parentId: string,
    body: string,
    path: string,
}

export class TemplateDicitonary extends Map<string, Template> { }


    export const getTemplatesList = async (): Promise<TemplateDicitonary> => {

        const t: any[] = await invoke("get_templates_file_data");

        const entries: TemplateDicitonary = new TemplateDicitonary();
        const pathes = new Map<string, string>();

        for (const entry of t) {

            const { name, contents, path, parent, kind } = entry;
            
            const id = crypto.randomUUID();
            const parentId = pathes.get(parent) ?? "";
            const template: Template = { id, parentId, body: contents, name, path};
            
            pathes.set(path, id);
            entries.set(id, template);
        }

        console.log(entries, pathes);

        // return new TemplateDicitonary(templates.map(t => {
        //     const { id, name, body } = t, parentId = t.parentId;
        //     return [ t.id, { id, name, parentId, body } ];
        // }));

        return entries;
    }

// export const getTemplatesList = async (): Promise<TemplateDicitonary> => {

//     const t: any[] = await invoke("get_templates_file_data");

//     const entries: TemplateDicitonary = new TemplateDicitonary();
//     const parentKeys = new Map<string, string>();

//     for (const entry of t) {

//         const { name, contents, path, parent } = entry;

//         if (parent && !parentKeys.has(parent)) parentKeys.set(parent, crypto.randomUUID());

//         const id = crypto.randomUUID();
//         const parentId = parent ? (parentKeys.get(parent) ?? "") : "";

//         const template: Template = { id, parentId, body: contents, name};

//         entries.set(id, template);
//     }

    

//     console.log(parentKeys);

//     // return new TemplateDicitonary(templates.map(t => {
//     //     const { id, name, body } = t, parentId = t.parentId;
//     //     return [ t.id, { id, name, parentId, body } ];
//     // }));

//     return entries;
// }

export const saveTemplate = async (template: Template, templates: TemplateDicitonary) => {
    const isUpdate = templates.get(template.id) != undefined;
    await invoke("write_template", { template, isUpdate });
    console.log("OK");
}

export const hasChildren = (targetId: string, templates: TemplateDicitonary) => {

    return Array.from(templates.values()).find(t => t.parentId == targetId) != undefined;
}

export const getChildren =  (targetId: string, templates: TemplateDicitonary) => {
    return Array.from(templates.values()).filter(t => t.parentId == targetId);
}