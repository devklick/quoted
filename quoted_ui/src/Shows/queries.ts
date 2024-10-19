import Pagination, { usePaginatedQuery } from "../components/Pagination";
import {
  getEpisodes,
  getSeasons,
  getShows,
} from "../services/quoted-api-service";
import { InitialPageParams } from "../components/Pagination/usePaginatedQuery";
import { useState } from "react";

const defaultInitialPageParams: InitialPageParams = {
  initialPage: 1,
  initialLimit: Pagination.defaultValues.pageSize,
};

export function useGetShows() {
  const [searchTerm, setSearchTerm] = useState<string | null>(null);
  const [a, b] = usePaginatedQuery({
    ...defaultInitialPageParams,
    optionsBuilder: (page, limit) => ({
      queryKey: ["shows", page, limit, searchTerm],
      queryFn: () => getShows({ limit, page, name: searchTerm ?? undefined }),
    }),
  });

  return [a, { ...b, searchShows: setSearchTerm }] as const;
}

interface UseGetSeasonsParams {
  showName: string;
  enabled: boolean;
}

export function useGetSeasons({ showName, enabled }: UseGetSeasonsParams) {
  return usePaginatedQuery({
    ...defaultInitialPageParams,
    optionsBuilder: (page, limit) => ({
      queryKey: ["show", showName, "seasons", page, limit],
      queryFn: () => getSeasons({ limit, page, show: showName }),
      enabled,
    }),
  });
}

interface UseGetEpisodesParams extends UseGetSeasonsParams {
  seasonNo: number;
}
export function useGetEpisodes({
  showName,
  seasonNo,
  enabled,
}: UseGetEpisodesParams) {
  return usePaginatedQuery({
    ...defaultInitialPageParams,
    optionsBuilder: (page, limit) => ({
      queryKey: ["show", showName, "season", seasonNo, "episodes", page, limit],
      queryFn: () =>
        getEpisodes({ limit, page, season: seasonNo, show: showName }),
      enabled,
    }),
  });
}
