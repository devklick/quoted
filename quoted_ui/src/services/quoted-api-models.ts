//#region ========= Base Models =========
export interface PagedRequest {
  page: number;
  limit: number;
}

export interface PagedResponse<T> {
  page: number;
  limit: number;
  hasMore: number;
  data: Array<T>;
}

export interface ErrorDetail {
  message: string | undefined;
  key: string | undefined;
}

export type ApiResponse<T> =
  | (T & { success: true })
  | (ErrorDetail & { success: false });

//#endregion

//#region ========= Show Models =========
export type GetShowsRequest = PagedRequest & GetShowsRequestParams;
export type GetShowsResponse = PagedResponse<GetShowsResponseItem>;

interface GetShowsRequestParams {
  name?: string;
}
interface GetShowsResponseItem {
  name: string;
  quoteCount: number;
}
//#endregion

//#region ========= Season Models =========
export type GetSeasonsInShowRequest = PagedRequest &
  GetSeasonsInShowRequestRouteParams;

export type GetSeasonsInShowResponse =
  PagedResponse<GetSeasonsInShowResponseItem>;

interface GetSeasonsInShowRequestRouteParams {
  show: string;
}

interface GetSeasonsInShowResponseItem {
  seasonNo: number;
  seasonName: string | undefined;
  quoteCount: number;
}
//#endregion

//#region ========= Episode Models =========
export type GetEpisodesInSeasonRequest = PagedRequest &
  GetEpisodesInSeasonRequestParams;

export type GetEpisodesInSeasonResponse =
  PagedResponse<GetEpisodesInSeasonResponseItem>;

interface GetEpisodesInSeasonRequestParams {
  show: string;
  season: number;
}

interface GetEpisodesInSeasonResponseItem {
  episodeNo: number;
  episodeName: string | undefined;
  quoteCount: number;
}
//#endregion

//#region ========= Quote Models =========
interface GetRandomQuoteRequestParams {
  showName: string;
  seasonNo: number;
  episodeNo: number;
  characterName: string;
}
// prettier-ignore
export type GetRandomQuoteRequest =
  | Pick<GetRandomQuoteRequestParams, 'showName'>
  | Pick<GetRandomQuoteRequestParams, 'showName'|'seasonNo'>
  | Pick<GetRandomQuoteRequestParams, 'showName'|'seasonNo'|'episodeNo'>
  | Pick<GetRandomQuoteRequestParams, 'characterName'>
  | Pick<GetRandomQuoteRequestParams, 'characterName'|'showName'>
  | Pick<GetRandomQuoteRequestParams, 'characterName'|'showName'|'seasonNo'>
  | Pick<GetRandomQuoteRequestParams, 'characterName'|'showName'|'seasonNo'|'episodeNo'>
  | GetRandomQuoteRequestParams;

export type GetQuotesRequest = PagedRequest & GetQuotesRequestParams;

export type GetQuotesResponse = PagedResponse<GetQuotesResponseItem>;

export type GetRandomQuoteResponse = GetRandomQuoteResponseItem;

interface GetQuotesRequestParams {
  showName?: string;
  seasonNo?: number;
  episodeNo?: number;
}

interface GetQuotesResponseItem {
  showName: string;
  seasonNo: number;
  seasonName?: string;
  episodeNo: number;
  episodeName?: string;
  parts: Array<QuotePart>;
}

interface GetRandomQuoteResponseItem {
  showName: string;
  seasonNo: number;
  seasonName: string | undefined;
  episodeNo: number;
  episodeName: string | undefined;
  parts: Array<QuotePart>;
}

export interface QuotePart {
  characterName: string;
  order: number;
  quoteText: string;
}
//#endregion
