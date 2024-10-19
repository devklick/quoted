import { RouteObject } from "react-router-dom";
import Layout from "./Layout/Layout";
import Shows from "./Shows";
import Home from "./Home";
import PageContent from "./components/Page";
import { NavItem } from "./Layout/Navigation/NavList";

const basePath = import.meta.env.BASE_URL || "/";

interface RouteDefinition {
  path: string;
}

function defineRoute(path: string): RouteDefinition {
  return {
    path: [basePath, path]
      .filter(Boolean)
      .map((p) => (p.endsWith("/") ? p : p + "/"))
      .join("")
      .slice(0, -1),
  };
}

export const routeDefinitions = {
  shows: defineRoute("shows"),
  showQuotes: defineRoute("show/quotes"),
  characters: defineRoute("characters"),
  randomQuote: defineRoute("quote/random"),
} as const;

const navItems: Array<NavItem> = [
  { title: "Shows", path: routeDefinitions.shows.path },
  { title: "Characters", path: routeDefinitions.characters.path },
  { title: "Random Quote", path: routeDefinitions.randomQuote.path },
];

const routes: Array<RouteObject> = [
  {
    path: basePath,
    element: <Layout navItems={navItems} />,
    children: [
      {
        index: true,
        element: (
          <PageContent>
            <Home />
          </PageContent>
        ),
      },
      {
        path: routeDefinitions.shows.path,
        element: (
          <PageContent>
            <Shows.Page />
          </PageContent>
        ),
      },
      {
        path: routeDefinitions.showQuotes.path,
        element: (
          <PageContent>
            <Shows.Quotes.Page />
          </PageContent>
        ),
      },
      {
        path: routeDefinitions.randomQuote.path,
        element: (
          <PageContent>
            <Shows.Page />
          </PageContent>
        ),
      },
      {
        path: routeDefinitions.characters.path,
        element: (
          <PageContent>
            <Shows.Page />
          </PageContent>
        ),
      },
    ],
  },
];

export default routes;
