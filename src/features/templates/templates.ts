import { invoke } from "@tauri-apps/api";

export type Template = {
    id: string,
    name: string,
    parentId: string,
    body: string,
}

export class TemplateDicitonary extends Map<string, Template> { }

export const getTemplatesList = async (): Promise<TemplateDicitonary> => {

    const templates: any[] = await invoke("get_templates_info");

    return new TemplateDicitonary(templates.map(t => {
        const { id, name, body } = t, parentId = t.parentId;
        return [ t.id, { id, name, parentId, body } ];
    }));
}

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