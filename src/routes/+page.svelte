<script lang="ts">
  import GasStation from "$lib/GasStation.svelte";
  import GasConsumption from "$lib/GasConsumption.svelte";
  import GasTripPrice from "$lib/GasTripPrice.svelte";
  import Range from "$lib/Range.svelte";
</script>

<div class="container">
  <div class="row">
    <img src="/gas-tank_256.png" class="logo" alt="Logo" />
  </div>

  <fieldset class="feature">
    <legend>
      <img src="/gas-tank_128.png" class="b-head" alt="Gas Station" />
    </legend>
    <GasStation />
  </fieldset>

  <fieldset class="feature">
    <legend>
      <img src="/fuel-coms_256.png" class="b-head" alt="Fuel Consumption" />
    </legend>
    <GasConsumption />
  </fieldset>

  <fieldset class="feature">
    <legend>
      <img src="/fuel-cost_256.png" class="b-head" alt="Fuel Cost" />
    </legend>
    <GasTripPrice />
  </fieldset>

  <fieldset class="feature">
    <legend>
      <img src="/gas-range_256.png" class="b-head" alt="Range" />
    </legend>
    <Range />
  </fieldset>
</div>

<style>
  .logo:hover {
    filter: drop-shadow(0 0 2em #e5a505);
  }

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

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .row {
    justify-content: center;
  }

  .feature {
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
</style>
