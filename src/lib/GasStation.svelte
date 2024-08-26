<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let distance: string = "";
    let economy: string = "";
    let price: string = "";
    let to_fill: string = "";
    let cost_str: string[] = [];

    let headers: string[] = ["", "", ""];
    let stations: string[][] = [[...headers]];

    function addStation() {
        stations = [...stations, [...headers]];
    }

    function deleteStation(toBeDeleted: string[]) {
        stations = stations.filter((row) => row != toBeDeleted);
    }

    async function calculate() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

        cost_str = [];

        let economy_n = Number(economy);
        if (Number.isNaN(economy_n)) {
            return;
        }

        let to_fill_n = Number(to_fill);
        if (Number.isNaN(to_fill_n)) {
            return;
        }

        stations.forEach(async (station) => {
            let distance_n = Number(station[1]);
            if (Number.isNaN(distance_n)) {
                return;
            }

            let price_n = Number(station[2]);
            if (Number.isNaN(price_n)) {
                return;
            }

            let cost: number = await invoke("calculate_prices", {
                distance: distance_n,
                price: price_n,
                economy: economy_n,
                to_fill: to_fill_n,
            });

            cost = Math.round((cost + Number.EPSILON) * 100) / 100;

            // loop through all the gas stations and append this string with all the costs
            cost_str = [...cost_str, `${station[0]} will cost $${cost}.\n`];
        });
    }
</script>

<div>
    <table>
        <tr>
            <td>
                <input
                    id="calculate-fill"
                    placeholder="Amount to fill"
                    bind:value={to_fill}
                />
            </td>
            <td> L </td>
        </tr>
        <tr>
            <td>
                <input
                    id="calculate-economy"
                    placeholder="Fuel Economy"
                    bind:value={economy}
                />
            </td>
            <td> L/100KM </td>
        </tr>
        {#each stations as station}
            <tr>
                <td>
                    <input
                        id="station-name"
                        placeholder="Station Name"
                        bind:value={station[0]}
                    />
                </td>
            </tr>
            <tr>
                <td>
                    <input
                        id="calculate-distance"
                        placeholder="Distance"
                        bind:value={station[1]}
                    />
                </td>
                <td> KM </td>
            </tr>
            <tr>
                <td>
                    <input
                        id="calculate-price"
                        placeholder="Price"
                        bind:value={station[2]}
                    />
                </td>
                <td> $ </td>
            </tr>
            <tr>
                <button on:click={() => deleteStation(station)}
                    >Remove Station</button
                >
            </tr>
        {/each}
        <button on:click={addStation}>Add Station</button>

        <button on:click={calculate}>Calculate</button>
    </table>

    {#each cost_str as str}
        <p>{str}</p>
    {/each}
</div>  

<style>
    p {
        margin: .3rem;
    }
</style>
