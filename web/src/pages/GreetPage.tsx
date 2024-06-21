import { createSignal, type Component } from "solid-js"

const GreetPage: Component = () => {
  const [greetMsg, setGreetMsg] = createSignal("Hello World")
  const [name, setName] = createSignal("")

  const greet = () => {
    setGreetMsg(`Hello ${name().toUpperCase()}`)
  }

  return (
    <div class="flex flex-col place-items-center tetx-2xl">
      <p class="m-40 text-5xl font-bold">{greetMsg()}</p>

      <form
        class="h-4/5 w-1/3 flex flex-col place-items-center"
        onSubmit={(e) => {
          e.preventDefault()
          greet()
        }}
      >
        <input
          class="w-1/2 p-2 m-2 border-2 border-gray-400 rounded-lg"
          type="text"
          placeholder="Enter your message"
          onChange={(e) => setName(e.currentTarget.value)}
        />

        <button class="w-1/2 p-2 m-2 bg-blue-400 text-white rounded-lg" type="submit">
          Submit
        </button>
      </form>
    </div>
  )
}

export default GreetPage
