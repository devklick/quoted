import { useEffect, useState } from "react";
import Pagination from "../components/Pagination";
import Loading from "../components/Loading";
import Accordion from "../components/Accordion";

import Episodes from "./Episodes";
import { useGetSeasons } from "./queries";

import styles from "./Seasons.module.scss";
import NavToQuotesButton from "./NavToQuotesButton";

interface SeasonsProps {
  showName: string;
  getSeasonsEnabled: boolean;
}

function Seasons({ showName, getSeasonsEnabled }: SeasonsProps) {
  const [enabled, setEnabled] = useState(false);

  useEffect(() => {
    setEnabled(getSeasonsEnabled);
  }, [getSeasonsEnabled]);

  const [result, { limit, nextPage, page, previousPage, setLimit, setPage }] =
    useGetSeasons({ showName, enabled });

  return (
    <div className={styles["seasons"]}>
      {result.isLoading && <Loading />}
      {result.isSuccess &&
        result.data.data.map(({ seasonName, seasonNo, quoteCount }) => (
          <SeasonListItem
            key={`show-${showName}-season-${seasonNo}`}
            showName={showName}
            seasonName={seasonName}
            seasonNo={seasonNo}
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

interface SeasonListItemProps {
  showName: string;
  seasonNo: number;
  seasonName: string | undefined;
  quoteCount: number;
}

function SeasonListItem({
  showName,
  seasonName,
  seasonNo,
  quoteCount,
}: SeasonListItemProps) {
  const [displayEpisodes, setDisplayEpisodes] = useState(false);

  function onExpanded() {
    setDisplayEpisodes(true);
  }

  function onCollapsed() {
    setDisplayEpisodes(false);
  }

  const title = [`Season ${seasonNo}`, seasonName].filter(Boolean).join(" - ");

  return (
    <Accordion
      title={title}
      actionIcon={
        <NavToQuotesButton
          quoteCount={quoteCount}
          seasonNo={seasonNo}
          showName={showName}
        />
      }
      onCollapse={onCollapsed}
      onExpanded={onExpanded}
    >
      <Episodes
        showName={showName}
        seasonNo={seasonNo}
        getEpisodesEnabled={displayEpisodes}
      />
    </Accordion>
  );
}

export default Seasons;
