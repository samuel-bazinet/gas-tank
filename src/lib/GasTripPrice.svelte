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
            return;
        }

        let economy_n = Number(economy);
        if (Number.isNaN(economy_n)) {
            return;
        }

        let price_n = Number(price);
        if (Number.isNaN(price_n)) {
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

    <form class="row" on:submit|preventDefault={calculate}>
        <table>
            <tr>
                <td>
                    <input
                        id="calculate-distance"
                        placeholder="Distance"
                        bind:value={distance}
                    />
                </td>
                <td> KM </td>
            </tr>
            <tr>
                <td>
                    <input
                        id="calculate-price"
                        placeholder="Price"
                        bind:value={price}
                    />
                </td>
                <td> $ </td>
            </tr>
            <tr>
                <td>
                    <input
                        id="calculate-economy"
                        placeholder="Economy"
                        bind:value={economy}
                    />
                </td>
                <td> L/100KM </td>
            </tr>
            <button type="submit">Calculate</button>
        </table>
    </form>

    <p>{cost_str}</p>
</div>

<style>
    
</style>
