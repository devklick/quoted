# Quoted UI

This project is a React app using Vite. It serves as a web front end on top of 
shows, seasons, episodes, quotes etc. 

It's hosted on vercel and can be found at https://devklick-quoted.vercel.app, and 
consumes data served by the [Quoted API](../quoted_api/).

## Running locally

Just CD into this directory and start the server:
```
cd quoted_ui && npm start
```

### Target API running locally

If you're running the API locally, and you want to run the UI locally and target 
the local API, you can do so using the `VITE_BASE_API_URL` env var. Just set this 
to the address of your local API, e.g.
```
VITE_BASE_API_URL=http://localhost:3001
```