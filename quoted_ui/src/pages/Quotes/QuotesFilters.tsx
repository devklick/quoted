import useToggle from "../../hooks/useToggle";
import Select from "../../components/Select";
import { useGetEpisodes, useGetSeasons, useGetShows } from "../Shows/queries";
import { useEffect, useState } from "react";

import styles from "./QuotesFilters.module.scss";
import { SelectItem } from "../../components/Select/Select";
import { isSelectedItemObject } from "../../components/Select/Select.utils";

export type Filters = Partial<{
  showName: string;
  seasonNo: number;
  episodeNo: number;
}>;

interface QuotesFiltersProps {
  onFiltersChanged(params: Filters): void;
  initialParams: Filters;
}

export default function QuotesFilters({
  initialParams,
  onFiltersChanged,
}: QuotesFiltersProps) {
  const [open, { toggle: toggleOpen }] = useToggle({ defaultValue: false });
  const [show, setShow] = useState<string | undefined>(initialParams.showName);
  const [season, setSeason] = useState<number | undefined>(
    initialParams.seasonNo,
  );
  const [episode, setEpisode] = useState<number | undefined>(
    initialParams.episodeNo,
  );

  const [allShows, setAllShows] = useState<Array<string>>([]);
  const [allSeasons, setAllSeasons] = useState<Array<string>>([]);
  const [allEpisodes, setAllEpisodes] = useState<Array<string>>([]);

  const [shows, showsFns] = useGetShows();

  const [seasons, seasonsFns] = useGetSeasons({
    showName: show ?? "",
    enabled: !!show,
  });

  const [episodes, episodesFns] = useGetEpisodes({
    showName: show ?? "",
    seasonNo: season ?? 0,
    enabled: !!show && season !== undefined,
  });

  useEffect(() => {
    setAllShows(shows.data?.data.map((s) => s.name) ?? []);
    setAllSeasons([]);
    setAllEpisodes([]);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [shows.data?.page]);

  useEffect(() => {
    setAllSeasons((prev) => [
      ...prev,
      ...(seasons.data?.data.map((s) => s.seasonNo.toString()) ?? []),
    ]);
    setAllEpisodes([]);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [seasons.data?.page]);

  useEffect(() => {
    setAllEpisodes((prev) => [
      ...prev,
      ...(episodes.data?.data.map((s) => s.episodeNo.toString()) ?? []),
    ]);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [episodes.data?.page]);

  function handleShowChanged(item: SelectItem | undefined) {
    const showName = isSelectedItemObject(item) ? item.value : item;
    setShow(showName);
    setSeason(undefined);
    setEpisode(undefined);
    setAllSeasons([]);
    setAllEpisodes([]);
    onFiltersChanged({ showName, seasonNo: undefined, episodeNo: undefined });
  }

  function handleSeasonChanged(item: SelectItem | undefined) {
    const seasonNo = isSelectedItemObject(item)
      ? Number(item.value)
      : item
        ? Number(item)
        : undefined;
    setSeason(seasonNo);
    setEpisode(undefined);
    setAllEpisodes([]);
    onFiltersChanged({ seasonNo, episodeNo: undefined });
  }

  function handleEpisodeChanged(item: SelectItem | undefined) {
    const episodeNo = isSelectedItemObject(item)
      ? Number(item.value)
      : item
        ? Number(item)
        : undefined;
    setEpisode(episodeNo);
    onFiltersChanged({ episodeNo });
  }

  return (
    <div className={styles["quotes-filters__outer-container"]}>
      <div className={styles["quotes-filters__inner-container"]}>
        <button
          className={styles["quotes-filters__toggle"]}
          onClick={toggleOpen}
        >
          {open ? "↑" : "↓"}
        </button>
        {open && (
          <div className={styles["quotes-filters"]}>
            <Select
              placeholder="Show"
              items={allShows}
              value={show}
              onChanged={handleShowChanged}
              paginated
              loadMore={showsFns.nextPage}
              loading={shows.isLoading}
              hasMore={shows.data?.hasMore}
            />
            <Select
              placeholder="Season"
              items={allSeasons}
              value={season ? String(season) : undefined}
              onChanged={handleSeasonChanged}
              paginated
              loadMore={seasonsFns.nextPage}
              loading={seasons.isLoading}
              disabled={!show}
              hasMore={seasons.data?.hasMore}
            />
            <Select
              placeholder="Episode"
              items={allEpisodes}
              value={episode ? String(episode) : undefined}
              onChanged={handleEpisodeChanged}
              paginated
              loadMore={episodesFns.nextPage}
              loading={episodes.isLoading}
              disabled={!season}
              hasMore={episodes.data?.hasMore}
            />
          </div>
        )}
      </div>
    </div>
  );
}
