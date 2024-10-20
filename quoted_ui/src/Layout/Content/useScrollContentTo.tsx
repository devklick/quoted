import { useEffect, useRef } from "react";

interface UseScrollContentToProps {
  elementRef?: React.RefObject<HTMLDivElement>;
  enabled?: boolean;
}

function useScrollContentTo({
  elementRef,
  enabled: _enabled,
}: UseScrollContentToProps) {
  // the element which will be scrolled
  const content = useRef(document.getElementById("content"));

  // Whether or not auto scrolling is enabled
  const enabled = useRef(_enabled);

  // Need to listen for the target element (if there is one yet) changing height,
  // so that when it changes height, we scroll it to the top of the screen
  useEffect(() => {
    if (!elementRef?.current) return;
    const observer = new ResizeObserver(() => {
      if (enabled.current) {
        scrollToElement(elementRef);
      }
    });
    observer.observe(elementRef.current);
    return () => observer.disconnect();
  }, [elementRef]);

  function scrollToElement(element: React.RefObject<HTMLDivElement>) {
    const rect = element.current?.getBoundingClientRect();

    if (!rect) return;

    content.current?.scrollTo({
      behavior: "smooth",
      left: rect.left,
      // Need to do something about this magic number 67.
      // It consists of:
      //    60px for the height of the accordion title bar
      //    + 5 px gap between accordions
      //    + 1px border around the accordion (1px top, 1px bottom)
      top: rect.top + content.current?.scrollTop - 67,
    });
  }

  function enable() {
    enabled.current = true;
  }

  function disable() {
    enabled.current = false;
  }

  return {
    scrollToElement,
    enable,
    disable,
  } as const;
}

export default useScrollContentTo;
