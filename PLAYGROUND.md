# Mnemo Arcade Playground

Mnemo Arcade is a browser interface for the standalone Rust Mnemonist protocol.
The page sends each action to Rust and renders the response; it does not load
the N-API addon or a JavaScript implementation of the collections.

## Play Online

[Open Mnemo Arcade on Vercel](https://mnemo-arcade-rust.vercel.app). It exposes
the same five playable modules as the local server: Stack, Queue, LRU Cache,
Bit Vector, and SymSpell.

## How To Play

1. Choose a module from the left-hand selector.
2. Enter a value. Inputs accept JSON, so use `42`, `true`, `{"level": 3}`, or
   a quoted string such as `"checkpoint"`.
3. For LRU Cache, provide both a key and a value, then press `SET`.
4. Use the action buttons to mutate or inspect the collection.
5. Read the Rust State Buffer and Protocol Trace after every action. The reset
   icon starts a fresh collection of the selected kind.

## Run Locally

```powershell
cargo run --release --bin mnemonist -- --web
```

Open `http://127.0.0.1:8787`.

## Run In Docker

```powershell
docker build -t mnemonist-port .
docker run --rm -p 8787:8787 -e MNEMONIST_WEB_ADDR=0.0.0.0:8787 mnemonist-port --web
```

The Vercel deployment uses Rust serverless functions. The browser sends its
current command history with each action, and Rust replays it before returning
the latest result. This keeps each collection session reliable across cold
starts without moving collection state into JavaScript.
