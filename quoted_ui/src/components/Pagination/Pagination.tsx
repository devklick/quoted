import { useRef } from "react";

import Button from "../Button";

import styles from "./Pagination.module.scss";

const defaultValues = {
  pageSize: 10,
  validPageSizes: [10, 20, 50],
  maxPageNumbersDisplayed: 5,
} as const;

interface PaginationProps {
  currentPage: number;
  pageSize?: number;
  hasMore: boolean;
  maxPageNumbersDisplayed?: number;
  validPageSizes?: ReadonlyArray<number>;
  onPreviousClicked(): void;
  onNextClicked(): void;
  onPageNumberChanged(pageNumber: number): void;
  onPageSizeChanged(pageSize: number): void;
}

function getPageNumbersInScope(
  currentPage: number,
  maxPageNumbersDisplayed: number,
  hasMore: boolean,
  maxPageVisited: number
) {
  const maxPageNumber = Math.max(currentPage + Number(hasMore), maxPageVisited);
  const pageNumbers = [];
  for (
    let pageNumber = maxPageNumber - maxPageNumbersDisplayed;
    pageNumber <= maxPageNumber;
    pageNumber++
  ) {
    if (pageNumber > 0) {
      pageNumbers.push(pageNumber);
    }
  }
  return pageNumbers;
}

function Pagination({
  currentPage,
  hasMore,
  pageSize = defaultValues.pageSize,
  validPageSizes = defaultValues.validPageSizes,
  maxPageNumbersDisplayed = defaultValues.maxPageNumbersDisplayed,
  onNextClicked,
  onPageNumberChanged: onPageNumberClicked,
  onPreviousClicked,
  onPageSizeChanged,
}: PaginationProps) {
  // in the case where the user has navigated forwards through the pages,
  //then backwards, we know that there are X number of pages, even though we
  // dont get that information passed in via props.
  // Lets retain that information so we can generate the max page number
  // that we know exists
  const maxPageVisited = useRef(0);

  if (currentPage > maxPageVisited.current) {
    maxPageVisited.current = currentPage;
  }

  const pageNumbers = getPageNumbersInScope(
    currentPage,
    maxPageNumbersDisplayed,
    hasMore,
    maxPageVisited.current
  );

  function handlePageSizeChanged(pageSize: number) {
    maxPageVisited.current = 0;
    onPageSizeChanged(pageSize);
  }

  return (
    <div className={styles["pagination"]}>
      <PageNumbers
        currentPage={currentPage}
        hasMore={hasMore}
        onNextClicked={onNextClicked}
        onPageNumberClicked={onPageNumberClicked}
        onPreviousClicked={onPreviousClicked}
        pageNumbers={pageNumbers}
      />
      <PageSizes
        handlePageSizeChanged={handlePageSizeChanged}
        pageSize={pageSize}
        validPageSizes={validPageSizes}
      />
    </div>
  );
}

Pagination.defaultValues = defaultValues;

interface PageNumbersProps {
  onPreviousClicked(): void;
  currentPage: number;
  pageNumbers: Array<number>;
  onPageNumberClicked(pageNumber: number): void;
  onNextClicked(): void;
  hasMore: boolean;
}
function PageNumbers({
  currentPage,
  hasMore,
  onNextClicked,
  onPageNumberClicked,
  onPreviousClicked,
  pageNumbers,
}: PageNumbersProps) {
  return (
    <div className={styles["page-numbers"]}>
      <span>Page number</span>
      <ul className={styles["page-number-buttons"]}>
        <li key={`page-prev`}>
          <Button
            type="subtle"
            onClick={onPreviousClicked}
            disabled={currentPage === 1}
          >
            {"<"}
          </Button>
        </li>
        {pageNumbers.map((pn) => (
          <li key={`page-${pn}`}>
            <Button
              type="subtle"
              onClick={() => onPageNumberClicked(pn)}
              active={pn === currentPage}
            >
              {pn}
            </Button>
          </li>
        ))}
        <li key={`page-next`}>
          <Button type="subtle" onClick={onNextClicked} disabled={!hasMore}>
            {">"}
          </Button>
        </li>
      </ul>
    </div>
  );
}

interface PageSizesProps {
  validPageSizes: ReadonlyArray<number>;
  pageSize: number;
  handlePageSizeChanged(pageSize: number): void;
}

function PageSizes({
  handlePageSizeChanged,
  pageSize,
  validPageSizes,
}: PageSizesProps) {
  return (
    <div className={styles["page-sizes"]}>
      <span>Page size</span>
      <ul className={styles["page-size-buttons"]}>
        {validPageSizes.map((ps) => (
          <li key={`page-size-${ps}`}>
            <Button
              type="subtle"
              onClick={() => handlePageSizeChanged(ps)}
              active={ps === pageSize}
            >
              {ps}
            </Button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default Pagination;
