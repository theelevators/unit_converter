<script>
  import { invoke } from "@tauri-apps/api";
  let leftSelection;
  let rightSelection;
  let left = 0;
  let right = 0;

  let measurements = [
    { id: 1, text: `Distance` },
    { id: 2, text: `Volume` },
  ];


  let ditances = [
    { id: 1, text: `Inch` },
    { id: 2, text: `Milimeter` },
    { id: 3, text: `Centimeter` },
    { id: 4, text: `Meter` },
    { id: 5, text: `Kilometer` },
    { id: 6, text: `Foot` },
    { id: 7, text: `Yard` },
    { id: 8, text: `Mile` },
  ];

  let volumes = [
    { id: 1, text: `Gallon` },
    { id: 2, text: `Quart` },
    { id: 3, text: `Pint` },
    { id: 4, text: `Cup` },
    { id: 5, text: `Tablespoon` },
    { id: 6, text: `Teaspoon` },
    { id: 7, text: `Liter` },
    { id: 8, text: `Milliliter` },
    { id: 8, text: `Cubic Meter` },
    { id: 8, text: `Cubic Foot` },
    { id: 8, text: `Cubit Inch` },


  ];

  let handleRightUnit = async (e) => {
    let entry = e.target.value;
    right = entry;

    const res = await invoke("convert_distance", {
      num: right,
      fromUom: rightSelection.text,
      toUom: leftSelection.text,
    })
      .then((e) => e)
      .catch((err) => 0);

    left = res;
  };

  let handleUpdateUom = async () => {
    const res = await invoke("convert_distance", {
      num: left,
      fromUom: leftSelection.text,
      toUom: rightSelection.text,
    })
      .then((e) => e)
      .catch((err) => 0);

    right = res;
  };

  let handleLeftUnit = async (e) => {
    let entry = e.target.value;
    left = entry;

    const res = await invoke("convert_distance", {
      num: left,
      fromUom: leftSelection.text,
      toUom: rightSelection.text,
    })
      .then((e) => e)
      .catch((err) => err);

    right = res;
  };
</script>

<main class="container">
  <h1>Unit Conversion Calculator</h1>

  <div class="convert-screen">
    <div class="header-wrapper">
      <div class="section">
        <input
          type="text"
          class="in-val"
          value={left}
          on:blur={handleLeftUnit}
        />
        <select
          class="drop-down"
          bind:value={leftSelection}
          on:change={handleUpdateUom}
        >
          {#each ditances as measurement}
            <option class="drop-down" value={measurement}>
              {measurement.text}
            </option>
          {/each}
        </select>
      </div>
    </div>
    <div class="header-wrapper">
      <div class="section">
        <h3>=</h3>
      </div>
    </div>
    <div class="header-wrapper">
      <div class="section">
        <input
          type="text"
          class="in-val"
          value={right}
          on:blur={handleRightUnit}
        />

        <select
          class="drop-down"
          bind:value={rightSelection}
          on:change={handleUpdateUom}
        >
          {#each ditances as measurement}
            <option class="drop-down" value={measurement}>
              {measurement.text}
            </option>
          {/each}
        </select>
      </div>
    </div>
  </div>
</main>

<style>
  
  main{
    margin-top: 25%;
  }


  .convert-screen {
    display: flex;
    justify-content: space-around;
    height: 100%;
  }
  .section {
    display: flex;
    width: 100%;
    align-items: center;
    flex-direction: column;
    justify-content: center;
    align-content: center;
  }
  .drop-down {
    color: #f6f6f6;
    background-color: #2f2f2f;
    border: none;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    width: 50%;
    margin-top: 1rem;

  }
  .in-val {
    color: #f6f6f6;
    background-color: #2f2f2f;
    border: none;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    line-height: 24px;
    font-weight: 400;
    width: 50%;


  }
  
  .header-wrapper{
    display: flex;
    flex-direction: column;
    text-align: center;
    justify-content: center;
    align-content: center;
  }

  h3{
    position: relative;
    top: 30%;
  }


  input{
    font-size: 2.5rem;
    padding-bottom: 1rem;
    text-align: center;
  }

</style>
