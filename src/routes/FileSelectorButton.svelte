<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/dialog";

  export let file = "";
  
  const handleFileButton = async () => {
    try {
      const path = await open({
        multiple: false,
        filters: [{
          name: "dll",
          extensions: ["dll"]
        }]
      });

      if (typeof path === "string") {
        file = path;
      } else {
        file = "";
      }
    } catch (error) {
      console.error("Failed to select file", error);
      file = "";
    }

    invoke("selectfile", {file: file})
  };

  const handleFileInput = () =>{
    if(file != ""){
      invoke("selectfile", {file: file}).catch((error) => alert(error))
    }
  }
</script>

<div class="container">
  <button class="selectorbutton" on:click={handleFileButton}>Select</button>
  <input class="filepath" type="text" bind:value={file} on:change={handleFileInput} />
</div>

<style>
  .selectorbutton {
    margin-right: 10px;
    width: 100px;
  }

  .selectorbutton:hover{
    cursor: pointer;
  }

  .filepath {
    height: 25px;
  }

  .container {
    display: flex;
  }
</style>
