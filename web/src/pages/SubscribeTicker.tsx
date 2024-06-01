import { Component, For, createSignal, onCleanup } from "solid-js"
import { SubscribeTickerRequest, SubscribeTickerResponse } from "../types"
import { baseUrl } from "../utils/constants"
import SymbolCard from "../components/SymbolCard"
import Calculator from "../components/Calculator"

const SubscribeTicker: Component = () => {
  const [selectSymbol, setSelectSymbol] = createSignal("")
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
    <div class="relative z-0 flex h-full w-full">
      <div class="relative h-full w-2/3 flex flex-col items-center justify-center">
        <div class="flex-1 h-32 flex my-6">
          <div id="symbol-form">
            <div class="flex-1 items-center justify-center p-4 shadow-md rounded-lg m-4">
              <p class="mb-4 text-xl font-bold">SUBSCRIBE</p>
              <form class="">
                <input
                  type="text"
                  class="h-11 rounded-lg text-center"
                  placeholder="BTCUSDT"
                  value={symbol()}
                  onInput={(e) => setSymbol(e.currentTarget.value)}
                />
                <button
                  type="submit"
                  class="font-medium tracking-tight text-white rounded-lg bg-blue-700 hover:bg-blue-800 focus:ring-blue-300 ml-2 p-2"
                  onClick={(e) => {
                    e.preventDefault()
                    subscribeTicker(symbol())
                    setSymbol("")
                  }}
                >
                  Submit
                </button>
              </form>
            </div>

            <div class="flex-1 items-center justify-center p-4 shadow-md rounded-lg m-4">
              <p class="mb-4 text-xl font-bold">UNSUBSCRIBE</p>
              <form class="">
                <input
                  type="text"
                  class="h-11 rounded-lg text-center"
                  placeholder="BTCUSDT"
                  value={symbol()}
                  onInput={(e) => setSymbol(e.currentTarget.value)}
                />
                <button
                  type="submit"
                  class="font-medium tracking-tight text-white rounded-lg bg-blue-700 hover:bg-blue-800 focus:ring-blue-300 ml-2 p-2"
                  onClick={(e) => {
                    e.preventDefault()
                    unsubscribeTicker(symbol())
                    setSymbol("")
                  }}
                >
                  Submit
                </button>
              </form>
            </div>
          </div>
        </div>
        <div class="flex-1 flex justify-center">
          <div id="symbol-cards" class="container mx-auto">
            <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
              <For each={symbolList()}>
                {(symbol) => (
                  <SymbolCard
                    onClick={() => setSelectSymbol(symbol)}
                    symbol={symbol}
                    price={tickerMap().get(symbol)}
                    class="cursor-default"
                  />
                )}
              </For>
            </div>
          </div>
        </div>
      </div>

      <div class="relative h-full w-1/3 flex items-center justify-center">
        <Calculator symbol={selectSymbol()} price={tickerMap().get(selectSymbol())} />
      </div>
    </div>
  )
}

export default SubscribeTicker
