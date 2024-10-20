import { RouteObject } from "react-router-dom";
import Layout from "./Layout/Layout";
import Shows from "./Shows";
import Home from "./Home";
import PageContent from "./components/Page";
import { NavItem } from "./Layout/Navigation/NavList";
import routeDefinitions, { baseRoute } from "./route-definitions";

/*
  TODO: Reset or retain scroll position based on action. 
  Currently, when navigating from one page to another (e.g. /shows to / (home)), 
  the scroll position from shows is persisted when you reach home, 
  so that if you were half way down the /shows page, you end up half way down
  the home page. This is unwanted. 

  Ideally, if navigating via a UI button/navlink, the content scroll should 
  always be reset. 

  However, if navigating forwards or backwards via browser controls, you should 
  automatically be scrolled to wherever you were on that page previously. 
  
  For example, 
    - Scroll 100px down home page
    - Navigate to shows page - auto scroll to top of scrolls page
    - Scroll 200px down shows page
    - Click browser back button - auto scroll to 100px on home page
    - Click browser forward button - auto scroll to 200px on shows page

  The <ScrollRestoration/> component seems like it should take care of this, 
  but I cant get it to work. 
  https://reactrouter.com/en/main/components/scroll-restoration
*/

const navItems: Array<NavItem> = [
  { title: "Shows", path: routeDefinitions.shows.path },
  { title: "Characters (WIP)", path: routeDefinitions.characters.path },
  { title: "Random Quote (WIP)", path: routeDefinitions.randomQuote.path },
];

const routes: Array<RouteObject> = [
  {
    path: baseRoute,
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
