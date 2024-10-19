import {
  QueryKey,
  UndefinedInitialDataOptions,
  useQuery,
} from "@tanstack/react-query";
import { ErrorDetail } from "../../services/quoted-api-models";
import { useState } from "react";

export interface InitialPageParams {
  initialPage: number;
  initialLimit: number;
}

type UsePaginatedQueryParams<
  TQueryFnData = unknown,
  TError = ErrorDetail,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey
> = InitialPageParams & {
  optionsBuilder: (
    page: number,
    limit: number
  ) => UndefinedInitialDataOptions<TQueryFnData, TError, TData, TQueryKey>;
};

export default function usePaginatedQuery<
  TQueryFnData = unknown,
  TError = ErrorDetail,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey
>(params: UsePaginatedQueryParams<TQueryFnData, TError, TData, TQueryKey>) {
  const [page, setPage] = useState(params.initialPage);
  const [limit, setLimit] = useState(params.initialLimit);

  const options: UndefinedInitialDataOptions<
    TQueryFnData,
    TError,
    TData,
    TQueryKey
  > = {
    ...params.optionsBuilder(page, limit),
    refetchOnMount: false,
    refetchOnReconnect: false,
    staleTime: 60000 * 10,
  };

  const result = useQuery(options);

  function nextPage() {
    setPage(page + 1);
  }

  function previousPage() {
    setPage(page - 1);
  }

  return [
    result,
    {
      page,
      limit,
      nextPage,
      previousPage,
      setPage,
      setLimit,
    },
  ] as const;
}
