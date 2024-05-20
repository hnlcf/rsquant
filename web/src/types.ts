export type SubscribeTickerRequest = TickerApiRequest | string

export interface TickerApiRequest {
  symbol: string
  interval: number
}

export interface TickerApiResponse {
  symbol: string
  price: number
}
