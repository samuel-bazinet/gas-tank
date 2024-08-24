<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let distance: string = "";
    let economy: string = "";
    let price: string = "";
    let to_fill: string = "";
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

        let to_fill_n = Number(to_fill);
        if (Number.isNaN(to_fill_n)) {
            alert("Amount to fill isn't a number");
            return;
        }

        let cost: number = await invoke("calculate_prices", {
            distance: distance_n,
            price: price_n,
            economy: economy_n,
            to_fill: to_fill_n,
        });

        cost = Math.round((cost + Number.EPSILON) * 100) / 100;

        cost_str = `Your tank will cost $${cost}.`;
    }
</script>

<div>
    <h3>Cost to fill at gas station</h3>

    <form class="row" on:submit|preventDefault={calculate}>
        <table>
            <tr>
                <td>
                    <input
                        id="calculate-fill"
                        placeholder="Enter amount to fill..."
                        bind:value={to_fill}
                    />
                </td>
                <td> L </td>
            </tr>
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
                        id="calculate-price"
                        placeholder="Enter price..."
                        bind:value={price}
                    />
                </td>
                <td> $ </td>
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

    <p>{cost_str}</p>
</div>
