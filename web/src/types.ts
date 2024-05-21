export type SubscribeTickerRequest = MultipleTickerApiRequest
export type SubscribeTickerResponse = TickerPrice[]

export interface MultipleTickerApiRequest {
  symbols: string[]
  interval: number
}

export interface TickerPrice {
  symbol: string
  price: number
}
