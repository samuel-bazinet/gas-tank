<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let distance: string = "";
    let economy: string = "";
    let consumption_str: string = "";

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

        let consumption: number = await invoke("calculate_gas_consumption", {
            distance: distance_n,
            economy: economy_n,
        });

        consumption = Math.round((consumption + Number.EPSILON) * 100) / 100;

        consumption_str = `You will need ${consumption}L of gas to complete this trip.`;
    }
</script>

<div>

    <form class="row" on:submit|preventDefault={calculate}>
        <table>
            <tr>
                <td>
                    <input
                        id="calculate-distance"
                        placeholder="Enter distance..."
                        bind:value={distance}
                    />
                </td>
                <td> KM </td>
            </tr>
            <tr>
                <td>
                    <input
                        id="calculate-economy"
                        placeholder="Enter economy..."
                        bind:value={economy}
                    />
                </td>
                <td> L/100KM </td>
            </tr>
            <button type="submit">Calculate</button>
        </table>
    </form>

    <p>{consumption_str}</p>
</div>
