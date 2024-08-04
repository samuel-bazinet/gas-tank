<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let distance: string = "";
    let economy: string = "";
    let price: string = "";
    let cost_str: string = "";

    async function calculate() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

        let distance_n = Number(distance);
        if (Number.isNaN(distance_n)) {
            alert("Distance isn't a number");
            return;
        }

        let economy_n = Number(economy);
        if (Number.isNaN(economy_n)) {
            alert("Economy isn't a number");
            return;
        }

        let price_n = Number(price);
        if (Number.isNaN(price_n)) {
            alert("Price isn't a number");
            return;
        }

        let cost: number = await invoke("calculate_trip_cost", {
            distance: distance_n,
            price: price_n,
            economy: economy_n,
        });

        cost = Math.round((cost + Number.EPSILON) * 100) / 100;

        cost_str = `Your trip will cost $${cost}.`;
    }
</script>

<div>

    <h3>Gas cost for a trip</h3>

    <form class="row" on:submit|preventDefault={calculate}>
        
        <input
            id="calculate-distance"
            placeholder="Enter distance..."
            bind:value={distance}
        />
        <input
            id="calculate-price"
            placeholder="Enter price..."
            bind:value={price}
        />
        <input
            id="calculate-economy"
            placeholder="Enter economy..."
            bind:value={economy}
        />
        <button type="submit">Calculate</button>
    </form>

    <p>{cost_str}</p>
</div>
