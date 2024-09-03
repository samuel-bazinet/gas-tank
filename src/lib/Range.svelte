<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let fuel_capacity: string = "";
    let economy: string = "";
    let consumption_str: string = "";

    async function calculate() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

        let fuel_capacity_n = Number(fuel_capacity);
        if (Number.isNaN(fuel_capacity_n)) {
            return;
        }

        let economy_n = Number(economy);
        if (Number.isNaN(economy_n)) {
            return;
        }

        let range: number = await invoke("calculate_range", {
            fuel_capacity: fuel_capacity_n,
            economy: economy_n,
        });

        range = Math.round((range + Number.EPSILON) * 100) / 100;

        consumption_str = `You have ${range}KM of range.`;
    }
</script>

<div>
    <form class="row" on:submit|preventDefault={calculate}>
        <table>
            <tr>
                <td>
                    <input
                        id="calculate-fuel_capacity"
                        placeholder="Fuel Capacity"
                        bind:value={fuel_capacity}
                    />
                </td>
                <td> L </td>
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

    <p>{consumption_str}</p>
</div>
