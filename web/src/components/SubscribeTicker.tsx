import { Component, For, createSignal, onCleanup } from "solid-js"
import { baseUrl } from "../utils/constants"
import { SubscribeTickerRequest, SubscribeTickerResponse } from "../types"

const SubscribeTicker: Component = () => {
  const [symbol, setSymbol] = createSignal("")
  const [symbolList, setSymbolList] = createSignal<string[]>([])
  const [tickerMap, setTickerMap] = createSignal<Map<string, number>>(new Map())

  const ws = new WebSocket(baseUrl)
  onCleanup(() => {
    if (ws.readyState == WebSocket.OPEN) ws.close()
  })

  ws.onmessage = (event) => {
    const data: SubscribeTickerResponse = JSON.parse(event.data)

    for (const ticker of data) {
      setTickerMap((prev) => {
        const newMap = new Map(prev)
        newMap.set(ticker.symbol, ticker.price)
        return newMap
      })
    }
  }

  const subscribeTicker = (symbol: string) => {
    if (!tickerMap().has(symbol) && !symbolList().includes(symbol)) {
      const symbols = [...symbolList(), symbol]
      const req: SubscribeTickerRequest = {
        symbols: symbols,
        interval: 5,
      }

      ws.send(JSON.stringify(req))
      setSymbolList(symbols)
    }
  }

  const unsubscribeTicker = (symbol: string) => {
    const symbols = symbolList().filter((s) => s !== symbol)
    const req: SubscribeTickerRequest = {
      symbols: symbols,
      interval: 5,
    }
    ws.send(JSON.stringify(req))

    setSymbolList(symbols)
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
            <p>{tickerMap().get(symbol)}</p>
          </div>
        )}
      </For>
    </div>
  )
}

export default SubscribeTicker
