import Pagination, { usePaginatedQuery } from "../../components/Pagination";
import { getQuotes } from "../../services/quoted-api-service";

interface UseGetQuotesParams {
  showName?: string;
  seasonNo?: number;
  episodeNo?: number;
}

function queryKey(params: UseGetQuotesParams, page: number, limit: number) {
  const { showName, seasonNo, episodeNo } = params;
  return [
    "show",
    showName,
    "season",
    seasonNo,
    "episode",
    episodeNo,
    "quotes",
    page,
    limit,
  ];
}

export function useGetQuotes(params: UseGetQuotesParams) {
  const { showName, seasonNo, episodeNo } = params;
  return usePaginatedQuery({
    initialPage: 1,
    initialLimit: Pagination.defaultValues.pageSize,
    optionsBuilder: (page, limit) => ({
      queryKey: queryKey(params, page, limit),
      queryFn: () => getQuotes({ limit, page, showName, seasonNo, episodeNo }),
    }),
  });
}
