<script lang="ts">
  import GasStation from "$lib/GasStation.svelte";
  import GasConsumption from "$lib/GasConsumption.svelte";
  import GasTripPrice from "$lib/GasTripPrice.svelte";
  import Range from "$lib/Range.svelte";

  import { tooltip } from "@svelte-plugins/tooltips";
</script>

<div class="container">

  <fieldset class="feature">
    <legend>
      <img src="/gas-tank_128.png" class="b-head" alt="Gas Station" />
    </legend>
    <div class="tt">
      <i class="fa fa-question-circle" style="font-size: 1.5rem;" use:tooltip={{ content: 'Compare the price of gas stations using their distance from your current location and how much fuel you need to pump.', arrow: false}}></i>
    </div>
    <GasStation />
  </fieldset>

  <fieldset class="feature">
    <legend>
      <img src="/fuel-coms_256.png" class="b-head" alt="Fuel Consumption" />
    </legend>
    <div class="tt">
      <i class="fa fa-question-circle" style="font-size: 1.5rem;" use:tooltip={{ content: 'Calculate the fuel usage based on the distance and fuel economy of your car.', arrow: false}}></i>
    </div>
    <GasConsumption />
  </fieldset>

  <fieldset class="feature">
    <legend>
      <img src="/fuel-cost_256.png" class="b-head" alt="Fuel Cost" />
    </legend>
    <div class="tt">
      <i class="fa fa-question-circle" style="font-size: 1.5rem;" use:tooltip={{ content: 'Calculate the price of a trip based on the distance, fuel economy, and expected gas price', arrow: false}}></i>
    </div>
    <GasTripPrice />
  </fieldset>

  <fieldset class="feature">
    <legend>
      <img src="/gas-range_256.png" class="b-head" alt="Range" />
    </legend>
    <div class="tt">
      <i class="fa fa-question-circle" style="font-size: 1.5rem;" use:tooltip={{ content: 'Calculate the range of your car based on the fuel capacity and fuel economy', arrow: false}}></i>
    </div>
    <Range />
  </fieldset>
</div>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }


  .feature {
    position: relative;
    display: inline-flex;
    border-radius: 1rem;
    border: 2px solid #a3a3a3;
    padding: 1rem;
    margin: 0.2rem;
  }

  .b-head {
    height: 4.5rem;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }

  .tt {
    position: absolute;
    top: -1.5rem;
    right: 1rem;
  }
</style>
