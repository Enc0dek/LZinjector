<script>
  import FileSelectorButton from "./FileSelectorButton.svelte";
  import InjectorButton from "./InjectorButton.svelte";
  import Others from "./others.svelte";
  //import { invoke } from "@tauri-apps/api/tauri";

  // async function getconfig(){
  //   try{
  //     let config = await invoke("get_config")

  //     if(config !== null){
  //       //pass  
  //     }else{
  //       //pass
  //     }
      
  //   }catch (error){
  //     alert("error getting the config")
  //   }
  // }
  
  // getconfig();
  let customtarjet = false;
  let autoinject = false;

  function switchCustomTarjet() {
      customtarjet = !customtarjet;
  }

  function switchAutoInject() {
      autoinject = !autoinject;
  }
</script>

<main class="container">
  <InjectorButton customtarjet_option={customtarjet}/>
  <Others switchCustomTarjet={switchCustomTarjet} switchAutoInject={switchAutoInject}, custom_target={customtarjet}/>
  <FileSelectorButton/>
</main>