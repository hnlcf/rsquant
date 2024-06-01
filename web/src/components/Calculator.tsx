import { Component, Show, createSignal } from "solid-js"

type CalculatorProps = {
  symbol: string
  price: number | undefined
}

type ClosePrice = {
  stop_loss_price: number
  take_profit_price: number
}

function compute_margin(
  side: "LONG" | "SHORT",
  price: number,
  leverage: number,
  stop_loss_percent: number,
  take_profit_percent: number
) {
  const leverage_factor = 1 / leverage

  if (side == "LONG") {
    const lower_price = (1 - stop_loss_percent * 0.01 * leverage_factor) * price
    const higher_price = (1 + take_profit_percent * 0.01 * leverage_factor) * price
    return { stop_loss_price: lower_price, take_profit_price: higher_price }
  } else {
    const lower_price = (1 - take_profit_percent * 0.01 * leverage_factor) * price
    const higher_price = (1 + stop_loss_percent * 0.01 * leverage_factor) * price
    return { stop_loss_price: higher_price, take_profit_price: lower_price }
  }
}

const Calculator: Component<CalculatorProps> = (props) => {
  const [leverage, setLeverage] = createSignal<number>(10)
  const [price, setPrice] = createSignal<number>(100)
  const [side, setSide] = createSignal<"LONG" | "SHORT">("LONG")
  const [stopLoss, setStopLoss] = createSignal<number>(10)
  const [takeProfit, setTakeProfit] = createSignal<number>(30)

  const [closePrice, setClosePrice] = createSignal<ClosePrice>()

  const updateClosePrice = () => {
    const close_price: ClosePrice = compute_margin(side(), price(), leverage(), stopLoss(), takeProfit())
    setClosePrice(close_price)
  }

  return (
    <div class="flex flex-col">
      <form class="flex flex-col p-4 shadow-md rounded-lg">
        <div class="block text-center text-xl font-bold mb-8 text-gray-900 dark:text-white">CALCULATOR</div>
        <div class="m-2">
          <label for="leverage-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
            Leverage:
          </label>
          <div class="relative flex items-center max-w-[14rem]">
            <input
              type="text"
              id="leverage-input"
              value={leverage()}
              onInput={(e) => setLeverage(parseInt(e.currentTarget.value))}
              data-input-counter
              aria-describedby="helper-text-explanation"
              class="rounded-lg bg-gray-50 border-x-0 border-gray-300 h-11 text-center text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full py-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
              placeholder="10X"
              required
            />
          </div>
        </div>

        <div class="m-2 flex flex-col">
          <label for="price-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
            Price ($):
          </label>
          <div class="relative flex items-center max-w-[14rem]">
            <input
              type="text"
              id="price-input"
              value={price()}
              onInput={(e) => setPrice(parseInt(e.currentTarget.value))}
              data-input-counter
              aria-describedby="helper-text-explanation"
              class="rounded-lg bg-gray-50 border-x-0 border-gray-300 h-11 text-center text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full py-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
              placeholder="67600"
              required
            />
          </div>
        </div>

        <div class="m-2 flex flex-col">
          <label for="side-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
            Side:
          </label>
          <select
            id="side-input"
            onInput={(e) => setSide(e.currentTarget.value as "LONG" | "SHORT")}
            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
          >
            <option>LONG</option>
            <option>SHORT</option>
          </select>
        </div>

        <div class="m-2 flex flex-col">
          <label for="stop-loss-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
            Stop Loss (%):
          </label>
          <input
            type="text"
            id="stop-loss-input"
            value={stopLoss()}
            onInput={(e) => setStopLoss(parseInt(e.currentTarget.value))}
            data-input-counter
            aria-describedby="helper-text-explanation"
            class="rounded-lg bg-gray-50 border-x-0 border-gray-300 h-11 text-center text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full py-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            placeholder="10"
            required
          />
        </div>

        <div class="m-2 flex flex-col">
          <label for="take-profit-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
            Take profit (%):
          </label>
          <input
            type="text"
            id="take-profit-input"
            value={takeProfit()}
            onInput={(e) => setTakeProfit(parseInt(e.currentTarget.value))}
            data-input-counter
            aria-describedby="helper-text-explanation"
            class="rounded-lg bg-gray-50 border-x-0 border-gray-300 h-11 text-center text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full py-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            placeholder="30"
            required
          />
        </div>

        <div class="m-2 flex flex-col">
          <button
            type="submit"
            class="font-medium tracking-tight text-white rounded-lg bg-blue-700 hover:bg-blue-800 focus:ring-blue-300 mt-4 ml-2 p-2"
            onClick={(e) => {
              e.preventDefault()
              updateClosePrice()
            }}
          >
            Submit
          </button>
        </div>
      </form>

      <Show
        when={closePrice() != undefined}
        fallback={<div class="p-4 m-2 text-center text-xl font-bold">Loading...</div>}
      >
        <div class="flex flex-col p-4 m-4 shadow-md rounded-lg">
          <p class="block mb-2 text-xl font-mono font-bold text-gray-900 dark:text-white">
            Stop Loss Price ({stopLoss()}%): {closePrice()?.stop_loss_price}
          </p>
          <p class="block mb-2 text-xl font-mono font-bold text-gray-900 dark:text-white">
            Take Profit Price ({takeProfit()}%): {closePrice()?.take_profit_price}
          </p>
        </div>
      </Show>
    </div>
  )
}

export default Calculator
