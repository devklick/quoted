import {
  GetEpisodesInSeasonRequest,
  GetEpisodesInSeasonResponse,
  GetQuotesRequest,
  GetQuotesResponse,
  GetRandomQuoteRequest,
  GetRandomQuoteResponse,
  GetSeasonsInShowRequest,
  GetSeasonsInShowResponse,
  GetShowsRequest,
  GetShowsResponse,
} from "./quoted-api-models";

import { proxy } from "./quoted-api-proxy";

// prettier-ignore
const endpoint = {
  shows: () => "/shows",
  randomQuote: () => "/quote/random",
  seasons: (showName: string) => `/show/${showName}/seasons`,
  episodes: (showName: string, seasonNo: number) => `/show/${showName}/season/${seasonNo}/episodes`,
  quotes: () => `/quotes`,
} as const;

export async function getShows(
  request: GetShowsRequest
): Promise<GetShowsResponse> {
  const result = await proxy.get<GetShowsResponse>(endpoint.shows(), {
    params: request,
  });
  return result.data;
}

export async function getSeasons(
  request: GetSeasonsInShowRequest
): Promise<GetSeasonsInShowResponse> {
  const { show, ...query } = request;
  const result = await proxy.get<GetSeasonsInShowResponse>(
    endpoint.seasons(show),
    {
      params: query,
    }
  );
  return result.data;
}

export async function getEpisodes(
  request: GetEpisodesInSeasonRequest
): Promise<GetEpisodesInSeasonResponse> {
  const { show, season, ...query } = request;
  const result = await proxy.get(endpoint.episodes(show, season), {
    params: query,
  });
  return result.data;
}

export async function getQuotes(
  request: GetQuotesRequest
): Promise<GetQuotesResponse> {
  const result = await proxy.get(endpoint.quotes(), {
    params: request,
  });
  return result.data;
}

export async function getRandomQuote(
  request: GetRandomQuoteRequest
): Promise<GetRandomQuoteResponse> {
  const result = await proxy.get(endpoint.randomQuote(), {
    params: request,
  });
  return result.data;
}
