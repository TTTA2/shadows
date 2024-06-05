<script lang="ts">
    import * as Tabs from "$lib/components/ui/tabs/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import LogEditor from "../logEditor.svelte";
  </script>

  <Tabs.Root value="log" class="h-full grid grid-rows-[auto_minmax(0,1fr)]">
    <Tabs.List class="tabList">
      <Tabs.Trigger value="templates">テンプレート</Tabs.Trigger>
      <Tabs.Trigger value="log">応対履歴</Tabs.Trigger>
    </Tabs.List>

    <Tabs.Content class="tabContent" value="templates">
    </Tabs.Content>
    
    <Tabs.Content class="tabContent" value="log">
        <LogEditor></LogEditor>
    </Tabs.Content>

  </Tabs.Root>

  <style>

    .tabContent {
        height: 100%;
        overflow: auto;
    }

  </style>