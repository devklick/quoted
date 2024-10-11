# Quoted API

The Quoted API is an API written in Rust. Each endpoint (or group of endpoints) is hosted hosted as a Vercel serverless function.
At the time of writing this, it's deployed to https://quoted-delta.vercel.app, and consists of the following endpoints. 

- `/api/shows` - Lists the available shows
- `/api/quote/random` - Gets a random quote from a show/season/episode/character

Each serverless function is defined defined as a `[[bin]]` in the projects [Cargo.toml](./Cargo.toml).

## Running Locally

To run the API locally you can use the `vercel dev` command, however there's some initial setup to do first. 

### Vercel project.json

You need to have a `project.json` defined in `quoted_api/.vercel/`. Example content of this file:

```json
{
  "projectId": "<Project ID from Vercel>",
  "orgId": "Team ID from Vercel",
  "settings": {
    "createdAt": 1726774289575,
    "framework": null,
    "devCommand": null,
    "installCommand": null,
    "buildCommand": null,
    "outputDirectory": null,
    "rootDirectory": null,
    "directoryListing": false,
    "nodeVersion": "20.x"
  }
}
```

### Environment Variables

Add a `quoted_api/.env` file which defines the variables used by the function. 
```env
DATABASE_URL=postgresql://username:password@host:port/database
```

### Starting development server

With the prerequisites mentioned above added, you can now run the server with from the workspace root:

```
vercel dev --cwd quoted_api
```

### Comment on setup

I've had a lot of trouble trying to get this working, and through trial and error, this was the only approach that seems to work. 

Ideally, we'd have only one `.env` file in the root of the workspace, one `.vercel/project.json` in the root of the workspace etc, but I cant get that to work. So instead, to run the API locally, we need to define these within the `quoted_api` crate's folder and specify this folder as the current working directory when running `vercel dev`.