<script lang="ts">

    import CodeArea from "./codeArea.svelte";
    import { Svelvet, Node } from 'svelvet';
    import markdownit from 'markdown-it';

    type ElementType = "notice" | "text" | "code";

    type ElementNode = {
        type: ElementType,
        content: string,
    }

    export let source = "";

    const md = markdownit()
    $: result = md.render(source);

    console.log(result);

    const toElements = (src: string) => {

        const el: ElementNode[] = [];

        if (src == undefined) return el;

        let dmark = 0;
        let dopen = false;

        let tokens = [];

        for (const s of src) {

            if (s == "$")  {

                if (!dopen && dmark == 0 && tokens.length > 0) {
                    el.push({ content: tokens.join("").trim(), type: "text" });
                    tokens.length = 0;
                }

                dmark++;
            }
            else {
                dmark = 0;
                tokens.push(s);
            }

            if (dmark == 5) {

                if (dopen) {

                    const ntype: ElementType = 
                        tokens.length == 0 && 
                        tokens[0] == "[" && 
                        tokens[tokens.length - 1] == "]" ? "code" : "notice"

                    if (tokens.length > 0 && tokens[0] == "[" && tokens[tokens.length - 1] == "]") {
                        el.push({ content: tokens.slice(1, tokens.length - 1).join("").trim(), type: "code" });
                    }
                    else {
                        el.push({ content: tokens.join("").trim(), type: "notice" });
                    }

                    tokens.length = 0;
                }

                dopen = !dopen;
                dmark = 0;
            }
        }

        if (tokens.length > 0) {
            el.push({ content: tokens.join("").trim(), type: "text" });
        }

        return el;
    }

    $: elements = toElements(source);

    
	const mermaidStr = `A[コック]-->>[フライパン: ハンバーグを焼く]
    B[フライパン]-->>[コック : 焼き上がり]`;

</script>

<!-- <Svelvet theme="dark" width={800} height={800} editable={false}  controls>
    <Node></Node>
    <Node></Node>
    <Node></Node>
</Svelvet> -->

<div class="prose prose  p-8 my-4 mx-auto bg-gray-100 dark:bg-gray-300 viewer">
    {@html result}
</div>

<!-- <div class="viewer">

    {#each elements as e}
        
        {#if e.type == "notice"}
            <div class="text notice">{e.content}</div>
        {:else if e.type == "text"}
            <div class="text">{e.content}</div>
        {:else if e.type == "code"}
            <CodeArea text={e.content} />
        {/if}

    {/each}

</div> -->

<style>

    .viewer {
        width: 100%;
        height: 100%;
        padding: 16px;
        overflow: auto;
    }



    .text {
        padding: 8px;
        white-space: pre-wrap;
    }

    .text.notice {
        border-radius: 4px;
        background-color: #fff0b3;
    }


</style>