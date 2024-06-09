<script>
  // @ts-ignore
  export let customtarjet_option = false;
  import { invoke } from "@tauri-apps/api/tauri";
  let process = "Minecraft.Windows.exe";
  const defaultProcess = "Minecraft.Windows.exe"


  function inject(){
    // invoke("save_config").catch((error) => alert(error))
    invoke("inject", {process: process, custom_target: customtarjet_option}).catch((error) => alert(error))
  }

  function ChangeProcess(){
    process = defaultProcess
  }

  $: if(!customtarjet_option) {
    ChangeProcess();
  }

  function set_process(){
    invoke("get_process", {process: process})
  }

  set_process()
  
</script>

<div class="container">
  <button class="InjectorButton" on:click={inject}> Inject </button>

  {#if !customtarjet_option}
    <input type="text" class="processReadOnly" bind:value={process} readonly />
  {:else}
    <input type="text" class="process" bind:value={process} on:change={set_process}/>
  {/if}
</div>

<style>
  .process {
    text-align: center;
  }

  .processReadOnly {
    text-align: center;
  }

  .processReadOnly:hover {
    cursor: not-allowed;
    text-align: center;
  }

  .InjectorButton {
    width: 100px;
    height: 40px;
    margin-right: 10px;
  }

  .InjectorButton:hover {
    cursor: pointer;
  }

  .container {
    display: flex;
  }
</style>
