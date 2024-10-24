import Pagination from "../../components/Pagination";
import Loading from "../../components/Loading";

import useValidatedQueryParams from "../../hooks/useValidatedQueryParams";
import { QuotePart } from "../../services/quoted-api-models";

import { quotesQueryParamsSchema } from "./schema";
import { useGetQuotes } from "./queries";

import styles from "./Quotes.module.scss";

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface QuotesProps {}

// eslint-disable-next-line no-empty-pattern
function Quotes({}: QuotesProps) {
  const queryValidation = useValidatedQueryParams(quotesQueryParamsSchema);

  const showName = queryValidation.data?.showName;
  const seasonNo = queryValidation.data?.seasonNo;
  const episodeNo = queryValidation.data?.episodeNo;

  const [result, { limit, nextPage, page, previousPage, setLimit, setPage }] =
    useGetQuotes({
      episodeNo,
      seasonNo,
      showName,
    });

  return (
    <div className={styles["quotes"]}>
      <h2>Quotes</h2>
      {result.isLoading && <Loading />}
      {result.isSuccess && (
        <ul className={styles["quotes__list"]}>
          {result.data.data.map((d) => (
            <QuoteListItem
              key={`quote-${d.showName}-${d.seasonNo}-${
                d.episodeNo
              }=${JSON.stringify(d.parts)}`}
              episodeNo={d.episodeNo}
              parts={d.parts}
              seasonNo={d.seasonNo}
              showName={d.showName}
              episodeName={d.episodeName}
              seasonName={d.seasonName}
            />
          ))}
        </ul>
      )}
      <Pagination
        currentPage={page}
        hasMore={Boolean(result.data?.hasMore)}
        onNextClicked={nextPage}
        onPageNumberChanged={setPage}
        onPageSizeChanged={setLimit}
        onPreviousClicked={previousPage}
        pageSize={limit}
      />
    </div>
  );
}

interface QuoteListItemProps {
  parts: Array<QuotePart>;
  showName: string;
  seasonNo: number;
  seasonName?: string;
  episodeNo: number;
  episodeName?: string;
}

function QuoteListItem({
  episodeNo,
  parts,
  seasonNo,
  showName,
  episodeName,
  seasonName,
}: QuoteListItemProps) {
  const buildName = (type: string, no: number, name?: string) =>
    [`${type} ${no}`, name].filter(Boolean).join(" - ");
  return (
    <li className={styles["quote-list-item"]}>
      <div className={styles["quote-list-item__title"]}>
        <div className={styles["quote-list-item__title-row"]}>
          <h4>{showName}</h4>
        </div>
        <div className={styles["quote-list-item__title-row"]}>
          <span>{buildName("Season", seasonNo, seasonName)}</span>
          <span>{buildName("Episode", episodeNo, episodeName)}</span>
        </div>
      </div>
      <ul className={styles["quote-list-item__parts-list"]}>
        {parts
          .sort((a, b) => a.order - b.order)
          .map((p) => (
            <li
              key={`quote-${showName}-${seasonNo}-${episodeNo}=${JSON.stringify(
                p
              )}`}
              className={styles["quote-list-item__parts-list-item"]}
            >
              <span
                className={
                  styles["quote-list-item__parts-list-item__quote-text"]
                }
              >
                {p.quoteText}
              </span>
              <span
                className={
                  styles["quote-list-item__parts-list-item__character"]
                }
              >
                {p.characterName}
              </span>
            </li>
          ))}
      </ul>
    </li>
  );
}
export default Quotes;
