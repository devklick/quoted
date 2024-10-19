import { createSearchParams, useNavigate } from "react-router-dom";
import Button from "../components/Button";
import { routeDefinitions } from "../routes";

interface NavToQuotesButtonProps {
  showName?: string;
  seasonNo?: number;
  episodeNo?: number;
  quoteCount: number;
}

function NavToQuotesButton({
  quoteCount,
  episodeNo,
  seasonNo,
  showName,
}: NavToQuotesButtonProps) {
  const nav = useNavigate();

  const params = Object.entries({
    showName,
    seasonNo,
    episodeNo,
  })
    .filter(([_, value]) => value !== undefined)
    .map(([key, value]) => [key.toString(), value?.toString()]) as Array<
    [string, string]
  >;

  const searchParams = createSearchParams(params);
  return (
    <Button
      padding={12}
      height={"auto"}
      type={quoteCount ? "secondary" : "subtle"}
      disabled={!quoteCount}
      onClick={() => nav(`${routeDefinitions.showQuotes.path}?${searchParams}`)}
    >
      {`Quotes (${quoteCount})`}
    </Button>
  );
}

export default NavToQuotesButton;
