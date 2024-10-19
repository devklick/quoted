import { useEffect, useState } from "react";
import { useGetEpisodes } from "./queries";
import Pagination from "../components/Pagination";

import styles from "./Episodes.module.scss";
import Loading from "../components/Loading";
import NavToQuotesButton from "./NavToQuotesButton";

interface EpisodesProps {
  showName: string;
  seasonNo: number;
  enabled: boolean;
}

function Episodes({ enabled: _enabled, seasonNo, showName }: EpisodesProps) {
  const [enabled, setEnabled] = useState(false);

  useEffect(() => {
    setEnabled(_enabled);
  }, [_enabled]);

  const [result, { limit, nextPage, page, previousPage, setLimit, setPage }] =
    useGetEpisodes({ showName, seasonNo, enabled });

  return (
    <div className={styles["episodes"]}>
      {result.isLoading && <Loading />}
      {result.data?.data.map(({ episodeName, episodeNo, quoteCount }) => (
        <EpisodeListItem
          key={`show-${showName}-season-${seasonNo}-episode-${episodeNo}`}
          showName={showName}
          seasonNo={seasonNo}
          episodeNo={episodeNo}
          episodeName={episodeName}
          quoteCount={quoteCount}
        />
      ))}
      <Pagination
        currentPage={page}
        hasMore={Boolean(result.data?.hasMore)}
        onNextClicked={nextPage}
        onPageNumberChanged={(page) => setPage(page)}
        onPageSizeChanged={(limit) => setLimit(limit)}
        onPreviousClicked={previousPage}
        pageSize={limit}
      />
    </div>
  );
}

interface EpisodeListItemProps {
  showName: string;
  seasonNo: number;
  episodeNo: number;
  episodeName: string | undefined;
  quoteCount: number;
}

function EpisodeListItem({
  episodeNo,
  episodeName,
  quoteCount,
  seasonNo,
  showName,
}: EpisodeListItemProps) {
  const title = [`Episode ${episodeNo}`, episodeName]
    .filter(Boolean)
    .join(" - ");

  return (
    <div className={styles["episode-list-item"]}>
      <span className={styles["episode-list-item__episode-name"]}>{title}</span>
      <NavToQuotesButton
        quoteCount={quoteCount}
        episodeNo={episodeNo}
        seasonNo={seasonNo}
        showName={showName}
      />
    </div>
  );
}

export default Episodes;
