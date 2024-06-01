import { Component } from "solid-js"

type SymbolCardProps = {
  symbol: string
  price: number | undefined
}

const SymbolCard: Component<SymbolCardProps> = (props) => {
  return (
    <div class="max-w-sm bg-white border border-gray-200 rounded-lg shadow-md">
      <div class="p-4">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900">{props.symbol}</h5>
        <p class="mb-3 font-normal text-gray-700">{props.symbol}</p>
        <a
          href="#"
          class="inline-flex items-center px-3 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300"
        >
          {props.price} USDT
        </a>
      </div>
    </div>
  )
}

export default SymbolCard
