<script>
  import {invoke} from "@tauri-apps/api"
  let promise;
  let unit = 0;
  let measurements = [
    { id: 1, text: `Inch` },
    { id: 2, text: `Milimeter` },
    { id: 3, text: `Centimeter` },
    { id: 4, text: `Meter` },
    { id: 5, text: `Kilometer` },
    { id: 6, text: `Inch` },
    { id: 7, text: `Foot` },
    { id: 8, text: `Yard` },
    { id: 9, text: `Mile` },
  ];
  let selected;
  let handleUnit = (e) => {
    let entry = e.target.value;
    unit = entry;
  };
  let handleClick = ()=>{
    promise = invoke('convert_unit', {uom: selected.text, num:unit})
  }

</script>

<main class="container">
  <h1>Unit Conversion Calculator</h1>
  <div class="convert-screen">
    <div class="section">
      <input type="text" class="in-val" value={unit} on:blur={handleUnit} />

      <select class="drop-down" bind:value={selected}>
        {#each measurements as measurement}
          <option class="drop-down" value={measurement} >
            {measurement.text}
          </option>
        {/each}
      </select>
    </div>
    <div class="section">
      <button type="button" class="convert-button" on:click={handleClick}>Convert Unit</button>
    </div>
    <div class="section">
      {#if promise != null}
  {#await promise}
    <p>...waiting</p>
  {:then number}
    <p>The number is {number}</p>
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}
{/if}
    </div>
  </div>
</main>

<style>
  .convert-screen {
    display: flex;
    justify-content: space-around;
  }
  .section {
    display: flex;
    align-items: center;
  }
  .drop-down {
    color: #f6f6f6;
    background-color: #2f2f2f;
    border: none;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    width: 10em;
  }
  .in-val {
    color: #f6f6f6;
    background-color: #2f2f2f;
    border: none;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    width: 10em;
  }

  .convert-button {
    background-color: darkslateblue;
    color: #f6f6f6;
    border: none;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    height: 2.5em;
    width: 10em;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    border-radius: 4px;
  }

  p {
    color: #f6f6f6;
    font-size: 16px;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-weight: 400;
  }
</style>
