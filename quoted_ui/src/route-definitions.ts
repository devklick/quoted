interface RouteDefinition {
  path: string;
}

export const baseRoute = import.meta.env.BASE_URL || "/";
function defineRoute(path: string): RouteDefinition {
  return {
    path: [baseRoute, path]
      .filter(Boolean)
      .map((p) => (p.endsWith("/") ? p : p + "/"))
      .join("")
      .slice(0, -1),
  };
}

const routeDefinitions = {
  home: defineRoute(""),
  shows: defineRoute("shows"),
  quotes: defineRoute("quotes"),
} as const;

export default routeDefinitions;
