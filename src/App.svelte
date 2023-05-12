<script>
  
  let text;
  import UnitConverter from "./lib/UnitConverter.svelte";

  let handleClick = async () => {
    navigator.clipboard.readText().then((e) => (text = e));
  };

  let grid = [...Array(100)].map((e) => Array(27));
  grid[0] = [..." abcdefghijklmnopqrstuvwxyz".split("")];
</script>

<div class="grid">
  {#each grid as row, ridx}
    <div class="row">
      {#each row as col, cidx}
        {#if ridx === 0 && cidx == 0}
          <div id="row-{ridx} col-{cidx}" class="zero-zero">
            {col}
          </div>
        {/if}
        {#if ridx === 0 && cidx >= 1}
          <div id="row-{ridx} col-{cidx}" class="col-indicator">
            <p>
              
              {col.toUpperCase()}
            </p>
          </div>
        {/if}
        {#if cidx === 0 && ridx >= 1}
          <div id="row-{ridx} col-{cidx}" class="row-indicator">{ridx}</div>
        {/if}

        {#if ridx >= 1 && cidx >= 1}
          <div id="row-{ridx} col-{cidx}" class="cell">
            <input type="text" bind:value={col} />
          </div>
        {/if}
      {/each}
    </div>
  {/each}
</div>

<style>
  .grid {
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
  }

  .row {
    display: flex;
    justify-content: space-between;
    width: 100%;
    min-height: 0.8rem;
  }
  .zero-zero {
    min-width: 16px;
  
  }
  p{
    display: flex;
    align-items: center;
    justify-content: center;
    height: 1.3rem;
    width: 100%;
    background-color: #8294c4;
    padding: 0px;
    margin: 0;
  }

  .col-indicator {
    display: flex;
    color: white;
    background-color: #8294c4;
    min-width: 70px;
    width: 100px;
    padding: 0px;
    margin: 0.0315rem;
    border: none;
    box-shadow: 0.01rem 0px 0px white;
    font-size: 0.8rem;
    align-items: center;
    justify-content: center;
    text-align: center;
  }
  .row-indicator {
    min-width: 16px;
    font-size: 0.5rem;
    align-items: center;
    justify-content: center;
    text-align: center;
  }
  .cell {
    background-color: white;
    min-width: 70px;
    color: black;
    padding: 0px;
    margin: 0.0315rem;
    border: none;
    box-shadow: 0.01rem 0px 0px gray;
    font-size: 12px;
  }
  input {
    all: unset;
    padding: 1px;
    width: 100%;
  }

</style>
