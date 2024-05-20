import { Component, For, createSignal } from "solid-js"
import { baseUrl } from "./utils/constants"
import { SubscribeRequest, TickerApiRequest, TickerApiResponse } from "./types"

const App: Component = () => {
  const [tickerMap, setTickerMap] = createSignal<Map<string, TickerApiResponse>>(new Map())
  const [symbol, setSymbol] = createSignal("")

  const [symbolList, setSymbolList] = createSignal<string[]>([])

  const ws = new WebSocket(baseUrl)

  ws.onmessage = (event) => {
    const data: TickerApiResponse = JSON.parse(event.data)
    setTickerMap((prev) => {
      const newMap = new Map(prev)
      newMap.set(data.symbol, data)
      return newMap
    })
  }

  ws.onclose = (event) => {
    console.log("WebSocket is closed now.")
  }

  const subscribeTicker = (symbol: string) => {
    const req: SubscribeRequest = {
      symbol: symbol,
      interval: 5,
    }

    ws.send(JSON.stringify(req))
    setSymbolList([...symbolList(), symbol])
  }

  const unsubscribeTicker = (symbol: string) => {
    const req: SubscribeRequest = symbol
    ws.send(JSON.stringify(req))
    setSymbolList(symbolList().filter((s) => s !== symbol))
    setTickerMap((prev) => {
      const newMap = new Map(prev)
      newMap.delete(symbol)
      return newMap
    })
  }

  return (
    <div class="flex place-content-center place-items-center">
      <form class="flex place-content-center place-items-center">
        <input type="text" value={symbol()} onInput={(e) => setSymbol(e.currentTarget.value)} />
        <button
          type="submit"
          onClick={(e) => {
            e.preventDefault()
            subscribeTicker(symbol())
          }}
        >
          Submit
        </button>
      </form>
      <form class="flex place-content-center place-items-center">
        <input type="text" value={symbol()} onInput={(e) => setSymbol(e.currentTarget.value)} />
        <button
          type="submit"
          onClick={(e) => {
            e.preventDefault()
            unsubscribeTicker(symbol())
          }}
        >
          Submit
        </button>
      </form>
      <For each={symbolList()}>
        {(symbol) => (
          <div>
            <p>{symbol}</p>
            <p>{tickerMap().get(symbol)?.price}</p>
          </div>
        )}
      </For>
    </div>
  )
}

export default App
