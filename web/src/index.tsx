/* @refresh reload */
import { render } from "solid-js/web"
import { Router, Route, A } from "@solidjs/router"

import Home from "./components/Home"
import SubscribeTicker from "./components/SubscribeTicker"

import "./index.css"

const root = document.getElementById("root")

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    "Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?"
  )
}

const App = (props: any) => (
  <>
    <nav>
      <A href="/">Home</A>
      <A href="/ticker">Ticker</A>
    </nav>
    <h1>Rsquant</h1>
    {props.children}
  </>
)

render(
  () => (
    <Router root={App}>
      <Route path="/" component={Home} />
      <Route path="/ticker" component={SubscribeTicker} />
    </Router>
  ),
  root!
)
