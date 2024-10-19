import React, { useState } from "react";

import Loading from "../components/Loading";
import Accordion from "../components/Accordion";
import Pagination from "../components/Pagination";
import Button from "../components/Button";

import Seasons from "./Seasons";
import { useGetShows } from "./queries";

import styles from "./Shows.module.scss";
import NavToQuotesButton from "./NavToQuotesButton";

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface ShowsProps {}

// eslint-disable-next-line no-empty-pattern
function Shows({}: ShowsProps) {
  const [searchTerm, setSearchTerm] = useState<string | null>(null);
  const [
    result,
    { page, limit, nextPage, previousPage, setPage, setLimit, searchShows },
  ] = useGetShows();

  function handleSearchTermChanged(e: React.ChangeEvent<HTMLInputElement>) {
    setSearchTerm(e.currentTarget.value);
  }
  function handleSearchTermKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (["Enter", "NumpadEnter"].includes(e.code)) {
      searchShows(searchTerm);
    }
  }

  return (
    <div className={styles["shows"]}>
      <h1>Shows</h1>

      <p>
        Here, you can find the shows that have been cataloged. However, note
        that not all shows listed here will have quotes cataloged yet.
      </p>

      <div className={styles["shows__search"]}>
        <span>Know which show you're looking for?</span>
        <div className={styles["shows__search-input"]}>
          <input
            onChange={handleSearchTermChanged}
            onKeyDown={handleSearchTermKeyDown}
          ></input>
          <Button
            type="primary"
            radius={{ tl: 0, bl: 0 }}
            onClick={() => searchShows(searchTerm)}
          >
            GO!
          </Button>
        </div>
      </div>

      {result.status === "pending" && <Loading />}
      {result.status === "success" && (
        <div className={styles["shows__list"]}>
          {result.data.data.map((show) => (
            <ShowListItem
              key={`show-${show.name}`}
              showName={show.name}
              quoteCount={show.quoteCount}
            />
          ))}
        </div>
      )}
      <Pagination
        currentPage={page}
        pageSize={limit}
        hasMore={Boolean(result.data?.hasMore)}
        onNextClicked={nextPage}
        onPreviousClicked={previousPage}
        onPageNumberChanged={(pageNumber) => setPage(pageNumber)}
        onPageSizeChanged={(pageSize) => setLimit(pageSize)}
      />
    </div>
  );
}

interface ShowListItemProps {
  showName: string;
  quoteCount: number;
}

function ShowListItem({ showName, quoteCount }: ShowListItemProps) {
  const [displaySeasons, setDisplaySeasons] = useState(false);

  function onExpanded() {
    setDisplaySeasons(true);
  }

  function onCollapsed() {
    setDisplaySeasons(false);
  }

  return (
    <Accordion
      title={showName}
      actionIcon={
        <NavToQuotesButton quoteCount={quoteCount} showName={showName} />
      }
      onCollapse={onCollapsed}
      onExpanded={onExpanded}
    >
      <Seasons showName={showName} enabled={displaySeasons} />
    </Accordion>
  );
}

export default Shows;
