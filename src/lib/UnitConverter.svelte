<script>
    import { invoke } from "@tauri-apps/api";
    let uomSelection;
    let leftSelection;
    let rightSelection;
    let left = 0;
    let right = 0;
  
    let measurements = [
      { id: 1, text: `Distance` },
      { id: 2, text: `Volume` },
    ];
  
    let distances = [
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
      { id: 9, text: `Cubic Meter` },
      { id: 10, text: `Cubic Foot` },
      { id: 11, text: `Cubic Inch` },
    ];
  
    let units = [...distances];
  
    let handleRightUnit = async (e) => {
      let entry = e.target.value;
      right = entry;
      let fun =  uomSelection.text == "Distance" ? "convert_distance" : "convert_volume"
      const res = await invoke(fun, {
        num: right,
        fromUom: rightSelection.text,
        toUom: leftSelection.text,
      })
        .then((e) => e)
        .catch((err) => 0);
  
      left = res;
    };
  
    let handleUomSelection = ()=>{
  
      if (uomSelection.text == "Distance"){
        units = distances 
        leftSelection = distances[0].text
        rightSelection = distances[1].text
        return
      }
      if (uomSelection.text == "Volume"){
        units = volumes
        leftSelection = volumes[0].text
        rightSelection = volumes[1].text
        return
      }
  
  
    }
  
  
  
  
  
    let handleDistanceRefresh = async () => {
  
      let fun =  uomSelection.text == "Distance" ? "convert_distance" : "convert_volume"
  
  
      const res = await invoke(fun, {
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
      let fun =  uomSelection.text == "Distance" ? "convert_distance" : "convert_volume"
      const res = await invoke(fun, {
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
    <select class="drop-down" bind:value={uomSelection}
    on:change={handleUomSelection}>
      {#each measurements as measurement}
        <option class="drop-down" value={measurement}>
          {measurement.text}
        </option>
      {/each}
    </select>
    <div class="convert-screen">
      <div class="header-wrapper">
        <div class="section">
          <input
            type="text"
            class="in-val"
            value={left}
            on:input={handleLeftUnit}
          />
          <select
            class="drop-down"
  
            bind:value={leftSelection}
            on:change={handleDistanceRefresh}
          >
          {#each units as unit}
          <option class="drop-down" value={unit}>
            {unit.text}
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
            on:input={handleRightUnit}
          />
  
          <select
            class="drop-down"
            bind:value={rightSelection}
            on:change={handleDistanceRefresh}
          >
          
            {#each units as unit}
              <option class="drop-down" value={unit}
              >
                {unit.text}
              </option>
            {/each}
          </select>
        </div>
      </div>
    </div>
  </main>
  
  <style>
    main {
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
  
    .header-wrapper {
      display: flex;
      flex-direction: column;
      text-align: center;
      justify-content: center;
      align-content: center;
    }
  
    h3 {
      position: relative;
      top: 30%;
    }
  
    input {
      font-size: 2.5rem;
      padding-bottom: 1rem;
      text-align: center;
    }
  </style>
  