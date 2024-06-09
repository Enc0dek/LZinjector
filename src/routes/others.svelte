<script>
    import { invoke } from "@tauri-apps/api/tauri";
    // @ts-ignore
    // @ts-ignore
    export let switchCustomTarjet;
    // @ts-ignore
    export let custom_target;
    // @ts-ignore
    export let switchAutoInject;

    function changeStateTarjet() {
        // @ts-ignore
        switchCustomTarjet();
        // @ts-ignore
        invoke("get_custom_target", {customTarget: custom_target});
    }

    function changeStateAuto() {
        // @ts-ignore
        switchAutoInject();
    }

    function hide(){
        invoke("hide_tray");
    }

</script>

<div class="container">
    <button type="button" class="hidemenubutton" on:click={hide}>Hide menu</button>
    <div class="checkboxes">
        <label for="CustomTarjet">Custom Tarjet</label>
        <input class="checkbox" type="checkbox" name="CustomTarjet" id="CustomTarjet" on:change={changeStateTarjet}>
        <br>
        <label for="AutoInject">AutoInject</label>
        <input class="checkbox" type="checkbox" name="AutoInject" id="AutoInject" on:change={changeStateAuto}>
    </div>
</div>

<style>
    .hidemenubutton {
        margin-top: 5px;
        margin-bottom: 5px;
        width: 100px;
    }

    .container {
        display: flex;
    }

    .checkboxes {
        margin-left: 50px;
    }

    .checkbox:hover{
        cursor: pointer;
    }

    .hidemenubutton:hover{
        cursor: pointer;
    }
</style>